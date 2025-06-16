#![cfg_attr(not(feature = "std"), no_std)]

pub mod agent;
pub mod burn;
mod ext;
pub mod fee;
pub mod migrations;
pub mod namespace;
pub mod stake;

pub mod benchmarking;
pub mod weights;

pub(crate) use ext::*;
use frame::{
    arithmetic::Percent,
    prelude::{ensure_root, ensure_signed},
};
use namespace::{NamespaceMetadata, NamespacePath};
pub use pallet::*;
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::{ValueQuery, *},
        traits::Currency,
        Identity,
    },
    frame_system::pallet_prelude::OriginFor,
    polkadot_sdk_frame as frame, sp_std,
};
use scale_info::prelude::vec::Vec;

use crate::{agent::Agent, burn::BurnConfiguration, fee::ValidatorFeeConstraints};

#[frame::pallet]
pub mod pallet {
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(4);

    use frame::prelude::BlockNumberFor;
    use pallet_emission0_api::Emission0Api;
    use pallet_governance_api::GovernanceApi;
    use pallet_permission0_api::Permission0NamespacesApi;
    use pallet_torus0_api::NamespacePathInner;
    use polkadot_sdk::{
        frame_support::traits::{ExistenceRequirement, ReservableCurrency},
        frame_system,
        sp_runtime::traits::Zero,
    };
    use weights::WeightInfo;

    use super::*;

    /// Max allowed of validators. This is used then calculating emissions, only
    /// the top staked agents up to this value will have their weights
    /// considered.
    #[pallet::storage]
    pub type MaxAllowedValidators<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAllowedValidators>;

    /// Amount of tokens to burn from a payer key when registering new agents.
    #[pallet::storage]
    pub type Burn<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Number of agent registrations that happened in the last
    /// [`BurnConfiguration::target_registrations_interval`] blocks.
    #[pallet::storage]
    pub type RegistrationsThisInterval<T: Config> = StorageValue<_, u16, ValueQuery>;

    /// Minimum required stake for an agent to be considered a validator.
    #[pallet::storage]
    pub type MinValidatorStake<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, T::DefaultMinValidatorStake>;

    /// Number of blocks between emissions.
    #[pallet::storage]
    pub type RewardInterval<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultRewardInterval>;

    /// Known registered network agents indexed by the owner's key.
    #[pallet::storage]
    pub type Agents<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, Agent<T>>;

    /// Maximum number of characters allowed in an agent name.
    #[pallet::storage]
    pub type MaxNameLength<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultMaxNameLength>;

    /// Minimum number of characters required in an agent name.
    #[pallet::storage]
    pub type MinNameLength<T: Config> = StorageValue<_, u16, ValueQuery, T::DefaultMinNameLength>;

    /// Maximum number of characters allowed in an agent URL.
    #[pallet::storage]
    pub type MaxAgentUrlLength<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxAgentUrlLength>;

    /// Number of agent registrations that happened this block.
    #[pallet::storage]
    pub type RegistrationsThisBlock<T> = StorageValue<_, u16, ValueQuery>;

    /// Maximum amount of agent registrations per block, tracked by
    /// [`RegistrationsThisBlock`].
    #[pallet::storage]
    pub type MaxRegistrationsPerBlock<T: Config> =
        StorageValue<_, u16, ValueQuery, T::DefaultMaxRegistrationsPerBlock>;

    // Map of staked tokens prefixed by the staker, and indexed by the staked agents
    // mapping to the amount in tokens.
    #[pallet::storage]
    pub type StakingTo<T: Config> =
        StorageDoubleMap<_, Identity, T::AccountId, Identity, T::AccountId, BalanceOf<T>>;

    // Map of staked tokens prefixed by the staked agent, and indexed by the staker
    // keys mapping to the amount in tokens.
    #[pallet::storage]
    pub type StakedBy<T: Config> =
        StorageDoubleMap<_, Identity, T::AccountId, Identity, T::AccountId, BalanceOf<T>>;

    /// The total amount of stake in the network.
    #[pallet::storage]
    pub type TotalStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Minimum amount of stake in tokens a key has to deposit in an agent.
    #[pallet::storage]
    pub type MinAllowedStake<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery, T::DefaultMinAllowedStake>;

    /// The weight dividends have when finding agents to prune. 100% meaning it
    /// is taking fully into account.
    #[pallet::storage]
    pub type DividendsParticipationWeight<T: Config> =
        StorageValue<_, Percent, ValueQuery, T::DefaultDividendsParticipationWeight>;

    /// Constraints defining validation of agent fees.
    #[pallet::storage]
    pub type FeeConstraints<T: Config> = StorageValue<_, ValidatorFeeConstraints<T>, ValueQuery>;

