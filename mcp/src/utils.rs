use rmcp::ErrorData;
use torus_client::{subxt::utils::AccountId32, subxt_signer::sr25519::Keypair};

use crate::ACCOUNTS;

pub fn keypair_from_name(name: impl AsRef<str>) -> Result<Keypair, ErrorData> {
    let name = name.as_ref().to_lowercase();
    ACCOUNTS
        .get(&name)
        .ok_or_else(|| {
            ErrorData::invalid_request(format!("{name} is not a valid account name."), None)
        })
        .cloned()
}

pub fn name_or_key(account_id: &AccountId32) -> String {
    ACCOUNTS
        .iter()
        .find_map(|(name, keypair)| {
            if &keypair.public_key().to_account_id() == account_id {
                Some(name.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| account_id.to_string())
}
