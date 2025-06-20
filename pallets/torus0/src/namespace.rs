use codec::{Decode, Encode, MaxEncodedLen};
use pallet_governance_api::GovernanceApi;
use polkadot_sdk::{
    frame_support::{
        traits::{ExistenceRequirement, ReservableCurrency},
        CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound,
    },
    frame_system::{self, pallet_prelude::BlockNumberFor},
    sp_runtime::{
        traits::{One, Saturating, Zero},
        DispatchResult, FixedPointNumber, FixedU128,
    },
};
use scale_info::TypeInfo;

use crate::*;

pub use pallet_torus0_api::{
    NamespacePath, MAX_NAMESPACE_PATH_LENGTH, MAX_NAMESPACE_SEGMENTS, MAX_SEGMENT_LENGTH,
    NAMESPACE_SEPARATOR,
};

/// Describes the ownership of the namespace.
#[derive(
    Encode, Decode, CloneNoBound, DebugNoBound, PartialEqNoBound, EqNoBound, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub enum NamespaceOwnership<T: Config> {
    /// Owned by the system. Used to register root-level namespaces, like agent.
    System,
    /// Owned by a SS58 account. Used to represent agent-owned namespaces.
    Account(T::AccountId),
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct NamespacePricingConfig<T: Config> {
    /// Tokens held per byte inserted.
    pub deposit_per_byte: BalanceOf<T>,
    /// Used as the base fee for the logistic function that
    /// calculates the fees based on the amount of namespace entries
    /// for a given agent.
    pub base_fee: BalanceOf<T>,

    /// The logistic function's midpoint.
    /// The function is performed over the number of namespace entries
    /// the agent has registered. The midpoint determines at how many
    /// entries for that agent the price reaches it's mid-point.
    pub count_midpoint: u32,
    /// How steep the pricing increase is.
    pub fee_steepness: Percent,
    /// The maximum multiplier for the base fee.
    pub max_fee_multiplier: u32,
}

impl<T: Config> NamespacePricingConfig<T> {
    /// Calculate namespace cost using sigmoid pricing model.
    pub fn namespace_fee(
        &self,
        account_namespace_count: u32,
    ) -> Result<BalanceOf<T>, polkadot_sdk::sp_runtime::DispatchError> {
        let Self {
            base_fee,
            count_midpoint,
            fee_steepness,
            max_fee_multiplier,
            ..
        } = self;

        let multiplier = {
            let fee_steepness = fee_steepness.deconstruct() as f64 / 100.;
            let position = (account_namespace_count as i64).saturating_sub(*count_midpoint as i64);
            let adjusted = -fee_steepness * position as f64;
            let exp = (libm::exp(adjusted) * FixedU128::DIV as f64) as u128;

            let max_fee_multiplier = FixedU128::from_u32(*max_fee_multiplier);

            FixedU128::one().saturating_add(
                max_fee_multiplier.saturating_mul(
                    FixedU128::one()
                        .const_checked_div(
                            FixedU128::one().saturating_add(FixedU128::from_inner(exp)),
                        )
                        .unwrap_or_default(),
                ),
            )
        };

        let base_fee = FixedU128::from_inner(*base_fee);
        Ok(base_fee.saturating_mul(multiplier).into_inner())
    }

    /// Calculates the deposit needed to register a namespace. If a path with the agent prefix
    /// is provided and contains more segments, the prefix (agent literal and agent name)
    /// will be dropped.
    pub fn namespace_deposit(&self, path: &NamespacePath) -> BalanceOf<T> {
        let segments: Vec<_> = path.segments().collect();
        let mut segments = &segments[..];

        if segments.first() == Some(&&b"agent"[..]) && segments.len() > 2 {
            segments = segments.get(2..).unwrap_or(segments);
        }

        let bytes: u32 = segments.iter().map(|segment| segment.len() as u32).sum();
        let dots = segments.len().saturating_sub(1) as u32;

        self.deposit_per_byte
            .saturating_mul(bytes.saturating_add(dots).into())
    }

    /// The fee midpoint.
    pub fn fee_midpoint(&self) -> BalanceOf<T> {
        self.base_fee.saturating_add(
            FixedU128::from_u32(self.max_fee_multiplier)
                .const_checked_div(FixedU128::from_u32(2))
                .unwrap_or_default()
                .saturating_mul(FixedU128::from_inner(self.base_fee))
                .into_inner(),
        )
    }

    /// The fee theoretical ceiling.
    pub fn fee_ceiling(&self) -> BalanceOf<T> {
        self.base_fee.saturating_add(
            FixedU128::from_u32(self.max_fee_multiplier)
                .saturating_mul(FixedU128::from_inner(self.base_fee))
                .into_inner(),
        )
    }
}

/// Metadata stored for each namespace
#[derive(
    Encode, Decode, CloneNoBound, PartialEqNoBound, EqNoBound, TypeInfo, MaxEncodedLen, DebugNoBound,
)]
#[scale_info(skip_type_params(T))]
pub struct NamespaceMetadata<T: Config> {
    /// Block number when the namespace was created
    pub created_at: BlockNumberFor<T>,
    /// Storage deposit paid for this namespace
    pub deposit: BalanceOf<T>,
}

