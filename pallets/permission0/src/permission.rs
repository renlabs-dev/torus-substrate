use codec::{Decode, Encode, MaxEncodedLen};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult, ensure, CloneNoBound, DebugNoBound, DefaultNoBound, EqNoBound,
        PartialEqNoBound,
    },
    frame_system::{self, ensure_signed_or_root},
    polkadot_sdk_frame::prelude::{BlockNumberFor, OriginFor},
    sp_core::{H256, U256},
    sp_runtime::{
        traits::{BlakeTwo256, Hash},
        BoundedBTreeMap, BoundedVec, DispatchError, Percent,
    },
    sp_std::vec::Vec,
};
use scale_info::TypeInfo;

use crate::*;

pub use curator::{CuratorPermissions, CuratorScope};
pub use emission::{DistributionControl, EmissionAllocation, EmissionScope};

pub mod curator;
pub mod emission;

/// Type for permission ID
pub type PermissionId = H256;

/// Generate a unique permission ID by hashing a concat of
/// `grantee | scope | block number`
pub fn generate_permission_id<T: Config>(
    grantor: &T::AccountId,
    grantee: &T::AccountId,
    scope: &PermissionScope<T>,
) -> Result<PermissionId, DispatchError> {
    let mut data = grantor.encode();
    data.extend(grantee.encode());
    data.extend(scope.encode());

    data.extend(<frame_system::Pallet<T>>::block_number().encode());

    if let Some(extrinsic_index) = <frame_system::Pallet<T>>::extrinsic_index() {
        data.extend(extrinsic_index.encode());
    }

    let id = BlakeTwo256::hash(&data);
    ensure!(
        !Permissions::<T>::contains_key(id),
        Error::<T>::DuplicatePermissionInBlock
    );

    Ok(id)
}

#[derive(Encode, Decode, CloneNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct PermissionContract<T: Config> {
    pub grantor: T::AccountId,
    pub grantee: T::AccountId,
    pub scope: PermissionScope<T>,
    pub duration: PermissionDuration<T>,
    pub revocation: RevocationTerms<T>,
    /// Enforcement authority that can toggle the permission
    pub enforcement: EnforcementAuthority<T>,
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
            PermissionDuration::UntilBlock(block) => current_block > block,
            PermissionDuration::Indefinite => false,
        }
    }

    pub fn revoke(self, origin: OriginFor<T>, permission_id: H256) -> DispatchResult {
        // The grantee is also always allowed to revoke the permission.
        let who = ensure_signed_or_root(origin)?.filter(|who| who != &self.grantee);

        let grantor = self.grantor.clone();
        let grantee = self.grantee.clone();

        // `who` will not be present if the origin is a root key
        if let Some(who) = &who {
            match &self.revocation {
                RevocationTerms::RevocableByGrantor => {
                    ensure!(who == &grantor, Error::<T>::NotAuthorizedToRevoke)
                }
                RevocationTerms::RevocableByArbiters {
                    accounts,
                    required_votes,
                } if accounts.contains(who) => {
                    let votes = RevocationTracking::<T>::get(permission_id)
                        .into_iter()
                        .filter(|id| id != who)
                        .filter(|id| accounts.contains(id))
                        .count();
                    if votes.saturating_add(1) < *required_votes as usize {
                        return RevocationTracking::<T>::mutate(permission_id, |votes| {
                            votes
                                .try_insert(who.clone())
                                .map_err(|_| Error::<T>::TooManyRevokers)?;
                            Ok(())
                        });
                    }
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

    /// Updates the enforcement authority for this permission.
    ///
    /// When the enforcement authority changes, all ongoing enforcement
    /// referendums for this permission are wiped.
    pub fn update_enforcement(
        mut self,
        permission_id: H256,
        enforcement: EnforcementAuthority<T>,
    ) -> DispatchResult {
        let (controllers, required_votes) = match enforcement {
            EnforcementAuthority::None => {
                self.enforcement = EnforcementAuthority::None;
                Permissions::<T>::insert(permission_id, self);

                let _ = EnforcementTracking::<T>::clear_prefix(permission_id, u32::MAX, None);

                Pallet::<T>::deposit_event(Event::EnforcementAuthoritySet {
                    permission_id,
                    controllers_count: 0,
                    required_votes: 0,
                });

                return Ok(());
            }
            EnforcementAuthority::ControlledBy {
                controllers,
                required_votes,
            } => (controllers, required_votes),
        };

        ensure!(
            !controllers.is_empty(),
            Error::<T>::InvalidNumberOfControllers
        );
        ensure!(required_votes > 0, Error::<T>::InvalidNumberOfControllers);
        ensure!(
            required_votes as usize <= controllers.len(),
            Error::<T>::InvalidNumberOfControllers
        );

        let event = Event::EnforcementAuthoritySet {
            permission_id,
            controllers_count: controllers.len() as u32,
            required_votes,
        };

        self.enforcement = EnforcementAuthority::ControlledBy {
            controllers,
            required_votes,
        };
        Permissions::<T>::insert(permission_id, self);

        let _ = EnforcementTracking::<T>::clear_prefix(permission_id, u32::MAX, None);

        <Pallet<T>>::deposit_event(event);

        Ok(())
    }

    fn cleanup(self, permission_id: H256) {
        crate::remove_permission_from_indices::<T>(&self.grantor, &self.grantee, permission_id);

        Permissions::<T>::remove(permission_id);
        RevocationTracking::<T>::remove(permission_id);
        let _ = EnforcementTracking::<T>::clear_prefix(permission_id, u32::MAX, None);

        match self.scope {
            PermissionScope::Emission(emission) => {
                emission.cleanup(permission_id, &self.last_execution, &self.grantor)
            }
            PermissionScope::Curator(curator) => {
                curator.cleanup(permission_id, &self.last_execution, &self.grantor)
            }
            PermissionScope::Namespace(_) => {
                // No cleanup needed for namespace permissions
            }
        }
    }

    pub fn is_updatable(&self) -> bool {
        let current_block = frame_system::Pallet::<T>::block_number();

        match &self.revocation {
            RevocationTerms::RevocableByGrantor => true,
            RevocationTerms::RevocableAfter(block) => &current_block > block,
            _ => false,
        }
    }
}

/// Defines what the permission applies to
#[derive(Encode, Decode, CloneNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub enum PermissionScope<T: Config> {
    Emission(EmissionScope<T>),
    Curator(CuratorScope<T>),
    Namespace(NamespaceScope<T>),
}

/// Scope for namespace permissions
#[derive(Encode, Decode, CloneNoBound, TypeInfo, MaxEncodedLen, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct NamespaceScope<T: Config> {
    /// Set of namespace paths this permission grants access to
    pub paths: BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
}

#[derive(
    Encode, Decode, CloneNoBound, PartialEqNoBound, EqNoBound, TypeInfo, MaxEncodedLen, DebugNoBound,
)]
#[scale_info(skip_type_params(T))]
pub enum PermissionDuration<T: Config> {
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
        accounts: BoundedVec<T::AccountId, T::MaxRevokersPerPermission>,
        required_votes: u32,
    },
    /// Time-based revocation
    RevocableAfter(BlockNumberFor<T>),
}

