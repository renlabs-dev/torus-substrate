#![cfg_attr(not(feature = "std"), no_std)]

mod agent;
mod balance;
mod ext;
mod fee;
pub mod stake;

use crate::agent::Agent;
use crate::fee::ValidatorFee;
use crate::fee::ValidatorFeeConstraints;
pub(crate) use ext::*;
use frame::arithmetic::Percent;
pub use pallet::*;
use polkadot_sdk::frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::{ValueQuery, *},
    traits::Currency,
    Identity,
};
use polkadot_sdk::frame_system::pallet_prelude::OriginFor;
use polkadot_sdk::polkadot_sdk_frame as frame;
use scale_info::prelude::vec::Vec;

#[frame::pallet(dev_mode)]
pub mod pallet {

    use super::*;

    #[pallet::storage]
    pub type MaxAllowedValidators<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedValidators>;

    #[pallet::storage]
    pub type Burn<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub type MaximumSetWeightCallsPerEpoch<T: Config> = StorageValue<_, u16>;

    #[pallet::storage]
    pub type SetWeightCallsPerEpoch<T: Config> = StorageMap<_, Identity, T::AccountId, u16>;

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
    pub type MinAllowedWeights<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMinAllowedWeights>;

    #[pallet::storage]
    pub type MaxWeightAge<T: Config> =
        StorageValue<_, BlockAmount, ValueQuery, T::DefaultMaxWeightAge>;

    #[pallet::storage]
    pub type MaxAllowedWeights<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedWeights>;

    #[pallet::storage]
    pub type Tempo<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultTempo>;

    #[pallet::storage]
    pub type Agents<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, Agent<T>>;

    #[pallet::storage]
    pub type LastUpdate<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, Block>;

    #[pallet::storage]
    pub type RegistrationBlock<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, Block>;

    #[pallet::storage]
    pub type MaxNameLength<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultMaxNameLength>;

    #[pallet::storage]
    pub type MinNameLength<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultMinNameLength>;

    #[pallet::storage]
    pub type MaxAllowedAgents<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedAgents>;

    #[pallet::storage]
    pub type MinWeightStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub type RegistrationsPerBlock<T> = StorageValue<_, u16, ValueQuery>;

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
    pub type MinimumAllowedStake<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, T::DefaultMinimumAllowedStake>;

    #[pallet::storage]
    pub type FeeConstraints<T: Config> = StorageValue<_, ValidatorFeeConstraints<T>, ValueQuery>;

    #[pallet::storage]
    pub type Fee<T: Config> = StorageMap<_, Identity, T::AccountId, ValidatorFee<T>, ValueQuery>;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        #[pallet::constant]
        type DefaultMaxAllowedValidators: Get<u16>;

        #[pallet::constant]
        type DefaultMinValidatorStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultImmunityPeriod: Get<u16>;

        #[pallet::constant]
        type DefaultMinAllowedWeights: Get<u16>;

        #[pallet::constant]
        type DefaultMaxWeightAge: Get<BlockAmount>;

        #[pallet::constant]
        type DefaultMaxAllowedWeights: Get<u16>;

        #[pallet::constant]
        type DefaultTempo: Get<u16>;

        #[pallet::constant]
        type DefaultMinNameLength: Get<u16>;

        #[pallet::constant]
        type DefaultMaxNameLength: Get<u16>;

        #[pallet::constant]
        type DefaultMaxAllowedAgents: Get<u16>;

        #[pallet::constant]
        type DefaultMaxRegistrationsPerBlock: Get<u16>;

        #[pallet::constant]
        type DefaultMinimumAllowedStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultMinStakingFee: Get<u8>;

        #[pallet::constant]
        type DefaultMinWeightControlFee: Get<u8>;

        /// The storage MaxNameLength should be constrained to be no more than the value of this.
        /// This is needed on agent::Agent to set the `name` field BoundedVec max length.
        #[pallet::constant]
        type MaxAgentNameLengthConstraint: Get<u32>;

        /// This is needed on agent::Agent to set the `address` field BoundedVec max length.
        #[pallet::constant]
        type MaxAgentAddressLengthConstraint: Get<u32>;

        type Currency: Currency<Self::AccountId, Balance = u128> + Send + Sync;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn add_stake_extrinsic(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            stake::add_stake::<T>(origin, agent_key, amount)
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn remove_stake_extrinsic(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            stake::remove_stake::<T>(origin, agent_key, amount)
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn transfer_stake_extrinsic(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            new_agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            stake::transfer_stake::<T>(origin, agent_key, new_agent_key, amount)
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn transfer_balance_extrinsic(
            origin: OriginFor<T>,
            destination: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            balance::transfer_balance_multiple::<T>(origin, destination, amount)
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn register_agent_extrinsic(
            origin: OriginFor<T>,
            name: Vec<u8>,
            address: Vec<u8>,
            agent_key: T::AccountId,
            metadata: Option<Vec<u8>>,
        ) -> DispatchResult {
            agent::register::<T>(origin, name, address, agent_key, metadata)
        }

        #[pallet::call_index(5)]
        #[pallet::weight(0)]
        pub fn deregister_agent_extrinsic(origin: OriginFor<T>) -> DispatchResult {
            agent::deregister::<T>(origin)
        }

        #[pallet::call_index(6)]
        #[pallet::weight(0)]
        pub fn update_agent_extrinsic(
            origin: OriginFor<T>,
            name: Vec<u8>,
            address: Vec<u8>,
            metadata: Option<Vec<u8>>,
            staking_fee: Option<Percent>,
            weight_control_fee: Option<Percent>,
        ) -> DispatchResult {
            agent::update::<T>(
                origin,
                name,
                address,
                metadata,
                staking_fee,
                weight_control_fee,
            )
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The specified module does not exist.
        AgentDoesNotExist,
        /// Insufficient stake to withdraw the requested amount.
        NotEnoughStakeToWithdraw,
        /// Insufficient balance in the cold key account to stake the requested amount.
        NotEnoughBalanceToStake,
        /// The number of agent registrations in this block exceeds the allowed limit.
        TooManyAgentRegistrationsPerBlock,
        /// The number of agent registrations in this interval exceeds the allowed limit.
        TooManyAgentRegistrationsPerInterval,
        /// The agent is already registered in the active set.
        AgentAlreadyRegistered,
        /// Failed to convert between u128 and T::Balance.
        CouldNotConvertToBalance,
        /// Failed to add balance to the account.
        BalanceNotAdded,
        /// Failed to remove stake from the account.
        StakeNotRemoved,
        /// The key is already registered.
        KeyAlreadyRegistered,
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
        /// Attempted to set max allowed modules to a value less than the current number of
        /// registered modules.
        MaxAllowedModules,
        /// Insufficient balance to transfer.
        NotEnoughBalanceToTransfer,
        /// The module metadata is invalid.
        InvalidAgentMetadata,
        /// The module metadata is too long.
        AgentMetadataTooLong,
        /// The minimum burn value is invalid, likely too small.
        InvalidMinBurn,
        /// The maximum burn value is invalid.
        InvalidMaxBurn,
        /// The module name is too long.
        AgentNameTooLong,
        /// The module name is too short.
        AgentNameTooShort,
        /// The module name is invalid. It must be a UTF-8 encoded string.
        InvalidAgentName,
        /// The module address is too long.
        AgentAddressTooLong,
        /// The module address is invalid.
        InvalidAgentAddress,
        /// A module with this name already exists in the subnet.
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
    }
}
