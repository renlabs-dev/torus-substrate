#![allow(unused, clippy::arithmetic_side_effects)]

use pallet_emission0::PendingEmission;
use pallet_governance::{
    proposal::{Proposal, ProposalData, ProposalStatus},
    DaoTreasuryAddress, Error, GlobalGovernanceConfig, Proposals, TreasuryEmissionFee,
};
use pallet_governance_api::GovernanceApi;
use polkadot_sdk::{frame_support::assert_err, frame_system::RawOrigin};
use polkadot_sdk::{frame_support::assert_ok, sp_runtime::Percent};
use polkadot_sdk::{frame_support::traits::Get, sp_runtime::BoundedVec};
use test_utils::{
    add_balance, get_balance, get_origin, new_test_ext, step_block, to_nano, zero_min_burn,
    AccountId, Governance, Test,
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
fn removes_vote_correctly() {
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

        register(FOR, 0, 0, to_nano(5));
        register(AGAINST, 0, 1, to_nano(10));

        config(1, 100);

        assert_ok!(
            pallet_governance::Pallet::<Test>::add_global_custom_proposal(
                get_origin(FOR),
                b"metadata".to_vec()
            )
        );

        vote(FOR, 0, true);
        vote(AGAINST, 0, false);

        pallet_governance::voting::remove_vote::<Test>(FOR, 0).unwrap();
        pallet_governance::voting::remove_vote::<Test>(AGAINST, 0).unwrap();

        match pallet_governance::Proposals::<Test>::get(0).unwrap().status {
            ProposalStatus::Open {
                votes_for,
                votes_against,
                ..
            } => {
                assert!(!votes_for.contains(&FOR));
                assert!(!votes_against.contains(&AGAINST));
            }
            _ => unreachable!(),
        }

        assert_eq!(
            get_balance(Governance::dao_treasury_address()),
            balance + crate::GlobalGovernanceConfig::<Test>::get().proposal_cost
        );
    });
}

#[test]
fn adds_vote_correctly() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        const FOR: u32 = 0;
        const AGAINST: u32 = 1;

        register(FOR, 0, 0, to_nano(10));
        register(AGAINST, 0, 1, to_nano(10));

        config(1, 100);

        assert_ok!(
            pallet_governance::Pallet::<Test>::add_global_custom_proposal(
                get_origin(FOR),
                b"metadata".to_vec()
            )
        );

        vote(FOR, 0, true);
        vote(AGAINST, 0, false);

        match pallet_governance::Proposals::<Test>::get(0).unwrap().status {
            ProposalStatus::Open {
                votes_for,
                votes_against,
                ..
            } => {
                assert!(votes_for.contains(&FOR));
                assert!(votes_against.contains(&AGAINST));
            }
            _ => unreachable!(),
        }
    });
}

#[test]
fn ensures_proposal_exists() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        const MODULE: u32 = 0;
        PendingEmission::<Test>::set(0);
        TreasuryEmissionFee::<Test>::set(Percent::zero());
        let balance = get_balance(DaoTreasuryAddress::<Test>::get());

        let min_stake: u128 = <Test as pallet_torus0::Config>::DefaultMinAllowedStake::get();
        let default_proposal_expiration: u64 =
            <Test as pallet_governance::Config>::DefaultProposalExpiration::get();

        config(1, 100);

        let origin = get_origin(0);
        add_balance(0, to_nano(2));
        register(0, 0, 0, to_nano(1) - min_stake);

        if pallet_torus0::stake::sum_staked_by::<Test>(&MODULE) < 1 {
            stake(MODULE, MODULE, to_nano(1));
        }

        let _ = pallet_governance::roles::penalize_agent::<Test>(RawOrigin::Root.into(), 0, 100);
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
            get_balance(Governance::dao_treasury_address()),
            balance + crate::GlobalGovernanceConfig::<Test>::get().proposal_cost
        );
    });
}

#[test]
fn creates_emission_proposal_with_invalid_params_and_it_fails() {
    new_test_ext().execute_with(|| {
        const MODULE: AccountId = 0;

        zero_min_burn();

        let default_proposal_expiration: u64 =
            <Test as pallet_governance::Config>::DefaultProposalExpiration::get();

        let min_stake: u128 = <Test as pallet_torus0::Config>::DefaultMinAllowedStake::get();

        config(1, default_proposal_expiration);

        let origin = get_origin(MODULE);
        add_balance(MODULE, to_nano(2));
        register(MODULE, 0, MODULE, to_nano(1) - min_stake);

        assert_err!(
            pallet_governance::Pallet::<Test>::vote_proposal(origin.clone(), 0, true),
            Error::<Test>::ProposalNotFound
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::vote_proposal(origin.clone(), 0, false),
            Error::<Test>::ProposalNotFound
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_vote_proposal(origin, 0),
            Error::<Test>::ProposalNotFound
        );
    });
}

#[test]
fn ensures_proposal_is_open() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        const MODULE: u32 = 0;

        register(MODULE, 0, 0, to_nano(10));

        config(1, 100);

        Proposals::<Test>::set(
            0,
            Some(Proposal {
                id: 0,
                proposer: MODULE,
                expiration_block: 0,
                data: ProposalData::GlobalCustom,
                status: ProposalStatus::Expired,
                metadata: BoundedVec::truncate_from("test".as_bytes().to_vec()),
                proposal_cost: 1,
                creation_block: 0,
            }),
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::vote_proposal(get_origin(MODULE), 0, true),
            Error::<Test>::ProposalClosed
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::vote_proposal(get_origin(MODULE), 0, false),
            Error::<Test>::ProposalClosed
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_vote_proposal(get_origin(MODULE), 0),
            Error::<Test>::ProposalClosed
        );
    });
}

#[test]
fn ensures_module_hasnt_voted() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        const MODULE: u32 = 0;

        register(MODULE, 0, 0, to_nano(10));

        config(1, 100);

        assert_ok!(
            pallet_governance::Pallet::<Test>::add_global_custom_proposal(
                get_origin(MODULE),
                b"metadata".to_vec()
            )
        );

        assert_ok!(pallet_governance::Pallet::<Test>::vote_proposal(
            get_origin(MODULE),
            0,
            true
        ),);

        assert_err!(
            pallet_governance::Pallet::<Test>::vote_proposal(get_origin(MODULE), 0, false),
            Error::<Test>::AlreadyVoted
        );
    });
}

#[test]
fn ensures_module_has_voted() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        const MODULE: u32 = 0;

        register(MODULE, 0, 0, to_nano(10));

        config(1, 100);

        assert_ok!(
            pallet_governance::Pallet::<Test>::add_global_custom_proposal(
                get_origin(MODULE),
                b"metadata".to_vec()
            )
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_vote_proposal(get_origin(MODULE), 0),
            Error::<Test>::NotVoted
        );
    });
}
