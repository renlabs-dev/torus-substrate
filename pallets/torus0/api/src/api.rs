#![allow(clippy::multiple_bound_locations)]

use codec::{Decode, Encode};
use polkadot_sdk::sp_runtime::DispatchError;

polkadot_sdk::sp_api::decl_runtime_apis! {
    /// RPC related to Torus0.
    pub trait Torus0RuntimeApi<AccountId: Encode, Balance: Decode> {
        /// Calculates the total creation cost of a namespace: (Fee, Deposit).
        fn namespace_path_creation_cost(account_id: AccountId, path: crate::NamespacePathInner) -> Result<(Balance, Balance), DispatchError>;
    }
}
