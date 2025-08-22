use polkadot_sdk::{
    frame_support::traits::{
        Currency, ExistenceRequirement, Imbalance, ReservableCurrency, WithdrawReasons,
    },
    frame_system,
    sp_arithmetic::FixedU128,
    sp_runtime::traits::{Saturating, Zero},
};

use super::*;

/// Type for stream ID
pub type StreamId = H256;

/// Stream-specific permission scope
#[derive(Encode, Decode, CloneNoBound, PartialEq, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct StreamScope<T: Config> {
    /// Recipients of the strams and its weights
    pub recipients: BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
    /// What portion of streams this permission applies to
    pub allocation: StreamAllocation<T>,
    /// Distribution control parameters
    pub distribution: DistributionControl<T>,
    /// Whether strams should accumulate (can be toggled by enforcement authority)
    pub accumulating: bool,
    /// An account responsible for managing the recipients to this permission's streams.
    /// If left empty, the delegator will be
    pub recipient_managers: BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>,
    /// An account responsible for updating the weights of existing recipients. Useful
    /// for third-party agents to manage how the streams will be distributed.
    pub weight_setters: BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>,
}

impl<T: Config> StreamScope<T> {
    pub(super) fn cleanup(
        self,
        permission_id: H256,
        last_executed: &Option<BlockNumberFor<T>>,
        delegator: &T::AccountId,
    ) {
        match self.allocation {
            StreamAllocation::Streams(streams) => {
                for stream in streams.keys() {
                    AccumulatedStreamAmounts::<T>::remove((delegator, stream, &permission_id));
                }
            }
            StreamAllocation::FixedAmount(amount) if last_executed.is_none() => {
                T::Currency::unreserve(delegator, amount);
            }
            _ => {}
        }
    }
}

/// Defines what portion of streams the permission applies to
#[derive(Encode, Decode, CloneNoBound, PartialEqNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub enum StreamAllocation<T: Config> {
    /// Permission applies to a percentage of each stream
    Streams(BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>),
    /// Permission applies to a specific fixed amount
    FixedAmount(BalanceOf<T>),
}

#[derive(Encode, Decode, CloneNoBound, PartialEq, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub enum DistributionControl<T: Config> {
    /// Manual distribution by the delegator
    Manual,
    /// Automatic distribution after accumulation threshold
    Automatic(BalanceOf<T>),
    /// Distribution at specific block
    AtBlock(BlockNumberFor<T>),
    /// Distribution at fixed intervals
    Interval(BlockNumberFor<T>),
}

/// Accumulate emissions for a specific agent, distributes if control is met.
pub(crate) fn do_accumulate_streams<T: Config>(
    agent: &T::AccountId,
    stream_id: &StreamId,
    imbalance: &mut NegativeImbalanceOf<T>,
) {
    let initial_balance = imbalance.peek();
    let total_initial_amount =
        FixedU128::from_inner(initial_balance.try_into().unwrap_or_default());
    if total_initial_amount.is_zero() {
        return;
    }

    let streams = AccumulatedStreamAmounts::<T>::iter_prefix((agent, stream_id));
    for (permission_id, accumulated) in streams {
        let Some(contract) = Permissions::<T>::get(permission_id) else {
            continue;
        };

        // Only process stream permissions with percentage allocations,
        // fixed-amount emission reserves balance upfront on permission creation
        let PermissionScope::Stream(StreamScope {
            allocation: StreamAllocation::Streams(streams),
            accumulating,
            ..
        }) = contract.scope
        else {
            continue;
        };

        if !accumulating {
            continue;
        }

        let Some(percentage) = streams.get(stream_id) else {
            continue;
        };

        let delegated_amount = percentage.mul_floor(total_initial_amount.into_inner());
        if delegated_amount.is_zero() {
            continue;
        }

        let delegated_amount = imbalance
            .extract(delegated_amount.try_into().unwrap_or_default())
            .peek();

        AccumulatedStreamAmounts::<T>::set(
            (agent, stream_id, &permission_id),
            Some(accumulated.saturating_add(delegated_amount)),
        );

        Pallet::<T>::deposit_event(Event::AccumulatedEmission {
            permission_id,
            stream_id: *stream_id,
            amount: delegated_amount,
        });
    }
}

pub(crate) fn do_auto_distribution<T: Config>(
    stream_scope: &StreamScope<T>,
    permission_id: H256,
    current_block: BlockNumberFor<T>,
    contract: &PermissionContract<T>,
) -> DispatchResult {
    match stream_scope.distribution {
        DistributionControl::Automatic(threshold) => {
            let accumulated = match &stream_scope.allocation {
                StreamAllocation::Streams(streams) => streams
                    .keys()
                    .filter_map(|id| {
                        AccumulatedStreamAmounts::<T>::get((&contract.delegator, id, permission_id))
                    })
                    .fold(BalanceOf::<T>::zero(), |acc, e| acc.saturating_add(e)), // The Balance AST does not enforce the Sum trait
                StreamAllocation::FixedAmount(amount) => *amount,
            };

            if accumulated >= threshold {
                do_distribute_stream::<T>(permission_id, contract, DistributionReason::Automatic)?;
            }
        }

        DistributionControl::AtBlock(target_block) if current_block > target_block => {
            // As we only verify once every 10 blocks, we have to check if current block
            // is GTE to the target block. To avoid, triggering on every block,
            // we also verify that the last execution occurred before the target block
            // (or haven't occurred at all)
            if contract
                .last_execution()
                .is_some_and(|last_execution| last_execution >= target_block)
            {
                return Ok(());
            }

            do_distribute_stream::<T>(permission_id, contract, DistributionReason::Automatic)?;
        }

        DistributionControl::Interval(interval) => {
            let last_execution = contract.last_execution.unwrap_or(contract.created_at);
            if current_block.saturating_sub(last_execution) < interval {
                return Ok(());
            }

            do_distribute_stream::<T>(permission_id, contract, DistributionReason::Automatic)?;
        }

        // Manual distribution doesn't need auto-processing
        _ => {}
    }

    Ok(())
}

