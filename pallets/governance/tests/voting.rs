#![allow(clippy::arithmetic_side_effects)]

use pallet_emission0::PendingEmission;
use pallet_governance::{
    config::GovernanceConfiguration,
    proposal::{GlobalParamsData, ProposalStatus},
    DaoTreasuryAddress, Error, GlobalGovernanceConfig, Proposals, TreasuryEmissionFee,
};
use pallet_governance_api::GovernanceApi;
use polkadot_sdk::frame_support::traits::Get;
use polkadot_sdk::{frame_support::assert_err, sp_runtime::BoundedBTreeSet};
use polkadot_sdk::{frame_support::assert_ok, sp_runtime::Percent};
use test_utils::{
    add_balance, get_balance, get_origin, new_test_ext, step_block, to_nano, zero_min_burn,
    AccountId, Test,
};

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
        stake(account, account, to_nano(1));
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

pub fn stake(account: u32, module: u32, stake: u128) {
    // if get_balance(account) <= stake {
    add_balance(account, stake);
    // }

    if get_balance(account) - stake < 1 {
        add_balance(account, 1);
    }

    assert_ok!(pallet_torus0::stake::add_stake::<Test>(
        account, module, stake
    ));
}

#[test]
fn global_governance_config_validates_parameters_correctly() {
    new_test_ext().execute_with(|| {
        assert_err!(
            GlobalParamsData::<Test> {
                min_name_length: 1,
                ..GlobalParamsData::default()
            }
            .validate(),
            Error::<Test>::InvalidMinNameLength
        );

        assert_err!(
            GlobalParamsData::<Test> {
                max_name_length: 300,
                ..GlobalParamsData::default()
            }
            .validate(),
            Error::<Test>::InvalidMaxNameLength
        );

        assert_err!(
            GlobalParamsData::<Test> {
                max_allowed_agents: 50001,
                ..GlobalParamsData::default()
            }
            .validate(),
            Error::<Test>::InvalidMaxAllowedAgents
        );

        assert_err!(
            GlobalParamsData::<Test> {
                max_allowed_weights: 2001,
                ..GlobalParamsData::default()
            }
            .validate(),
            Error::<Test>::InvalidMaxAllowedWeights
        );

        assert_err!(
            GlobalParamsData::<Test> {
                min_weight_control_fee: 101,
                ..GlobalParamsData::default()
            }
            .validate(),
            Error::<Test>::InvalidMinWeightControlFee
        );

        assert_err!(
            GlobalParamsData::<Test> {
                min_staking_fee: 101,
                ..GlobalParamsData::default()
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
        add_balance(KEY, to_nano(100_000));

        let test = |global_params| {
            pallet_governance::Pallet::<Test>::add_global_params_proposal(
                get_origin(KEY),
                global_params,
                b"metadata".to_vec(),
            )
        };

        test(GlobalParamsData {
            min_name_length: 0,
            ..Default::default()
        })
        .expect_err("created proposal with invalid max name length");

        test(GlobalParamsData::default()).expect("failed to create proposal with valid parameters");
    });
}

#[test]
fn global_custom_proposal_is_accepted_correctly() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        const FOR: u32 = 0;
        const AGAINST: u32 = 1;

        let key = 0;

        register(FOR, 0, 0, to_nano(10));
        register(AGAINST, 0, 1, to_nano(5));

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
                stake_for: to_nano(10),
                stake_against: to_nano(5),
            }
        );
    });
}

#[test]
fn global_proposal_is_refused_correctly() {
    new_test_ext().execute_with(|| {
        PendingEmission::<Test>::set(0);
        TreasuryEmissionFee::<Test>::set(Percent::zero());
        let balance = get_balance(DaoTreasuryAddress::<Test>::get());

        zero_min_burn();
        const FOR: u32 = 0;
        const AGAINST: u32 = 1;

        let key = 0;

        register(FOR, 0, 0, to_nano(5));
        register(AGAINST, 0, 1, to_nano(10));

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
        assert_eq!(
            get_balance(Test::dao_treasury_address()),
            balance + crate::GlobalGovernanceConfig::<Test>::get().proposal_cost
        );
    });
}