/// Types of enforcement actions that can be voted on
#[derive(
    Encode, Decode, CloneNoBound, PartialEqNoBound, EqNoBound, TypeInfo, MaxEncodedLen, DebugNoBound,
)]
#[scale_info(skip_type_params(T))]
pub enum EnforcementReferendum {
    /// Toggle emission accumulation state
    EmissionAccumulation(bool),
    /// Execute the permission
    Execution,
}

/// Defines how a permission's enforcement is controlled
#[derive(
    Encode,
    Decode,
    CloneNoBound,
    PartialEqNoBound,
    EqNoBound,
    TypeInfo,
    MaxEncodedLen,
    DebugNoBound,
    DefaultNoBound,
)]
#[scale_info(skip_type_params(T))]
pub enum EnforcementAuthority<T: Config> {
    /// No special enforcement (standard permission execution)
    #[default]
    None,
    /// Permission can be toggled active/inactive by controllers
    ControlledBy {
        controllers: BoundedVec<T::AccountId, T::MaxControllersPerPermission>,
        required_votes: u32,
    },
}

/// Process all auto-distributions and time-based distributions
pub(crate) fn do_auto_permission_execution<T: Config>(current_block: BlockNumberFor<T>) {
    // Only check every 10 blocks to reduce computational overhead
    if <BlockNumberFor<T> as Into<U256>>::into(current_block)
        .checked_rem(10.into())
        .unwrap_or_default()
        != U256::zero()
    {
        return;
    }

    let permissions: Vec<_> = Permissions::<T>::iter().collect();
    let mut expired = Vec::with_capacity(permissions.len());

    for (permission_id, contract) in Permissions::<T>::iter() {
        #[allow(clippy::single_match)]
        match &contract.scope {
            PermissionScope::Emission(emission_scope) => {
                emission::do_auto_distribution(
                    emission_scope,
                    permission_id,
                    current_block,
                    &contract,
                );
            }
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
