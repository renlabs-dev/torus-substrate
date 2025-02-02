#![cfg_attr(not(feature = "std"), no_std)]

pub mod application;
pub mod config;
pub mod ext;
pub mod migrations;
pub mod proposal;
pub mod roles;
pub mod voting;
pub mod whitelist;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarks;
pub mod weights;

use crate::application::AgentApplication;
use crate::config::GovernanceConfiguration;
use crate::proposal::Proposal;
use crate::proposal::ProposalId;
use crate::proposal::UnrewardedProposal;
pub(crate) use ext::*;
use frame::prelude::ensure_root;
pub use pallet::*;
use polkadot_sdk::frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::{ValueQuery, *},
    sp_runtime::Percent,
    traits::Currency,
    Identity, PalletId,
};
use polkadot_sdk::frame_system::pallet_prelude::{ensure_signed, BlockNumberFor, OriginFor};
use polkadot_sdk::polkadot_sdk_frame::traits::AccountIdConversion;
use polkadot_sdk::polkadot_sdk_frame::{self as frame};
use polkadot_sdk::sp_std::vec::Vec;

#[frame::pallet]
pub mod pallet {
    #![allow(clippy::too_many_arguments)]

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

    use proposal::GlobalParamsData;
    use weights::WeightInfo;

    use super::*;

    #[pallet::storage]
    pub type Proposals<T: Config> = StorageMap<_, Identity, ProposalId, Proposal<T>>;

    #[pallet::storage]
    pub type UnrewardedProposals<T: Config> =
        StorageMap<_, Identity, ProposalId, UnrewardedProposal<T>>;

    #[pallet::storage]
    pub type NotDelegatingVotingPower<T: Config> =
        StorageValue<_, BoundedBTreeSet<AccountIdOf<T>, ConstU32<{ u32::MAX }>>, ValueQuery>;

    #[pallet::storage]
    pub type GlobalGovernanceConfig<T: Config> =
        StorageValue<_, GovernanceConfiguration<T>, ValueQuery>;

    // This has to be different than DefaultKey, so we are not conflicting in tests.
    #[pallet::type_value]
    pub fn DefaultDaoTreasuryAddress<T: Config>() -> AccountIdOf<T> {
        <T as Config>::PalletId::get().into_account_truncating()
    }

    #[pallet::storage]
    pub type DaoTreasuryAddress<T: Config> =
        StorageValue<_, AccountIdOf<T>, ValueQuery, DefaultDaoTreasuryAddress<T>>;

    #[pallet::storage]
    pub type AgentApplications<T: Config> = StorageMap<_, Identity, u32, AgentApplication<T>>;

    #[pallet::storage]
    pub type Whitelist<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, ()>;

    #[pallet::storage]
    pub type Curators<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, ()>;

    #[pallet::storage]
    pub type Allocators<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, ()>;

    #[pallet::storage]
    pub type TreasuryEmissionFee<T: Config> =
        StorageValue<_, Percent, ValueQuery, T::DefaultTreasuryEmissionFee>;

