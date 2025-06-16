use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound},
    frame_system::pallet_prelude::BlockNumberFor,
    sp_runtime::{
        traits::{One, Saturating, Zero},
        FixedPointNumber, FixedU128,
    },
};
use scale_info::TypeInfo;

use crate::*;

pub use pallet_torus0_api::{
    NamespacePath, MAX_NAMESPACE_PATH_LENGTH, MAX_NAMESPACE_SEGMENTS, MAX_SEGMENT_LENGTH,
    NAMESPACE_SEPARATOR,
};

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

    /// Calculates the deposit needed to register a namespace.
    pub fn namespace_deposit(&self, path: &NamespacePath) -> BalanceOf<T> {
        self.deposit_per_byte
            .saturating_mul((path.as_bytes().len() as u32).into())
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
    owner: &T::AccountId,
    path: &NamespacePath,
) -> Vec<NamespacePath> {
    let mut paths_to_create = path.parents();
    paths_to_create.insert(0, path.clone());

    for (i, segment) in paths_to_create.iter().enumerate().rev() {
        if !Namespaces::<T>::contains_key(owner, segment) {
            return paths_to_create.get(..=i).unwrap_or_default().to_vec();
        }
    }

    Default::default()
}

/// Calculates the total cost for registering, (Fee, Deposit)
pub fn calculate_cost<T: Config>(
    owner: &T::AccountId,
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
