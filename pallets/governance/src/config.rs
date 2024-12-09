use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{frame_support::DebugNoBound, sp_runtime::Percent};
use scale_info::TypeInfo;

#[derive(Clone, TypeInfo, Decode, Encode, PartialEq, Eq, DebugNoBound, MaxEncodedLen)]
pub struct GovernanceConfiguration {
    pub proposal_cost: u64,
    pub proposal_expiration: u32,
    pub agent_application_cost: u64,
    pub proposal_reward_treasury_allocation: Percent,
    pub max_proposal_reward_treasury_allocation: u64,
    pub proposal_reward_interval: u64,
}

impl Default for GovernanceConfiguration {
    fn default() -> Self {
        Self {
            proposal_cost: 10_000_000_000_000,
            proposal_expiration: 130_000,
            agent_application_cost: 1_000_000_000_000,
            proposal_reward_treasury_allocation: Percent::from_percent(2),
            max_proposal_reward_treasury_allocation: 10_000_000_000_000,
            proposal_reward_interval: 75_600,
        }
    }
}
