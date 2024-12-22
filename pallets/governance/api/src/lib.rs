#![no_std]

pub trait GovernanceApi<AccountId> {
    fn get_dao_treasury_address() -> AccountId;

    fn is_whitelisted(key: &AccountId) -> bool;
}
