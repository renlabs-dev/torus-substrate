use frame_support::PalletId;
use pallet_transaction_payment::{FungibleAdapter, Multiplier, TargetedFeeAdjustment};
use polkadot_sdk::{
    frame_support::{
        traits::VariantCountOf,
        weights::{
            ConstantMultiplier, WeightToFeeCoefficient, WeightToFeeCoefficients,
            WeightToFeePolynomial,
            constants::{RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
        },
    },
    pallet_aura::MinimumPeriodTimesTwo,
    sp_arithmetic::Perquintill,
    sp_consensus_aura::sr25519::AuthorityId as AuraId,
    sp_std::num::NonZeroU128,
};
use sp_runtime::{Perbill, Percent};

use crate::*;

pub mod eth;

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
    /// Block length limits set to 10MB with normal dispatch ratio
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(10 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    /// The default identifier used to distinguish between accounts.
    type AccountId = sp_runtime::AccountId32;
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
    /// Contains version information about the runtime
    /// Used for runtime upgrades and compatibility
    type Version = Version;
    /// Defines the weight (computational and storage) costs of blocks and
    /// extrinsics Including base values and limits
    type BlockWeights = BlockWeights;
    /// Specifies the maximum size of a block in bytes
    type BlockLength = BlockLength;
    /// Defines the weight/cost of database operations
    /// RocksDbWeight provides weights for RocksDB operations
    type DbWeight = RocksDbWeight;
    /// Specifies how many recent block hashes to keep in storage
    /// Older block hashes are pruned when this limit is reached
    type BlockHashCount = BlockHashCount;
    /// The prefix used in the SS58 address format for this chain.
    type SS58Prefix = ConstU16<42>;
}

// --- Balances ---

pub const EXISTENTIAL_DEPOSIT: u128 = as_tors(1) / 10;

impl pallet_balances::Config for Runtime {
    /// The means of storing the balances of an account
    type AccountStore = System;
    /// The overarching event type
    type RuntimeEvent = RuntimeEvent;
    /// The type for recording an account's reason for being unable to withdraw
    /// funds
    type RuntimeHoldReason = RuntimeHoldReason;
    /// The type for recording an account's freezing reason
    type RuntimeFreezeReason = ();
    /// The type for recording account balances
    type Balance = u128;
    /// The minimum amount required to keep an account open
    /// 0.1 Unit
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    /// The identifier for reserved tokens
    type ReserveIdentifier = [u8; 8];
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
    /// The type that represents all calls that can be dispatched in this
    /// runtime
    type RuntimeCall = RuntimeCall;
    /// Weight information for the extrinsics in this pallet
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

// --- Multisig ---

parameter_types! {
    // Base: 1 token + (88 bytes * 0.01 token)
    pub const DepositBase: Balance = 10u128.saturating_pow(TOKEN_DECIMALS) // 1 token
        .saturating_add(88u128.saturating_mul(10u128.saturating_pow(TOKEN_DECIMALS.saturating_sub(2))));  // 0.01 token per byte
    // Factor: (32 bytes * 0.01 token)
    pub const DepositFactor: Balance =
        32u128.saturating_mul(10u128.saturating_pow(TOKEN_DECIMALS.saturating_sub(2)));  // 0.01 token per byte
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
    /// The type used to store timestamps. In this case, it's an unsigned 64-bit
    /// integer.
    type Moment = u64;
    /// A hook that is called after the timestamp is set. In this case, it's
    /// empty (unit type).
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

/// Fee adjustment mechanism that slowly adjusts transaction fees based on block
/// fullness
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<
    R,
    TargetBlockFullness,
    AdjustmentVariable,
    MinimumMultiplier,
    MaximumMultiplier,
>;

impl WeightToFeePolynomial for LengthToFee {
    type Balance = Balance;

    /// Returns coefficients for a polynomial that converts transaction length
    /// to fee
    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        sp_std::vec![
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
        .into()
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

const fn as_tors(val: u128) -> u128 {
    val.saturating_mul(10u128.pow(TOKEN_DECIMALS))
}

parameter_types! {
    pub const DefaultDividendsParticipationWeight: Percent = Percent::from_parts(40);

    pub NamespaceBaseFee: Balance = as_tors(5);
    pub NamespaceDepositPerByte: Balance = as_tors(1);

    pub DefaultNamespacePricingConfig: pallet_torus0::namespace::NamespacePricingConfig<Runtime> = pallet_torus0::namespace::NamespacePricingConfig {
        base_fee: as_tors(3),
        deposit_per_byte: as_tors(1).saturating_div(5),

        count_midpoint: 35,
        fee_steepness: Percent::from_percent(10),
        max_fee_multiplier: 3,
    };
}

impl pallet_torus0::Config for Runtime {
    type DefaultMinValidatorStake = ConstU128<{ as_tors(50_000) }>;

    type DefaultRewardInterval = ConstU16<100>;

    type DefaultMinNameLength = ConstU16<2>;

    type DefaultMaxNameLength = ConstU16<32>;

    type DefaultMaxAgentUrlLength = ConstU16<64>;

    type DefaultMaxAllowedValidators = ConstU16<128>;

    type DefaultMaxRegistrationsPerBlock = ConstU16<10>;

    type DefaultMinAllowedStake = ConstU128<{ as_tors(5) / 10 }>;

    type DefaultMinStakingFee = ConstU8<0>;

    type DefaultMinWeightControlFee = ConstU8<4>;

    type DefaultMinBurn = ConstU128<{ as_tors(10) }>;
    type DefaultMaxBurn = ConstU128<{ as_tors(150) }>;

    type DefaultAdjustmentAlpha = ConstU64<{ u64::MAX / 2 }>;

    type DefaultTargetRegistrationsInterval = ConstU64<142>;
    type DefaultTargetRegistrationsPerInterval = ConstU16<3>;
    type DefaultMaxRegistrationsPerInterval = ConstU16<32>;

    type DefaultAgentUpdateCooldown = ConstU64<32_400>; // 3 days

    #[doc = " The storage MaxNameLength should be constrained to be no more than the value of this."]
    #[doc = " This is needed on agent::Agent to set the `name` field BoundedVec max length."]
    type MaxAgentNameLengthConstraint = ConstU32<256>;
    #[doc = " This is needed on agent::Agent to set the `address` field BoundedVec max length."]
    type MaxAgentUrlLengthConstraint = ConstU32<256>;
    type MaxAgentMetadataLengthConstraint = ConstU32<256>;

    type DefaultDividendsParticipationWeight = DefaultDividendsParticipationWeight;

    type DefaultNamespacePricingConfig = DefaultNamespacePricingConfig;

    type RuntimeEvent = RuntimeEvent;

    type Currency = Balances;
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;

    type Governance = Governance;

    type Emission = Emission0;
    type Permission0 = Permission0;

    type WeightInfo = pallet_torus0::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const GovernancePalletId: PalletId = PalletId(*b"torusgov");
    pub const DefaultTreasuryEmissionFee: Percent = Percent::from_percent(20);
    pub const MaxPenaltyPercentage: Percent = Percent::one();
    pub const DefaultProposalRewardTreasuryAllocation: Percent = Percent::from_percent(2);
}

impl pallet_governance::Config for Runtime {
    type PalletId = GovernancePalletId;

    type MinApplicationDataLength = ConstU32<2>;
    type MaxApplicationDataLength = ConstU32<256>;

    type ApplicationExpiration = ConstU64<2000>;

    type MaxPenaltyPercentage = MaxPenaltyPercentage;

    type DefaultTreasuryEmissionFee = DefaultTreasuryEmissionFee;

    type DefaultProposalCost = ConstU128<{ as_tors(10_000) }>;
    type DefaultProposalExpiration = ConstU64<75_600>;

    type DefaultAgentApplicationCost = ConstU128<{ as_tors(100) }>;
    type DefaultAgentApplicationExpiration = ConstU64<216_000>;

    type DefaultProposalRewardTreasuryAllocation = DefaultProposalRewardTreasuryAllocation;
    type DefaultMaxProposalRewardTreasuryAllocation = ConstU128<{ as_tors(10_000) }>;
    type DefaultProposalRewardInterval = ConstU64<75_600>;

    type RuntimeEvent = RuntimeEvent;

    type Currency = Balances;
    type Permission0 = Permission0;

    type WeightInfo = pallet_governance::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const DefaultEmissionRecyclingPercentage: Percent = Percent::one();
    pub const DefaultIncentivesRatio: Percent = Percent::from_parts(50);
}

#[cfg(not(feature = "testnet"))]
parameter_types! {
    // SAFETY: `NonZeroU128::new` only fails if the passed value is 0, which is not the case here.
    pub HalvingInterval: NonZeroU128 = NonZeroU128::new(as_tors(144_000_000)).unwrap();
    pub MaxSupply: NonZeroU128 = NonZeroU128::new(as_tors(144_000_000)).unwrap();
}

#[cfg(feature = "testnet")]
parameter_types! {
    // SAFETY: `NonZeroU128::new` only fails if the passed value is 0, which is not the case here.
    pub HalvingInterval: NonZeroU128 = NonZeroU128::new(as_tors(144_000_000_000)).unwrap();
    // Fun story: some funny little thing set itself a lot of torus in testnet, so we set a higher
    // max supply to keep it emitting tokens.
    pub MaxSupply: NonZeroU128 = NonZeroU128::new(as_tors(144_000_000_000)).unwrap();
}

impl pallet_emission0::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type HalvingInterval = HalvingInterval;

    type MaxSupply = MaxSupply;

    type BlockEmission = ConstU128<{ as_tors(64_000) / 10800 }>;

    type DefaultEmissionRecyclingPercentage = DefaultEmissionRecyclingPercentage;

    type DefaultIncentivesRatio = DefaultIncentivesRatio;

    type Currency = Balances;

    type Torus = Torus0;

    type Governance = Governance;

    type Permission0 = Permission0;

    type WeightInfo = pallet_emission0::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const PermissionPalletId: PalletId = PalletId(*b"torusper");

    pub const MaxControllersPerPermission: u32 = 10;
    pub const MaxRevokersPerPermission: u32 = 10;

    pub const MaxTargetsPerPermission: u32 = 100;
    pub const MaxStreamsPerPermission: u32 = 100;
    pub const MinAutoDistributionThreshold: u128 = as_tors(100);

    pub const MaxNamespacesPerPermission: u32 = 16;
    pub const MaxChildrenPerPermission: u32 = 16;
    pub const MaxCuratorSubpermissionsPerPermission: u32 = 16;
}

impl pallet_permission0::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type WeightInfo = ();

    type Currency = Balances;

    type Torus = Torus0;

    type PalletId = PermissionPalletId;

    type MaxControllersPerPermission = MaxControllersPerPermission;
    type MaxRevokersPerPermission = MaxRevokersPerPermission;

    type MaxTargetsPerPermission = MaxTargetsPerPermission;
    type MaxStreamsPerPermission = MaxStreamsPerPermission;
    type MinAutoDistributionThreshold = MinAutoDistributionThreshold;

    type MaxNamespacesPerPermission = MaxNamespacesPerPermission;
    type MaxChildrenPerPermission = MaxChildrenPerPermission;
    type MaxCuratorSubpermissionsPerPermission = MaxCuratorSubpermissionsPerPermission;
}

impl pallet_faucet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Torus = Torus0;
}