pub fn find_missing_paths<T: Config>(
    owner: &NamespaceOwnership<T>,
    path: &NamespacePath,
) -> Vec<NamespacePath> {
    let mut paths_to_create = path.parents();
    paths_to_create.insert(0, path.clone());

    let mut iter = paths_to_create.iter().enumerate().rev();
    if matches!(owner, NamespaceOwnership::Account(_))
        && path.root().as_ref().map(NamespacePath::as_bytes) == Some(&b"agent"[..])
    {
        // We drop the first segment, agent
        let _ = iter.by_ref().next();
    }

    for (i, segment) in iter {
        if !Namespaces::<T>::contains_key(owner, segment) {
            return paths_to_create.get(..=i).unwrap_or_default().to_vec();
        }
    }

    Default::default()
}

/// Calculates the total cost for registering, (Fee, Deposit)
pub fn calculate_cost<T: Config>(
    owner: &NamespaceOwnership<T>,
    missing_paths: &[NamespacePath],
) -> Result<(BalanceOf<T>, BalanceOf<T>), DispatchError> {
    let current_count = NamespaceCount::<T>::get(owner);

    let pricing_config = crate::NamespacePricingConfig::<T>::get();
    let mut total_fee = BalanceOf::<T>::zero();
    let mut total_deposit = BalanceOf::<T>::zero();

    for (index, path) in missing_paths.iter().enumerate() {
        let count = current_count.saturating_add(index as u32);
        let fee = pricing_config.namespace_fee(count)?;
        let deposit = pricing_config.namespace_deposit(path);

        total_fee = total_fee.saturating_add(fee);
        total_deposit = total_deposit.saturating_add(deposit);
    }

    Ok((total_fee, total_deposit))
}

pub fn create_namespace<T: Config>(
    owner: NamespaceOwnership<T>,
    path: NamespacePath,
) -> DispatchResult {
    #[allow(deprecated)]
    create_namespace0(owner, path, true)
}

#[doc(hidden)]
#[deprecated = "use crate_namespace instead"]
pub(crate) fn create_namespace0<T: Config>(
    owner: NamespaceOwnership<T>,
    path: NamespacePath,
    charge: bool,
) -> DispatchResult {
    ensure!(
        !Namespaces::<T>::contains_key(&owner, &path),
        Error::<T>::NamespaceAlreadyExists
    );

    let missing_paths = find_missing_paths::<T>(&owner, &path);

    if charge {
        let (total_fee, total_deposit) = calculate_cost::<T>(&owner, &missing_paths)?;

        if let NamespaceOwnership::Account(owner) = &owner {
            T::Currency::reserve(owner, total_deposit)?;

            <T as crate::Config>::Currency::transfer(
                owner,
                &<T as crate::Config>::Governance::dao_treasury_address(),
                total_fee,
                ExistenceRequirement::AllowDeath,
            )
            .map_err(|_| crate::Error::<T>::NotEnoughBalanceToRegisterAgent)?;
        }
    }

    let current_block = <frame_system::Pallet<T>>::block_number();
    let pricing_config = crate::NamespacePricingConfig::<T>::get();

    for path in missing_paths.iter() {
        let deposit = if charge {
            pricing_config.namespace_deposit(path)
        } else {
            Zero::zero()
        };

        let metadata = NamespaceMetadata {
            created_at: current_block,
            deposit,
        };

        Namespaces::<T>::insert(&owner, path, metadata);
    }

    NamespaceCount::<T>::mutate(&owner, |count| {
        *count = count.saturating_add(missing_paths.len() as u32)
    });

    Pallet::<T>::deposit_event(Event::NamespaceCreated { owner, path });

    Ok(())
}

pub fn delete_namespace<T: Config>(
    owner: NamespaceOwnership<T>,
    path: NamespacePath,
) -> DispatchResult {
    ensure!(
        Namespaces::<T>::contains_key(&owner, &path),
        Error::<T>::NamespaceNotFound
    );

    let mut total_deposit = BalanceOf::<T>::zero();
    let namespaces_to_delete: Vec<_> = Namespaces::<T>::iter_prefix(&owner)
        .filter_map(|(other, metadata)| {
            if other == path || path.is_parent_of(&other) {
                Some((other, metadata.deposit))
            } else {
                None
            }
        })
        .collect();

    let deleted_count = namespaces_to_delete.len() as u32;

    for (path_to_delete, deposit) in namespaces_to_delete {
        total_deposit = total_deposit.saturating_add(deposit);
        Namespaces::<T>::remove(&owner, &path_to_delete);
    }

    NamespaceCount::<T>::mutate(&owner, |count| *count = count.saturating_sub(deleted_count));

    if let NamespaceOwnership::Account(owner) = &owner {
        T::Currency::unreserve(owner, total_deposit);
    }

    Pallet::<T>::deposit_event(Event::NamespaceDeleted { owner, path });

    Ok(())
}
