#![cfg_attr(not(feature = "std"), no_std)]

mod ext;
mod weights;

pub(crate) use ext::*;
pub use pallet::*;
use polkadot_sdk::frame_support::dispatch::DispatchResult;
use polkadot_sdk::frame_support::{pallet_prelude::*, DefaultNoBound};
use polkadot_sdk::frame_system;
use polkadot_sdk::frame_system::pallet_prelude::OriginFor;
use polkadot_sdk::polkadot_sdk_frame::prelude::BlockNumberFor;
use polkadot_sdk::polkadot_sdk_frame::{self as frame, traits::Currency};

mod distribute;

#[frame::pallet(dev_mode)]
pub mod pallet {
    use core::num::NonZeroU128;

    use frame::prelude::BlockNumberFor;
    use pallet_torus0_api::Torus0Api;
    use polkadot_sdk::sp_std;

    use super::*;
    #[pallet::storage]
    pub type ConsensusMembers<T: Config> =
        StorageMap<_, Identity, AccountIdOf<T>, ConsensusMember<T>>;

    #[pallet::storage]
    pub type WeightControlDelegation<T: Config> =
        StorageDoubleMap<_, Identity, u16, Identity, T::AccountId, T::AccountId>;

    #[pallet::storage]
    pub type MinAllowedWeights<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMinAllowedWeights>;

    #[pallet::storage]
    pub type MaxAllowedWeights<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedWeights>;

    #[pallet::storage]
    pub type PendingEmission<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        /// Tokens emitted in an interval before halving the emissions in NANOs.
        #[pallet::constant]
        type HalvingInterval: Get<NonZeroU128>;

        /// Max token supply in NANOs.
        #[pallet::constant]
        type MaxSupply: Get<NonZeroU128>;

        // TODO: add to config ConstU128<(250.000 * 10^18 - 1) / 10800>, where 18 is the number of decimals.
        /// Emissions per block in NANOs. Not taking into account halving and recycling.
        #[pallet::constant]
        type BlockEmission: Get<u128>;

        #[pallet::constant]
        type DefaultMinAllowedWeights: Get<u16>;

        #[pallet::constant]
        type DefaultMaxAllowedWeights: Get<u16>;

        type Currency: Currency<Self::AccountId, Balance = u128> + Send + Sync;

        type Torus: Torus0Api<
            Self::AccountId,
            <Self::Currency as Currency<Self::AccountId>>::Balance,
            <Self::Currency as Currency<Self::AccountId>>::NegativeImbalance,
        >;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            distribute::distribute_emission::<T>(block_number);

            Weight::zero()
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Agent tried setting more than 2 ^ 32 weights.
        WeightSetTooLarge,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO: configure price
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_weights_extrinsic(
            origin: OriginFor<T>,
            weights: sp_std::vec::Vec<(AccountIdOf<T>, u16)>,
        ) -> DispatchResult {
            weights::set_weights::<T>(origin, weights)
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn delegate_weight_control_extrinsic(
            origin: OriginFor<T>,
            target: AccountIdOf<T>,
        ) -> DispatchResult {
            weights::delegate_weight_control::<T>(origin, target)
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn regain_weight_control_extrinsic(origin: OriginFor<T>) -> DispatchResult {
            weights::regain_weight_control::<T>(origin)
        }
    }
}

type Weights<T> = BoundedVec<(<T as frame_system::Config>::AccountId, u16), ConstU32<{ u32::MAX }>>;

#[derive(DebugNoBound, DefaultNoBound, Decode, Encode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ConsensusMember<T: Config> {
    weights: Weights<T>,
    weights_last_updated_at: BlockNumberFor<T>,
    pruning_score: u16,
}

impl<T: Config> ConsensusMember<T> {
    pub fn update_weights(&mut self, weights: Weights<T>) {
        self.weights = weights;
        self.weights_last_updated_at = <frame_system::Pallet<T>>::block_number();
    }
}