    /// [`Burn`] configuration values.
    #[pallet::storage]
    pub type BurnConfig<T: Config> = StorageValue<_, BurnConfiguration<T>, ValueQuery>;

    /// Cooldown (in blocks) in which an agent needs to wait between each `update_agent` call.
    #[pallet::storage]
    pub type AgentUpdateCooldown<T: Config> =
        StorageValue<_, BlockNumberFor<T>, ValueQuery, T::DefaultAgentUpdateCooldown>;

    /// Namespace registry - maps (owner, path) to metadata
    #[pallet::storage]
    pub type Namespaces<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        NamespacePath,
        NamespaceMetadata<T>,
    >;

    /// Count of namespaces registered per account
    #[pallet::storage]
    pub type NamespaceCount<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    #[pallet::storage]
    pub type NamespacePricingConfig<T: Config> = StorageValue<
        _,
        namespace::NamespacePricingConfig<T>,
        ValueQuery,
        T::DefaultNamespacePricingConfig,
    >;

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

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        #[pallet::constant]
        type DefaultMaxAllowedValidators: Get<u16>;

        #[pallet::constant]
        type DefaultMinValidatorStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultRewardInterval: Get<u16>;

        #[pallet::constant]
        type DefaultMinNameLength: Get<u16>;

        #[pallet::constant]
        type DefaultMaxNameLength: Get<u16>;

        #[pallet::constant]
        type DefaultMaxAgentUrlLength: Get<u16>;

        #[pallet::constant]
        type DefaultMaxRegistrationsPerBlock: Get<u16>;

        #[pallet::constant]
        type DefaultMinAllowedStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultMinStakingFee: Get<u8>;

        #[pallet::constant]
        type DefaultMinWeightControlFee: Get<u8>;

        #[pallet::constant]
        type DefaultMinBurn: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultMaxBurn: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultAdjustmentAlpha: Get<u64>;

        #[pallet::constant]
        type DefaultTargetRegistrationsInterval: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type DefaultTargetRegistrationsPerInterval: Get<u16>;

        #[pallet::constant]
        type DefaultMaxRegistrationsPerInterval: Get<u16>;

        /// The storage MaxNameLength should be constrained to be no more than
        /// the value of this. This is needed on agent::Agent to set the
        /// `name` field BoundedVec max length.
        #[pallet::constant]
        type MaxAgentNameLengthConstraint: Get<u32>;

        /// This is needed on agent::Agent to set the `url` field BoundedVec max
        /// length.
        #[pallet::constant]
        type MaxAgentUrlLengthConstraint: Get<u32>;

        #[pallet::constant]
        type MaxAgentMetadataLengthConstraint: Get<u32>;

        #[pallet::constant]
        type DefaultDividendsParticipationWeight: Get<Percent>;

        /// Default Cooldown (in blocks) in which an agent needs to wait between each `update_agent` call.
        #[pallet::constant]
        type DefaultAgentUpdateCooldown: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type DefaultNamespacePricingConfig: Get<namespace::NamespacePricingConfig<Self>>;

        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        type Currency: Currency<Self::AccountId, Balance = u128>
            + ReservableCurrency<Self::AccountId>
            + Send
            + Sync;

        type Governance: GovernanceApi<Self::AccountId>;

