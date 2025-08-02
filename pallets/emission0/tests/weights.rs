use pallet_emission0::{
    ConsensusMembers, Error, WeightControlDelegation, Weights,
    weight_control::{delegate_weight_control, regain_weight_control, set_weights},
};
use test_utils::{
    Test, add_stake, get_origin, pallet_governance::Allocators, pallet_torus0::MinValidatorStake,
    register_empty_agent,
};

#[test]
fn delegates_and_regains_weight_control() {
    test_utils::new_test_ext().execute_with(|| {
        let delegator = 0;
        let delegated = 1;

        assert_eq!(
            delegate_weight_control::<Test>(delegator, delegator),
            Err(Error::<Test>::CannotDelegateWeightControlToSelf.into())
        );

        assert_eq!(
            delegate_weight_control::<Test>(delegator, delegated),
            Err(Error::<Test>::AgentIsNotRegistered.into())
        );

        // TODO: reenable when weight control is enabled again
        // assert_eq!(
        //     regain_weight_control::<Test>(get_origin(delegator)),
        //     Err(Error::<Test>::AgentIsNotDelegating.into())
        // );

        register_empty_agent(delegator);

        assert_eq!(
            delegate_weight_control::<Test>(delegator, delegated),
            Err(Error::<Test>::AgentIsNotRegistered.into())
        );

        register_empty_agent(delegated);

        delegate_weight_control::<Test>(delegator, delegated)
            .expect_err("cannot delegate to not-allocator");

        Allocators::<Test>::set(delegated, Some(()));

        assert_eq!(
            delegate_weight_control::<Test>(delegator, delegated),
            Ok(())
        );

        assert!(WeightControlDelegation::<Test>::contains_key(delegator));

        assert_eq!(
            regain_weight_control::<Test>(get_origin(delegator)),
            Err(Error::<Test>::WeightControlNotEnabled.into())
        );

        // TODO: reenable when weight control is enabled
        // assert_eq!(regain_weight_control::<Test>(get_origin(delegator)), Ok(()));
        // assert!(!WeightControlDelegation::<Test>::contains_key(delegator));
    });
}

#[test]
#[allow(unreachable_code)]
fn sets_weights_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        let validator = 0;

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(0, 0); 5]),
            Err(test_utils::pallet_governance::Error::<Test>::NotAllocator.into()),
        );

        Allocators::<Test>::insert(0, ());

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(0, 0); 5]),
            Err(Error::<Test>::AgentIsNotRegistered.into()),
        );

        register_empty_agent(validator);

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(0, 0); 5]),
            Err(Error::<Test>::NotEnoughStakeToSetWeights.into()),
        );

        add_stake(validator, validator, MinValidatorStake::<Test>::get());

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(0, 0); 5]),
            Err(Error::<Test>::CannotSetWeightsForSelf.into()),
        );

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(1, 0); 5]),
            Err(Error::<Test>::AgentIsNotRegistered.into()),
        );

        register_empty_agent(1);
        register_empty_agent(2);

        Allocators::<Test>::set(1, Some(()));

        delegate_weight_control::<Test>(validator, 1).expect("failed to delegate weight control");

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(1, 0); 5]),
            Err(Error::<Test>::CannotSetWeightsWhileDelegating.into()),
        );

        regain_weight_control::<Test>(get_origin(validator))
            .expect("failed to regain weight control");

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(1, 0), (1, 0), (2, 0)]),
            Ok(()),
        );

        assert_eq!(
            ConsensusMembers::<Test>::get(validator)
                .expect("weights were not set")
                .weights,
            Weights::<Test>::truncate_from(vec![(1, 0), (2, 0)])
        );
    });
}
