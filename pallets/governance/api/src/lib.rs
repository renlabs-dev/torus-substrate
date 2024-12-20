use codec::Codec;
use polkadot_sdk::sp_api;

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait GovernanceApi<AccountId>
    where AccountId: Codec {
        fn get_dao_treasury_address() -> AccountId;

        fn is_whitelisted(key: &AccountId) -> bool;
    }
}
