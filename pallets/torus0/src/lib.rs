#![cfg_attr(not(feature = "std"), no_std)]

pub mod agent;
pub mod balance;
pub mod burn;
mod ext;
pub mod fee;
pub mod stake;

use crate::agent::Agent;
use crate::burn::BurnConfiguration;
use crate::fee::ValidatorFeeConstraints;
pub(crate) use ext::*;
use frame::arithmetic::Percent;
use frame::prelude::ensure_signed;
pub use pallet::*;
use polkadot_sdk::frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::{ValueQuery, *},
    traits::Currency,
    Identity,
};
use polkadot_sdk::frame_system::pallet_prelude::OriginFor;
use polkadot_sdk::polkadot_sdk_frame as frame;
use polkadot_sdk::sp_std;
use scale_info::prelude::vec::Vec;

#[frame::pallet]
pub mod pallet {

    use frame::prelude::BlockNumberFor;
    use pallet_emission0_api::Emission0Api;
    use pallet_governance_api::GovernanceApi;

    use super::*;

    #[pallet::storage]
    pub type MaxAllowedValidators<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedValidators>;

    #[pallet::storage]
    pub type Burn<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub type IncentiveRatio<T: Config> = StorageValue<_, u16, ValueQuery>;

    #[pallet::storage]
    pub type RegistrationsThisInterval<T: Config> = StorageValue<_, u16, ValueQuery>;

    #[pallet::storage]
    pub type MinValidatorStake<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, T::DefaultMinValidatorStake>;

    #[pallet::storage]
    pub type ImmunityPeriod<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultImmunityPeriod>;

    #[pallet::storage]
    pub type RewardInterval<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultRewardInterval>;

    #[pallet::storage]
    pub type Agents<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, Agent<T>>;

    #[pallet::storage]
    pub type RegistrationBlock<T: Config> =
        StorageMap<_, Identity, AccountIdOf<T>, BlockNumberFor<T>>;

    #[pallet::storage]
    pub type MaxNameLength<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultMaxNameLength>;

    #[pallet::storage]
    pub type MinNameLength<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultMinNameLength>;

    #[pallet::storage]
    pub type MaxAllowedAgents<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedAgents>;

    #[pallet::storage]
    pub type RegistrationsThisBlock<T> = StorageValue<_, u16, ValueQuery>;

    #[pallet::storage]
    pub type MaxRegistrationsPerBlock<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxRegistrationsPerBlock>;

    #[pallet::storage]
    pub type StakingTo<T: Config> =
        StorageDoubleMap<_, Identity, T::AccountId, Identity, T::AccountId, BalanceOf<T>>;

    #[pallet::storage]
    pub type StakedBy<T: Config> =
        StorageDoubleMap<_, Identity, T::AccountId, Identity, T::AccountId, BalanceOf<T>>;

    #[pallet::storage]
    pub type TotalStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub type MinAllowedStake<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, T::DefaultMinAllowedStake>;

    #[pallet::storage]
    pub type DividendsParticipationWeight<T: Config> =
        StorageValue<_, Percent, ValueQuery, T::DefaultDividendsParticipationWeight>;

    #[pallet::storage]
    pub type FeeConstraints<T: Config> = StorageValue<_, ValidatorFeeConstraints<T>, ValueQuery>;

    #[pallet::storage]
    pub type BurnConfig<T: Config> = StorageValue<_, BurnConfiguration<T>, ValueQuery>;

    #[pallet_section]
    pub mod hooks {
        #[pallet::hooks]
        impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
            fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
                let current_block: u64 = block_number
                    .try_into()
                    .ok()
                    .expect("blockchain won't pass 2 ^ 64 blocks");

                burn::adjust_burn::<T>(current_block);

                RegistrationsThisBlock::<T>::set(0);

