use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound},
    frame_system::pallet_prelude::BlockNumberFor,
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
        _account_namespace_count: u32,
    ) -> Result<BalanceOf<T>, polkadot_sdk::sp_runtime::DispatchError> {
        let Self {
            base_fee,
            // count_midpoint,
            // fee_steepness,
            // max_fee_multiplier,
            ..
        } = self;

        // let multiplier = {
        //     let position = (fee_steepness
        //         .mul_floor(account_namespace_count.saturating_sub(*count_midpoint))
        //         as i32)
        //         .checked_neg()
        //         .unwrap_or_default();
        //     FixedU128::from_rational(
        //         (((position as f64).exp()) * FixedU128::DIV as f64) as u128,
        //         *max_fee_multiplier as u128,
        //     ).saturating_pow(exp)
        // };

        // Ok(FixedU128::from_inner(*base_fee)
        //     .saturating_mul(multiplier)
        //     .into_inner())

        Ok(*base_fee)
    }

    /// Calculates the deposit needed to register a namespace.
    pub fn namespace_deposit(&self, path: &NamespacePath) -> BalanceOf<T> {
        self.deposit_per_byte
            .saturating_mul((path.as_bytes().len() as u32).into())
    }
}

// impl<T: Config> Default for NamespacePricingConfig<T> {
//     fn default() -> Self {
//         Self {
//             deposit_per_byte: (),
//             base_fee: (),
//             count_midpoint: (),
//             fee_steepness: (),
//             max_fee_multiplier: (),
//         }
//     }
// }

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