#[derive(
    Encode, Decode, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, TypeInfo, MaxEncodedLen,
)]
pub enum DistributionReason {
    Automatic,
    Manual,
}

/// Distribute accumulated emissions for a permission
pub(crate) fn do_distribute_stream<T: Config>(
    permission_id: PermissionId,
    contract: &PermissionContract<T>,
    reason: DistributionReason,
) -> DispatchResult {
    let PermissionScope::Stream(stream_scope) = &contract.scope else {
        return Ok(());
    };

    let total_weight =
        FixedU128::from_u32(stream_scope.recipients.values().map(|w| *w as u32).sum());
    if total_weight.is_zero() {
        trace!("permission {permission_id:?} does not have enough target weight");
        return Ok(());
    }

    match &stream_scope.allocation {
        StreamAllocation::Streams(streams) => {
            let streams = streams.keys().filter_map(|stream_id| {
                let acc = AccumulatedStreamAmounts::<T>::get((
                    &contract.delegator,
                    stream_id,
                    permission_id,
                ))?;

                // You cannot remove the stream from the storage as
                // it's needed in the accumulation code, avoid using `take`
                AccumulatedStreamAmounts::<T>::set(
                    (&contract.delegator, stream_id, permission_id),
                    Some(Zero::zero()),
                );

                if acc.is_zero() {
                    None
                } else {
                    // For percentage allocations, mint new tokens
                    // This is safe because we're only distributing a percentage of
                    // tokens that were already allocated to emission rewards
                    Some((stream_id, T::Currency::issue(acc)))
                }
            });

            for (stream, mut imbalance) in streams {
                do_distribute_to_targets(
                    &mut imbalance,
                    permission_id,
                    stream_scope,
                    Some(stream),
                    total_weight,
                    reason,
                );

                let remainder = imbalance.peek();
                if !remainder.is_zero() {
                    AccumulatedStreamAmounts::<T>::mutate(
                        (&contract.delegator, stream, permission_id),
                        |acc| *acc = Some(acc.unwrap_or_default().saturating_add(remainder)),
                    );
                }
            }
        }
        StreamAllocation::FixedAmount(amount) => {
            if contract.last_execution().is_some() {
                // The fixed amount was already distributed
                return Ok(());
            }

            // For fixed amount allocations, transfer from reserved funds
            let _ = T::Currency::unreserve(&contract.delegator, *amount);
            let mut imbalance = T::Currency::withdraw(
                &contract.delegator,
                *amount,
                WithdrawReasons::TRANSFER,
                ExistenceRequirement::KeepAlive,
            )
            .unwrap_or_else(|_| NegativeImbalanceOf::<T>::zero());

            do_distribute_to_targets(
                &mut imbalance,
                permission_id,
                stream_scope,
                None,
                total_weight,
                reason,
            );
        }
    }

    if let Some(mut contract) = Permissions::<T>::get(permission_id) {
        contract.tick_execution(<frame_system::Pallet<T>>::block_number())?;
        Permissions::<T>::set(permission_id, Some(contract));
    }

    Ok(())
}

fn do_distribute_to_targets<T: Config>(
    imbalance: &mut NegativeImbalanceOf<T>,
    permission_id: PermissionId,
    stream_scope: &StreamScope<T>,
    stream: Option<&StreamId>,
    total_weight: FixedU128,
    reason: DistributionReason,
) {
    let initial_balance = imbalance.peek();
    let total_initial_amount =
        FixedU128::from_inner(initial_balance.try_into().unwrap_or_default());
    if total_initial_amount.is_zero() {
        trace!("no amount to distribute for permission {permission_id:?} and stream {stream:?}");
        return;
    }

    for (target, weight) in stream_scope.recipients.iter() {
        let target_weight = FixedU128::from_u32(*weight as u32);
        let target_amount = total_initial_amount
            .saturating_mul(target_weight)
            .const_checked_div(total_weight)
            .unwrap_or_default();

        if target_amount.is_zero() {
            continue;
        }

        let target_amount =
            BalanceOf::<T>::try_from(target_amount.into_inner()).unwrap_or_default();
        let mut imbalance = imbalance.extract(target_amount);

        if let Some(stream) = stream {
            // Process recursive accumulation here, only deposit what remains
            do_accumulate_streams::<T>(target, stream, &mut imbalance);
        }

        T::Currency::resolve_creating(target, imbalance);

        Pallet::<T>::deposit_event(Event::StreamDistribution {
            permission_id,
            stream_id: stream.cloned(),
            recipient: target.clone(),
            amount: target_amount,
            reason,
        });
    }
}
