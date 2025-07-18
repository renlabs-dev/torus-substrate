#![allow(unused, clippy::arithmetic_side_effects)]

use std::{cell::RefCell, io::Read, num::NonZeroU128};

use crate::frame_support::assert_ok;
use polkadot_sdk::{
    frame_support::{
        self, parameter_types,
        traits::{Currency, Everything, Hooks},
        PalletId,
    },
    frame_system, pallet_balances,
    polkadot_sdk_frame::{prelude::BlockNumberFor, runtime::prelude::*},
    sp_core::{hex2array, keccak_256, ByteArray, Get, H256, U256},
    sp_io,
    sp_runtime::{
        self,
        traits::{AccountIdConversion, BlakeTwo256, BlockNumberProvider, IdentityLookup},
        BoundedVec, BuildStorage, Percent,
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

    #[runtime::pallet_index(5)]
    pub type Permission0 = pallet_permission0::Pallet<Runtime>;

    #[runtime::pallet_index(6)]
    pub type Faucet = pallet_faucet::Pallet<Runtime>;
}

pub type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = sp_runtime::AccountId32;

#[allow(dead_code)]
pub type BalanceCall = pallet_balances::Call<Test>;

#[allow(dead_code)]
pub type TestRuntimeCall = frame_system::Call<Test>;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u16 = 42;
}

thread_local! {
    static DEFAULT_MIN_BURN: RefCell<u128> = const { RefCell::new(as_tors(10)) };
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

    pub DefaultNamespacePricingConfig: pallet_torus0::namespace::NamespacePricingConfig<Test> = pallet_torus0::namespace::NamespacePricingConfig {
        base_fee: as_tors(5),
        deposit_per_byte: as_tors(5),

        count_midpoint: 20,
        fee_steepness: Percent::from_percent(20),
        max_fee_multiplier: 5,
    };
}

impl pallet_torus0::Config for Test {
    type DefaultMinValidatorStake = ConstU128<50_000_000_000_000_000_000_000>;

    type DefaultRewardInterval = ConstU16<100>;

    type DefaultMinNameLength = ConstU16<2>;

    type DefaultMaxNameLength = ConstU16<32>;

    type DefaultMaxAgentUrlLength = ConstU16<64>;

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

    type DefaultAgentUpdateCooldown = ConstU64<32_400>;

    #[doc = " The storage MaxNameLength should be constrained to be no more than the value of this."]
    #[doc = " This is needed on agent::Agent to set the `name` field BoundedVec max length."]
    type MaxAgentNameLengthConstraint = ConstU32<256>;

    #[doc = " This is needed on agent::Agent to set the `url` field BoundedVec max length."]
    type MaxAgentUrlLengthConstraint = ConstU32<256>;

    type MaxAgentMetadataLengthConstraint = ConstU32<256>;

    type DefaultDividendsParticipationWeight = DefaultDividendsParticipationWeight;

    type DefaultNamespacePricingConfig = DefaultNamespacePricingConfig;

    type RuntimeEvent = RuntimeEvent;

    type Currency = Balances;
    type ExistentialDeposit = ExistentialDeposit;

    type Governance = Governance;

    type Emission = Emission0;
    type Permission0 = Permission0;

    type WeightInfo = pallet_torus0::weights::SubstrateWeight<Test>;
}

parameter_types! {
    pub HalvingInterval: NonZeroU128 = NonZeroU128::new(as_tors(144_000_000)).unwrap();
    pub MaxSupply: NonZeroU128 = NonZeroU128::new(as_tors(144_000_000 * 4)).unwrap();
    pub const DefaultEmissionRecyclingPercentage: Percent = Percent::from_parts(70);
    pub const DefaultIncentivesRatio: Percent = Percent::from_parts(50);
}

impl pallet_emission0::Config for Test {
    type RuntimeEvent = RuntimeEvent;

    type HalvingInterval = HalvingInterval;

    type MaxSupply = MaxSupply;

    type BlockEmission = ConstU128<{ (as_tors(250_000) - 1) / 10800 }>;

    type DefaultEmissionRecyclingPercentage = DefaultEmissionRecyclingPercentage;

    type DefaultIncentivesRatio = DefaultIncentivesRatio;

    type Currency = Balances;

    type Torus = Torus0;

    type Governance = Governance;

    type WeightInfo = pallet_emission0::weights::SubstrateWeight<Test>;

    type Permission0 = Permission0;
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

    type DefaultProposalCost = ConstU128<{ as_tors(10_000) }>;

    type DefaultProposalExpiration = ConstU64<75_600>;

