#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use polkadot_sdk::{
    frame_support::dispatch::DispatchResult,
    sp_core::H256,
    sp_runtime::{DispatchError, Percent, Vec},
};
use scale_info::TypeInfo;

/// Type for permission ID
pub type PermissionId = H256;

/// Defines what portion of emissions the permission applies to
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum EmissionAllocation<Balance> {
    /// Permission applies to a percentage of all emissions (0-100)
    Percentage(Percent),
    /// Permission applies to a specific fixed amount
    FixedAmount(Balance),
}

/// Distribution control parameters
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum DistributionControl<Balance, BlockNumber> {
    /// Manual distribution by the grantee
    Manual,
    /// Automatic distribution after accumulation threshold
    Automatic(Balance),
    /// Distribution at specific block
    AtBlock(BlockNumber),
    /// Distribution at fixed intervals
    Interval(BlockNumber),
}

/// Duration of the permission
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum PermissionDuration<BlockNumber> {
    /// Permission lasts for a specific number of blocks
    Blocks(BlockNumber),
    /// Permission lasts until a specific block
    UntilBlock(BlockNumber),
    /// Permission lasts indefinitely
    Indefinite,
}

/// Terms for revocation
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum RevocationTerms<AccountId, BlockNumber> {
    /// Cannot be revoked
    Irrevocable,
    /// Can be revoked by the grantor at any time
    RevocableByGrantor,
    /// Can be revoked by third party arbiters
    RevocableByArbiters {
        accounts: Vec<AccountId>,
        required_votes: u32,
    },
    /// Time-based revocation
    RevocableAfter(BlockNumber),
}

/// The Permission0 API trait
pub trait Permission0Api<AccountId, Origin, BlockNumber, Balance, NegativeImbalance> {
    /// Check if a permission exists
    fn permission_exists(id: &PermissionId) -> bool;

    /// Grant a permission for emission delegation
    fn grant_emission_permission(
        grantor: AccountId,
        grantee: AccountId,
        allocation: EmissionAllocation<Balance>,
        targets: Vec<(AccountId, u16)>,
        distribution: DistributionControl<Balance, BlockNumber>,
        duration: PermissionDuration<BlockNumber>,
        revocation: RevocationTerms<AccountId, BlockNumber>,
    ) -> Result<PermissionId, DispatchError>;

    /// Revoke a permission
    fn revoke_permission(who: Origin, permission_id: &PermissionId) -> DispatchResult;

    /// Execute a manual distribution for a permission
    fn execute_permission(who: Origin, permission_id: &PermissionId) -> DispatchResult;

    /// Accumulate emissions for an agent with permissions
    fn accumulate_emissions(agent: &AccountId, amount: &mut NegativeImbalance);

    /// Check and process automatic distributions
    fn process_auto_distributions(current_block: BlockNumber);

    /// Get the accumulated amount for a permission
    fn get_accumulated_amount(permission_id: &PermissionId) -> Balance;
}
