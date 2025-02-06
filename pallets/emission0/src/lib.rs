#![cfg_attr(not(feature = "std"), no_std)]

mod ext;
pub mod migrations;

pub(crate) use ext::*;
pub use pallet::*;
use pallet_emission0_api::Emission0Api;
use polkadot_sdk::frame_support::dispatch::DispatchResult;
use polkadot_sdk::frame_support::{pallet_prelude::*, DefaultNoBound};
use polkadot_sdk::frame_system;
use polkadot_sdk::frame_system::pallet_prelude::OriginFor;
use polkadot_sdk::polkadot_sdk_frame::{self as frame, traits::Currency};
use polkadot_sdk::sp_runtime::Percent;

#[doc(hidden)]
pub mod distribute;
#[doc(hidden)]
pub mod weight_control;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarks;
pub mod weights;

#[frame::pallet]
pub mod pallet {
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    use core::num::NonZeroU128;

    use frame::prelude::BlockNumberFor;
    use pallet_governance_api::GovernanceApi;
    use pallet_torus0_api::Torus0Api;
    use polkadot_sdk::sp_std;
    use weights::WeightInfo;

    use super::*;

    /// Map of consensus members indexed by their keys. A consensus member is
    /// any agent eligible for emissions in the next epoch. This means
    /// unregistered agents will also receive emissions.
    #[pallet::storage]
    pub type ConsensusMembers<T: Config> =
        StorageMap<_, Identity, AccountIdOf<T>, ConsensusMember<T>>;

    /// Map of agents delegating weight control to other agents. Emissions
    /// derived from weight delegation are taxed and the fees go the original
    /// weight setter.
    #[pallet::storage]
    pub type WeightControlDelegation<T: Config> =
        StorageMap<_, Identity, T::AccountId, T::AccountId>;

    // TODO: remove
    #[pallet::storage]
    pub type MinAllowedWeights<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMinAllowedWeights>;

    /// Maximum number of weights a validator can set.
    #[pallet::storage]
    pub type MaxAllowedWeights<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedWeights>;

    // TODO: cap weights on distribution.
    /// Minimum stake a validator needs for each weight it sets.
    #[pallet::storage]
    pub type MinStakePerWeight<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Percentage of issued tokens to be burned every epoch.
    #[pallet::storage]
    pub type EmissionRecyclingPercentage<T: Config> =
        StorageValue<_, Percent, ValueQuery, T::DefaultEmissionRecyclingPercentage>;

    /// Ratio between incentives and dividends on distribution. 50% means they
    /// are distributed equally.
    #[pallet::storage]
    pub type IncentivesRatio<T: Config> =
        StorageValue<_, Percent, ValueQuery, T::DefaultIncentivesRatio>;

    /// Amount of tokens accumulated since the last epoch. This increases on
    /// every block. See [`distribute::get_total_emission_per_block`].
    #[pallet::storage]
    pub type PendingEmission<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        /// Tokens emitted in an interval before halving the emissions in NANOs.
        #[pallet::constant]
        type HalvingInterval: Get<NonZeroU128>;

        /// Max token supply in NANOs.
        #[pallet::constant]
        type MaxSupply: Get<NonZeroU128>;

        /// Emissions per block in NANOs. Not taking into account halving and
        /// recycling.
        #[pallet::constant]
        type BlockEmission: Get<u128>;

        #[pallet::constant]
        type DefaultMinAllowedWeights: Get<u16>;

        #[pallet::constant]
        type DefaultMaxAllowedWeights: Get<u16>;

        #[pallet::constant]
        type DefaultEmissionRecyclingPercentage: Get<Percent>;

        #[pallet::constant]
        type DefaultIncentivesRatio: Get<Percent>;

        type Currency: Currency<Self::AccountId, Balance = u128> + Send + Sync;

        type Torus: Torus0Api<
            Self::AccountId,
            <Self::Currency as Currency<Self::AccountId>>::Balance,
            <Self::Currency as Currency<Self::AccountId>>::NegativeImbalance,
        >;

        type Governance: GovernanceApi<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
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

        /// Tried setting weights for an agent that does not exist.
        AgentIsNotRegistered,

        /// Tried setting weights for itself.
        CannotSetWeightsForSelf,

        /// Tried setting weights while delegating weight control.
        CannotSetWeightsWhileDelegating,

        /// Tried delegating weight control to itself.
        CannotDelegateWeightControlToSelf,

        /// Tried regaining weight control without delegating it.
        AgentIsNotDelegating,

        /// Agent does not have enough stake to set weights.
        NotEnoughStakeToSetWeights,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub fn deposit_event)]
    pub enum Event<T: Config> {
        /// An agent set weights in the network.
        WeightsSet(T::AccountId),
        /// An agent gave weight control to the second agent.
        DelegatedWeightControl(T::AccountId, T::AccountId),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((T::WeightInfo::set_weights(), DispatchClass::Normal, Pays::Yes))]
        pub fn set_weights(
            origin: OriginFor<T>,
            weights: sp_std::vec::Vec<(AccountIdOf<T>, u16)>,
        ) -> DispatchResult {
            weight_control::set_weights::<T>(origin, weights)
        }

        #[pallet::call_index(1)]
        #[pallet::weight((T::WeightInfo::delegate_weight_control(), DispatchClass::Normal, Pays::Yes))]
        pub fn delegate_weight_control(
            origin: OriginFor<T>,
            target: AccountIdOf<T>,
        ) -> DispatchResult {
            weight_control::delegate_weight_control::<T>(origin, target)
        }

        #[pallet::call_index(2)]
        #[pallet::weight((T::WeightInfo::regain_weight_control(), DispatchClass::Normal, Pays::Yes))]
        pub fn regain_weight_control(origin: OriginFor<T>) -> DispatchResult {
            weight_control::regain_weight_control::<T>(origin)
        }
    }
}

pub type Weights<T> =
    BoundedVec<(<T as frame_system::Config>::AccountId, u16), ConstU32<{ u32::MAX }>>;

#[derive(CloneNoBound, DebugNoBound, DefaultNoBound, Decode, Encode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ConsensusMember<T: Config> {
    pub weights: Weights<T>,
    pub last_incentives: u16,
    pub last_dividends: u16,
}

impl<T: Config> ConsensusMember<T> {
    pub fn update_weights(&mut self, weights: Weights<T>) {
        self.weights = weights;
    }
}

impl<T: Config> Emission0Api<T::AccountId> for Pallet<T> {
    fn consensus_stats(
        member: &T::AccountId,
    ) -> Option<pallet_emission0_api::ConsensusMemberStats> {
        ConsensusMembers::<T>::get(member).map(|member| {
            pallet_emission0_api::ConsensusMemberStats {
                dividends: member.last_dividends,
                incentives: member.last_incentives,
            }
        })
    }
}
