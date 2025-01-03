use pallet_governance::AgentApplications;
use pallet_governance::GlobalGovernanceConfig;
use test_utils::*;

#[test]
fn whitelist_executes_application_correctly() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let key = 0;
        let adding_key = 1;
        pallet_governance::Curators::<Test>::insert(key, ());

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, proposal_cost + 1);
        // first submit an application
        let balance_before = get_balance(key);

        assert_ok!(pallet_governance::Pallet::<Test>::submit_application(
            get_origin(key),
            adding_key,
            data.clone(),
        ));

        let balance_after = get_balance(key);
        assert_eq!(balance_after, balance_before - proposal_cost);

        let mut application_id: u32 = u32::MAX;
        for (_, value) in AgentApplications::<Test>::iter() {
            assert_eq!(value.agent_key, adding_key);
            assert_eq!(value.data, data);
            application_id = value.id;
        }

        assert_ok!(pallet_governance::Pallet::<Test>::accept_application(
            get_origin(key),
            application_id
        ));

        let balance_after_accept = get_balance(key);

        assert_eq!(balance_after_accept, balance_before);

        assert!(pallet_governance::whitelist::is_whitelisted::<Test>(
            &adding_key
        ));
    });
}

#[test]
fn application_denied_doesnt_add_to_whitelist() {
    new_test_ext().execute_with(|| {
        let key = 0;
        let adding_key = 1;
        pallet_governance::Curators::<Test>::insert(key, ());

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, proposal_cost + 1);
        let balance_before = get_balance(key);

        assert_ok!(pallet_governance::Pallet::<Test>::submit_application(
            get_origin(key),
            adding_key,
            data.clone(),
        ));

        let balance_after = get_balance(key);
        assert_eq!(balance_after, balance_before - proposal_cost);

        let mut application_id: u32 = u32::MAX;
        for (_, value) in AgentApplications::<Test>::iter() {
            assert_eq!(value.agent_key, adding_key);
            assert_eq!(value.data, data);
            application_id = value.id;
        }

        assert_ok!(pallet_governance::Pallet::<Test>::deny_application(
            get_origin(key),
            application_id
        ));

        let balance_after_accept = get_balance(key);

        assert_eq!(balance_after_accept, balance_after);

        assert!(!pallet_governance::whitelist::is_whitelisted::<Test>(
            &adding_key
        ));
    });
}
