//! # proof-of-work
//!
//! The following steps are required to generate a valid proof-of-work:
//! - Concatenate the current block hash with the receiver key bytes and hash the result using keccak256. (reference [hash_block_and_key])
//! - Concatenate a random nonce with the result of the previous step, hash it with sha256 and again with keccak256. (reference [create_seal_hash])
//! - Transform the result hash to a u128 number and multiply it by 1_000_000. (reference [hash_meets_difficulty])
//!     - If the multiplication result exceeds the maximum possible u128 value, the hash is invalid. Try again with a new random nonce and updated block hash.
//!     - If it doesn't, the proof-of-work is valid and the result may be submitted.
//!
//! ## Pseudocode
//! ```x
//! u128 difficulty = 1_000_000;
//! while true {
//!     u64 nonce = random_u64();
//!     u8[] block_hash = current_block_hash;
//!
//!     // This should result in a byte array with size 64 (trim the end of key bytes if necessary)
//!     u8[] block_key_bytes = concat(block_hash, key_bytes);
//!     u8[] block_key_hash = keccak_256(block_key_bytes);
//!     
//!     // This should result in a byte array with size 40 (8 nonce bytes + 32 block_key_hash bytes)
//!     u8[] seal_bytes = concat(to_byte_array(nonce), block_key_hash);
//!     u8[] seal_hash = sha_256(keccak_256(seal_bytes));
//!         
//!     // Overflowing means that the multiplication result exceeded the max value possible for the type, u128 on this case.
//!     if overflows(as_u128(seal_hash) * difficulty) {
//!         // The seal hash doesn't meet the difficulty requirement, change the nonce and try again.
//!         continue;
//!     }
//!
//!     // The generated seal hash meets the desired difficulty and may be submitted.
//! }
//! ```

use crate::Vec;
use crate::{AccountIdOf, BalanceOf};
use crate::{Error, Event};
use codec::Encode;
use polkadot_sdk::frame_support::{LOG_TARGET, traits::Currency};
use polkadot_sdk::polkadot_sdk_frame::prelude::BlockNumberFor;
use polkadot_sdk::sp_core::{H256, keccak_256};
use polkadot_sdk::sp_core::{U256, sha2_256};
use polkadot_sdk::sp_runtime::{DispatchError, MultiAddress};
use polkadot_sdk::{
    frame_support::ensure,
    frame_system,
    sp_tracing::{info, trace},
};

const FAUCET_AMOUNT: u128 = 50_000_000_000_000_000_000;

pub fn execute<T: crate::Config>(
    key: AccountIdOf<T>,
    block_number: u64,
    nonce: u64,
    work: Vec<u8>,
) -> crate::DispatchResult {
    // Ensure the account exists in the system
    if <frame_system::Pallet<T>>::account(&key) == Default::default() {
        <frame_system::Pallet<T>>::inc_providers(&key);
    }

    info!(
        "do faucet with key: {key:?} and block number: {block_number} and nonce: {nonce} and hash: {work:?}"
    );

    let current_block_number: u64 = frame_system::Pallet::<T>::block_number()
        .try_into()
        .map_err(|_| "block number exceeded u64")?;

    // Ensure the block number is not in the future
    ensure!(
        block_number <= current_block_number,
        Error::<T>::InvalidWorkBlock
    );

    // Ensure the block is recent (less than 3 blocks old)
    ensure!(
        current_block_number.saturating_sub(block_number) < 3,
        Error::<T>::InvalidWorkBlock
    );

    // Validate the proof-of-work difficulty
    let difficulty: U256 = U256::from(1_000_000);
    let work_hash: H256 = H256::from_slice(&work);
    ensure!(
        hash_meets_difficulty(&work_hash, difficulty),
        Error::<T>::InvalidDifficulty
    );

    // Verify that the submitted work hash matches the expected seal hash
    let seal: H256 = create_seal_hash::<T>(block_number, nonce, &key)?;
    ensure!(seal == work_hash, Error::<T>::InvalidSeal);

    // Award tokens to the account
    let amount: BalanceOf<T> = FAUCET_AMOUNT.try_into().map_err(|_| "Invalid amount")?;
    let _ = T::Currency::deposit_creating(&key, amount);

    // Log success and emit event
    info!("faucet done successfully with key: {key:?} and amount: {amount:?})");
    crate::Pallet::<T>::deposit_event(Event::<T>::Faucet(key, amount));

    Ok(())
}

