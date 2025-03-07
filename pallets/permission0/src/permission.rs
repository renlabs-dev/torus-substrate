use codec::{Decode, Encode, MaxEncodedLen};
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult, ensure, CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound,
    },
    frame_system,
    frame_system::ensure_signed_or_root,
    polkadot_sdk_frame::prelude::{BlockNumberFor, OriginFor},
    sp_core::{H256, U256},
    sp_runtime::{
        traits::{BlakeTwo256, Hash},
        BoundedBTreeMap, BoundedVec, Percent,
    },
};
use scale_info::TypeInfo;

use crate::{BalanceOf, Config, Error, Event, Pallet, Permissions};

pub use emission::{DistributionControl, EmissionAllocation, EmissionScope};

pub mod emission;

/// Type for permission ID
pub type PermissionId = H256;

/// Generate a unique permission ID by hashing a concat of
/// `grantee | scope | block number`
pub fn generate_permission_id<T: Config>(
    grantor: &T::AccountId,
    grantee: &T::AccountId,
    scope: &PermissionScope<T>,
) -> PermissionId {
    let mut data = grantor.encode();
    data.extend(grantee.encode());
    data.extend(scope.encode());

    data.extend(<frame_system::Pallet<T>>::block_number().encode());

    // Permission type as well in the future.

    BlakeTwo256::hash(&data).into()
}

#[derive(Encode, Decode, CloneNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct PermissionContract<T: Config> {
    pub grantor: T::AccountId,
    pub grantee: T::AccountId,
    pub scope: PermissionScope<T>,
    pub duration: PermissionDuration<T>,
    pub revocation: RevocationTerms<T>,
    /// Last execution block
    pub last_execution: Option<BlockNumberFor<T>>,
    /// Number of times the permission was executed
    pub execution_count: u32,
    /// Parent permission ID (None for root permissions)
    pub parent: Option<PermissionId>,
    pub created_at: BlockNumberFor<T>,
}

impl<T: Config> PermissionContract<T> {
    pub fn is_expired(&self, current_block: BlockNumberFor<T>) -> bool {
        match self.duration {
            PermissionDuration::Blocks(blocks) => current_block - self.created_at >= blocks,
            PermissionDuration::UntilBlock(block) => current_block >= block,
            PermissionDuration::Indefinite => false,
        }
    }

    pub fn revoke(self, origin: OriginFor<T>, permission_id: H256) -> DispatchResult {
        let who = ensure_signed_or_root(origin)?;

        let grantor = self.grantor.clone();
        let grantee = self.grantee.clone();

        // `who` will not be present if the origin is a root key.
        if let Some(who) = &who {
            match &self.revocation {
                RevocationTerms::RevocableByGrantor => {
                    ensure!(who == &grantor, Error::<T>::NotAuthorizedToRevoke)
                }
                RevocationTerms::RevocableByArbiters { accounts, .. }
                    if accounts.contains(&grantor) =>
                {
                    todo!("implement arbiter revocation") // TODO
                }
                RevocationTerms::RevocableByArbiters { .. } => {
                    return Err(Error::<T>::NotAuthorizedToRevoke.into())
                }
                RevocationTerms::RevocableAfter(block) if who == &grantor => ensure!(
                    <frame_system::Pallet<T>>::block_number() >= *block,
                    Error::<T>::NotAuthorizedToRevoke
                ),
                RevocationTerms::RevocableAfter(_) => {
                    return Err(Error::<T>::NotAuthorizedToRevoke.into())
                }
                RevocationTerms::Irrevocable => {
                    return Err(Error::<T>::NotAuthorizedToRevoke.into())
                }
            };
        }

        self.cleanup(permission_id);

        <Pallet<T>>::deposit_event(Event::PermissionRevoked {
            grantor,
            grantee,
            revoked_by: who,
            permission_id,
        });

        Ok(())
    }

    fn cleanup(self, permission_id: H256) {
        crate::remove_permission_from_indices::<T>(&self.grantor, &self.grantee, permission_id);

        Permissions::<T>::remove(permission_id);

        match self.scope {
            PermissionScope::Emission(emission) => emission.cleanup(permission_id, &self.grantor),
        }
    }
}

/// Defines what the permission applies to
#[derive(Encode, Decode, CloneNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub enum PermissionScope<T: Config> {
    Emission(EmissionScope<T>),
}

#[derive(
    Encode, Decode, CloneNoBound, PartialEqNoBound, EqNoBound, TypeInfo, MaxEncodedLen, DebugNoBound,
)]
#[scale_info(skip_type_params(T))]
pub enum PermissionDuration<T: Config> {
    /// Permission lasts for a specific number of blocks
    Blocks(BlockNumberFor<T>),
    /// Permission lasts until a specific block
    UntilBlock(BlockNumberFor<T>),
    /// Permission lasts indefinitely
    Indefinite,
}

#[derive(
    Encode, Decode, CloneNoBound, PartialEqNoBound, EqNoBound, TypeInfo, MaxEncodedLen, DebugNoBound,
)]
#[scale_info(skip_type_params(T))]
pub enum RevocationTerms<T: Config> {
    /// Cannot be revoked
    Irrevocable,
    /// Can be revoked by the grantor at any time
    RevocableByGrantor,
    /// Can be revoked by third party arbiters
    RevocableByArbiters {
        accounts: BoundedVec<T::AccountId, T::MaxTargetsPerPermission>,
        required_votes: u32,
    },
    /// Time-based revocation
    RevocableAfter(BlockNumberFor<T>),
}

/// Process all auto-distributions and time-based distributions
pub(crate) fn do_auto_permission_execution<T: Config>(current_block: BlockNumberFor<T>) {
    // Only check every 10 blocks to reduce computational overhead
    if current_block.into() % 10 != U256::zero() {
        return;
    }

    let permissions: Vec<_> = Permissions::<T>::iter().collect();
    let mut expired = Vec::with_capacity(permissions.len());

    for (permission_id, contract) in Permissions::<T>::iter() {
        match &contract.scope {
            PermissionScope::Emission(emission_scope) => {
                emission::do_auto_distribution(
                    emission_scope,
                    permission_id,
                    current_block,
                    &contract,
                );
            }
            #[allow(unreachable_patterns)]
            _ => (),
        }

        if contract.is_expired(current_block) {
            expired.push((permission_id, contract));
        }
    }

    for (permission_id, contract) in expired {
        let grantor = contract.grantor.clone();
        let grantee = contract.grantee.clone();

        contract.cleanup(permission_id);

        <Pallet<T>>::deposit_event(Event::PermissionExpired {
            grantor,
            grantee,
            permission_id,
        });
    }
}
