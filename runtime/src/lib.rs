#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;
use alloc::vec::Vec;
use interface::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use polkadot_sdk::{
    frame_executive, frame_support, frame_system,
    polkadot_sdk_frame::{self as frame, prelude::*, runtime::prelude::*},
    sp_arithmetic::FixedPointNumber,
    sp_core,
    sp_runtime::impl_opaque_keys,
    *,
};

pub mod apis;
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarks;
pub mod configs;
pub mod precompiles;

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
        pub grandpa: Grandpa,
    }
}

/// The runtime version.
#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("torus-runtime"),
    impl_name: create_runtime_str!("torus-runtime"),
    authoring_version: 1,
    spec_version: 0,
    impl_version: 1,
    apis: apis::RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// The signed extensions that are added to the runtime.
type SignedExtra = (
    // Checks that the sender is not the zero address.
    frame_system::CheckNonZeroSender<Runtime>,
    // Checks that the runtime version is correct.
    frame_system::CheckSpecVersion<Runtime>,
    // Checks that the transaction version is correct.
    frame_system::CheckTxVersion<Runtime>,
    // Checks that the genesis hash is correct.
    frame_system::CheckGenesis<Runtime>,
    // Checks that the era is valid.
    frame_system::CheckEra<Runtime>,
    // Checks that the nonce is valid.
    frame_system::CheckNonce<Runtime>,
    // Checks that the weight is valid.
    frame_system::CheckWeight<Runtime>,
    // Ensures that the sender has enough funds to pay for the transaction
    // and deducts the fee from the sender's account.
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Executive: handles dispatch to the various modules.
pub type RuntimeExecutive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;

// Composes the runtime by adding all the used pallets and deriving necessary types.
#[frame_construct_runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Runtime;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Runtime>;

    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp::Pallet<Runtime>;

    #[runtime::pallet_index(2)]
    pub type Aura = pallet_aura::Pallet<Runtime>;

    #[runtime::pallet_index(3)]
    pub type Grandpa = pallet_grandpa::Pallet<Runtime>;

    #[runtime::pallet_index(4)]
    pub type Balances = pallet_balances::Pallet<Runtime>;

    #[runtime::pallet_index(5)]
    pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;

    #[runtime::pallet_index(6)]
    pub type Sudo = pallet_sudo::Pallet<Runtime>;

    #[runtime::pallet_index(7)]
    pub type Multisig = pallet_multisig::Pallet<Runtime>;

    #[runtime::pallet_index(8)]
    pub type Ethereum = pallet_ethereum::Pallet<Runtime>;

    #[runtime::pallet_index(9)]
    pub type EVM = pallet_evm::Pallet<Runtime>;

    #[runtime::pallet_index(10)]
    pub type BaseFee = pallet_base_fee::Pallet<Runtime>;

    #[runtime::pallet_index(11)]
    pub type Torus0 = pallet_torus0::Pallet<Runtime>;

    #[runtime::pallet_index(12)]
    pub type Governance = pallet_governance::Pallet<Runtime>;
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
}

/// Some re-exports that the node side code needs to know. Some are useful in this context as well.
///
/// Other types should preferably be private.
// TODO: this should be standardized in some way, see:
// https://github.com/paritytech/substrate/issues/10579#issuecomment-1600537558
pub mod interface {
    use crate::RuntimeCall;

    use super::Runtime;
    use polkadot_sdk::{
        frame_system, pallet_balances,
        sp_core::H160,
        sp_runtime::{self, generic, traits::BlakeTwo256, MultiSignature},
    };

    pub type BlockNumber = u64;

    pub type Signature = MultiSignature;
    pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

    pub type UncheckedExtrinsic =
        fp_self_contained::UncheckedExtrinsic<Address, RuntimeCall, Signature, super::SignedExtra>;

    pub type CheckedExtrinsic =
        fp_self_contained::CheckedExtrinsic<AccountId, RuntimeCall, super::SignedExtra, H160>;

    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;

    pub type AccountId = <Runtime as frame_system::Config>::AccountId;
    pub type Nonce = <Runtime as frame_system::Config>::Nonce;
    pub type Hash = <Runtime as frame_system::Config>::Hash;
    pub type Balance = <Runtime as pallet_balances::Config>::Balance;
    pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
}
