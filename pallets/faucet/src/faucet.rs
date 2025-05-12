use crate::Vec;
use crate::{AccountIdOf, BalanceOf};
use crate::{Error, Event};
use codec::Encode;
use polkadot_sdk::frame_support::{traits::Currency, LOG_TARGET};
use polkadot_sdk::polkadot_sdk_frame::prelude::BlockNumberFor;
use polkadot_sdk::sp_core::{keccak_256, H256};
use polkadot_sdk::sp_core::{sha2_256, U256};
use polkadot_sdk::sp_runtime::{DispatchError, MultiAddress};
use polkadot_sdk::{
    frame_support::ensure,
    frame_system,
    sp_tracing::{info, trace},
};

pub fn execute<T: crate::Config>(
    key: AccountIdOf<T>,
    block_number: u64,
    nonce: u64,
    work: Vec<u8>,
) -> crate::DispatchResult {
    if <frame_system::Pallet<T>>::account(&key) == Default::default() {
        <frame_system::Pallet<T>>::inc_providers(&key);
    }

    info!("do faucet with key: {key:?} and block number: {block_number} and nonce: {nonce}");

    let current_block_number: u64 = frame_system::Pallet::<T>::block_number()
        .try_into()
        .map_err(|_| "block number exceeded u64")?;

    ensure!(
        block_number <= current_block_number,
        Error::<T>::InvalidWorkBlock
    );
    ensure!(
        current_block_number.saturating_sub(block_number) < 3,
        Error::<T>::InvalidWorkBlock
    );

    let difficulty: U256 = U256::from(1_000_000);
    let work_hash: H256 = H256::from_slice(&work);
    ensure!(
        hash_meets_difficulty(&work_hash, difficulty),
        Error::<T>::InvalidDifficulty
    );

    let seal: H256 = create_seal_hash::<T>(block_number, nonce, &key)?;
    ensure!(seal == work_hash, Error::<T>::InvalidSeal);

    let amount: u64 = 15_000_000_000;
    let amount: BalanceOf<T> = amount.try_into().map_err(|_| "Invalid amount")?;
    let _ = T::Currency::deposit_creating(&key, amount);

    info!("faucet done successfully with key: {key:?} and amount: {amount:?})");
    crate::Pallet::<T>::deposit_event(Event::<T>::Faucet(key, amount));

    Ok(())
}

pub fn hash_block_and_key<T: crate::Config>(
    block_hash_bytes: &[u8; 32],
    key: &T::AccountId,
) -> Result<H256, DispatchError> {
    let key_pubkey: MultiAddress<_, ()> = MultiAddress::Id(key.clone());
    let binding = key_pubkey.encode();

    let key_bytes = binding.get(1..).ok_or("Extrinsic panicked")?;
    let mut full_bytes = [0u8; 64];
    let (first_half, second_half) = full_bytes.split_at_mut(32);
    first_half.copy_from_slice(block_hash_bytes);
    second_half.copy_from_slice(key_bytes.get(..32).ok_or("Extrinsic panicked")?);
    let keccak_256_seal_hash_vec: [u8; 32] = keccak_256(&full_bytes[..]);

    Ok(H256::from_slice(&keccak_256_seal_hash_vec))
}

pub fn create_seal_hash<T: crate::Config>(
    block_number_u64: u64,
    nonce_u64: u64,
    hotkey: &T::AccountId,
) -> Result<H256, DispatchError> {
    let nonce = nonce_u64.to_le_bytes();
    let block_hash_at_number: H256 = get_block_hash_from_u64::<T>(block_number_u64)?;
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
            "hotkey:{hotkey:?} \nblock_number: {block_number_u64:?}, \nnonce_u64: {nonce_u64:?}, \nblock_hash: {block_hash_at_number:?}, \nfull_bytes: {full_bytes:?}, \nsha256_seal_hash_vec: {sha256_seal_hash_vec:?},  \nkeccak_256_seal_hash_vec: {keccak_256_seal_hash_vec:?}, \nseal_hash: {seal_hash:?}",
        );

    Ok(seal_hash)
}

pub fn get_block_hash_from_u64<T: crate::Config>(block_number: u64) -> Result<H256, DispatchError> {
    let block_number: BlockNumberFor<T> = block_number.try_into().map_err(|_| {
        "Block number {block_number} is too large to be converted to BlockNumberFor<T>"
    })?;
    let block_hash_at_number = frame_system::Pallet::<T>::block_hash(block_number);
    let vec_hash: Vec<u8> = block_hash_at_number.as_ref().to_vec();
    let real_hash: H256 = H256::from_slice(&vec_hash);

    trace!(
        target: LOG_TARGET,
        "block_number: vec_hash: {vec_hash:?}, real_hash: {real_hash:?}",
    );

    Ok(real_hash)
}

pub fn hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
    let bytes: &[u8] = hash.as_bytes();
    let num_hash: U256 = U256::from(bytes);
    let (value, overflowed) = num_hash.overflowing_mul(difficulty);

    trace!(
        target: LOG_TARGET,
        "Difficulty: hash: {hash:?}, hash_bytes: {bytes:?}, hash_as_num: {num_hash:?}, difficulty: {difficulty:?}, value: {value:?} overflowed: {overflowed:?}",
    );
    !overflowed
}
