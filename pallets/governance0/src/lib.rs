#![cfg_attr(not(feature = "std"), no_std)]

mod application;
mod curator;
mod proposal;
mod voting;
mod whitelist;

use frame::testing_prelude::{DispatchResult, OriginFor};
pub use pallet::*;
use polkadot_sdk::polkadot_sdk_frame as frame;
use polkadot_sdk::sp_std::vec::Vec;

#[frame::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn add_curator_extrinsic(origin: OriginFor<T>, key: T::AccountId) -> DispatchResult {
            curator::add_curator::<T>(origin, key)
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn remove_curator_extrinsic(origin: OriginFor<T>, key: T::AccountId) -> DispatchResult {
            curator::remove_curator::<T>(origin, key)
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn add_to_whitelist_extrinsic(
            origin: OriginFor<T>,
            key: T::AccountId,
        ) -> DispatchResult {
            whitelist::add_to_whitelist::<T>(origin, key)
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn remove_from_whitelist_extrinsic(
            origin: OriginFor<T>,
            key: T::AccountId,
        ) -> DispatchResult {
            whitelist::remove_from_whitelist::<T>(origin, key)
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn submit_application_extrinsic(
            origin: OriginFor<T>,
            agent_key: T::AccountId,
            data: Vec<u8>,
        ) -> DispatchResult {
            application::submit_application::<T>(origin, agent_key, data)
        }

        #[pallet::call_index(5)]
        #[pallet::weight(0)]
        pub fn accept_application_extrinsic(
            origin: OriginFor<T>,
            application_id: u32,
        ) -> DispatchResult {
            application::accept_application::<T>(origin, application_id)
        }

        #[pallet::call_index(6)]
        #[pallet::weight(0)]
        pub fn deny_application_extrinsic(
            origin: OriginFor<T>,
            application_id: u32,
        ) -> DispatchResult {
            application::deny_application::<T>(origin, application_id)
        }

        #[pallet::call_index(7)]
        #[pallet::weight(0)]
        pub fn add_global_custom_proposal_extrinsic(
            origin: OriginFor<T>,
            data: Vec<u8>,
        ) -> DispatchResult {
            proposal::add_global_custom_proposal::<T>(origin, data)
        }

        #[pallet::call_index(8)]
        #[pallet::weight(0)]
        pub fn add_dao_treasury_transfer_proposal_extrinsic(
            origin: OriginFor<T>,
            value: u64,
            destination_key: T::AccountId,
            data: Vec<u8>,
        ) -> DispatchResult {
            proposal::add_dao_treasury_transfer_proposal::<T>(origin, value, destination_key, data)
        }

        #[pallet::call_index(9)]
        #[pallet::weight(0)]
        pub fn vote_proposal_extrinsic(
            origin: OriginFor<T>,
            proposal_id: u64,
            agree: bool,
        ) -> DispatchResult {
            voting::add_vote::<T>(origin, proposal_id, agree)
        }

        #[pallet::call_index(10)]
        #[pallet::weight(0)]
        pub fn remove_vote_proposal_extrinsic(
            origin: OriginFor<T>,
            proposal_id: u64,
        ) -> DispatchResult {
            voting::remove_vote::<T>(origin, proposal_id)
        }

        #[pallet::call_index(11)]
        #[pallet::weight(0)]
        pub fn enable_vote_delegation_extrinsic(origin: OriginFor<T>) -> DispatchResult {
            voting::enable_delegation::<T>(origin)
        }

        #[pallet::call_index(12)]
        #[pallet::weight(0)]
        pub fn disable_vote_delegation_extrinsic(origin: OriginFor<T>) -> DispatchResult {
            voting::disable_delegation::<T>(origin)
        }
    }

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
        /// The network vote mode must be authority for changes to be imposed.
        VoteModeIsNotAuthority,
        /// An internal error occurred, probably relating to the size of the bounded sets.
        InternalError,
        /// The application data is too small or empty.
        ApplicationTooSmall,
        /// The application data is too large, exceeding the maximum allowed size.
        InvalidApplicationSize,
        /// The application is not in a pending state.
        ApplicationNotPending,
        /// The application key is already used in another application.
        ApplicationKeyAlreadyUsed,
        /// The application data is invalid or malformed.
        InvalidApplication,
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
    }
}
