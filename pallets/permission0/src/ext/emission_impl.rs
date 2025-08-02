use crate::{
    AccumulatedStreamAmounts, BalanceOf, Config, DistributionControl, EmissionAllocation,
    EmissionScope, EnforcementTracking, Error, Event, NegativeImbalanceOf, Pallet,
    PermissionContract, PermissionDuration, PermissionId, PermissionScope, Permissions,
    generate_permission_id, get_total_allocated_percentage, pallet,
    permission::{emission::*, *},
    update_permission_indices,
};

use pallet_permission0_api::{
    DistributionControl as ApiDistributionControl, EmissionAllocation as ApiEmissionAllocation,
    EnforcementAuthority as ApiEnforcementAuthority, Permission0EmissionApi,
    PermissionDuration as ApiPermissionDuration, RevocationTerms as ApiRevocationTerms, StreamId,
};
use polkadot_sdk::{
    frame_support::{dispatch::DispatchResult, ensure, traits::ReservableCurrency},
    frame_system::{ensure_signed, ensure_signed_or_root},
    polkadot_sdk_frame::prelude::{BlockNumberFor, OriginFor},
    sp_core::{Get, TryCollect},
    sp_runtime::{
        BoundedBTreeMap, DispatchError, Percent, Vec,
        traits::{CheckedAdd, Saturating, Zero},
    },
};

use pallet_torus0_api::Torus0Api;

impl<T: Config>
    Permission0EmissionApi<
        T::AccountId,
        OriginFor<T>,
        BlockNumberFor<T>,
        crate::BalanceOf<T>,
        NegativeImbalanceOf<T>,
    > for pallet::Pallet<T>
{
    fn delegate_emission_permission(
        delegator: T::AccountId,
        recipient: T::AccountId,
        allocation: ApiEmissionAllocation<crate::BalanceOf<T>>,
        targets: Vec<(T::AccountId, u16)>,
        distribution: ApiDistributionControl<crate::BalanceOf<T>, BlockNumberFor<T>>,
        duration: ApiPermissionDuration<BlockNumberFor<T>>,
        revocation: ApiRevocationTerms<T::AccountId, BlockNumberFor<T>>,
        enforcement: ApiEnforcementAuthority<T::AccountId>,
    ) -> Result<PermissionId, DispatchError> {
        let internal_allocation = match allocation {
            ApiEmissionAllocation::Streams(streams) => EmissionAllocation::Streams(
                streams
                    .try_into()
                    .map_err(|_| crate::Error::<T>::TooManyStreams)?,
            ),
            ApiEmissionAllocation::FixedAmount(amount) => EmissionAllocation::FixedAmount(amount),
        };

        let internal_distribution = match distribution {
            ApiDistributionControl::Manual => DistributionControl::Manual,
            ApiDistributionControl::Automatic(threshold) => {
                DistributionControl::Automatic(threshold)
            }
            ApiDistributionControl::AtBlock(block) => DistributionControl::AtBlock(block),
            ApiDistributionControl::Interval(interval) => DistributionControl::Interval(interval),
        };

        let duration = super::translate_duration(duration)?;
        let revocation = super::translate_revocation_terms(revocation)?;
        let enforcement = super::translate_enforcement_authority(enforcement)?;

        let targets = targets
            .into_iter()
            .try_collect()
            .map_err(|_| crate::Error::<T>::TooManyTargets)?;

        delegate_emission_permission_impl::<T>(
            delegator,
            recipient,
            internal_allocation,
            targets,
            internal_distribution,
            duration,
            revocation,
            enforcement,
            None, // No parent by default
        )
    }

    fn accumulate_emissions(
        agent: &T::AccountId,
        stream: &StreamId,
        amount: &mut NegativeImbalanceOf<T>,
    ) {
        crate::permission::emission::do_accumulate_emissions::<T>(agent, stream, amount);
    }

    fn process_auto_distributions(current_block: BlockNumberFor<T>) {
        crate::permission::do_auto_permission_execution::<T>(current_block);
    }

    fn get_accumulated_amount(
        permission_id: &PermissionId,
        stream: &StreamId,
    ) -> crate::BalanceOf<T> {
        let Some(contract) = Permissions::<T>::get(permission_id) else {
            return Zero::zero();
        };

        crate::AccumulatedStreamAmounts::<T>::get((contract.delegator, stream, permission_id))
            .unwrap_or_default()
    }
}

