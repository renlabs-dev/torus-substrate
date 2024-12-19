use codec::{Decode, Encode, MaxEncodedLen};
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
            proposal_cost: 10_000_000_000_000,
            proposal_expiration: 130_000,
            agent_application_cost: 1_000_000_000_000,
            agent_application_expiration: 2_000,
            proposal_reward_treasury_allocation: Percent::from_percent(2),
            max_proposal_reward_treasury_allocation: 10_000_000_000_000,
            proposal_reward_interval: 75_600,
        }
    }
}
