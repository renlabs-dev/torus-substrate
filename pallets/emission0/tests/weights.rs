use pallet_emission0::{
    weights::{delegate_weight_control, regain_weight_control, set_weights},
    ConsensusMembers, Error, MaxAllowedWeights, MinStakePerWeight, WeightControlDelegation,
    Weights,
};
use test_utils::{add_stake, get_origin, register_empty_agent, Test};

#[test]
fn delegates_and_regains_weight_control() {
    test_utils::new_test_ext().execute_with(|| {
        let delegator = 0;
        let delegated = 1;

        assert_eq!(
            delegate_weight_control::<Test>(get_origin(delegator), delegator),
            Err(Error::<Test>::CannotDelegateWeightControlToSelf.into())
        );

        assert_eq!(
            delegate_weight_control::<Test>(get_origin(delegator), delegated),
            Err(Error::<Test>::AgentIsNotRegistered.into())
        );

        assert_eq!(
            regain_weight_control::<Test>(get_origin(delegator)),
            Err(Error::<Test>::AgentIsNotDelegating.into())
        );

        register_empty_agent(delegator);

        assert_eq!(
            delegate_weight_control::<Test>(get_origin(delegator), delegated),
            Err(Error::<Test>::AgentIsNotRegistered.into())
        );

        register_empty_agent(delegated);

        assert_eq!(
            delegate_weight_control::<Test>(get_origin(delegator), delegated),
            Ok(())
        );

        assert!(WeightControlDelegation::<Test>::contains_key(delegator));

        assert_eq!(regain_weight_control::<Test>(get_origin(delegator)), Ok(()));

        assert!(!WeightControlDelegation::<Test>::contains_key(delegator));
    });
}

#[test]
fn sets_weights_correctly() {
    test_utils::new_test_ext().execute_with(|| {
        MaxAllowedWeights::<Test>::set(5);
        MinStakePerWeight::<Test>::set(1);

        let validator = 0;

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(0, 0); 6]),
            Err(Error::<Test>::WeightSetTooLarge.into()),
        );

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(0, 0); 5]),
            Err(Error::<Test>::AgentIsNotRegistered.into()),
        );

        register_empty_agent(validator);

        assert_eq!(
            set_weights::<Test>(get_origin(validator), vec![(0, 0); 5]),
            Err(Error::<Test>::NotEnoughStakeToSetWeights.into()),
        );

        add_stake(validator, validator, 3);

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
