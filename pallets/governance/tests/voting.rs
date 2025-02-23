use pallet_governance::proposal::{Proposal, ProposalData};
use pallet_governance::Error;
use pallet_governance::{proposal::ProposalStatus, Proposals};
use polkadot_sdk::frame_support::{assert_err, assert_ok};
use polkadot_sdk::sp_runtime::BoundedVec;
use test_utils::{
    add_balance, get_balance, get_origin, new_test_ext, to_nano, zero_min_burn, AccountId, Test,
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

        register(MODULE, 0, 0, to_nano(10));

        config(1, 100);

        if pallet_torus0::stake::sum_staked_by::<Test>(&MODULE) < 1 {
            stake(MODULE, MODULE, to_nano(1));
        }

        assert_err!(
            pallet_governance::Pallet::<Test>::vote_proposal(get_origin(MODULE), 0, true),
            Error::<Test>::ProposalNotFound
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::vote_proposal(get_origin(MODULE), 0, false),
            Error::<Test>::ProposalNotFound
        );

        assert_err!(
            pallet_governance::Pallet::<Test>::remove_vote_proposal(get_origin(MODULE), 0),
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