        type Emission: Emission0Api<Self::AccountId>;
        type Permission0: Permission0NamespacesApi<Self::AccountId, NamespacePath>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Adds stakes from origin to the agent key.
        #[pallet::call_index(0)]
        #[pallet::weight((T::WeightInfo::add_stake(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_stake(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let key = ensure_signed(origin)?;
            ensure!(
                amount >= crate::MinAllowedStake::<T>::get(),
                crate::Error::<T>::StakeTooSmall
            );
            stake::add_stake::<T>(key, agent_key, amount)
        }

        /// Removes stakes from origin to the agent key.
        #[pallet::call_index(1)]
        #[pallet::weight((T::WeightInfo::remove_stake(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_stake(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let key = ensure_signed(origin)?;
            stake::remove_stake::<T>(key, agent_key, amount)
        }

        /// Transfers origin's stakes from an agent to another.
        #[pallet::call_index(2)]
        #[pallet::weight((T::WeightInfo::transfer_stake(), DispatchClass::Normal, Pays::Yes))]
        pub fn transfer_stake(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            new_agent_key: AccountIdOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let key = ensure_signed(origin)?;
            stake::transfer_stake::<T>(key, agent_key, new_agent_key, amount)
        }

        /// Registers a new agent on behalf of an arbitrary key.
        #[pallet::call_index(3)]
        #[pallet::weight((T::WeightInfo::register_agent(), DispatchClass::Normal, Pays::Yes))]
        pub fn register_agent(
            origin: OriginFor<T>,
            agent_key: T::AccountId,
            name: Vec<u8>,
            url: Vec<u8>,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            let payer = ensure_signed(origin)?;
            agent::register::<T>(payer, agent_key, name, url, metadata)
        }

        /// Unregister origin's key agent.
        #[pallet::call_index(4)]
        #[pallet::weight((T::WeightInfo::unregister_agent(), DispatchClass::Normal, Pays::Yes))]
        pub fn unregister_agent(origin: OriginFor<T>) -> DispatchResult {
            let agent_key = ensure_signed(origin)?;
            agent::unregister::<T>(agent_key)
        }

        /// Updates origin's key agent metadata.
        #[pallet::call_index(5)]
        #[pallet::weight((T::WeightInfo::update_agent(), DispatchClass::Normal, Pays::Yes))]
        pub fn update_agent(
            origin: OriginFor<T>,
            name: Vec<u8>,
            url: Vec<u8>,
            metadata: Option<Vec<u8>>,
            staking_fee: Option<Percent>,
            weight_control_fee: Option<Percent>,
        ) -> DispatchResult {
            let agent_key = ensure_signed(origin)?;
            agent::update::<T>(
                agent_key,
                name,
                url,
                metadata,
                staking_fee,
                weight_control_fee,
            )
        }

        /// Updates origin's key agent metadata.
        #[pallet::call_index(6)]
        #[pallet::weight((T::WeightInfo::set_agent_update_cooldown(), DispatchClass::Normal, Pays::Yes))]
        pub fn set_agent_update_cooldown(
            origin: OriginFor<T>,
            new_cooldown: BlockNumberFor<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            AgentUpdateCooldown::<T>::set(new_cooldown);
            Ok(())
        }

        /// Create a new namespace, automatically creating missing intermediate nodes
        #[pallet::call_index(7)]
        #[pallet::weight(Weight::default())]
        pub fn create_namespace(origin: OriginFor<T>, path: NamespacePathInner) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            ensure!(
                Agents::<T>::contains_key(&owner),
                Error::<T>::AgentDoesNotExist
            );

            let namespace_path =
                NamespacePath::new(&path).map_err(|_| Error::<T>::InvalidNamespacePath)?;

            ensure!(
                !Namespaces::<T>::contains_key(&owner, &namespace_path),
                Error::<T>::NamespaceAlreadyExists
            );

            let missing_paths = namespace::find_missing_paths::<T>(&owner, &namespace_path);
            let (total_fee, total_deposit) =
                namespace::calculate_cost::<T>(&owner, &missing_paths)?;

            T::Currency::reserve(&owner, total_deposit)?;

            <T as crate::Config>::Currency::transfer(
                &owner,
                &<T as crate::Config>::Governance::dao_treasury_address(),
                total_fee,
                ExistenceRequirement::AllowDeath,
            )
            .map_err(|_| crate::Error::<T>::NotEnoughBalanceToRegisterAgent)?;

            let current_block = <frame_system::Pallet<T>>::block_number();
            let pricing_config = crate::NamespacePricingConfig::<T>::get();

            for path in missing_paths.iter() {
                let deposit = pricing_config.namespace_deposit(path);

                let metadata = NamespaceMetadata {
                    created_at: current_block,
                    deposit,
                };

                Namespaces::<T>::insert(&owner, path, metadata);
            }

            NamespaceCount::<T>::mutate(&owner, |count| {
                *count = count.saturating_add(missing_paths.len() as u32)
            });

            Self::deposit_event(Event::NamespaceCreated {
                owner,
                path: namespace_path,
                deposit: total_deposit,
            });

            Ok(())
        }

        /// Delete a namespace and all its children
        #[pallet::call_index(8)]
        #[pallet::weight(Weight::default())]
        pub fn delete_namespace(origin: OriginFor<T>, path: Vec<u8>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let namespace_path =
                NamespacePath::new(&path).map_err(|_| Error::<T>::InvalidNamespacePath)?;

            ensure!(
                Namespaces::<T>::contains_key(&owner, &namespace_path),
                Error::<T>::NamespaceNotFound
            );

            ensure!(
                !T::Permission0::is_delegating_namespace(&owner, &namespace_path),
                Error::<T>::NamespaceBeingDelegated
            );

            let mut total_deposit = BalanceOf::<T>::zero();
            let namespaces_to_delete: Vec<_> = Namespaces::<T>::iter_prefix(&owner)
                .filter_map(|(path, metadata)| {
                    if path == namespace_path || namespace_path.is_parent_of(&path) {
                        Some((path, metadata.deposit))
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

            NamespaceCount::<T>::mutate(&owner, |count| {
                *count = count.saturating_sub(deleted_count)
            });

            T::Currency::unreserve(&owner, total_deposit);

            Self::deposit_event(Event::NamespaceDeleted {
                owner,
                path: namespace_path,
                deposit_released: total_deposit,
            });

            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event created when stake has been transferred from the coldkey
        /// account onto the key staking account
        StakeAdded(AccountIdOf<T>, AccountIdOf<T>, BalanceOf<T>),
        /// Event created when stake has been removed from the key staking
        /// account onto the coldkey account
        StakeRemoved(AccountIdOf<T>, AccountIdOf<T>, BalanceOf<T>),
        /// Event created when a new agent account has been registered to the
        /// chain
        AgentRegistered(AccountIdOf<T>),
        /// Event created when a agent account has been deregistered from the
        /// chain
        AgentUnregistered(AccountIdOf<T>),
        /// Event created when the agent's updated information is added to the
        /// network
        AgentUpdated(AccountIdOf<T>),
        /// Namespace created
        NamespaceCreated {
            owner: T::AccountId,
            path: NamespacePath,
            deposit: BalanceOf<T>,
        },
        /// Namespace deleted
        NamespaceDeleted {
            owner: T::AccountId,
            path: NamespacePath,
            deposit_released: BalanceOf<T>,
        },
        /// Namespace transferred
        NamespaceTransferred {
            from: T::AccountId,
            to: T::AccountId,
            path: NamespacePath,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The specified agent does not exist.
        AgentDoesNotExist,
        /// Insufficient stake to withdraw the requested amount.
        NotEnoughStakeToWithdraw,
        /// Insufficient balance in the cold key account to stake the requested
        /// amount.
        NotEnoughBalanceToStake,
        /// The number of agent registrations in this block exceeds the allowed
        /// limit.
        TooManyAgentRegistrationsThisBlock,
        /// The number of agent registrations in this interval exceeds the
        /// allowed limit.
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
        /// The stake amount to add or remove is too small. Minimum is 0.5 unit.
        StakeTooSmall,
        /// Key is not present in Whitelist, it needs to be whitelisted by a
        /// Curator
        AgentKeyNotWhitelisted,
        /// The amount given is 0
        InvalidAmount,
        /// The staking fee given is lower than the minimum fee
        InvalidStakingFee,
        /// The weight control fee given is lower than the minimum fee
        InvalidWeightControlFee,
        /// The agent already updated recently
        AgentUpdateOnCooldown,
        /// Invalid namespace path
        InvalidNamespacePath,
        /// Namespace already exists
        NamespaceAlreadyExists,
        /// Namespace not found
        NamespaceNotFound,
        /// Parent namespace not found
        ParentNamespaceNotFound,
        /// Not the owner of the namespace
        NotNamespaceOwner,
        /// Cannot delete namespace with children
        NamespaceHasChildren,
        /// Namespace depth exceeded
        NamespaceDepthExceeded,
        /// The namespace is being delegated through a permission. Revoke that first.
        NamespaceBeingDelegated,
    }
}

impl<T: Config>
    pallet_torus0_api::Torus0Api<T::AccountId, <T::Currency as Currency<T::AccountId>>::Balance>
    for Pallet<T>
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

    fn sum_staking_to(staker: &T::AccountId) -> BalanceOf<T> {
        stake::sum_staking_to::<T>(staker)
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

    fn namespace_exists(agent: &T::AccountId, path: &NamespacePath) -> bool {
        Namespaces::<T>::contains_key(agent, path)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn force_register_agent(
        id: &T::AccountId,
        name: Vec<u8>,
        url: Vec<u8>,
        metadata: Vec<u8>,
    ) -> DispatchResult {
        crate::Agents::<T>::set(
            id,
            Some(Agent {
                key: id.clone(),
                name: name
                    .try_into()
                    .map_err(|_| DispatchError::Other("failed to trim fields"))?,
                url: url
                    .try_into()
                    .map_err(|_| DispatchError::Other("failed to trim fields"))?,
                metadata: metadata
                    .try_into()
                    .map_err(|_| DispatchError::Other("failed to trim fields"))?,
                weight_penalty_factor: Default::default(),
                registration_block: Default::default(),
                fees: Default::default(),
                last_update_block: Default::default(),
            }),
        );

        Ok(())
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn force_set_stake(
        staker: &T::AccountId,
        staked: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        stake::add_stake::<T>(staker.clone(), staked.clone(), amount)
    }
}