    #[pallet::config(with_default)]
    pub trait Config:
        polkadot_sdk::frame_system::Config + pallet_torus0::Config + pallet_emission0::Config
    {
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        #[pallet::constant]
        type MinApplicationDataLength: Get<u32>;

        #[pallet::constant]
        type MaxApplicationDataLength: Get<u32>;

        #[pallet::constant]
        type ApplicationExpiration: Get<BlockAmount>;

        #[pallet::constant]
        type MaxPenaltyPercentage: Get<Percent>;

        #[pallet::constant]
        type DefaultTreasuryEmissionFee: Get<Percent>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultProposalCost: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultProposalExpiration: Get<BlockAmount>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultAgentApplicationCost: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultAgentApplicationExpiration: Get<BlockAmount>;

        #[pallet::constant]
        type DefaultProposalRewardTreasuryAllocation: Get<Percent>;

        #[pallet::constant]
        #[pallet::no_default_bounds]
        type DefaultMaxProposalRewardTreasuryAllocation: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type DefaultProposalRewardInterval: Get<BlockAmount>;

        #[pallet::no_default_bounds]
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        type Currency: Currency<Self::AccountId, Balance = u128> + Send + Sync;

        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            let current_block: u64 = block_number
                .try_into()
                .ok()
                .expect("blockchain won't pass 2 ^ 64 blocks");

            application::resolve_expired_applications::<T>(current_block);
            proposal::tick_proposals::<T>(current_block);
            proposal::tick_proposal_rewards::<T>(current_block);

            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight((<T as Config>::WeightInfo::add_curator(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_curator(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            roles::manage_role::<T, Curators<T>>(key, true, Error::<T>::AlreadyCurator)
        }

        #[pallet::call_index(1)]
        #[pallet::weight((<T as Config>::WeightInfo::remove_curator(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_curator(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            roles::manage_role::<T, Curators<T>>(key, false, Error::<T>::NotAllocator)
        }

        #[pallet::call_index(2)]
        #[pallet::weight((<T as Config>::WeightInfo::add_allocator(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_allocator(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            roles::manage_role::<T, Allocators<T>>(key, true, Error::<T>::AlreadyAllocator)
        }

        #[pallet::call_index(3)]
        #[pallet::weight((<T as Config>::WeightInfo::remove_allocator(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_allocator(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            roles::manage_role::<T, Allocators<T>>(key, false, Error::<T>::NotAllocator)
        }

        #[pallet::call_index(4)]
        #[pallet::weight((<T as Config>::WeightInfo::add_to_whitelist(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_to_whitelist(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            roles::ensure_curator::<T>(origin)?;
            whitelist::add_to_whitelist::<T>(key)
        }

