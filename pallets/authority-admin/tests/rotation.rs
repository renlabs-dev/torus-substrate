use pallet_authority_admin::Error;
use polkadot_sdk::{
    frame_support::{
        assert_noop, assert_ok,
        traits::{Everything, Hooks},
    },
    frame_system::{self, RawOrigin},
    pallet_aura::{self, MinimumPeriodTimesTwo},
    pallet_grandpa, pallet_timestamp,
    polkadot_sdk_frame::runtime::prelude::*,
    sp_consensus_aura::sr25519::AuthorityId as AuraId,
    sp_core::{H256, ed25519, sr25519},
    sp_runtime::{BuildStorage, traits::IdentityLookup},
};

#[frame_construct_runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(RuntimeCall, RuntimeEvent, RuntimeError, RuntimeOrigin)]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Runtime>;

    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp::Pallet<Runtime>;

    #[runtime::pallet_index(2)]
    pub type Aura = pallet_aura::Pallet<Runtime>;

    #[runtime::pallet_index(3)]
    pub type Grandpa = pallet_grandpa::Pallet<Runtime>;

    #[runtime::pallet_index(4)]
    pub type AuthorityAdmin = pallet_authority_admin::Pallet<Runtime>;
}

type Block = frame_system::mocking::MockBlock<Test>;

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type Block = Block;
    type BlockWeights = ();
    type BlockLength = ();
    type AccountId = u32;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = polkadot_sdk::sp_runtime::traits::BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<1>;
    type WeightInfo = ();
}

impl pallet_aura::Config for Test {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<128>;
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
    type SlotDuration = MinimumPeriodTimesTwo<Test>;
}

impl pallet_grandpa::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type KeyOwnerProof = polkadot_sdk::sp_core::Void;
    type MaxAuthorities = ConstU32<128>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type EquivocationReportSystem = ();
    type MaxNominators = ConstU32<200>;
    type WeightInfo = ();
}

impl pallet_authority_admin::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AdminOrigin = frame_system::EnsureRoot<u32>;
    type WeightInfo = ();
}

fn new_test_ext() -> polkadot_sdk::sp_io::TestExternalities {
    let storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .expect("test storage should build");
    let mut ext = polkadot_sdk::sp_io::TestExternalities::new(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

fn aura_authority(seed: u8) -> AuraId {
    sr25519::Public::from_raw([seed; 32]).into()
}

fn grandpa_authority(seed: u8) -> polkadot_sdk::sp_consensus_grandpa::AuthorityId {
    ed25519::Public::from_raw([seed; 32]).into()
}

#[test]
fn set_authorities_requires_admin_origin() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            AuthorityAdmin::set_authorities(
                RawOrigin::Signed(1).into(),
                vec![aura_authority(1)],
                vec![(grandpa_authority(1), 1)],
            ),
            polkadot_sdk::sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn set_authorities_rejects_invalid_inputs() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            AuthorityAdmin::set_authorities(
                RawOrigin::Root.into(),
                Vec::new(),
                vec![(grandpa_authority(1), 1)],
            ),
            Error::<Test>::EmptyAuraAuthorities
        );

        assert_noop!(
            AuthorityAdmin::set_authorities(
                RawOrigin::Root.into(),
                vec![aura_authority(1)],
                Vec::new(),
            ),
            Error::<Test>::EmptyGrandpaAuthorities
        );

        assert_noop!(
            AuthorityAdmin::set_authorities(
                RawOrigin::Root.into(),
                vec![aura_authority(1)],
                vec![(grandpa_authority(1), 0)],
            ),
            Error::<Test>::InvalidGrandpaAuthorityWeight
        );
    });
}

#[test]
fn set_authorities_rejects_new_change_while_pending() {
    new_test_ext().execute_with(|| {
        assert_ok!(AuthorityAdmin::set_authorities(
            RawOrigin::Root.into(),
            vec![aura_authority(1)],
            vec![(grandpa_authority(1), 1)],
        ));

        assert_noop!(
            AuthorityAdmin::set_authorities(
                RawOrigin::Root.into(),
                vec![aura_authority(2)],
                vec![(grandpa_authority(2), 1)],
            ),
            Error::<Test>::GrandpaAuthorityChangePending
        );
    });
}

#[test]
fn set_authorities_updates_aura_and_grandpa() {
    new_test_ext().execute_with(|| {
        let aura_authorities = vec![aura_authority(11), aura_authority(12)];
        let grandpa_authorities = vec![(grandpa_authority(21), 1), (grandpa_authority(22), 1)];

        assert!(Grandpa::pending_change().is_none());
        assert_eq!(Grandpa::current_set_id(), 0);
        assert!(pallet_aura::Authorities::<Test>::get().is_empty());
        assert!(Grandpa::grandpa_authorities().is_empty());

        assert_ok!(AuthorityAdmin::set_authorities(
            RawOrigin::Root.into(),
            aura_authorities.clone(),
            grandpa_authorities.clone(),
        ));

        assert_eq!(
            pallet_aura::Authorities::<Test>::get().into_inner(),
            aura_authorities
        );
        assert_eq!(Grandpa::current_set_id(), 1);

        let pending_change = Grandpa::pending_change().expect("pending change should exist");
        assert_eq!(pending_change.delay, 0);
        assert_eq!(
            pending_change.next_authorities.to_vec(),
            grandpa_authorities
        );

        Grandpa::on_finalize(System::block_number());

        assert!(Grandpa::pending_change().is_none());
        assert_eq!(Grandpa::grandpa_authorities(), grandpa_authorities);
    });
}
