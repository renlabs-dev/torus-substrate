#![allow(non_camel_case_types)]

use std::{cell::RefCell, num::NonZeroU128};

pub use pallet_emission0;
pub use pallet_governance;
pub use pallet_torus0;
use pallet_torus0::MinAllowedStake;
use polkadot_sdk::{
    frame_support::{
        self,
        pallet_prelude::DispatchResult,
        parameter_types,
        traits::{Currency, Everything, Hooks},
        PalletId,
    },
    frame_system, pallet_balances,
    polkadot_sdk_frame::runtime::prelude::*,
    sp_core::{Get, H256},
    sp_io,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage, Percent,
    },
    sp_tracing,
};

#[frame_construct_runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(RuntimeCall, RuntimeEvent, RuntimeError, RuntimeOrigin)]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Runtime>;

    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances::Pallet<Runtime>;

    #[runtime::pallet_index(2)]
    pub type Torus0 = pallet_torus0::Pallet<Runtime>;

    #[runtime::pallet_index(3)]
    pub type Emission0 = pallet_emission0::Pallet<Runtime>;

    #[runtime::pallet_index(4)]
    pub type Governance = pallet_governance::Pallet<Runtime>;
}

pub type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u32;

#[allow(dead_code)]
pub type BalanceCall = pallet_balances::Call<Test>;

#[allow(dead_code)]
pub type TestRuntimeCall = frame_system::Call<Test>;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u16 = 42;
}

thread_local! {
    static DEFAULT_MIN_BURN: RefCell<u128> = const { RefCell::new(to_nano(10)) };
}

pub struct MinBurnConfig;

impl Get<u128> for MinBurnConfig {
    fn get() -> u128 {
        DEFAULT_MIN_BURN.with(|v| *v.borrow())
    }
}

// Balance of an account.
pub type Balance = u128;

// An index to a block.
pub type BlockNumber = u64;

parameter_types! {
    pub const ExistentialDeposit: Balance = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
    pub const DefaultDividendsParticipationWeight: Percent = Percent::from_parts(40);
}

impl pallet_governance_api::GovernanceApi<AccountId> for Test {
    fn dao_treasury_address() -> AccountId {
        pallet_governance::DaoTreasuryAddress::<Test>::get()
    }

    fn treasury_emission_fee() -> Percent {
        pallet_governance::TreasuryEmissionFee::<Test>::get()
    }

    fn is_whitelisted(key: &AccountId) -> bool {
        pallet_governance::Whitelist::<Test>::contains_key(key)
    }

    fn ensure_allocator(key: &AccountId) -> DispatchResult {
        pallet_governance::roles::ensure_allocator::<Test>(key)
    }

    fn set_allocator(key: &AccountId) {
        pallet_governance::Allocators::<Test>::insert(key, ());
    }
}

impl pallet_torus0::Config for Test {
    type DefaultMinValidatorStake = ConstU128<50_000_000_000_000_000_000_000>;

    type DefaultImmunityPeriod = ConstU16<0>;

    type DefaultRewardInterval = ConstU16<100>;

    type DefaultMinNameLength = ConstU16<2>;

    type DefaultMaxNameLength = ConstU16<32>;

    type DefaultMaxAgentUrlLength = ConstU16<64>;

    type DefaultMaxAllowedAgents = ConstU16<10_000>;

    type DefaultMaxAllowedValidators = ConstU16<128>;

    type DefaultMaxRegistrationsPerBlock = ConstU16<10>;

    type DefaultMinAllowedStake = ConstU128<500_000_000_000_000_000>;

    type DefaultMinStakingFee = ConstU8<5>;

    type DefaultMinWeightControlFee = ConstU8<5>;

    type DefaultMinBurn = MinBurnConfig;

    type DefaultMaxBurn = ConstU128<150_000_000_000_000_000_000>;

    type DefaultAdjustmentAlpha = ConstU64<{ u64::MAX / 2 }>;

