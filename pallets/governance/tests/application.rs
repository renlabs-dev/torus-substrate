use pallet_emission0::PendingEmission;
use pallet_governance::{
    AgentApplications, GlobalGovernanceConfig, Whitelist, application::ApplicationStatus,
};
use pallet_governance::{DaoTreasuryAddress, TreasuryEmissionFee};
use pallet_governance_api::GovernanceApi;
use pallet_permission0::CuratorPermissions;
use polkadot_sdk::frame_support::assert_err;
use polkadot_sdk::sp_runtime::Percent;
use test_utils::*;

#[test]
fn error_is_thrown_on_applying_add_already_whitelisted() {
    new_test_ext().execute_with(|| {
        zero_min_burn();

        let key = 0;
        let adding_key = 1;
        delegate_curator_permission(key, CuratorPermissions::all(), None);
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
        delegate_curator_permission(key, CuratorPermissions::all(), None);

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
        delegate_curator_permission(key, CuratorPermissions::all(), None);

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
        let removing_key = 1;
        delegate_curator_permission(key, CuratorPermissions::all(), None);

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, proposal_cost + 1);
        // first submit an application
        let balance_before = get_balance(key);

        Whitelist::<Test>::set(removing_key, Some(()));

        assert_ok!(pallet_governance::Pallet::<Test>::submit_application(
            get_origin(key),
            removing_key,
            data.clone(),
            true
        ));

        let balance_after = get_balance(key);
        assert_eq!(balance_after, balance_before - proposal_cost);

        let mut application_id: u32 = u32::MAX;
        for (_, value) in AgentApplications::<Test>::iter() {
            assert_eq!(value.agent_key, removing_key);
            assert_eq!(value.data, data);
            application_id = value.id;
        }

        assert_ok!(pallet_governance::Pallet::<Test>::accept_application(
            get_origin(key),
            application_id
        ));

        let balance_after_accept = get_balance(key);

        assert_eq!(balance_after_accept, balance_before);

        assert!(!pallet_governance::whitelist::is_whitelisted::<Test>(
            &removing_key
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
        delegate_curator_permission(key, CuratorPermissions::all(), None);

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
        PendingEmission::<Test>::set(0);
        TreasuryEmissionFee::<Test>::set(Percent::zero());
        let balance = get_balance(DaoTreasuryAddress::<Test>::get());

        let key = 0;
        let adding_key = 1;
        delegate_curator_permission(key, CuratorPermissions::all(), None);

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
        assert_eq!(
            get_balance(Governance::dao_treasury_address()),
            balance + crate::GlobalGovernanceConfig::<Test>::get().agent_application_cost
        );
    });
}

#[test]
fn application_expires() {
    new_test_ext().execute_with(|| {
        let expiration_blocks =
            pallet_governance::GlobalGovernanceConfig::<Test>::mutate(|config| {
                config.agent_application_expiration = 200;
                config.agent_application_expiration
            });

        PendingEmission::<Test>::set(0);
        TreasuryEmissionFee::<Test>::set(Percent::zero());
        let balance = get_balance(DaoTreasuryAddress::<Test>::get());

        let key = 0;
        let adding_key = 1;
        delegate_curator_permission(key, CuratorPermissions::all(), None);

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

        step_block(expiration_blocks);

        let application =
            pallet_governance::AgentApplications::<Test>::get(application_id).unwrap();
        assert_eq!(application.status, ApplicationStatus::Expired);
        assert_eq!(
            get_balance(Governance::dao_treasury_address()),
            balance + crate::GlobalGovernanceConfig::<Test>::get().agent_application_cost
        );
    });
}

#[test]
fn error_is_thrown_on_resolving_non_open_application() {
    new_test_ext().execute_with(|| {
        let expiration_blocks =
            pallet_governance::GlobalGovernanceConfig::<Test>::mutate(|config| {
                config.agent_application_expiration = 200;
                config.agent_application_expiration
            });

        let key = 0;
        let adding_key = 1;
        delegate_curator_permission(key, CuratorPermissions::all(), None);

        let proposal_cost = GlobalGovernanceConfig::<Test>::get().agent_application_cost;
        let data = "test".as_bytes().to_vec();

        add_balance(key, proposal_cost + 1);

        assert_ok!(pallet_governance::Pallet::<Test>::submit_application(
            get_origin(key),
            adding_key,
            data.clone(),
            false
        ));
        let application_id = 0;

        // resolves the application as Accepted
        assert_ok!(pallet_governance::Pallet::<Test>::accept_application(
            get_origin(key),
            0
        ));

        let balance_after_accept = get_balance(key);
        assert_eq!(balance_after_accept, proposal_cost + 1);

        assert!(pallet_governance::whitelist::is_whitelisted::<Test>(
            &adding_key
        ));

        // tries to resolve the application as `accepted` again
        assert_err!(
            pallet_governance::Pallet::<Test>::accept_application(get_origin(key), application_id),
            pallet_governance::Error::<Test>::ApplicationNotOpen
        );

        // tries to resolve the application as not-accepted
        assert_err!(
            pallet_governance::Pallet::<Test>::deny_application(get_origin(key), application_id),
            pallet_governance::Error::<Test>::ApplicationNotOpen
        );

        // new application
        let adding_key = adding_key + 1;
        let data = "test".as_bytes().to_vec();
        assert_ok!(pallet_governance::Pallet::<Test>::submit_application(
            get_origin(key),
            adding_key,
            data.clone(),
            false
        ));
        let application_id = 1;

        // expires the application
        step_block(expiration_blocks);
        let balance_after_expire = get_balance(key);
        assert_eq!(balance_after_expire, 1);

        // tries to resolve the application as `accepted`
        assert_err!(
            pallet_governance::Pallet::<Test>::accept_application(get_origin(key), application_id),
            pallet_governance::Error::<Test>::ApplicationNotOpen
        );

        // tries to resolve the application as not-accepted
        assert_err!(
            pallet_governance::Pallet::<Test>::deny_application(get_origin(key), application_id),
            pallet_governance::Error::<Test>::ApplicationNotOpen
        );
    });
}
