use crate::{
    generate_permission_id, get_total_allocated_percentage, pallet, update_permission_indices,
    AccumulatedAmounts, BalanceOf, Config, DistributionControl, EmissionAllocation, EmissionScope,
    Error, Event, Pallet, PermissionContract, PermissionDuration, PermissionId, PermissionScope,
    Permissions, RevocationTerms,
};
use pallet_permission0_api::{
    DistributionControl as ApiDistributionControl, EmissionAllocation as ApiEmissionAllocation,
    Permission0Api, PermissionDuration as ApiPermissionDuration,
    RevocationTerms as ApiRevocationTerms,
};
use pallet_torus0_api::Torus0Api;
use polkadot_sdk::{
    frame_support::{
        ensure,
        traits::{Currency, Get, ReservableCurrency},
        BoundedBTreeMap, BoundedVec,
    },
    frame_system::{self, ensure_signed_or_root},
    polkadot_sdk_frame::prelude::{BlockNumberFor, OriginFor},
    sp_runtime::{
        traits::{Saturating, Zero},
        DispatchError, DispatchResult, Percent, Vec,
    },
    sp_std::collections::btree_map::BTreeMap,
};

/// Implementation of the Permission0Api trait to be used externally
impl<T: Config>
    Permission0Api<
        T::AccountId,
        OriginFor<T>,
        BlockNumberFor<T>,
        crate::BalanceOf<T>,
        <T::Currency as Currency<T::AccountId>>::NegativeImbalance,
    > for pallet::Pallet<T>
{
    fn permission_exists(id: &PermissionId) -> bool {
        Permissions::<T>::contains_key(id)
    }

    fn grant_emission_permission(
        grantor: T::AccountId,
        grantee: T::AccountId,
        allocation: ApiEmissionAllocation<crate::BalanceOf<T>>,
        targets: Vec<(T::AccountId, u16)>,
        distribution: ApiDistributionControl<crate::BalanceOf<T>, BlockNumberFor<T>>,
        duration: ApiPermissionDuration<BlockNumberFor<T>>,
        revocation: ApiRevocationTerms<T::AccountId, BlockNumberFor<T>>,
    ) -> Result<PermissionId, DispatchError> {
        let internal_allocation = match allocation {
            ApiEmissionAllocation::Percentage(percentage) => {
                EmissionAllocation::Percentage(percentage)
            }
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

        let internal_duration = match duration {
            ApiPermissionDuration::Blocks(blocks) => PermissionDuration::Blocks(blocks),
            ApiPermissionDuration::UntilBlock(block) => PermissionDuration::UntilBlock(block),
            ApiPermissionDuration::Indefinite => PermissionDuration::Indefinite,
        };

        let internal_revocation = match revocation {
            ApiRevocationTerms::Irrevocable => RevocationTerms::Irrevocable,
            ApiRevocationTerms::RevocableByGrantor => RevocationTerms::RevocableByGrantor,
            ApiRevocationTerms::RevocableByArbiters {
                accounts,
                required_votes,
            } => {
                let bounded_accounts =
                    BoundedVec::<T::AccountId, T::MaxTargetsPerPermission>::try_from(accounts)
                        .map_err(|_| crate::Error::<T>::TooManyTargets)?;

                RevocationTerms::RevocableByArbiters {
                    accounts: bounded_accounts,
                    required_votes,
                }
            }
            ApiRevocationTerms::RevocableAfter(blocks) => RevocationTerms::RevocableAfter(blocks),
        };

        grant_permission_impl::<T>(
            grantor,
            grantee,
            internal_allocation,
            targets,
            internal_distribution,
            internal_duration,
            internal_revocation,
            None,
        )
    }

    fn revoke_permission(who: OriginFor<T>, permission_id: &PermissionId) -> DispatchResult {
        revoke_permission_impl::<T>(who, permission_id)
    }

    fn execute_permission(who: OriginFor<T>, permission_id: &PermissionId) -> DispatchResult {
        execute_permission_impl::<T>(who, permission_id)
    }

    fn accumulate_emissions(
        agent: &T::AccountId,
        amount: &mut <T::Currency as Currency<T::AccountId>>::NegativeImbalance,
    ) {
        crate::permission::emission::do_accumulate_emissions::<T>(agent, amount);
    }

    fn process_auto_distributions(current_block: BlockNumberFor<T>) {
        crate::permission::do_auto_permission_execution::<T>(current_block);
    }

    fn get_accumulated_amount(permission_id: &PermissionId) -> crate::BalanceOf<T> {
        AccumulatedAmounts::<T>::get(permission_id).unwrap_or_else(Zero::zero)
    }
}

/// Grant a permission implementation
pub(crate) fn grant_permission_impl<T: Config>(
    grantor: T::AccountId,
    grantee: T::AccountId,
    allocation: EmissionAllocation<T>,
    targets: Vec<(T::AccountId, u16)>,
    distribution: DistributionControl<T>,
    duration: PermissionDuration<T>,
    revocation: RevocationTerms<T>,
    parent_id: Option<PermissionId>,
) -> Result<PermissionId, DispatchError> {
    use polkadot_sdk::frame_support::ensure;

    ensure!(
        T::Torus::is_agent_registered(&grantor),
        Error::<T>::NotRegisteredAgent
    );
    ensure!(
        T::Torus::is_agent_registered(&grantee),
        Error::<T>::NotRegisteredAgent
    );
    ensure!(grantor != grantee, Error::<T>::SelfPermissionNotAllowed);
    ensure!(!targets.is_empty(), Error::<T>::NoTargetsSpecified);

    for (target, _) in &targets {
        ensure!(
            T::Torus::is_agent_registered(target),
            Error::<T>::NotRegisteredAgent
        );
    }

    match allocation {
        EmissionAllocation::Percentage(percentage) => {
            ensure!(percentage <= Percent::one(), Error::<T>::InvalidPercentage);

            let total_allocated = get_total_allocated_percentage::<T>(&grantor);
            ensure!(
                total_allocated.saturating_add(percentage) <= Percent::one(),
                Error::<T>::TotalAllocationExceeded
            );
        }
        EmissionAllocation::FixedAmount(amount) => {
            ensure!(amount > BalanceOf::<T>::zero(), Error::<T>::InvalidAmount);
            ensure!(
                T::Currency::can_reserve(&grantor, amount),
                Error::<T>::InsufficientBalance
            );
        }
    }

    match &distribution {
        DistributionControl::Automatic(threshold) => {
            ensure!(
                *threshold >= T::MinAutoDistributionThreshold::get(),
                Error::<T>::InvalidThreshold
            );
        }
        DistributionControl::Interval(interval) => {
            ensure!(!interval.is_zero(), Error::<T>::InvalidInterval);
        }
        _ => {}
    }

    if let Some(parent) = parent_id {
        let parent_contract =
            Permissions::<T>::get(parent).ok_or(Error::<T>::ParentPermissionNotFound)?;

        ensure!(
            parent_contract.grantee == grantor,
            Error::<T>::NotPermissionGrantee
        );

        // Additional validations for parent-child relationship could be added here
    }

    let mut target_map = BTreeMap::new();
    for (target, weight) in targets {
        target_map.insert(target, weight);
    }

    let bounded_targets: BoundedBTreeMap<_, _, T::MaxTargetsPerPermission> =
        BoundedBTreeMap::try_from(target_map).map_err(|_| Error::<T>::TooManyTargets)?;

    let emission_scope = EmissionScope {
        allocation: allocation.clone(),
        distribution: distribution,
        targets: bounded_targets,
    };

    let scope = PermissionScope::Emission(emission_scope);

    let permission_id = generate_permission_id::<T>(&grantor, &grantee, &scope);

    let contract = PermissionContract {
        grantor: grantor.clone(),
        grantee: grantee.clone(),
        scope,
        duration,
        revocation,
        last_execution: None,
        execution_count: 0,
        parent: parent_id,
        created_at: <frame_system::Pallet<T>>::block_number(),
    };

    // Reserve funds if fixed amount allocation. We use the Balances API for this.
    // This means total issuance is always correct.
    if let EmissionAllocation::FixedAmount(amount) = allocation {
        T::Currency::reserve(&grantor, amount)?;
        AccumulatedAmounts::<T>::insert(permission_id, amount);
    }

    Permissions::<T>::insert(permission_id, contract);

    update_permission_indices::<T>(&grantor, &grantee, permission_id)?;

    let percentage = match allocation {
        EmissionAllocation::Percentage(p) => Some(p),
        _ => None,
    };

    <Pallet<T>>::deposit_event(Event::PermissionGranted {
        grantor,
        grantee,
        permission_id,
        percentage,
    });

    Ok(permission_id)
}

/// Revoke a permission implementation
pub(crate) fn revoke_permission_impl<T: Config>(
    origin: OriginFor<T>,
    permission_id: &PermissionId,
) -> DispatchResult {
    let contract = Permissions::<T>::get(permission_id).ok_or(Error::<T>::PermissionNotFound)?;
    contract.revoke(origin, *permission_id)
}

/// Execute a permission implementation
pub(crate) fn execute_permission_impl<T: Config>(
    who: OriginFor<T>,
    permission_id: &PermissionId,
) -> DispatchResult {
    let who = ensure_signed_or_root(who)?;

    let contract = Permissions::<T>::get(permission_id).ok_or(Error::<T>::PermissionNotFound)?;

    let grantor = contract.grantor.clone();
    let grantee = contract.grantee.clone();

    ensure!(
        who.is_none() || who.as_ref() == Some(&grantor),
        Error::<T>::NotPermissionGrantor
    );

    match &contract.scope {
        PermissionScope::Emission(emission_scope) => match emission_scope.distribution {
            DistributionControl::Manual => {
                let accumulated =
                    AccumulatedAmounts::<T>::get(permission_id).unwrap_or_else(Zero::zero);
                ensure!(!accumulated.is_zero(), Error::<T>::NoAccumulatedAmount);

                let amount = crate::permission::emission::do_distribute_emission::<T>(
                    *permission_id,
                    &contract,
                );

                <Pallet<T>>::deposit_event(crate::Event::PermissionExecuted {
                    grantor,
                    grantee,
                    permission_id: *permission_id,
                    amount,
                });
            }
            _ => return Err(Error::<T>::InvalidDistributionMethod.into()),
        },
        #[allow(unreachable_patterns)]
        _ => return Err(Error::<T>::UnsupportedPermissionType.into()),
    }

    Ok(())
}
