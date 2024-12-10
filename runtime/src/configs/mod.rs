// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>
// Substrate and Polkadot dependencies
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, VariantCountOf},
    weights::{
        constants::{RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee, Weight,
    },
};
use frame_system::limits::{BlockLength, BlockWeights};
use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::{traits::One, Perbill};
use sp_version::RuntimeVersion;

// Local module imports
use super::{
    AccountId, Aura, Balance, Balances, Block, BlockNumber, Hash, Nonce, PalletInfo, Runtime,
    RuntimeCall, RuntimeEvent, RuntimeFreezeReason, RuntimeHoldReason, RuntimeOrigin, RuntimeTask,
    System, EXISTENTIAL_DEPOSIT, SLOT_DURATION, VERSION,
};

/// The runtime has 18 token decimals
const TOKEN_DECIMALS: u32 = 18;
/// Percentage of block resources allocated to normal dispatches (75%)
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// Maximum allowed proof size for a block
const MAX_PROOF_SIZE: u64 = u64::MAX;

// --- Frame System ---

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
    type SS58Prefix = ConstU16<888>;
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
    type BlockHashCount = ConstU32<2400>;
}

// --- Balances ---

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
    type Balance = u128;
    /// The minimum amount required to keep an account open
    /// 0.1 Unit
    type ExistentialDeposit = ConstU128<{ 10_u128.pow(TOKEN_DECIMALS) / 10 }>;
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

// --- Sudo ---

impl pallet_sudo::Config for Runtime {
    /// The overarching event type that will be emitted by this pallet
    type RuntimeEvent = RuntimeEvent;
    /// The type that represents all calls that can be dispatched in this runtime
    type RuntimeCall = RuntimeCall;
    /// Weight information for the extrinsics in this pallet
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

// --- Multisig ---

parameter_types! {
    // Base: 1 token + (88 bytes * 0.01 token)
    pub const DepositBase: Balance = 10u128.saturating_pow(TOKEN_DECIMALS)  // 1 token
        + (88 * 10u128.saturating_pow(TOKEN_DECIMALS - 2));  // 0.01 token per byte
    // Factor: (32 bytes * 0.01 token)
    pub const DepositFactor: Balance =
        32 * 10u128.saturating_pow(TOKEN_DECIMALS - 2);  // 0.01 token per byte
}

impl pallet_multisig::Config for Runtime {
    /// The overarching event type.
    type RuntimeEvent = RuntimeEvent;
    /// The overarching call type.
    type RuntimeCall = RuntimeCall;
    /// The currency mechanism that handles deposits and balances.
    type Currency = Balances;
    /// The base deposit amount required for creating a multisig transaction.
    /// Calculated as: 1 token + (88 bytes * 0.01 token)
    type DepositBase = DepositBase;
    /// The additional deposit amount required per signatory.
    /// Calculated as: 0 token + (32 bytes * 0.01 token)
    type DepositFactor = DepositFactor;
    /// The maximum number of signatories allowed for a multisig transaction.
    type MaxSignatories = ConstU32<100>;
    /// Weight information for extrinsics in this pallet.
    type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}

// --- Timestamp ---

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

// --- Transaction Payment ---

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
                coeff_integer: currency::SUPPLY_FACTOR,
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
    pub const TRANSACTION_BYTE_FEE: Balance = GIGAWEI * SUPPLY_FACTOR;
}

impl pallet_transaction_payment::Config for Runtime {
    /// The overarching event type
    type RuntimeEvent = RuntimeEvent;
    /// Handler for withdrawing and depositing transaction fees
    type OnChargeTransaction = FungibleAdapter<Balances, ()>;
    /// Multiplier for operational transactions (set to 5x)
    type OperationalFeeMultiplier = ConstU8<5>;
    /// Converts transaction weight to fee using a constant multiplier
    type WeightToFee = ConstantMultiplier<Balance, ConstU128<{ currency::WEIGHT_FEE }>>;
    /// Converts transaction length to fee using polynomial formula
    type LengthToFee = LengthToFee;
    /// Dynamic fee adjustment based on block fullness
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Runtime>;
}

// --- Aura ---

impl pallet_aura::Config for Runtime {
    /// The identifier type for an authority.
    type AuthorityId = AuraId;
    /// The way to handle disabled validators.
    /// `()` means no special handling for disabled validators.
    type DisabledValidators = ();
    /// Maximum number of authorities that can be set in the AURA consensus.
    type MaxAuthorities = ConstU32<128>;
    /// Configuration parameter to allow or disallow multiple blocks per slot.
    /// Set to false to prevent multiple blocks in the same slot.
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
    /// The duration of a slot in the AURA consensus mechanism.
    /// Uses MinimumPeriodTimesTwo for slot duration calculation.
    type SlotDuration = MinimumPeriodTimesTwo<Runtime>;
}

// --- Grandpa ---

impl pallet_grandpa::Config for Runtime {
    /// The overarching event type for the runtime.
    type RuntimeEvent = RuntimeEvent;
    /// The proof of key ownership, using Void since it's not implemented.
    type KeyOwnerProof = sp_core::Void;
    /// Maximum number of authorities that can participate in GRANDPA consensus.
    type MaxAuthorities = ConstU32<128>;
    /// Maximum number of entries in the session changes per set ID.
    /// Set to 0 as it's not being utilized.
    type MaxSetIdSessionEntries = ConstU64<0>;
    /// System for reporting equivocations.
    /// Empty implementation as it's not being utilized.
    type EquivocationReportSystem = ();
    /// Maximum number of nominators allowed per validator.
    type MaxNominators = ConstU32<200>;
    /// Weight information for the pallet
    type WeightInfo = ();
}

// --- Torus ---

impl pallet_torus0::Config for Runtime {}
