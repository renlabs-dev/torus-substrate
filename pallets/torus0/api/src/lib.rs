#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate polkadot_sdk;

use polkadot_sdk::sp_runtime::Percent;

/// The Torus0 pallet API.
pub trait Torus0Api<AccountId, Balance, NegativeImbalance> {
    /// Interval of blocks in which rewards are distributed.
    fn reward_interval() -> u16;

    fn min_allowed_stake() -> Balance;
    fn max_validators() -> u16;

    fn weight_control_fee(who: &AccountId) -> Percent;
    fn weight_penalty_factor(who: &AccountId) -> Percent;
    fn staking_fee(who: &AccountId) -> Percent;

    fn staked_by(staked: &AccountId) -> alloc::vec::Vec<(AccountId, Balance)>;
    fn stake_to(staker: &AccountId, staked: &AccountId, amount: Balance) -> Result<(), Balance>;

    fn agent_ids() -> impl Iterator<Item = AccountId>;
    fn is_agent_registered(agent: &AccountId) -> bool;
}
