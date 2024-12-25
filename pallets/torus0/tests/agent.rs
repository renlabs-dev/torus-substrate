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
