#![no_std]

use polkadot_sdk::frame_support::dispatch::DispatchResult;

#[derive(Default)]
pub struct ConsensusMemberStats {
    pub incentives: u16,
    pub dividends: u16,
}

pub trait Emission0Api<AccountId> {
    /// Fetches stats emitted by the consensus for an agent.
    /// Returns `None` if the agent has not taken part in the last consensus
    /// run.
    fn consensus_stats(member: &AccountId) -> Option<ConsensusMemberStats>;

    fn delegate_weight_control(delegator: &AccountId, delegatee: &AccountId) -> DispatchResult;
}
