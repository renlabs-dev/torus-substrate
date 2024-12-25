use pallet_torus0::Error;
use polkadot_sdk::{frame_support::assert_err, sp_runtime::Percent};
use test_utils::{assert_ok, Test};

#[test]
fn test_register_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        let agent = pallet_torus0::Agents::<Test>::get(agent).expect("it should exists");

        assert_eq!(agent.name.to_vec(), name);
        assert_eq!(agent.url.to_vec(), url);
        assert_eq!(agent.metadata.to_vec(), metadata);
    });
}

#[test]
fn test_unregister_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent,
            name.clone(),
            url.clone(),
            metadata.clone(),
        ));

        assert_ok!(pallet_torus0::agent::unregister::<Test>(agent));

        assert!(pallet_torus0::Agents::<Test>::get(agent).is_none());
    });
}

#[test]
fn test_update_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent, name, url, metadata,
        ));

        let new_name = "new-agent".as_bytes().to_vec();
        let new_url = "new-idk://agent".as_bytes().to_vec();
        let new_metadata = "new-idk://agent".as_bytes().to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_ok!(pallet_torus0::agent::update::<Test>(
            agent,
            new_name.clone(),
            new_url.clone(),
            Some(new_metadata.clone()),
            Some(staking_fee),
            Some(weight_control_fee),
        ));

        let fee = pallet_torus0::Fee::<Test>::get(agent);
        let agent = pallet_torus0::Agents::<Test>::get(agent).expect("it should exists");

        assert_eq!(agent.name.to_vec(), new_name);
        assert_eq!(agent.url.to_vec(), new_url);
        assert_eq!(agent.metadata.to_vec(), new_metadata);
        assert_eq!(fee.staking_fee, staking_fee);
        assert_eq!(fee.weight_control_fee, weight_control_fee);
    });
}

#[test]
fn test_update_with_zero_staking_fee() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent, name, url, metadata,
        ));

        let new_name = "new-agent".as_bytes().to_vec();
        let new_url = "new-idk://agent".as_bytes().to_vec();
        let new_metadata = "new-idk://agent".as_bytes().to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
                new_name.clone(),
                new_url.clone(),
                Some(new_metadata.clone()),
                Some(Percent::zero()),
                Some(weight_control_fee),
            ),
            Error::<Test>::InvalidStakingFee,
        );

        let fee = pallet_torus0::Fee::<Test>::get(agent);
        let agent = pallet_torus0::Agents::<Test>::get(agent).expect("it should exists");

        assert_eq!(agent.name.to_vec(), new_name);
        assert_eq!(agent.url.to_vec(), new_url);
        assert_eq!(agent.metadata.to_vec(), new_metadata);
        assert_eq!(fee.staking_fee, staking_fee);
        assert_eq!(fee.weight_control_fee, weight_control_fee);
    });
}

#[test]
fn test_update_with_zero_weight_control_fee() {
    test_utils::new_test_ext().execute_with(|| {
        let agent = 0;
        let name = "agent".as_bytes().to_vec();
        let url = "idk://agent".as_bytes().to_vec();
        let metadata = "idk://agent".as_bytes().to_vec();

        assert_ok!(pallet_torus0::agent::register::<Test>(
            agent, name, url, metadata,
        ));

        let new_name = "new-agent".as_bytes().to_vec();
        let new_url = "new-idk://agent".as_bytes().to_vec();
        let new_metadata = "new-idk://agent".as_bytes().to_vec();

        let constraints = pallet_torus0::FeeConstraints::<Test>::get();
        let staking_fee = constraints.min_staking_fee;
        let weight_control_fee = constraints.min_weight_control_fee;

        assert_err!(
            pallet_torus0::agent::update::<Test>(
                agent,
                new_name.clone(),
                new_url.clone(),
                Some(new_metadata.clone()),
                Some(staking_fee),
                Some(Percent::zero()),
            ),
            Error::<Test>::InvalidWeightControlFee,
        );

        let fee = pallet_torus0::Fee::<Test>::get(agent);
        let agent = pallet_torus0::Agents::<Test>::get(agent).expect("it should exists");

        assert_eq!(agent.name.to_vec(), new_name);
        assert_eq!(agent.url.to_vec(), new_url);
        assert_eq!(agent.metadata.to_vec(), new_metadata);
        assert_eq!(fee.staking_fee, staking_fee);
        assert_eq!(fee.weight_control_fee, weight_control_fee);
    });
}
