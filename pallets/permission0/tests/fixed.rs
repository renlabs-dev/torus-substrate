use pallet_permission0_api::Permission0EmissionApi;
use polkadot_sdk::frame_support::assert_err;
use test_utils::*;

#[test]
fn fixed_fails_without_balance() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        assert_err!(
            Permission0::grant_emission_permission(
                agent_0,
                agent_1,
                pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
                vec![(agent_0, u16::MAX)],
                pallet_permission0_api::DistributionControl::Manual,
                pallet_permission0_api::PermissionDuration::Indefinite,
                pallet_permission0_api::RevocationTerms::Irrevocable,
                pallet_permission0_api::EnforcementAuthority::None,
            ),
            pallet_permission0::Error::<Test>::InsufficientBalance
        );
    });
}

#[test]
fn fixed_creates() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_0, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));
    });
}

#[test]
fn fixed_reserves() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        add_balance(agent_0, to_nano(10) + 1);

        let permission_id = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_0, u16::MAX)],
            pallet_permission0_api::DistributionControl::Manual,
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert_eq!(Balances::reserved_balance(agent_0), to_nano(10));
    });
}

#[test]
fn fixed_manual_executes() {
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
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert_ok!(Permission0::execute_permission(
            get_origin(agent_0),
            permission_id
        ));

        assert_eq!(get_balance(agent_1), to_nano(5));
        assert_eq!(get_balance(agent_2), to_nano(5));
        assert_eq!(get_balance(agent_0), 1);

        assert_ok!(Permission0::execute_permission(
            get_origin(agent_0),
            permission_id
        ));
    });
}

#[test]
fn fixed_manual_executes_only_once() {
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
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        assert!(pallet_permission0::Permissions::<Test>::contains_key(
            permission_id
        ));

        assert_ok!(Permission0::execute_permission(
            get_origin(agent_0),
            permission_id
        ));

        assert_eq!(get_balance(agent_1), to_nano(5));
        assert_eq!(get_balance(agent_2), to_nano(5));
        assert_eq!(get_balance(agent_0), 1);

        assert_ok!(Permission0::execute_permission(
            get_origin(agent_0),
            permission_id
        ));

        assert_eq!(get_balance(agent_1), to_nano(5));
        assert_eq!(get_balance(agent_2), to_nano(5));
        assert_eq!(get_balance(agent_0), 1);
    });
}

#[test]
fn fixed_at_block_executes() {
    new_test_ext().execute_with(|| {
        zero_min_burn();
        let agent_0 = 0;
        register_empty_agent(agent_0);

        let agent_1 = 1;
        register_empty_agent(agent_1);

        let agent_2 = 2;
        register_empty_agent(agent_2);

        add_balance(agent_0, to_nano(10) + 1);

        let _ = assert_ok!(Permission0::grant_emission_permission(
            agent_0,
            agent_1,
            pallet_permission0_api::EmissionAllocation::FixedAmount(to_nano(10)),
            vec![(agent_1, u16::MAX / 2), (agent_2, u16::MAX / 2)],
            pallet_permission0_api::DistributionControl::AtBlock(20),
            pallet_permission0_api::PermissionDuration::Indefinite,
            pallet_permission0_api::RevocationTerms::Irrevocable,
            pallet_permission0_api::EnforcementAuthority::None,
        ));

        step_block(40);

        assert_eq!(get_balance(agent_1), to_nano(5));
        assert_eq!(get_balance(agent_2), to_nano(5));
        assert_eq!(get_balance(agent_0), 1);
    });
}