    type DefaultAgentApplicationCost = ConstU128<{ as_tors(1_000) }>;

    type DefaultAgentApplicationExpiration = ConstU64<216_000>;

    type DefaultProposalRewardTreasuryAllocation = DefaultProposalRewardTreasuryAllocation;

    type DefaultMaxProposalRewardTreasuryAllocation = ConstU128<{ as_tors(10_000) }>;

    type DefaultProposalRewardInterval = ConstU64<75_600>;

    type RuntimeEvent = RuntimeEvent;

    type Currency = Balances;

    type Permission0 = Permission0;

    type WeightInfo = pallet_governance::weights::SubstrateWeight<Test>;
}

parameter_types! {
    pub const PermissionPalletId: PalletId = PalletId(*b"torusper");
    pub const MaxTargetsPerPermission: u32 = 100;
    pub const MaxStreamsPerPermission: u32 = 100;
    pub const MaxRevokersPerPermission: u32 = 10;
    pub const MaxControllersPerPermission: u32 = 10;
    pub const MinAutoDistributionThreshold: u128 = as_tors(100);
}

impl pallet_permission0::Config for Test {
    type PalletId = PermissionPalletId;

    type RuntimeEvent = RuntimeEvent;

    type WeightInfo = ();

    type Currency = Balances;

    type Torus = Torus0;

    type MaxTargetsPerPermission = MaxTargetsPerPermission;

    type MaxStreamsPerPermission = MaxStreamsPerPermission;

    type MaxRevokersPerPermission = MaxRevokersPerPermission;

    type MaxControllersPerPermission = MaxControllersPerPermission;

    type MinAutoDistributionThreshold = MinAutoDistributionThreshold;

    type MaxNamespacesPerPermission = ConstU32<0>;
    type MaxChildrenPerPermission = ConstU32<0>;
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
    type ReserveIdentifier = [u8; 8];
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

impl pallet_faucet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Torus = Torus0;
}

const TOKEN_DECIMALS: u32 = 18;

pub const fn as_tors(x: Balance) -> Balance {
    x.saturating_mul((10 as Balance).pow(TOKEN_DECIMALS))
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

pub fn goto_block(block: BlockNumber) {
    let current = System::block_number();
    // for block in current..current + count {
    Permission0::on_finalize(block - 1);
    Torus0::on_finalize(block - 1);
    Emission0::on_finalize(block - 1);
    Governance::on_finalize(block - 1);
    System::on_finalize(block - 1);

    System::set_block_number(block);

    System::on_initialize(block);
    Governance::on_initialize(block);
    Emission0::on_initialize(block);
    Torus0::on_initialize(block);
    Permission0::on_initialize(block);
}
// }

#[cfg(feature = "testnet")]
#[test]
fn faucet_fails_with_wrong_pow() {
    use polkadot_sdk::frame_support::assert_err;

    new_test_ext().execute_with(|| {
        goto_block(1);

        let origin = <<Test as frame_system::Config>::RuntimeOrigin>::none();
        let res = pallet_faucet::Pallet::<Test>::faucet(
            origin,
            0,
            0,
            vec![0; 32],
            sp_runtime::AccountId32::new([0u8; 32]),
        );

        assert_err!(res, pallet_faucet::Error::<Test>::InvalidSeal);
    });
}

#[cfg(feature = "testnet")]
#[test]
fn faucet_works() {
    use polkadot_sdk::frame_support::assert_err;

    new_test_ext().execute_with(|| {
        goto_block(6);
        let account_key = sp_runtime::AccountId32::new([4u8; 32]);

        let mut nonce = rand::random_range(0..u64::MAX);
        let mut hash =
            pallet_faucet::faucet::create_seal_hash::<Test>(4, nonce, &account_key).unwrap();

        let difficulty: U256 = U256::from(1_000_000);
        while !pallet_faucet::faucet::hash_meets_difficulty(&hash, difficulty) {
            nonce = rand::random_range(0..u64::MAX);
            hash = pallet_faucet::faucet::create_seal_hash::<Test>(4, nonce, &account_key).unwrap();
        }

        let origin = <<Test as frame_system::Config>::RuntimeOrigin>::none();
        let res = pallet_faucet::Pallet::<Test>::faucet(
            origin,
            4,
            nonce,
            hash.as_bytes().to_vec(),
            account_key.clone(),
        );

        assert_ok!(res);

        assert_eq!(
            <Balances as Currency<AccountId>>::free_balance(&account_key),
            15_000_000_000_000_000_000
        );
    });
}
