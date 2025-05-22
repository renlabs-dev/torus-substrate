#![no_std]

use polkadot_sdk::{frame_support::dispatch::DispatchResult, sp_runtime::Percent};

pub trait GovernanceApi<AccountId> {
    fn dao_treasury_address() -> AccountId;

    fn treasury_emission_fee() -> Percent;

    fn is_whitelisted(key: &AccountId) -> bool;

    fn ensure_allocator(key: &AccountId) -> DispatchResult;

    fn get_allocators() -> impl Iterator<Item = AccountId>;

    fn set_allocator(key: &AccountId);

    #[doc(hidden)]
    #[cfg(feature = "runtime-benchmarks")]
    fn force_set_whitelisted(key: &AccountId);
}