/// Delegate a permission implementation
#[allow(clippy::too_many_arguments)]
pub(crate) fn delegate_emission_permission_impl<T: Config>(
    delegator: T::AccountId,
    recipient: T::AccountId,
    allocation: EmissionAllocation<T>,
    targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
    distribution: DistributionControl<T>,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
    enforcement: EnforcementAuthority<T>,
    parent_id: Option<PermissionId>,
) -> Result<PermissionId, DispatchError> {
    use polkadot_sdk::frame_support::ensure;

    ensure!(
        T::Torus::is_agent_registered(&delegator),
        Error::<T>::NotRegisteredAgent
    );
    ensure!(
        T::Torus::is_agent_registered(&recipient),
        Error::<T>::NotRegisteredAgent
    );

    validate_emission_permission_target_weights::<T>(&targets)?;

    match &allocation {
        EmissionAllocation::Streams(streams) => {
            validate_emission_permission_streams::<T>(streams, &delegator)?;
        }
        EmissionAllocation::FixedAmount(amount) => {
            ensure!(*amount > BalanceOf::<T>::zero(), Error::<T>::InvalidAmount);
            ensure!(
                T::Currency::can_reserve(&delegator, *amount),
                Error::<T>::InsufficientBalance
            );
            ensure!(
                matches!(
                    &distribution,
                    DistributionControl::Manual | DistributionControl::AtBlock(_)
                ),
                Error::<T>::FixedAmountCanOnlyBeTriggeredOnce
            );
        }
    }

    validate_emission_permission_distribution::<T>(&distribution)?;

    if let Some(parent) = parent_id {
        let parent_contract =
            Permissions::<T>::get(parent).ok_or(Error::<T>::ParentPermissionNotFound)?;

        ensure!(
            parent_contract.recipient == delegator,
            Error::<T>::NotPermissionRecipient
        );
    }

    let emission_scope = EmissionScope {
        allocation: allocation.clone(),
        distribution,
        targets,
        accumulating: true, // Start with accumulation enabled by default
    };

    let scope = PermissionScope::Emission(emission_scope);

    let permission_id = generate_permission_id::<T>(&delegator, &recipient, &scope)?;

    let contract = PermissionContract::<T>::new(
        delegator.clone(),
        recipient.clone(),
        scope,
        duration,
        revocation,
        enforcement,
        1,
    );

    // Reserve funds if fixed amount allocation. We use the Balances API for this.
    // This means total issuance is always correct.
    match allocation {
        EmissionAllocation::FixedAmount(amount) => {
            T::Currency::reserve(&delegator, amount)?;
        }
        EmissionAllocation::Streams(streams) => {
            for stream in streams.keys() {
                AccumulatedStreamAmounts::<T>::set(
                    (&delegator, stream, permission_id),
                    Some(Zero::zero()),
                )
            }
        }
    }

    Permissions::<T>::insert(permission_id, contract);

    update_permission_indices::<T>(&delegator, &recipient, permission_id)?;

    <Pallet<T>>::deposit_event(Event::PermissionDelegated {
        delegator,
        recipient,
        permission_id,
    });

    Ok(permission_id)
}

pub fn execute_permission_impl<T: Config>(
    permission_id: &PermissionId,
    contract: &PermissionContract<T>,
    emission_scope: &EmissionScope<T>,
) -> DispatchResult {
    match &emission_scope.distribution {
        DistributionControl::Manual => {
            ensure!(
                emission_scope.accumulating,
                Error::<T>::UnsupportedPermissionType
            );

            let accumulated = match &emission_scope.allocation {
                EmissionAllocation::Streams(streams) => streams
                    .keys()
                    .filter_map(|id| {
                        AccumulatedStreamAmounts::<T>::get((&contract.delegator, id, permission_id))
                    })
                    .fold(BalanceOf::<T>::zero(), |acc, e| acc.saturating_add(e)), // The Balance AST does not enforce the Sum trait
                EmissionAllocation::FixedAmount(amount) => *amount,
            };

            ensure!(!accumulated.is_zero(), Error::<T>::NoAccumulatedAmount);

            crate::permission::emission::do_distribute_emission::<T>(
                *permission_id,
                contract,
                DistributionReason::Manual,
            )?;

            Ok(())
        }
        _ => Err(Error::<T>::InvalidDistributionMethod.into()),
    }
}

/// Toggle a permission's accumulation state
pub fn toggle_permission_accumulation_impl<T: Config>(
    origin: OriginFor<T>,
    permission_id: PermissionId,
    accumulating: bool,
) -> DispatchResult {
    let who = ensure_signed_or_root(origin)?;

    let mut contract =
        Permissions::<T>::get(permission_id).ok_or(Error::<T>::PermissionNotFound)?;

    if let Some(who) = &who {
        match &contract.enforcement {
            _ if who == &contract.delegator => {}
            EnforcementAuthority::None => {
                return Err(Error::<T>::NotAuthorizedToToggle.into());
            }
            EnforcementAuthority::ControlledBy {
                controllers,
                required_votes,
            } => {
                ensure!(controllers.contains(who), Error::<T>::NotAuthorizedToToggle);

                let referendum = EnforcementReferendum::EmissionAccumulation(accumulating);
                let votes = EnforcementTracking::<T>::get(permission_id, &referendum)
                    .into_iter()
                    .filter(|id| id != who)
                    .filter(|id| controllers.contains(id))
                    .count();

                if votes.saturating_add(1) < *required_votes as usize {
                    return EnforcementTracking::<T>::mutate(
                        permission_id,
                        referendum.clone(),
                        |votes| {
                            votes
                                .try_insert(who.clone())
                                .map_err(|_| Error::<T>::TooManyControllers)?;

                            <Pallet<T>>::deposit_event(Event::EnforcementVoteCast {
                                permission_id,
                                voter: who.clone(),
                                referendum,
                            });

                            Ok(())
                        },
                    );
                }
            }
        }
    }

    match &mut contract.scope {
        PermissionScope::Emission(emission_scope) => emission_scope.accumulating = accumulating,
        _ => return Err(Error::<T>::UnsupportedPermissionType.into()),
    }

    Permissions::<T>::insert(permission_id, contract);

    // Clear any votes for this referendum
    EnforcementTracking::<T>::remove(
        permission_id,
        EnforcementReferendum::EmissionAccumulation(accumulating),
    );

    <Pallet<T>>::deposit_event(Event::PermissionAccumulationToggled {
        permission_id,
        accumulating,
        toggled_by: who,
    });

    Ok(())
}