/// Creates a hash combining the block hash and account key
///
/// This function combines a block hash with an account key and produces a new hash.
/// It takes the 32-byte block hash and combines it with the first 32 bytes of the
/// account ID to create a 64-byte array, then hashes it with keccak-256.
pub fn hash_block_and_key<T: crate::Config>(
    block_hash_bytes: &[u8; 32],
    key: &T::AccountId,
) -> Result<H256, DispatchError> {
    let key_pubkey: MultiAddress<_, ()> = MultiAddress::Id(key.clone());
    let binding = key_pubkey.encode();

    // Skip the first byte of the encoded key (which is a type indicator)
    let key_bytes = binding.get(1..).ok_or("Key is smaller than 1 byte")?;

    let mut full_bytes = [0u8; 64];
    let (first_half, second_half) = full_bytes.split_at_mut(32);

    first_half.copy_from_slice(block_hash_bytes);

    second_half.copy_from_slice(key_bytes.get(..32).ok_or("Key is smaller than 32 bytes")?);

    let keccak_256_seal_hash_vec: [u8; 32] = keccak_256(&full_bytes[..]);

    Ok(H256::from_slice(&keccak_256_seal_hash_vec))
}

/// Creates the seal hash used for proof-of-work verification
///
/// This function generates the hash that users need to match with their proof-of-work.
/// The process is:
/// 1. Get the hash of the specified block
/// 2. Concatenate the block hash with the account key
/// 3. Concatenate the nonce with the result from step 2
/// 4. Hash the combined data with SHA-256
/// 5. Hash the result with keccak-256
pub fn create_seal_hash<T: crate::Config>(
    block_number: u64,
    nonce: u64,
    hotkey: &T::AccountId,
) -> Result<H256, DispatchError> {
    let nonce = nonce.to_le_bytes();

    let block_hash_at_number: H256 = get_block_hash_from_u64::<T>(block_number)?;
    let block_hash_bytes: &[u8; 32] = block_hash_at_number.as_fixed_bytes();

    let binding = hash_block_and_key::<T>(block_hash_bytes, hotkey)?;
    let block_and_hotkey_hash_bytes: &[u8; 32] = binding.as_fixed_bytes();

    let mut full_bytes = [0u8; 40];
    let (first_chunk, second_chunk) = full_bytes.split_at_mut(8);
    first_chunk.copy_from_slice(&nonce);
    second_chunk.copy_from_slice(block_and_hotkey_hash_bytes);

    let sha256_seal_hash_vec: [u8; 32] = sha2_256(&full_bytes[..]);
    let keccak_256_seal_hash_vec: [u8; 32] = keccak_256(&sha256_seal_hash_vec);
    let seal_hash: H256 = H256::from_slice(&keccak_256_seal_hash_vec);

    trace!(
        "hotkey:{hotkey:?} \nblock_number: {block_number:?}, \nnonce_u64: {nonce:?}, \nblock_hash: {block_hash_at_number:?}, \nfull_bytes: {full_bytes:?}, \nsha256_seal_hash_vec: {sha256_seal_hash_vec:?},  \nkeccak_256_seal_hash_vec: {keccak_256_seal_hash_vec:?}, \nseal_hash: {seal_hash:?}",
    );

    Ok(seal_hash)
}

pub fn get_block_hash_from_u64<T: crate::Config>(block_number: u64) -> Result<H256, DispatchError> {
    let block_number: BlockNumberFor<T> = block_number.try_into().map_err(
        |_| "Block number {block_number} is too large to be converted to BlockNumberFor<T>",
    )?;

    let block_hash_at_number = frame_system::Pallet::<T>::block_hash(block_number);

    let vec_hash: Vec<u8> = block_hash_at_number.as_ref().to_vec();
    let real_hash: H256 = H256::from_slice(&vec_hash);

    trace!(
        target: LOG_TARGET,
        "block_number: vec_hash: {vec_hash:?}, real_hash: {real_hash:?}",
    );

    Ok(real_hash)
}

/// Checks if a hash meets the required difficulty for proof-of-work
///
/// This function verifies that a hash meets the difficulty criteria by converting the
/// hash to a U256 value and checking if multiplying it by the difficulty overflows.
/// If there's no overflow, the hash meets the required difficulty.
pub fn hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
    let bytes: &[u8] = hash.as_bytes();
    let num_hash: U256 = U256::from(bytes);

    // Multiply the hash value by the difficulty
    // If it overflows, the hash doesn't meet the difficulty requirement
    let (value, overflowed) = num_hash.overflowing_mul(difficulty);

    trace!(
        target: LOG_TARGET,
        "Difficulty: hash: {hash:?}, hash_bytes: {bytes:?}, hash_as_num: {num_hash:?}, difficulty: {difficulty:?}, value: {value:?} overflowed: {overflowed:?}",
    );

    !overflowed
}
