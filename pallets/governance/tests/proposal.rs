#![allow(clippy::arithmetic_side_effects)]

use pallet_governance::{
    config::GovernanceConfiguration,
    proposal::{GlobalParamsData, ProposalStatus},
    Config, DaoTreasuryAddress, Error, GlobalGovernanceConfig, Proposals,
};
use polkadot_sdk::{
    frame_support::{assert_err, assert_ok, traits::Get},
    frame_system::RawOrigin,
    sp_runtime::{BoundedBTreeSet, Percent},
};
use test_utils::{
    add_balance, as_tors, get_balance, get_origin, new_test_ext, step_block, zero_min_burn,
    AccountId, Test,
};

fn default_params<T: Config>() -> GlobalParamsData<T> {
    GlobalParamsData {
        min_name_length: T::DefaultMinNameLength::get(),
        max_name_length: T::DefaultMaxNameLength::get(),
        min_weight_control_fee: T::DefaultMinWeightControlFee::get(),
        min_staking_fee: T::DefaultMinStakingFee::get(),
        dividends_participation_weight:
            <T as pallet_torus0::Config>::DefaultDividendsParticipationWeight::get(),
        namespace_pricing_config: <T as pallet_torus0::Config>::DefaultNamespacePricingConfig::get(
        ),
        proposal_cost: T::DefaultProposalCost::get(),
    }
}

fn register(account: AccountId, _unused: u16, module: AccountId, stake: u128) {
    if get_balance(account) < stake {
        add_balance(account, stake);
    }

    let _ = pallet_governance::whitelist::add_to_whitelist::<Test>(module);

    assert_ok!(pallet_torus0::agent::register::<Test>(
        module,
        module,
        b"agent".to_vec(),
        b"url".to_vec(),
        b"metadata".to_vec(),
    ));

    assert!(get_balance(account) >= stake);

    let min_stake: u128 = pallet_torus0::MinAllowedStake::<Test>::get();
    if stake >= min_stake {
        if get_balance(account) - stake < 1 {
            add_balance(account, 1);
        }
        assert_ok!(pallet_torus0::stake::add_stake::<Test>(
            account, module, stake
        ));
    }
}

fn config(proposal_cost: u128, proposal_expiration: u64) {
    pallet_governance::GlobalGovernanceConfig::<Test>::mutate(|config| {
        config.proposal_cost = proposal_cost;
        config.proposal_expiration = proposal_expiration;
    });
}

fn vote(account: u32, proposal_id: u64, agree: bool) {
    if pallet_torus0::stake::sum_staked_by::<Test>(&account) < 1 {
        stake(account, account, as_tors(1));
    }

    assert_ok!(pallet_governance::voting::add_vote::<Test>(
        account,
        proposal_id,
        agree
    ));
}

fn delegate(account: u32) {
    assert_ok!(pallet_governance::Pallet::<Test>::enable_vote_delegation(
        get_origin(account)
    ));
}

#[test]
fn global_governance_config_validates_parameters_correctly() {
    new_test_ext().execute_with(|| {
        assert_err!(
            GlobalParamsData::<Test> {
                min_name_length: 1,
                ..default_params::<Test>()
            }
            .validate(),
            Error::<Test>::InvalidMinNameLength
        );

        assert_err!(
            GlobalParamsData::<Test> {
                max_name_length: 300,
                ..default_params::<Test>()
            }
            .validate(),
            Error::<Test>::InvalidMaxNameLength
        );

        assert_err!(
            GlobalParamsData::<Test> {
                min_weight_control_fee: 101,
                ..default_params::<Test>()
            }
            .validate(),
            Error::<Test>::InvalidMinWeightControlFee
        );

        assert_err!(
            GlobalParamsData::<Test> {
                min_staking_fee: 101,
                ..default_params::<Test>()
            }
            .validate(),
            Error::<Test>::InvalidMinStakingFee
        );
    });
}

#[test]
fn global_proposal_validates_parameters() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        const KEY: u32 = 0;
        add_balance(KEY, as_tors(100_000));

        assert_err!(
            pallet_governance::Pallet::<Test>::add_global_params_proposal(
                get_origin(KEY),
                default_params::<Test>(),
                b"".to_vec(),
            ),
            Error::<Test>::ProposalDataTooSmall
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::add_global_params_proposal(
                get_origin(KEY),
                default_params::<Test>(),
                b"1".repeat(257),
            ),
            Error::<Test>::ProposalDataTooLarge
        );

        let test = |global_params| {
            pallet_governance::Pallet::<Test>::add_global_params_proposal(
                get_origin(KEY),
                global_params,
                b"metadata".to_vec(),
            )
        };

        test(GlobalParamsData {
            min_name_length: 0,
            ..default_params::<Test>()
        })
        .expect_err("created proposal with invalid max name length");

        test(GlobalParamsData {
            proposal_cost: 50_000_000_000_000_000_000_001,
            ..default_params::<Test>()
        })
        .expect_err("created proposal with invalid proposal cost");

        test(default_params::<Test>()).expect("failed to create proposal with valid parameters");
    });
}

