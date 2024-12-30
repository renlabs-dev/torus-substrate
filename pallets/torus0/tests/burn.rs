use test_utils::Test;

#[test]
fn burn_with_reached_interval() {
    test_utils::new_test_ext().execute_with(|| {
        pallet_torus0::BurnConfig::<Test>::mutate(|burn_config| {
            burn_config.min_burn = 10;
            burn_config.max_burn = 1000;
            burn_config.target_registrations_interval = 200;
            burn_config.target_registrations_per_interval = 27;
            burn_config.adjustment_alpha = 9223372036854775807;
        });

        pallet_torus0::burn::adjust_burn::<Test>(2200);

        assert_eq!(pallet_torus0::Burn::<Test>::get(), 10);
        assert_eq!(pallet_torus0::RegistrationsThisInterval::<Test>::get(), 0);
    });
}

#[test]
fn burn_with_min_burn() {
    test_utils::new_test_ext().execute_with(|| {
        pallet_torus0::BurnConfig::<Test>::mutate(|burn_config| {
            burn_config.min_burn = 10;
            burn_config.max_burn = 1000;
            burn_config.target_registrations_interval = 200;
            burn_config.target_registrations_per_interval = 25;
            burn_config.adjustment_alpha = 9223372036854775807;
        });

        pallet_torus0::burn::adjust_burn::<Test>(2200);

        assert_eq!(pallet_torus0::Burn::<Test>::get(), 10);
        assert_eq!(pallet_torus0::RegistrationsThisInterval::<Test>::get(), 0);
    });
}

#[test]
fn burn_with_max_burn() {
    test_utils::new_test_ext().execute_with(|| {
        pallet_torus0::BurnConfig::<Test>::mutate(|burn_config| {
            burn_config.min_burn = 10;
            burn_config.max_burn = 1000;
            burn_config.target_registrations_interval = 200;
            burn_config.target_registrations_per_interval = 25;
            burn_config.adjustment_alpha = 9223372036854775807;
        });

        pallet_torus0::Burn::<Test>::set(35);
        pallet_torus0::RegistrationsThisInterval::<Test>::set(3584);

        pallet_torus0::burn::adjust_burn::<Test>(22000);

        assert_eq!(pallet_torus0::Burn::<Test>::get(), 1000);
        assert_eq!(pallet_torus0::RegistrationsThisInterval::<Test>::get(), 0);
    });
}

#[test]
fn burn_in_bounds() {
    test_utils::new_test_ext().execute_with(|| {
        pallet_torus0::BurnConfig::<Test>::mutate(|burn_config| {
            burn_config.min_burn = 10;
            burn_config.max_burn = 1500;
            burn_config.target_registrations_interval = 200;
            burn_config.target_registrations_per_interval = 25;
            burn_config.adjustment_alpha = 9223372036854775807;
        });

        pallet_torus0::Burn::<Test>::set(35);
        pallet_torus0::RegistrationsThisInterval::<Test>::set(3584);

        pallet_torus0::burn::adjust_burn::<Test>(22000);

        assert_eq!(pallet_torus0::Burn::<Test>::get(), 1280);
        assert_eq!(pallet_torus0::RegistrationsThisInterval::<Test>::get(), 0);
    });
}
