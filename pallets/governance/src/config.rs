use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::frame_election_provider_support::Get;
use polkadot_sdk::{frame_support::DebugNoBound, sp_runtime::Percent};
use scale_info::TypeInfo;

use crate::{BalanceOf, BlockAmount};

#[derive(Clone, TypeInfo, Decode, Encode, PartialEq, Eq, DebugNoBound, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct GovernanceConfiguration<T: crate::Config> {
    pub proposal_cost: BalanceOf<T>,
    pub proposal_expiration: BlockAmount,
    pub agent_application_cost: BalanceOf<T>,
    pub agent_application_expiration: BlockAmount,
    pub proposal_reward_treasury_allocation: Percent,
    pub max_proposal_reward_treasury_allocation: BalanceOf<T>,
    pub proposal_reward_interval: BlockAmount,
}

impl<T: crate::Config> Default for GovernanceConfiguration<T> {
    fn default() -> Self {
        Self {
            proposal_cost: T::DefaultProposalCost::get(),
            proposal_expiration: T::DefaultProposalExpiration::get(), //130_000,
            agent_application_cost: T::DefaultAgentApplicationCost::get(), /* 100_000_000_000_000_000_000, */
            agent_application_expiration: T::DefaultAgentApplicationExpiration::get(), //2_000,
            proposal_reward_treasury_allocation: T::DefaultProposalRewardTreasuryAllocation::get(), /* Percent::from_percent(2), */
            max_proposal_reward_treasury_allocation:
                T::DefaultMaxProposalRewardTreasuryAllocation::get(), /* 10_000_000_000_000_000_000_000, */
            proposal_reward_interval: T::DefaultProposalRewardInterval::get(), //75_600,
        }
    }
}