#[test]
fn global_custom_proposal_is_accepted_correctly() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        const FOR: u32 = 0;
        const AGAINST: u32 = 1;

        let key = 0;

        register(FOR, 0, 0, as_tors(10));
        register(AGAINST, 0, 1, as_tors(5));

        config(1, 100);

        add_balance(key, 1);
        assert_ok!(
            pallet_governance::Pallet::<Test>::add_global_custom_proposal(
                get_origin(key),
                b"metadata".to_vec()
            )
        );

        vote(FOR, 0, true);
        vote(AGAINST, 0, false);

        step_block(100);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Accepted {
                block: 100,
                stake_for: as_tors(10),
                stake_against: as_tors(5),
            }
        );

        step_block(1);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Accepted {
                block: 100,
                stake_for: as_tors(10),
                stake_against: as_tors(5),
            }
        );
    });
}

#[test]
fn global_proposal_is_refused_correctly() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        const FOR: u32 = 0;
        const AGAINST: u32 = 1;

        let key = 0;

        register(FOR, 0, 0, as_tors(5));
        register(AGAINST, 0, 1, as_tors(10));

        config(1, 100);

        add_balance(FOR, 1);
        assert_ok!(
            pallet_governance::Pallet::<Test>::add_global_custom_proposal(
                get_origin(key),
                b"metadata".to_vec()
            )
        );

        vote(FOR, 0, true);
        vote(AGAINST, 0, false);

        step_block(100);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Refused {
                block: 100,
                stake_for: 5_000_000_000_000_000_000,
                stake_against: 10_000_000_000_000_000_000,
            }
        );
    });
}

#[test]
fn global_params_proposal_accepted() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        const KEY: u32 = 0;

        register(KEY, 0, 0, as_tors(10));
        config(1, 100);

        let data = GlobalParamsData {
            proposal_cost: 69_420,
            ..default_params::<Test>()
        };

        add_balance(KEY, 1);
        pallet_governance::Pallet::<Test>::add_global_params_proposal(
            get_origin(KEY),
            data,
            b"metadata".to_vec(),
        )
        .unwrap();

        vote(KEY, 0, true);
        step_block(100);

        assert_eq!(GlobalGovernanceConfig::<Test>::get().proposal_cost, 69_420);
    });
}

#[test]
fn global_proposals_counts_delegated_stake() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        const FOR: u32 = 0;
        const AGAINST: u32 = 1;
        const FOR_DELEGATED: u32 = 2;

        let origin = get_origin(0);

        register(FOR, 0, FOR, as_tors(5));
        register(AGAINST, 0, AGAINST, as_tors(10));

        stake(FOR_DELEGATED, FOR, as_tors(10));
        delegate(FOR_DELEGATED);

        // AGAINST does not delegate voting power, so it doesn't matter
        // to who it stakes.
        stake(AGAINST, FOR, as_tors(3));
        pallet_governance::voting::disable_delegation::<Test>(AGAINST).unwrap();

        config(1, 100);

        add_balance(FOR, 1);

        assert_ok!(
            pallet_governance::Pallet::<Test>::add_global_custom_proposal(origin, vec![b'0'; 64])
        );

        vote(FOR, 0, true);
        vote(AGAINST, 0, false);

        step_block(100);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Accepted {
                block: 100,
                stake_for: 15_000_000_000_000_000_000,
                stake_against: 13_000_000_000_000_000_000,
            }
        );
    });
}

#[test]
fn creates_treasury_transfer_proposal_and_transfers() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        pallet_governance::TreasuryEmissionFee::<Test>::set(Percent::from_percent(0));

        let origin = get_origin(0);
        pallet_governance::Pallet::<Test>::add_dao_treasury_transfer_proposal(
            origin.clone(),
            as_tors(5),
            0,
            vec![b'0'; 64],
        )
        .expect_err("proposal should not be created when treasury does not have enough money");

        add_balance(DaoTreasuryAddress::<Test>::get(), as_tors(10));
        add_balance(0, as_tors(3));
        register(0, 0, 0, 0);
        config(1, 100);

        pallet_governance::Pallet::<Test>::add_dao_treasury_transfer_proposal(
            origin,
            as_tors(5),
            0,
            vec![b'0'; 64],
        )
        .expect("proposal should be created");
        vote(0, 0, true);

        step_block(100);

        assert_eq!(get_balance(DaoTreasuryAddress::<Test>::get()), as_tors(5));
        assert_eq!(get_balance(0), as_tors(8));
    });
}