                Weight::default()
            }
        }
    }

    #[pallet::config(with_default)]
    pub trait Config: polkadot_sdk::frame_system::Config {
        #[pallet::constant]
        type DefaultMaxAllowedValidators: Get<u16>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultMinValidatorStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultImmunityPeriod: Get<u16>;

        #[pallet::constant]
        type DefaultRewardInterval: Get<u16>;

        #[pallet::constant]
        type DefaultMinNameLength: Get<u16>;

        #[pallet::constant]
        type DefaultMaxNameLength: Get<u16>;

        #[pallet::constant]
        type DefaultMaxAllowedAgents: Get<u16>;

        #[pallet::constant]
        type DefaultMaxRegistrationsPerBlock: Get<u16>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultMinAllowedStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultMinStakingFee: Get<u8>;

        #[pallet::constant]
        type DefaultMinWeightControlFee: Get<u8>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultMinBurn: Get<BalanceOf<Self>>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultMaxBurn: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultAdjustmentAlpha: Get<u64>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultTargetRegistrationsInterval: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultTargetRegistrationsPerInterval: Get<u16>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultMaxRegistrationsPerInterval: Get<u16>;

        /// The storage MaxNameLength should be constrained to be no more than the value of this.
        /// This is needed on agent::Agent to set the `name` field BoundedVec max length.
        #[pallet::constant]
        type MaxAgentNameLengthConstraint: Get<u32>;

        /// This is needed on agent::Agent to set the `address` field BoundedVec max length.
        #[pallet::constant]
        type MaxAgentUrlLengthConstraint: Get<u32>;

        #[pallet::constant]
        type MaxAgentMetadataLengthConstraint: Get<u32>;

        #[pallet::constant]
        type DefaultDividendsParticipationWeight: Get<Percent>;

        #[pallet::no_default_bounds]
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        type Currency: Currency<Self::AccountId, Balance = u128> + Send + Sync;

        type Governance: GovernanceApi<Self::AccountId>;

        type Emission: Emission0Api<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((Weight::zero(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_stake(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let key = ensure_signed(origin)?;
            stake::add_stake::<T>(key, agent_key, amount)
        }

        #[pallet::call_index(1)]
        #[pallet::weight((Weight::zero(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_stake(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let key = ensure_signed(origin)?;
            stake::remove_stake::<T>(key, agent_key, amount)
        }

        #[pallet::call_index(2)]
        #[pallet::weight((Weight::zero(), DispatchClass::Normal, Pays::Yes))]
        pub fn transfer_stake(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            new_agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let key = ensure_signed(origin)?;
            stake::transfer_stake::<T>(key, agent_key, new_agent_key, amount)
        }

        #[pallet::call_index(3)]
        #[pallet::weight((Weight::zero(), DispatchClass::Normal, Pays::Yes))]
        pub fn transfer_balance(
            origin: OriginFor<T>,
            destination: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let key = ensure_signed(origin)?;
            balance::transfer_balance::<T>(key, destination, amount)
        }

        #[pallet::call_index(4)]
        #[pallet::weight((Weight::zero(), DispatchClass::Normal, Pays::Yes))]
        pub fn register_agent(
            origin: OriginFor<T>,
            agent_key: T::AccountId,
            name: Vec<u8>,
            url: Vec<u8>,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            agent::register::<T>(agent_key, name, url, metadata)
        }

        #[pallet::call_index(5)]
        #[pallet::weight((Weight::zero(), DispatchClass::Normal, Pays::Yes))]
        pub fn unregister_agent(origin: OriginFor<T>) -> DispatchResult {
            let agent_key = ensure_signed(origin)?;
            agent::unregister::<T>(agent_key)
        }

        #[pallet::call_index(6)]
        #[pallet::weight((Weight::zero(), DispatchClass::Normal, Pays::Yes))]
        pub fn update_agent(
            origin: OriginFor<T>,
            name: Vec<u8>,
            address: Vec<u8>,
            metadata: Option<Vec<u8>>,
            staking_fee: Option<Percent>,
            weight_control_fee: Option<Percent>,
        ) -> DispatchResult {
            let agent_key = ensure_signed(origin)?;
            agent::update::<T>(
                agent_key,
                name,
                address,
                metadata,
                staking_fee,
                weight_control_fee,
            )
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event created when stake has been transferred from the coldkey account onto the key
        /// staking account
        StakeAdded(AccountIdOf<T>, AccountIdOf<T>, BalanceOf<T>),
        /// Event created when stake has been removed from the key staking account onto the coldkey
        /// account
        StakeRemoved(AccountIdOf<T>, AccountIdOf<T>, BalanceOf<T>),
        /// Event created when a new agent account has been registered to the chain
        AgentRegistered(AccountIdOf<T>),
        /// Event created when a agent account has been deregistered from the chain
        AgentUnregistered(AccountIdOf<T>),
        /// Event created when the agent's updated information is added to the network
        AgentUpdated(AccountIdOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The specified agent does not exist.
        AgentDoesNotExist,
        /// Insufficient stake to withdraw the requested amount.
        NotEnoughStakeToWithdraw,
        /// Insufficient balance in the cold key account to stake the requested amount.
        NotEnoughBalanceToStake,
        /// The number of agent registrations in this block exceeds the allowed limit.
        TooManyAgentRegistrationsThisBlock,
        /// The number of agent registrations in this interval exceeds the allowed limit.
        TooManyAgentRegistrationsThisInterval,
        /// The agent is already registered in the active set.
        AgentAlreadyRegistered,
        /// Failed to convert between u128 and T::Balance.
        CouldNotConvertToBalance,
        /// Failed to add balance to the account.
        BalanceNotAdded,
        /// Failed to remove stake from the account.
        StakeNotRemoved,
        /// Invalid shares distribution.
        InvalidShares,
        /// Insufficient balance to register.
        NotEnoughBalanceToRegisterAgent,
        /// Failed to add stake to the account.
        StakeNotAdded,
        /// Failed to remove balance from the account.
        BalanceNotRemoved,
        /// Balance could not be removed from the account.
        BalanceCouldNotBeRemoved,
        /// Insufficient stake to register.
        NotEnoughStakeToRegister,
        /// The entity is still registered and cannot be modified.
        StillRegistered,
        /// Attempted to set max allowed agents to a value less than the current number of
        /// registered agents.
        MaxAllowedAgents,
        /// Insufficient balance to transfer.
        NotEnoughBalanceToTransfer,
        /// The agent metadata is invalid.
        InvalidAgentMetadata,
        /// The agent metadata is too long.
        AgentMetadataTooLong,
        /// The agent metadata is too long.
        AgentMetadataTooShort,
        /// The minimum burn value is invalid, likely too small.
        InvalidMinBurn,
        /// The maximum burn value is invalid.
        InvalidMaxBurn,
        /// The agent name is too long.
        AgentNameTooLong,
        /// The agent name is too short.
        AgentNameTooShort,
        /// The agent name is invalid. It must be a UTF-8 encoded string.
        InvalidAgentName,
        /// The agent url is too long.
        AgentUrlTooLong,
        /// The agent url is too short.
        AgentUrlTooShort,
        /// The agent ur; is invalid.
        InvalidAgentUrl,
        /// A agent with this name already exists in the subnet.
        AgentNameAlreadyExists,
        /// An arithmetic error occurred during calculation.
        ArithmeticError,
        /// The extrinsic panicked during execution.
        ExtrinsicPanicked,
        /// A step in the process panicked.
        StepPanicked,
        /// The stake amount to add or remove is too small. Minimum is 0.5 unit.
        StakeTooSmall,
        /// Key is not present in Whitelist, it needs to be whitelisted by a Curator
        AgentKeyNotWhitelisted,
        /// The amount given is 0
        InvalidAmount,
        /// The staking fee given is lower than the minimum fee
        InvalidStakingFee,
        /// The weight control fee given is lower than the minimum fee
        InvalidWeightControlFee,
    }
}

impl<T: Config>
    pallet_torus0_api::Torus0Api<
        T::AccountId,
        <T::Currency as Currency<T::AccountId>>::Balance,
        <T::Currency as Currency<T::AccountId>>::NegativeImbalance,
    > for Pallet<T>
{
    fn reward_interval() -> u16 {
        RewardInterval::<T>::get()
    }

    fn min_validator_stake() -> u128 {
        MinValidatorStake::<T>::get()
    }

    fn max_validators() -> u16 {
        MaxAllowedValidators::<T>::get()
    }

    fn weight_control_fee(who: &T::AccountId) -> Percent {
        Agents::<T>::get(who)
            .map(|agent| agent.fees.weight_control_fee)
            .unwrap_or_else(|| FeeConstraints::<T>::get().min_weight_control_fee)
    }

    fn weight_penalty_factor(who: &T::AccountId) -> Percent {
        Agents::<T>::get(who)
            .map(|agent| agent.weight_penalty_factor)
            .unwrap_or_default()
    }

    fn staking_fee(who: &T::AccountId) -> Percent {
        Agents::<T>::get(who)
            .map(|agent| agent.fees.staking_fee)
            .unwrap_or_else(|| FeeConstraints::<T>::get().min_staking_fee)
    }

    fn staked_by(
        staked: &T::AccountId,
    ) -> sp_std::vec::Vec<(
        T::AccountId,
        <T::Currency as Currency<T::AccountId>>::Balance,
    )> {
        stake::get_staked_by_vector::<T>(staked)
    }

    fn stake_to(
        staker: &T::AccountId,
        staked: &T::AccountId,
        amount: <T::Currency as Currency<T::AccountId>>::Balance,
    ) -> Result<(), <T::Currency as Currency<T::AccountId>>::Balance> {
        stake::add_stake::<T>(staker.clone(), staked.clone(), amount).map_err(|_| amount)
    }

    fn agent_ids() -> impl Iterator<Item = T::AccountId> {
        Agents::<T>::iter_keys()
    }

    fn is_agent_registered(agent: &T::AccountId) -> bool {
        Agents::<T>::contains_key(agent)
    }
}