        #[pallet::call_index(5)]
        #[pallet::weight((<T as Config>::WeightInfo::remove_from_whitelist(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_from_whitelist(origin: OriginFor<T>, key: AccountIdOf<T>) -> DispatchResult {
            roles::ensure_curator::<T>(origin)?;
            whitelist::remove_from_whitelist::<T>(key)
        }

        #[pallet::call_index(6)]
        #[pallet::weight((<T as Config>::WeightInfo::accept_application(), DispatchClass::Normal, Pays::Yes))]
        pub fn accept_application(origin: OriginFor<T>, application_id: u32) -> DispatchResult {
            roles::ensure_curator::<T>(origin)?;
            application::accept_application::<T>(application_id)
        }

        #[pallet::call_index(7)]
        #[pallet::weight((<T as Config>::WeightInfo::deny_application(), DispatchClass::Normal, Pays::Yes))]
        pub fn deny_application(origin: OriginFor<T>, application_id: u32) -> DispatchResult {
            roles::ensure_curator::<T>(origin)?;
            application::deny_application::<T>(application_id)
        }

        #[pallet::call_index(8)]
        #[pallet::weight((<T as Config>::WeightInfo::penalize_agent(), DispatchClass::Normal, Pays::Yes))]
        pub fn penalize_agent(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            percentage: u8,
        ) -> DispatchResult {
            roles::ensure_curator::<T>(origin)?;
            roles::penalize_agent::<T>(agent_key, percentage)
        }

        #[pallet::call_index(9)]
        #[pallet::weight((<T as Config>::WeightInfo::submit_application(), DispatchClass::Normal, Pays::Yes))]
        pub fn submit_application(
            origin: OriginFor<T>,
            agent_key: AccountIdOf<T>,
            metadata: Vec<u8>,
            removing: bool,
        ) -> DispatchResult {
            let payer = ensure_signed(origin)?;
            application::submit_application::<T>(payer, agent_key, metadata, removing)
        }

        #[pallet::call_index(10)]
        #[pallet::weight((<T as Config>::WeightInfo::add_global_params_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_global_params_proposal(
            origin: OriginFor<T>,
            data: GlobalParamsData<T>,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_global_params_proposal::<T>(proposer, data, metadata)
        }

        #[pallet::call_index(11)]
        #[pallet::weight((<T as Config>::WeightInfo::add_global_custom_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_global_custom_proposal(
            origin: OriginFor<T>,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_global_custom_proposal::<T>(proposer, metadata)
        }

        #[pallet::call_index(12)]
        #[pallet::weight((<T as Config>::WeightInfo::add_dao_treasury_transfer_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_dao_treasury_transfer_proposal(
            origin: OriginFor<T>,
            value: BalanceOf<T>,
            destination_key: AccountIdOf<T>,
            data: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_dao_treasury_transfer_proposal::<T>(
                proposer,
                value,
                destination_key,
                data,
            )
        }

        #[pallet::call_index(13)]
        #[pallet::weight((<T as Config>::WeightInfo::vote_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn vote_proposal(
            origin: OriginFor<T>,
            proposal_id: u64,
            agree: bool,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            voting::add_vote::<T>(voter, proposal_id, agree)
        }

        #[pallet::call_index(14)]
        #[pallet::weight((<T as Config>::WeightInfo::remove_vote_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn remove_vote_proposal(origin: OriginFor<T>, proposal_id: u64) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            voting::remove_vote::<T>(voter, proposal_id)
        }

        #[pallet::call_index(15)]
        #[pallet::weight((<T as Config>::WeightInfo::enable_vote_delegation(), DispatchClass::Normal, Pays::Yes))]
        pub fn enable_vote_delegation(origin: OriginFor<T>) -> DispatchResult {
            let delegator = ensure_signed(origin)?;
            voting::enable_delegation::<T>(delegator)
        }

        #[pallet::call_index(16)]
        #[pallet::weight((<T as Config>::WeightInfo::disable_vote_delegation(), DispatchClass::Normal, Pays::Yes))]
        pub fn disable_vote_delegation(origin: OriginFor<T>) -> DispatchResult {
            let delegator = ensure_signed(origin)?;
            voting::disable_delegation::<T>(delegator)
        }

        #[pallet::call_index(17)]
        #[pallet::weight((<T as Config>::WeightInfo::add_emission_proposal(), DispatchClass::Normal, Pays::Yes))]
        pub fn add_emission_proposal(
            origin: OriginFor<T>,
            recycling_percentage: Percent,
            treasury_percentage: Percent,
            incentives_ratio: Percent,
            data: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            proposal::add_emission_proposal::<T>(
                proposer,
                recycling_percentage,
                treasury_percentage,
                incentives_ratio,
                data,
            )
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new proposal has been created.
        ProposalCreated(ProposalId),
        /// A proposal has been accepted.
        ProposalAccepted(ProposalId),
        /// A proposal has been refused.
        ProposalRefused(ProposalId),
        /// A proposal has expired.
        ProposalExpired(ProposalId),
        /// A vote has been cast on a proposal.
        ProposalVoted(u64, T::AccountId, bool),
        /// A vote has been unregistered from a proposal.
        ProposalVoteUnregistered(u64, T::AccountId),
        /// An agent account has been added to the whitelist.
        WhitelistAdded(T::AccountId),
        /// An agent account has been removed from the whitelist.
        WhitelistRemoved(T::AccountId),
        /// A new application has been created.
        ApplicationCreated(u32),
        /// An application has been accepted.
        ApplicationAccepted(u32),
        /// An application has been denied.
        ApplicationDenied(u32),
        /// An application has expired.
        ApplicationExpired(u32),
    }

    // TODO: organize this shit in semantic groups
    #[pallet::error]
    pub enum Error<T> {
        /// The proposal is already finished. Do not retry.
        ProposalIsFinished,
        /// Invalid parameters were provided to the finalization process.
        InvalidProposalFinalizationParameters,
        /// Invalid parameters were provided to the voting process.
        InvalidProposalVotingParameters,
        /// Negative proposal cost when setting global or subnet governance configuration.
        InvalidProposalCost,
        /// Negative expiration when setting global or subnet governance configuration.
        InvalidProposalExpiration,
        /// Key doesn't have enough tokens to create a proposal.
        NotEnoughBalanceToPropose,
        /// Proposal data is empty.
        ProposalDataTooSmall,
        /// Proposal data is bigger than 256 characters.
        ProposalDataTooLarge,
        /// The staked module is already delegating for 2 ^ 32 keys.
        ModuleDelegatingForMaxStakers,
        /// Proposal with given id doesn't exist.
        ProposalNotFound,
        /// Proposal was either accepted, refused or expired and cannot accept votes.
        ProposalClosed,
        /// Proposal data isn't composed by valid UTF-8 characters.
        InvalidProposalData,
        /// Invalid value given when transforming a u64 into T::Currency.
        InvalidCurrencyConversionValue,
        /// Dao Treasury doesn't have enough funds to be transferred.
        InsufficientDaoTreasuryFunds,
        /// Key has already voted on given Proposal.
        AlreadyVoted,
        /// Key hasn't voted on given Proposal.
        NotVoted,
        /// Key doesn't have enough stake to vote.
        InsufficientStake,
        /// The voter is delegating its voting power to their staked modules. Disable voting power
        /// delegation.
        VoterIsDelegatingVotingPower,
        /// An internal error occurred, probably relating to the size of the bounded sets.
        InternalError,
        /// The application is not in a pending state.
        ApplicationNotOpen,
        /// The application key is already used in another application.
        ApplicationKeyAlreadyUsed,
        /// The account doesn't have enough balance to submit an application.
        NotEnoughBalanceToApply,
        /// The operation can only be performed by the curator.
        NotCurator,
        /// The application with the given ID was not found.
        ApplicationNotFound,
        /// The account is already whitelisted and cannot be added again.
        AlreadyWhitelisted,
        /// The account is not whitelisted and cannot be removed from the whitelist.
        NotWhitelisted,
        /// Failed to convert the given value to a balance.
        CouldNotConvertToBalance,
        /// The application data provided does not meet the length requirement
        InvalidApplicationDataLength,
        /// The key is already a curator.
        AlreadyCurator,
        /// The key is already an allocator.
        AlreadyAllocator,
        /// The key is not an allocator.
        NotAllocator,
        /// Agent not found
        AgentNotFound,
        /// Invalid agent penalty percentage
        InvalidPenaltyPercentage,
        /// Invalid minimum name length in proposal
        InvalidMinNameLength,
        /// Invalid maximum name length in proposal
        InvalidMaxNameLength,
        /// Invalid maximum allowed agents in proposal
        InvalidMaxAllowedAgents,
        /// Invalid maximum allowed weights in proposal
        InvalidMaxAllowedWeights,
        /// Invalid minimum weight control fee in proposal
        InvalidMinWeightControlFee,
        /// Invalid minimum staking fee in proposal
        InvalidMinStakingFee,
        /// Invalid params given to Emission proposal
        InvalidEmissionProposalData,
    }
}

impl<T: Config> pallet_governance_api::GovernanceApi<T::AccountId> for Pallet<T> {
    fn dao_treasury_address() -> T::AccountId {
        DaoTreasuryAddress::<T>::get()
    }

    fn treasury_emission_fee() -> Percent {
        TreasuryEmissionFee::<T>::get()
    }

    fn is_whitelisted(key: &T::AccountId) -> bool {
        whitelist::is_whitelisted::<T>(key)
    }

    fn ensure_allocator(key: &T::AccountId) -> DispatchResult {
        crate::roles::ensure_allocator::<T>(key)
    }

    fn set_allocator(key: &T::AccountId) {
        crate::Allocators::<T>::insert(key, ());
    }
}