#[test]
fn creates_emission_proposal_and_it_runs_after_2_days() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let default_proposal_expiration: u64 =
            <Test as pallet_governance::Config>::DefaultProposalExpiration::get();

        config(1, default_proposal_expiration);

        let origin = get_origin(0);
        add_balance(0, as_tors(2));
        register(0, 0, 0, as_tors(1));
        let _ = pallet_governance::roles::penalize_agent::<Test>(RawOrigin::Root.into(), 0, 100);
        pallet_torus0::TotalStake::<Test>::set(as_tors(10));

        assert_ok!(pallet_governance::Pallet::<Test>::add_emission_proposal(
            origin.clone(),
            Percent::from_parts(20),
            Percent::from_parts(20),
            Percent::from_parts(20),
            vec![b'0'; 64],
        ));

        vote(0, 0, true);

        step_block(21_600);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Accepted {
                block: 21_600,
                stake_for: as_tors(1),
                stake_against: 0
            }
        );
    });
}

#[test]
fn creates_emission_proposal_and_it_runs_before_expiration() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let default_proposal_expiration: u64 =
            <Test as pallet_governance::Config>::DefaultProposalExpiration::get();

        let min_stake: u128 = <Test as pallet_torus0::Config>::DefaultMinAllowedStake::get();

        config(1, default_proposal_expiration);

        let origin = get_origin(0);
        add_balance(0, as_tors(2));
        register(0, 0, 0, as_tors(1) - min_stake);
        let _ = pallet_governance::roles::penalize_agent::<Test>(RawOrigin::Root.into(), 0, 100);
        pallet_torus0::TotalStake::<Test>::set(as_tors(10));

        assert_ok!(pallet_governance::Pallet::<Test>::add_emission_proposal(
            origin.clone(),
            Percent::from_parts(20),
            Percent::from_parts(20),
            Percent::from_parts(20),
            vec![b'0'; 64],
        ));

        vote(0, 0, true);

        step_block(21_600);

        let mut votes_for = BoundedBTreeSet::new();
        votes_for.try_insert(0).unwrap();

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Open {
                votes_for,
                votes_against: BoundedBTreeSet::new(),
                stake_for: as_tors(1) - min_stake,
                stake_against: 0
            }
        );

        stake(0, 0, min_stake);
        pallet_torus0::TotalStake::<Test>::set(as_tors(10));

        step_block(100);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Accepted {
                block: 21_700,
                stake_for: as_tors(1),
                stake_against: 0
            }
        );
    });
}

#[test]
fn creates_emission_proposal_and_it_expires() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let default_proposal_expiration: u64 =
            <Test as pallet_governance::Config>::DefaultProposalExpiration::get();

        let min_stake: u128 = <Test as pallet_torus0::Config>::DefaultMinAllowedStake::get();

        config(1, default_proposal_expiration);

        let origin = get_origin(0);
        add_balance(0, as_tors(2));
        register(0, 0, 0, as_tors(1) - min_stake);
        let _ = pallet_governance::roles::penalize_agent::<Test>(RawOrigin::Root.into(), 0, 100);
        pallet_torus0::TotalStake::<Test>::set(as_tors(10));

        assert_ok!(pallet_governance::Pallet::<Test>::add_emission_proposal(
            origin.clone(),
            Percent::from_parts(20),
            Percent::from_parts(20),
            Percent::from_parts(20),
            vec![b'0'; 64],
        ));

        vote(0, 0, true);

        step_block(default_proposal_expiration);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Expired
        );
    });
}

pub fn stake(account: u32, module: u32, stake: u128) {
    add_balance(account, stake);

    if get_balance(account) - stake < 1 {
        add_balance(account, 1);
    }

    assert_ok!(pallet_torus0::stake::add_stake::<Test>(
        account, module, stake
    ));
}

#[test]
fn rewards_wont_exceed_treasury() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        // Fill the governance address with 1 mil so we are not limited by the max
        // allocation
        let amount = as_tors(1_000_000_000);
        let key = DaoTreasuryAddress::<Test>::get();
        add_balance(key, amount);

        let governance_config: GovernanceConfiguration<Test> =
            GlobalGovernanceConfig::<Test>::get();
        let n = 0;
        let allocation =
            pallet_governance::proposal::get_reward_allocation::<Test>(&governance_config, n)
                .unwrap();
        assert_eq!(
            allocation.into_inner(),
            governance_config.max_proposal_reward_treasury_allocation
        );
    });
}

#[test]
fn creates_emission_proposal_with_invalid_params_and_it_fails() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let default_proposal_expiration: u64 =
            <Test as pallet_governance::Config>::DefaultProposalExpiration::get();

        let min_stake: u128 = <Test as pallet_torus0::Config>::DefaultMinAllowedStake::get();

        config(1, default_proposal_expiration);

        let origin = get_origin(0);
        add_balance(0, as_tors(2));
        register(0, 0, 0, as_tors(1) - min_stake);

        assert_err!(
            pallet_governance::Pallet::<Test>::add_emission_proposal(
                origin.clone(),
                Percent::from_parts(51),
                Percent::from_parts(50),
                Percent::from_parts(50),
                vec![b'0'; 64],
            ),
            Error::<Test>::InvalidEmissionProposalData
        );
    });
}
