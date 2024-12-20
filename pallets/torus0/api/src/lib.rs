#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate polkadot_sdk;

use polkadot_sdk::{frame_support::dispatch::DispatchResult, sp_runtime::Percent};

/// The Torus0 pallet API.
pub trait Torus0Api<AccountId, Balance, NegativeImbalance> {
    /// Interval of blocks in which rewards are distributed.
    fn reward_interval() -> u16;

    fn min_allowed_stake() -> Balance;
    fn max_validators() -> u16;
    fn staking_fee(who: &AccountId) -> Percent;

    fn stakes_on(who: &AccountId) -> alloc::vec::Vec<(AccountId, Balance)>;
    fn stake_to(
        staker: &AccountId,
        staked: &AccountId,
        amount: NegativeImbalance,
    ) -> Result<(), NegativeImbalance>;

    fn is_agent_registered(agent: &AccountId) -> bool;

    fn on_agent_registration(agent: &AccountId) -> DispatchResult;
    fn on_agent_deregistration(agent: &AccountId) -> DispatchResult;
}
