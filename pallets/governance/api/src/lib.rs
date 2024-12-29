#![no_std]

use polkadot_sdk::sp_runtime::Percent;

pub trait GovernanceApi<AccountId> {
    fn dao_treasury_address() -> AccountId;

    fn treasury_emission_fee() -> Percent;

    fn is_whitelisted(key: &AccountId) -> bool;
}
