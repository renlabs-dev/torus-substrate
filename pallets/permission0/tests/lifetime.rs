use pallet_permission0::Error;
use pallet_permission0_api::Permission0Api;
use polkadot_sdk::frame_support::assert_err;
use test_utils::*;

#[test]
fn manual_cant_execute_when_expires() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        add_balance(agent_0, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_1, u16::MAX / 2), (agent_2, u16::MAX / 2)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::UntilBlock(1),
            pallet_permission0_api::RevocationTerms::Irrevocable,
        ));

        step_block(20);

        assert_err!(
            Permission0::execute_permission(get_origin(agent_0), permission_id),
            pallet_permission0::Error::<Test>::PermissionNotFound
        );
    });
}

#[test]
fn irrevocable() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        add_balance(agent_0, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_1, u16::MAX / 2), (agent_2, u16::MAX / 2)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
        ));

        assert_err!(
            Permission0::revoke_permission(get_origin(agent_0), permission_id),
            pallet_permission0::Error::<Test>::NotAuthorizedToRevoke
        );

        assert_err!(
            Permission0::revoke_permission(get_origin(agent_2), permission_id),
            pallet_permission0::Error::<Test>::NotAuthorizedToRevoke
        );

        // should still be revocable by grantee
        assert_ok!(Permission0::revoke_permission(
            get_origin(agent_1),
            permission_id
        ),);
    });
}

#[test]
fn revocable_by_grantor() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        add_balance(agent_0, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_1, u16::MAX / 2), (agent_2, u16::MAX / 2)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableByGrantor,
        ));

        assert_err!(
            Permission0::revoke_permission(get_origin(agent_2), permission_id),
            pallet_permission0::Error::<Test>::NotAuthorizedToRevoke
        );

        assert_ok!(Permission0::revoke_permission(
            get_origin(agent_0),
            permission_id
        ),);
    });
}

#[test]
fn revocable_after_block() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        add_balance(agent_0, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_1, u16::MAX / 2), (agent_2, u16::MAX / 2)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableAfter(1),
        ));

        assert_err!(
            Permission0::revoke_permission(get_origin(agent_2), permission_id),
            pallet_permission0::Error::<Test>::NotAuthorizedToRevoke
        );

        step_block(2);

        assert_ok!(Permission0::revoke_permission(
            get_origin(agent_0),
            permission_id
        ),);
    });
}

#[test]
fn revocable_by_arbiters() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, to_nano(10) + 1);

        let grant_invalid = |accounts: &[AccountId], required_votes| {
            Permission0::grant_emission_permission(
                agent_0,
                agent_1,
                pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
                vec![(agent_1, u16::MAX / 2)],
                pallet_permission0_api::DistributionControl::Manual,
                pallet_permission0_api::PermissionDuration::Indefinite,
                pallet_permission0_api::RevocationTerms::RevocableByArbiters {
                    accounts: accounts.to_vec(),
                    required_votes,
                },
            )
        };

        assert_err!(
            grant_invalid(&[], 0),
            Error::<Test>::InvalidNumberOfRevokers
        );
        assert_err!(
            grant_invalid(&[0], 0),
            Error::<Test>::InvalidNumberOfRevokers
        );
        assert_err!(
            grant_invalid(&[0], 2),
            Error::<Test>::InvalidNumberOfRevokers
        );

        let arbiters = [2, 3, 4, 5];
        let not_arbiter = 6;

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_1, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::RevocableByArbiters {
                accounts: arbiters.to_vec(),
                required_votes: 2,
            },
        ));

        assert_ok!(Permission0::revoke_permission(
            get_origin(arbiters[0]),
            permission_id
        ));

        assert_err!(
            Permission0::revoke_permission(get_origin(not_arbiter), permission_id),
            pallet_permission0::Error::<Test>::NotAuthorizedToRevoke
        );

        assert_ok!(Permission0::revoke_permission(
            get_origin(arbiters[1]),
            permission_id
        ));

        assert_err!(
            Permission0::revoke_permission(get_origin(arbiters[2]), permission_id),
            Error::<Test>::PermissionNotFound
        );
    });
}