#[test]
fn global_params_proposal_accepted() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        const KEY: u32 = 0;

        register(KEY, 0, 0, to_nano(10));
        config(1, 100);

        let data = GlobalParamsData {
            proposal_cost: 69_420,
            ..Default::default()
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
        const AGAINST_DELEGATED: u32 = 3;

        let origin = get_origin(0);

        register(FOR, 0, FOR, to_nano(5));
        // delegate(FOR);
        register(AGAINST, 0, AGAINST, to_nano(10));
        // delegate(AGAINST);

        stake(FOR_DELEGATED, FOR, to_nano(10));
        delegate(FOR_DELEGATED);

        stake(AGAINST_DELEGATED, AGAINST, to_nano(3));
        delegate(AGAINST_DELEGATED);

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
            to_nano(5),
            0,
            vec![b'0'; 64],
        )
        .expect_err("proposal should not be created when treasury does not have enough money");

        add_balance(DaoTreasuryAddress::<Test>::get(), to_nano(10));
        add_balance(0, to_nano(3));
        register(0, 0, 0, 0);
        config(1, 100);

        pallet_governance::Pallet::<Test>::add_dao_treasury_transfer_proposal(
            origin,
            to_nano(5),
            0,
            vec![b'0'; 64],
        )
        .expect("proposal should be created");
        vote(0, 0, true);

        step_block(100);

        assert_eq!(get_balance(DaoTreasuryAddress::<Test>::get()), to_nano(5));
        assert_eq!(get_balance(0), to_nano(8));
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
        add_balance(0, to_nano(2));
        register(0, 0, 0, to_nano(1));
        let _ = pallet_governance::roles::penalize_agent::<Test>(0, 100);
        pallet_torus0::TotalStake::<Test>::set(to_nano(10));

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
                stake_for: to_nano(1),
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
        add_balance(0, to_nano(2));
        register(0, 0, 0, to_nano(1) - min_stake);
        let _ = pallet_governance::roles::penalize_agent::<Test>(0, 100);
        pallet_torus0::TotalStake::<Test>::set(to_nano(10));

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
                stake_for: to_nano(1) - min_stake,
                stake_against: 0
            }
        );

        stake(0, 0, min_stake);
        pallet_torus0::TotalStake::<Test>::set(to_nano(10));

        step_block(100);

        assert_eq!(
            Proposals::<Test>::get(0).unwrap().status,
            ProposalStatus::Accepted {
                block: 21_700,
                stake_for: to_nano(1),
                stake_against: 0
            }
        );
    });
}

#[test]
fn creates_emission_proposal_and_it_expires() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        PendingEmission::<Test>::set(0);
        TreasuryEmissionFee::<Test>::set(Percent::zero());
        let balance = get_balance(DaoTreasuryAddress::<Test>::get());

        let default_proposal_expiration: u64 =
            <Test as pallet_governance::Config>::DefaultProposalExpiration::get();

        let min_stake: u128 = <Test as pallet_torus0::Config>::DefaultMinAllowedStake::get();

        config(1, default_proposal_expiration);

        let origin = get_origin(0);
        add_balance(0, to_nano(2));
        register(0, 0, 0, to_nano(1) - min_stake);
        let _ = pallet_governance::roles::penalize_agent::<Test>(0, 100);
        pallet_torus0::TotalStake::<Test>::set(to_nano(10));

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
        assert_eq!(
            get_balance(Test::dao_treasury_address()),
            balance + crate::GlobalGovernanceConfig::<Test>::get().proposal_cost
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
        add_balance(0, to_nano(2));
        register(0, 0, 0, to_nano(1) - min_stake);

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

#[test]
fn rewards_wont_exceed_treasury() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        // Fill the governance address with 1 mil so we are not limited by the max allocation
        let amount = to_nano(1_000_000_000);
        let key = DaoTreasuryAddress::<Test>::get();
        add_balance(key, amount);

        let governance_config: GovernanceConfiguration<Test> =
            GlobalGovernanceConfig::<Test>::get();
        let n = 0;
        let allocation =
            pallet_governance::proposal::get_reward_allocation::<Test>(&governance_config, n)
                .unwrap();
        assert_eq!(
            allocation.to_num::<u128>(),
            governance_config.max_proposal_reward_treasury_allocation
        );
    });
}