pub(crate) fn update_emission_permission<T: Config>(
    origin: OriginFor<T>,
    permission_id: PermissionId,
    new_targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
    new_streams: Option<BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>>,
    new_distribution_control: Option<DistributionControl<T>>,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    let permission = Permissions::<T>::get(permission_id);

    let Some(mut permission) = permission else {
        return Err(Error::<T>::PermissionNotFound.into());
    };

    let allowed_delegator = permission.delegator == who && permission.is_updatable();
    let allowed_recipient = permission.recipient == who
        && (new_streams.is_none() && new_distribution_control.is_none());

    if !allowed_delegator && !allowed_recipient {
        return Err(Error::<T>::NotAuthorizedToEdit.into());
    }

    let mut scope = permission.scope.clone();
    match &mut scope {
        PermissionScope::Emission(emission_scope) => {
            validate_emission_permission_target_weights::<T>(&new_targets)?;

            emission_scope.targets = new_targets;

            let EmissionAllocation::Streams(streams) = &mut emission_scope.allocation else {
                return Err(Error::<T>::NotEditable.into());
            };

            if let Some(new_streams) = new_streams {
                crate::permission::emission::do_distribute_emission::<T>(
                    permission_id,
                    &permission,
                    DistributionReason::Manual,
                )?;

                for stream in streams.keys() {
                    AccumulatedStreamAmounts::<T>::remove((
                        &permission.delegator,
                        stream,
                        &permission_id,
                    ));
                }

                validate_emission_permission_streams::<T>(&new_streams, &permission.delegator)?;

                for stream in new_streams.keys() {
                    AccumulatedStreamAmounts::<T>::set(
                        (&permission.delegator, stream, permission_id),
                        Some(Zero::zero()),
                    )
                }

                *streams = new_streams;
            }

            if let Some(new_distribution_control) = new_distribution_control {
                validate_emission_permission_distribution::<T>(&new_distribution_control)?;

                emission_scope.distribution = new_distribution_control;
            }
        }
        _ => return Err(Error::<T>::NotEditable.into()),
    }

    permission.scope = scope;
    Permissions::<T>::set(permission_id, Some(permission));

    Ok(())
}

fn validate_emission_permission_target_weights<T: Config>(
    targets: &BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
) -> DispatchResult {
    ensure!(!targets.is_empty(), Error::<T>::NoTargetsSpecified);

    for (target, weight) in targets {
        ensure!(*weight > 0, Error::<T>::InvalidTargetWeight);
        ensure!(
            T::Torus::is_agent_registered(target),
            Error::<T>::NotRegisteredAgent
        );
    }

    Ok(())
}
fn validate_emission_permission_streams<T: Config>(
    streams: &BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>,
    delegator: &T::AccountId,
) -> DispatchResult {
    for (stream, percentage) in streams {
        ensure!(*percentage <= Percent::one(), Error::<T>::InvalidPercentage);

        let total_allocated = get_total_allocated_percentage::<T>(delegator, stream);
        let new_total_allocated = match total_allocated.checked_add(percentage) {
            Some(new_total_allocated) => new_total_allocated,
            None => return Err(Error::<T>::TotalAllocationExceeded.into()),
        };

        ensure!(
            new_total_allocated <= Percent::one(),
            Error::<T>::TotalAllocationExceeded
        );
    }

    Ok(())
}

fn validate_emission_permission_distribution<T: Config>(
    distribution: &DistributionControl<T>,
) -> DispatchResult {
    match distribution {
        DistributionControl::Automatic(threshold) => {
            ensure!(
                *threshold >= T::MinAutoDistributionThreshold::get(),
                Error::<T>::InvalidThreshold
            );
        }
        DistributionControl::Interval(interval) => {
            ensure!(!interval.is_zero(), Error::<T>::InvalidInterval);
        }
        DistributionControl::AtBlock(block) => {
            let current_block = <polkadot_sdk::frame_system::Pallet<T>>::block_number();
            ensure!(*block > current_block, Error::<T>::InvalidInterval);
        }
        _ => {}
    }

    Ok(())
}