    type DefaultTargetRegistrationsInterval = ConstU64<142>;

    type DefaultTargetRegistrationsPerInterval = ConstU16<3>;

    type DefaultMaxRegistrationsPerInterval = ConstU16<32>;

    #[doc = " The storage MaxNameLength should be constrained to be no more than the value of this."]
    #[doc = " This is needed on agent::Agent to set the `name` field BoundedVec max length."]
    type MaxAgentNameLengthConstraint = ConstU32<256>;

    #[doc = " This is needed on agent::Agent to set the `url` field BoundedVec max length."]
    type MaxAgentUrlLengthConstraint = ConstU32<256>;

    type MaxAgentMetadataLengthConstraint = ConstU32<256>;

    type DefaultDividendsParticipationWeight = DefaultDividendsParticipationWeight;

    type RuntimeEvent = RuntimeEvent;

    type Currency = Balances;

    type Governance = Governance;

    type Emission = Emission0;
}

parameter_types! {
    pub HalvingInterval: NonZeroU128 = NonZeroU128::new(to_nano(144_000_000)).unwrap();
    pub MaxSupply: NonZeroU128 = NonZeroU128::new(to_nano(144_000_000 * 4)).unwrap();
    pub const DefaultEmissionRecyclingPercentage: Percent = Percent::from_parts(70);
    pub const DefaultIncentivesRatio: Percent = Percent::from_parts(50);
}

impl pallet_emission0::Config for Test {
    type RuntimeEvent = RuntimeEvent;

    type HalvingInterval = HalvingInterval;

    type MaxSupply = MaxSupply;

    type BlockEmission = ConstU128<{ (to_nano(250_000) - 1) / 10800 }>;

    type DefaultMinAllowedWeights = ConstU16<1>;

    type DefaultMaxAllowedWeights = ConstU16<420>;

    type DefaultEmissionRecyclingPercentage = DefaultEmissionRecyclingPercentage;

    type DefaultIncentivesRatio = DefaultIncentivesRatio;

    type Currency = Balances;

    type Torus = Torus0;

    type Governance = Governance;

    type WeightInfo = pallet_emission0::weights::SubstrateWeight<Test>;
}

parameter_types! {
    pub const GovernancePalletId: PalletId = PalletId(*b"torusgov");
    pub const DefaultTreasuryEmissionFee: Percent = Percent::from_percent(20);
    pub const MaxPenaltyPercentage: Percent = Percent::one();
    pub const DefaultProposalRewardTreasuryAllocation: Percent = Percent::from_percent(2);
}

impl pallet_governance::Config for Test {
    type PalletId = GovernancePalletId;

    type MinApplicationDataLength = ConstU32<2>;

    type MaxApplicationDataLength = ConstU32<256>;

    type ApplicationExpiration = ConstU64<2000>;

    type MaxPenaltyPercentage = MaxPenaltyPercentage;

    type DefaultTreasuryEmissionFee = DefaultTreasuryEmissionFee;

    type DefaultProposalCost = ConstU128<{ to_nano(10_000) }>;

    type DefaultProposalExpiration = ConstU64<75_600>;

    type DefaultAgentApplicationCost = ConstU128<{ to_nano(1_000) }>;

    type DefaultAgentApplicationExpiration = ConstU64<216_000>;

    type DefaultProposalRewardTreasuryAllocation = DefaultProposalRewardTreasuryAllocation;

    type DefaultMaxProposalRewardTreasuryAllocation = ConstU128<{ to_nano(10_000) }>;

    type DefaultProposalRewardInterval = ConstU64<75_600>;

    type RuntimeEvent = RuntimeEvent;

    type Currency = Balances;

    type WeightInfo = pallet_governance::weights::SubstrateWeight<Test>;
}

