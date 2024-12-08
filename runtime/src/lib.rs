// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

use alloc::{vec, vec::Vec};
use core::u64;

use crate::interface::Balance;
use frame_support::{
    traits::VariantCountOf,
    weights::{
        constants::{RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
        ConstantMultiplier, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
    },
};
use pallet_transaction_payment::{
    FeeDetails, FungibleAdapter, Multiplier, RuntimeDispatchInfo, TargetedFeeAdjustment,
};
use polkadot_sdk::{
    polkadot_sdk_frame::{self as frame, prelude::*, runtime::prelude::*},
    sp_arithmetic::FixedPointNumber,
    *,
};
use smallvec::smallvec;
use sp_runtime::{Perbill, Perquintill};

pub mod apis;

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

// Composes the runtime by adding all the used pallets and deriving necessary types.
#[frame_construct_runtime]
mod runtime {
    /// The main runtime type.
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

    /// Mandatory system pallet that should always be included in a FRAME runtime.
    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Runtime>;

    /// Provides a way for consensus systems to set and check the onchain time.
    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp::Pallet<Runtime>;

    /// Provides the ability to keep track of balances.
    #[runtime::pallet_index(2)]
    pub type Balances = pallet_balances::Pallet<Runtime>;

    /// Provides a way to execute privileged functions.
    #[runtime::pallet_index(3)]
    pub type Sudo = pallet_sudo::Pallet<Runtime>;

    /// Provides the ability to charge for extrinsic execution.
    #[runtime::pallet_index(4)]
    pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;

    /// The Torus pallet.
    #[runtime::pallet_index(5)]
    pub type Torus = pallet_torus0::Pallet<Runtime>;
}

/// Percentage of block resources allocated to normal dispatches (75%)
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// Maximum allowed proof size for a block
const MAX_PROOF_SIZE: u64 = u64::MAX;

parameter_types! {
    /// Runtime version identifier
    pub const Version: RuntimeVersion = VERSION;

    /// Block weight limits with default configurations and 2x reference time
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::with_sensible_defaults(
            Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), MAX_PROOF_SIZE),
            NORMAL_DISPATCH_RATIO
        );

    /// Block length limits set to 5MB with normal dispatch ratio
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(10 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

    /// SS58 address format identifier
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    /// The type that defines the structure of blocks in the runtime
    type Block = Block;
    /// The type that handles all events emitted by the runtime
    type RuntimeEvent = RuntimeEvent;
    /// The type that represents the source/origin of a runtime call
    type RuntimeOrigin = RuntimeOrigin;
    /// The type that represents all possible calls/transactions in the runtime
    type RuntimeCall = RuntimeCall;
    /// Information about all pallets in the runtime
    type PalletInfo = PalletInfo;
    /// Filter that determines which calls can be made
    /// Everything means all calls are allowed
    type BaseCallFilter = frame_support::traits::Everything;
    /// Defines the data structure stored in each account
    /// Uses the Balance type from pallet_balances
    type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
    /// The SS58 prefix used to generate addresses for this chain
    /// Helps distinguish addresses between different chains
    type SS58Prefix = SS58Prefix;
    /// Contains version information about the runtime
    /// Used for runtime upgrades and compatibility
    type Version = Version;
    /// Defines the weight (computational and storage) costs of blocks and extrinsics
    /// Including base values and limits
    type BlockWeights = BlockWeights;
    /// Specifies the maximum size of a block in bytes
    type BlockLength = BlockLength;
    /// Defines the weight/cost of database operations
    /// RocksDbWeight provides weights for RocksDB operations
    type DbWeight = RocksDbWeight;
    /// Specifies how many recent block hashes to keep in storage
    /// Older block hashes are pruned when this limit is reached
    type BlockHashCount = frame_support::traits::ConstU32<768>;
}

