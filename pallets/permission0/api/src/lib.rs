#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use polkadot_sdk::{
    frame_support::dispatch::DispatchResult,
    sp_core::{blake2_256, H256},
    sp_runtime::{DispatchError, Percent},
    sp_std::collections::btree_map::BTreeMap,
    sp_std::vec::Vec,
};
use scale_info::TypeInfo;

/// Type for permission ID
pub type PermissionId = H256;

/// Type for stream ID
pub type StreamId = H256;

/// Static identifier for root emission stream
pub const ROOT_STREAM_PREFIX: &[u8] = b"torus:emission:root";

/// Generates the root stream ID for an agent
pub fn generate_root_stream_id<AccountId: Encode>(agent_id: &AccountId) -> StreamId {
    let mut data = ROOT_STREAM_PREFIX.to_vec();
    data.extend(agent_id.encode());
    blake2_256(&data).into()
}

/// Generates a derived stream ID based on source stream and permission
pub fn generate_derived_stream_id(
    source_stream: &StreamId,
    permission_id: &PermissionId,
) -> StreamId {
    let mut data = source_stream.encode();
    data.extend(permission_id.encode());
    blake2_256(&data).into()
}

/// Defines what portion of emissions the permission applies to
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum EmissionAllocation<Balance> {
    /// Permission applies to a percentage of all emissions (0-100)
    Streams(BTreeMap<StreamId, Percent>),
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

/// Types of enforcement actions that can be voted on
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum EnforcementAuthority<AccountId> {
    /// No special enforcement (standard permission execution)
    None,
    /// Permission can be toggled active/inactive by controllers
    ControlledBy {
        controllers: Vec<AccountId>,
        required_votes: u32,
    },
}

/// The Permission0 API trait
pub trait Permission0Api<AccountId, Origin, BlockNumber, Balance, NegativeImbalance>:
    Permission0EmissionApi<AccountId, Origin, BlockNumber, Balance, NegativeImbalance>
    + Permission0CuratorApi<AccountId, Origin, BlockNumber>
{
    /// Check if a permission exists
    fn permission_exists(id: &PermissionId) -> bool;

    /// Revoke a permission
    fn revoke_permission(who: Origin, permission_id: &PermissionId) -> DispatchResult;

    /// Execute a manual distribution for a permission
    fn execute_permission(who: Origin, permission_id: &PermissionId) -> DispatchResult;
}

pub trait Permission0EmissionApi<AccountId, Origin, BlockNumber, Balance, NegativeImbalance> {
    /// Grant a permission for emission delegation
    #[allow(clippy::too_many_arguments)]
    fn grant_emission_permission(
        grantor: AccountId,
        grantee: AccountId,
        allocation: EmissionAllocation<Balance>,
        targets: Vec<(AccountId, u16)>,
        distribution: DistributionControl<Balance, BlockNumber>,
        duration: PermissionDuration<BlockNumber>,
        revocation: RevocationTerms<AccountId, BlockNumber>,
        enforcement: EnforcementAuthority<AccountId>,
    ) -> Result<PermissionId, DispatchError>;

    /// Accumulate emissions for an agent with permissions
    fn accumulate_emissions(agent: &AccountId, stream: &StreamId, amount: &mut NegativeImbalance);

    /// Check and process automatic distributions
    fn process_auto_distributions(current_block: BlockNumber);

    /// Get the accumulated amount for a permission
    fn get_accumulated_amount(permission_id: &PermissionId, stream: &StreamId) -> Balance;
}

bitflags::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct CuratorPermissions: u32 {
        /// Permission to review and process agent applications
        const APPLICATION_REVIEW = 0b0000_0010;
        /// Permission to manage the whitelist (add/remove accounts)
        const WHITELIST_MANAGE   = 0b0000_0100;
        /// Permission to apply penalty factors to agents
        const PENALTY_CONTROL    = 0b0000_1000;
    }
}

pub trait Permission0CuratorApi<AccountId, Origin, BlockNumber> {
    /// Grants a curator permission, bounded by the given flags.
    /// Only available for the root key, currently.
    fn grant_curator_permission(
        grantor: Origin,
        grantee: AccountId,
        flags: CuratorPermissions,
        cooldown: Option<BlockNumber>,
        duration: PermissionDuration<BlockNumber>,
        revocation: RevocationTerms<AccountId, BlockNumber>,
    ) -> Result<PermissionId, DispatchError>;

    /// Verifies the grantee's curator permission and returns the registered
    /// cooldown between actions.
    fn ensure_curator_permission(
        grantee: Origin,
        flags: CuratorPermissions,
    ) -> Result<(), DispatchError>;

    /// Finds the curator permission granted to [`grantee`].
    fn get_curator_permission(grantee: &AccountId) -> Option<PermissionId>;
}
