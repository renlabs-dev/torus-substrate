use crate::{
    AccumulatedStreamAmounts, BalanceOf, Config, DistributionControl, EmissionAllocation,
    EmissionScope, EnforcementTracking, Error, Event, NegativeImbalanceOf, Pallet,
    PermissionContract, PermissionDuration, PermissionId, PermissionScope, Permissions,
    generate_permission_id, get_total_allocated_percentage, pallet,
    permission::{emission::*, *},
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
        BoundedBTreeMap, BoundedBTreeSet, DispatchError, Percent, Vec,
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
        recipients: Vec<(T::AccountId, u16)>,
        allocation: ApiEmissionAllocation<crate::BalanceOf<T>>,
        distribution: ApiDistributionControl<crate::BalanceOf<T>, BlockNumberFor<T>>,
        duration: ApiPermissionDuration<BlockNumberFor<T>>,
        revocation: ApiRevocationTerms<T::AccountId, BlockNumberFor<T>>,
        enforcement: ApiEnforcementAuthority<T::AccountId>,
        recipient_manager: Option<T::AccountId>,
        weight_setter: Option<T::AccountId>,
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

        let recipients = recipients
            .into_iter()
            .try_collect()
            .map_err(|_| crate::Error::<T>::TooManyTargets)?;

        delegate_emission_permission_impl::<T>(
            delegator,
            recipients,
            internal_allocation,
            internal_distribution,
            duration,
            revocation,
            enforcement,
            recipient_manager,
            weight_setter,
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
    recipients: BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
    allocation: EmissionAllocation<T>,
    distribution: DistributionControl<T>,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
    enforcement: EnforcementAuthority<T>,
    recipient_manager: Option<T::AccountId>,
    weight_setter: Option<T::AccountId>,
) -> Result<PermissionId, DispatchError> {
    use polkadot_sdk::frame_support::ensure;

    ensure!(
        T::Torus::is_agent_registered(&delegator),
        Error::<T>::NotRegisteredAgent
    );

    validate_emission_permission_recipients::<T>(&delegator, &recipients)?;

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

    let recipients_ids: Vec<_> = recipients.keys().cloned().collect();

    let scope = PermissionScope::Emission(EmissionScope {
        recipients,
        allocation: allocation.clone(),
        distribution,
        accumulating: true, // Start with accumulation enabled by default
        recipient_managers: validate_emission_managers::<T>(&delegator, recipient_manager)?,
        weight_setters: validate_emission_managers::<T>(&delegator, weight_setter)?,
    });

    let permission_id = generate_permission_id::<T>(&delegator, &scope)?;

    let contract = PermissionContract::<T>::new(
        delegator.clone(),
        scope,
        duration,
        revocation,
        enforcement,
        1,
    );

    Permissions::<T>::insert(permission_id, contract);

    add_permission_indices::<T>(&delegator, recipients_ids.iter(), permission_id)?;

    <Pallet<T>>::deposit_event(Event::PermissionDelegated {
        delegator: delegator.clone(),
        permission_id,
    });

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
    new_recipients: Option<BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>>,
    new_streams: Option<BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>>,
    new_distribution_control: Option<DistributionControl<T>>,
    new_recipient_manager: Option<Option<T::AccountId>>,
    new_weight_setter: Option<Option<T::AccountId>>,
) -> DispatchResult {
    let caller = ensure_signed(origin)?;

    let permission = Permissions::<T>::get(permission_id);

    let Some(mut permission) = permission else {
        return Err(Error::<T>::PermissionNotFound.into());
    };

    ensure!(permission.is_updatable(), Error::<T>::NotAuthorizedToEdit);

    let PermissionScope::Emission(mut scope) = permission.scope.clone() else {
        return Err(Error::<T>::NotEditable.into());
    };

    let allowed_delegator = permission.delegator == caller;
    let allowed_weights = allowed_delegator || scope.weight_setters.contains(&caller);
    let allowed_recipients = allowed_delegator || scope.recipient_managers.contains(&caller);

    if !allowed_delegator && !allowed_weights && !allowed_recipients {
        return Err(Error::<T>::NotAuthorizedToEdit.into());
    }

    if let Some(new_recipients) = new_recipients {
        if !allowed_recipients
            && (new_recipients.len() != scope.recipients.len()
                || new_recipients
                    .keys()
                    .any(|k| !scope.recipients.contains_key(k)))
        {
            return Err(Error::<T>::NotAuthorizedToEdit.into());
        }

        validate_emission_permission_recipients::<T>(&permission.delegator, &new_recipients)?;

        // Remove old indices for current recipients
        crate::permission::remove_permission_from_indices::<T>(
            &permission.delegator,
            scope.recipients.keys(),
            permission_id,
        );

        // Update recipients
        scope.recipients = new_recipients;

        // Add new indices for updated recipients
        crate::permission::add_permission_indices::<T>(
            &permission.delegator,
            scope.recipients.keys(),
            permission_id,
        )?;
    }

    if let Some(new_streams) = new_streams {
        ensure!(allowed_delegator, Error::<T>::NotAuthorizedToEdit);

        let EmissionAllocation::Streams(streams) = &mut scope.allocation else {
            return Err(Error::<T>::NotEditable.into());
        };

        crate::permission::emission::do_distribute_emission::<T>(
            permission_id,
            &permission,
            DistributionReason::Manual,
        )?;

        for stream in streams.keys() {
            AccumulatedStreamAmounts::<T>::remove((&permission.delegator, stream, &permission_id));
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
        ensure!(allowed_delegator, Error::<T>::NotAuthorizedToEdit);

        validate_emission_permission_distribution::<T>(&new_distribution_control)?;
        scope.distribution = new_distribution_control;
    }

    if let Some(new_recipient_manager) = new_recipient_manager {
        ensure!(allowed_delegator, Error::<T>::NotAuthorizedToEdit);
        scope.recipient_managers =
            validate_emission_managers::<T>(&permission.delegator, new_recipient_manager)?;
    }

    if let Some(new_weight_setter) = new_weight_setter {
        ensure!(allowed_delegator, Error::<T>::NotAuthorizedToEdit);
        scope.weight_setters =
            validate_emission_managers::<T>(&permission.delegator, new_weight_setter)?;
    }

    permission.scope = PermissionScope::Emission(scope);
    Permissions::<T>::set(permission_id, Some(permission));

    Ok(())
}

fn validate_emission_managers<T: Config>(
    delegator: &T::AccountId,
    entry: Option<T::AccountId>,
) -> Result<BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>, DispatchError> {
    let mut set = BoundedBTreeSet::new();
    let _ = set.try_insert(delegator.clone());
    if let Some(entry) = entry {
        let _ = set.try_insert(entry);
    }
    Ok(set)
}

fn validate_emission_permission_recipients<T: Config>(
    delegator: &T::AccountId,
    recipients: &BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
) -> DispatchResult {
    ensure!(!recipients.is_empty(), Error::<T>::NoTargetsSpecified);

    for (recipient, weight) in recipients {
        ensure!(delegator != recipient, Error::<T>::InvalidTargetWeight);
        ensure!(*weight > 0, Error::<T>::InvalidTargetWeight);
        ensure!(
            T::Torus::is_agent_registered(recipient),
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
