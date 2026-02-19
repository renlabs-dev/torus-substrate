//! Shared utility functions used across all MCP tool modules.
//!
//! The main helpers here solve the "name ↔ keypair/AccountId" mapping problem:
//! - `keypair_from_name`: "alice" → Keypair (for signing transactions)
//! - `account_id_from_name_or_ss58`: "alice" or "5Grw..." → AccountId32 (for read-only queries)
//! - `name_or_key`: AccountId → "alice" or "5Grw..." (for display)

use rmcp::ErrorData;
use torus_client::{subxt::utils::AccountId32, subxt_signer::sr25519::Keypair};

use crate::ACCOUNTS;

/// Converts a dev account name (like "alice") into a signing Keypair.
///
/// This is used by every write tool to get the private key for signing
/// transactions. The name is case-insensitive ("Alice" and "alice" both work).
///
/// Returns an MCP error if the name doesn't match any dev account.
/// Valid names: alice, bob, charlie, dave, eve, ferdie, one, two
pub fn keypair_from_name(name: impl AsRef<str>) -> Result<Keypair, ErrorData> {
    let name = name.as_ref().to_lowercase();
    ACCOUNTS
        .get(&name)
        .ok_or_else(|| {
            ErrorData::invalid_request(format!("{name} is not a valid account name."), None)
        })
        .cloned() // Keypair implements Clone, so we clone from the global HashMap
}

/// Resolves a dev account name (e.g. "alice") or an SS58 address to an AccountId32.
///
/// Useful for read-only tools that don't need to sign transactions but need to
/// query on-chain data for arbitrary accounts — including mainnet agents.
///
/// Resolution order:
/// 1. Tries as a known dev account name (case-insensitive)
/// 2. Falls back to SS58 address parsing
pub fn account_id_from_name_or_ss58(input: impl AsRef<str>) -> Result<AccountId32, ErrorData> {
    let input = input.as_ref();

    if let Some(keypair) = ACCOUNTS.get(&input.to_lowercase()) {
        return Ok(keypair.public_key().to_account_id());
    }

    input.parse::<AccountId32>().map_err(|_| {
        ErrorData::invalid_request(
            format!("'{input}' is neither a valid account name nor a valid SS58 address."),
            None,
        )
    })
}

/// Converts an AccountId32 back to a human-readable name.
///
/// If the AccountId matches a known dev account, returns the name (e.g. "alice").
/// Otherwise, returns the SS58-encoded address string (e.g. "5GrwvaEF5zXb26F...").
///
/// This is used by read tools to make output more readable — instead of
/// showing raw 32-byte addresses, you see "alice" or "bob".
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