impl pallet_balances::Config for Runtime {
    /// The means of storing the balances of an account
    type AccountStore = System;
    /// The overarching event type
    type RuntimeEvent = RuntimeEvent;
    /// The type for recording an account's reason for being unable to withdraw funds
    type RuntimeHoldReason = ();
    /// The type for recording an account's freezing reason
    type RuntimeFreezeReason = ();
    /// The type for recording account balances
    type Balance = u64;
    /// The minimum amount required to keep an account open
    /// 0.1 Unit
    type ExistentialDeposit = ConstU64<100_000_000>;
    /// The identifier for reserved tokens
    type ReserveIdentifier = ();
    /// The identifier for frozen tokens
    type FreezeIdentifier = Self::RuntimeFreezeReason;
    /// Handler for the unspent dust that gets burned
    /// If not specified, will burn the dust
    type DustRemoval = ();
    /// Maximum number of locks that can exist on an account
    type MaxLocks = ConstU32<50>;
    /// Maximum number of reserves that can exist on an account
    type MaxReserves = ConstU32<100>;
    /// Maximum number of freezes that can exist on an account
    type MaxFreezes = VariantCountOf<Self::RuntimeFreezeReason>;
    /// Weight information for the extrinsics in this pallet
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
    /// The overarching event type that will be emitted by this pallet
    type RuntimeEvent = RuntimeEvent;
    /// The type that represents all calls that can be dispatched in this runtime
    type RuntimeCall = RuntimeCall;
    /// Weight information for the extrinsics in this pallet
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

impl pallet_timestamp::Config for Runtime {
    /// The type used to store timestamps. In this case, it's an unsigned 64-bit integer.
    type Moment = u64;
    /// A hook that is called after the timestamp is set. In this case, it's empty (unit type).
    type OnTimestampSet = ();
    /// The minimum period between blocks. Set to 4000 (8000/2) milliseconds.
    /// This helps prevent timestamp manipulation by validators.
    type MinimumPeriod = frame_support::traits::ConstU64<{ 8000 / 2 }>;
    /// Weight information for the extrinsics in this pallet
    type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(35);
    /// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
    /// change the fees more rapidly. This low value causes changes to occur slowly over time.
    pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(4, 1_000);
    pub MinimumMultiplier: Multiplier = Multiplier::from(1u128);
    /// Maximum multiplier. We pick a value that is expensive but not impossibly so; it should act
    /// as a safety net.
    pub MaximumMultiplier: Multiplier = Multiplier::from(100_000u128);
}

/// Converts transaction length to fee using a polynomial formula
pub struct LengthToFee;

/// Fee adjustment mechanism that slowly adjusts transaction fees based on block fullness
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<
    R,
    TargetBlockFullness,
    AdjustmentVariable,
    MinimumMultiplier,
    MaximumMultiplier,
>;

impl WeightToFeePolynomial for LengthToFee {
    type Balance = Balance;

    /// Returns coefficients for a polynomial that converts transaction length to fee
    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        smallvec![
            WeightToFeeCoefficient {
                degree: 1,
                coeff_frac: Perbill::zero(),
                coeff_integer: currency::TRANSACTION_BYTE_FEE,
                negative: false,
            },
            WeightToFeeCoefficient {
                degree: 3,
                coeff_frac: Perbill::zero(),
                coeff_integer: 1 * currency::SUPPLY_FACTOR,
                negative: false,
            },
        ]
    }
}

/// Constants for currency denominations and fee calculations
pub mod currency {
    use crate::Balance;
    /// Scaling factor based on total token supply of 10_000_000
    pub const SUPPLY_FACTOR: Balance = 100;
    /// One kilowei (1,000 wei)
    pub const KILOWEI: Balance = 1_000;
    /// One gigawei (1,000,000,000 wei)
    pub const GIGAWEI: Balance = 1_000_000_000;
    /// Base fee per weight unit
    pub const WEIGHT_FEE: Balance = 50 * KILOWEI * SUPPLY_FACTOR / 4;
    /// Fee per transaction byte
    pub const TRANSACTION_BYTE_FEE: Balance = 1 * GIGAWEI * SUPPLY_FACTOR;
}

impl pallet_transaction_payment::Config for Runtime {
    /// The overarching event type
    type RuntimeEvent = RuntimeEvent;
    /// Handler for withdrawing and depositing transaction fees
    type OnChargeTransaction = FungibleAdapter<Balances, ()>;
    /// Multiplier for operational transactions (set to 5x)
    type OperationalFeeMultiplier = ConstU8<5>;
    /// Converts transaction weight to fee using a constant multiplier
    type WeightToFee = ConstantMultiplier<Balance, ConstU64<{ currency::WEIGHT_FEE }>>;
    /// Converts transaction length to fee using polynomial formula
    type LengthToFee = LengthToFee;
    /// Dynamic fee adjustment based on block fullness
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Runtime>;
}

impl pallet_torus0::Config for Runtime {}

type Block = frame::runtime::types_common::BlockOf<Runtime, SignedExtra>;
type Header = HeaderFor<Runtime>;

type RuntimeExecutive =
    Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

/// Some re-exports that the node side code needs to know. Some are useful in this context as well.
///
/// Other types should preferably be private.
// TODO: this should be standardized in some way, see:
// https://github.com/paritytech/substrate/issues/10579#issuecomment-1600537558
pub mod interface {
    use super::Runtime;
    use polkadot_sdk::{polkadot_sdk_frame as frame, *};

    pub type Block = super::Block;
    pub use frame::runtime::types_common::OpaqueBlock;
    pub type AccountId = <Runtime as frame_system::Config>::AccountId;
    pub type Nonce = <Runtime as frame_system::Config>::Nonce;
    pub type Hash = <Runtime as frame_system::Config>::Hash;
    pub type Balance = <Runtime as pallet_balances::Config>::Balance;
    pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
}
