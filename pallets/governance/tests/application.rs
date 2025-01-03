use pallet_governance::application::ApplicationStatus;
use pallet_governance::AgentApplications;
use pallet_governance::GlobalGovernanceConfig;
use polkadot_sdk::frame_support::assert_err;
use test_utils::*;

#[test]
fn error_is_thrown_on_applying_add_already_whitelisted() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let key = 0;
        let adding_key = 1;
        pallet_governance::Curators::<Test>::insert(key, ());
        pallet_governance::Whitelist::<Test>::insert(adding_key, ());

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, proposal_cost + 1);

        assert_err!(
            pallet_governance::Pallet::<Test>::submit_application(
                get_origin(key),
                adding_key,
                data.clone(),
                false
            ),
            pallet_governance::Error::<Test>::AlreadyWhitelisted
        );
    });
}

#[test]
fn error_is_thrown_on_applying_remove_not_whitelisted() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let key = 0;
        let adding_key = 1;
        pallet_governance::Curators::<Test>::insert(key, ());

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, proposal_cost + 1);

        assert_err!(
            pallet_governance::Pallet::<Test>::submit_application(
                get_origin(key),
                adding_key,
                data.clone(),
                true
            ),
            pallet_governance::Error::<Test>::NotWhitelisted
        );
    });
}

#[test]
fn whitelist_executes_application_correctly_add() {
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
            false,
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

        let application =
            pallet_governance::AgentApplications::<Test>::get(application_id).unwrap();
        assert_eq!(
            application.status,
            ApplicationStatus::Resolved { accepted: true }
        );
    });
}

#[test]
fn whitelist_executes_application_correctly_remove() {
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
            false
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

        let application =
            pallet_governance::AgentApplications::<Test>::get(application_id).unwrap();
        assert_eq!(
            application.status,
            ApplicationStatus::Resolved { accepted: true }
        );
    });
}

#[test]
fn error_is_thrown_on_multiple_applications_same_key() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let key = 0;
        let adding_key = 1;
        pallet_governance::Curators::<Test>::insert(key, ());

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, (proposal_cost * 2) + 1);

        assert_ok!(pallet_governance::Pallet::<Test>::submit_application(
            get_origin(key),
            adding_key,
            data.clone(),
            false
        ));

        assert_err!(
            pallet_governance::Pallet::<Test>::submit_application(
                get_origin(key),
                adding_key,
                data.clone(),
                false
            ),
            pallet_governance::Error::<Test>::ApplicationKeyAlreadyUsed
        );
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
            false,
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

        let application =
            pallet_governance::AgentApplications::<Test>::get(application_id).unwrap();
        assert_eq!(
            application.status,
            ApplicationStatus::Resolved { accepted: false }
        );
    });
}

#[test]
fn application_expires() {
    new_test_ext().execute_with(|| {
        let key = 0;
        let adding_key = 1;
        pallet_governance::Curators::<Test>::insert(key, ());

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, proposal_cost + 1);

        assert_ok!(pallet_governance::Pallet::<Test>::submit_application(
            get_origin(key),
            adding_key,
            data.clone(),
            false
        ));

        let mut application_id: u32 = u32::MAX;
        for (_, value) in AgentApplications::<Test>::iter() {
            assert_eq!(value.agent_key, adding_key);
            assert_eq!(value.data, data);
            application_id = value.id;
        }

        step_block(
            pallet_governance::GlobalGovernanceConfig::<Test>::get().agent_application_expiration,
        );

        let application =
            pallet_governance::AgentApplications::<Test>::get(application_id).unwrap();
        assert_eq!(application.status, ApplicationStatus::Expired);
    });
}