impl pallet_balances::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = MaxLocks;
    type WeightInfo = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = ();
    type RuntimeHoldReason = ();
    type FreezeIdentifier = ();
    type MaxFreezes = polkadot_sdk::frame_support::traits::ConstU32<16>;
    type RuntimeFreezeReason = ();
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type Block = Block;
    type BlockWeights = ();
    type BlockLength = ();
    type AccountId = AccountId;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;

    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

// Utility functions
//===================

const TOKEN_DECIMALS: u32 = 18;

pub const fn to_nano(x: Balance) -> Balance {
    x.saturating_mul((10 as Balance).pow(TOKEN_DECIMALS))
}

pub const fn from_nano(x: Balance) -> Balance {
    x.saturating_div((10 as Balance).pow(TOKEN_DECIMALS))
}

pub fn add_balance(key: AccountId, amount: Balance) {
    drop(<Balances as Currency<AccountId>>::deposit_creating(
        &key, amount,
    ));
}

pub fn add_stake(staker: AccountId, staked: AccountId, amount: Balance) {
    let amount = MinAllowedStake::<Test>::get().max(amount);
    let existential = ExistentialDeposit::get().saturating_sub(get_balance(staker));

    drop(<Balances as Currency<AccountId>>::deposit_creating(
        &staker,
        amount + existential,
    ));

    if !pallet_torus0::Agents::<Test>::contains_key(staked) {
        register_empty_agent(staked);
    }

    pallet_torus0::stake::add_stake::<Test>(staker, staked, amount).expect("failed to add stake");
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    new_test_ext_with_block(0)
}

pub fn new_test_ext_with_block(block: BlockNumber) -> sp_io::TestExternalities {
    sp_tracing::try_init_simple();
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(block));
    ext
}

pub fn get_origin(key: AccountId) -> RuntimeOrigin {
    <<Test as frame_system::Config>::RuntimeOrigin>::signed(key)
}

pub fn step_block(count: BlockNumber) {
    let current = System::block_number();
    for block in current..current + count {
        Torus0::on_finalize(block);
        Emission0::on_finalize(block);
        Governance::on_finalize(block);
        System::on_finalize(block);
        System::set_block_number(block + 1);
        Governance::on_initialize(block + 1);
        System::on_initialize(block + 1);
        Emission0::on_initialize(block + 1);
        Torus0::on_initialize(block + 1);
    }
}

pub fn run_to_block(target: BlockNumber) {
    step_block(target - System::block_number());
}

pub fn get_balance(key: AccountId) -> Balance {
    <Balances as Currency<AccountId>>::free_balance(&key)
}

pub fn register_empty_agent(key: AccountId) {
    pallet_torus0::Agents::<Test>::set(
        key,
        Some(pallet_torus0::agent::Agent {
            key,
            name: Default::default(),
            url: Default::default(),
            metadata: Default::default(),
            weight_penalty_factor: Default::default(),
            registration_block: <polkadot_sdk::frame_system::Pallet<Test>>::block_number(),
            fees: Default::default(),
        }),
    );
}

pub fn round_first_five(num: u64) -> u64 {
    let place_value = 10_u64.pow(num.to_string().len() as u32 - 5);
    let first_five = num / place_value;

    if first_five % 10 >= 5 {
        (first_five / 10 + 1) * place_value * 10
    } else {
        (first_five / 10) * place_value * 10
    }
}

pub fn zero_min_burn() {
    DEFAULT_MIN_BURN.set(0);
}

#[macro_export]
macro_rules! assert_ok {
    ( $x:expr $(,)? ) => {
        match $x {
            Ok(v) => v,
            is => panic!("Expected Ok(_). Got {is:#?}"),
        }
    };
    ( $x:expr, $y:expr $(,)? ) => {
        assert_eq!($x, Ok($y));
    };
}

#[macro_export]
macro_rules! assert_in_range {
    ($value:expr, $expected:expr, $margin:expr) => {
        assert!(
            ($expected - $margin..=$expected + $margin).contains(&$value),
            "value {} is out of range {}..={}",
            $value,
            $expected,
            $margin
        );
    };
}
