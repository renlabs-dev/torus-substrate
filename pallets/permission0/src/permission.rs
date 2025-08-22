use codec::{Decode, Encode, MaxEncodedLen};
use pallet_torus0_api::NamespacePath;
use polkadot_sdk::{
    frame_support::{
        CloneNoBound, DebugNoBound, DefaultNoBound, EqNoBound, PartialEqNoBound,
        dispatch::DispatchResult, ensure,
    },
    frame_system::{self, RawOrigin, ensure_signed_or_root},
    polkadot_sdk_frame::prelude::{BlockNumberFor, OriginFor},
    sp_core::{H256, U256},
    sp_runtime::{
        BoundedBTreeMap, BoundedVec, DispatchError, Percent,
        traits::{BlakeTwo256, Hash},
    },
    sp_std::vec::Vec,
    sp_tracing::{error, info, trace},
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
/// `recipient | scope | block number`
pub fn generate_permission_id<T: Config>(
    delegator: &T::AccountId,
    scope: &PermissionScope<T>,
) -> Result<PermissionId, DispatchError> {
    let mut data = delegator.encode();
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
    pub delegator: T::AccountId,
    pub scope: PermissionScope<T>,
    pub duration: PermissionDuration<T>,
    pub revocation: RevocationTerms<T>,
    /// Enforcement authority that can toggle the permission
    pub enforcement: EnforcementAuthority<T>,
    /// Last execution block
    #[doc(hidden)]
    pub last_execution: Option<BlockNumberFor<T>>,
    /// Number of times the permission was executed
    #[doc(hidden)]
    pub execution_count: u32,
    /// Maximum number of instances of this permission
    pub max_instances: u32,
    /// Children permissions
    pub children: BoundedBTreeSet<H256, T::MaxChildrenPerPermission>,
    pub created_at: BlockNumberFor<T>,
}

impl<T: Config> PermissionContract<T> {
    pub(crate) fn new(
        delegator: T::AccountId,
        scope: PermissionScope<T>,
        duration: PermissionDuration<T>,
        revocation: RevocationTerms<T>,
        enforcement: EnforcementAuthority<T>,
        max_instances: u32,
    ) -> Self {
        Self {
            delegator,
            scope,
            duration,
            revocation,
            enforcement,
            max_instances,

            last_execution: None,
            execution_count: 0,
            children: BoundedBTreeSet::new(),
            created_at: frame_system::Pallet::<T>::block_number(),
        }
    }
}

impl<T: Config> PermissionContract<T> {
    pub fn is_expired(&self, current_block: BlockNumberFor<T>) -> bool {
        match self.duration {
            PermissionDuration::UntilBlock(block) => current_block > block,
            PermissionDuration::Indefinite => false,
        }
    }

    /// Returns the last execution block of this permission.
    pub fn last_execution(&self) -> Option<BlockNumberFor<T>> {
        self.last_execution
    }

    /// Returns the number of times this permission was executed.
    pub fn execution_count(&self) -> u32 {
        self.execution_count
    }

    /// Returns the number of used instances of this permission.
    pub fn used_instances(&self) -> u32 {
        let mut used = 0;
        for child in &self.children {
            used = used.saturating_add(
                Permissions::<T>::get(child).map_or(0, |child| child.max_instances),
            );
        }
        used
    }

    /// Returns the number of available instances of this permission.
    pub fn available_instances(&self) -> u32 {
        self.max_instances.saturating_sub(self.used_instances())
    }

    pub fn tick_execution(&mut self, block: BlockNumberFor<T>) -> DispatchResult {
        if self.available_instances() == 0 {
            return Err(Error::<T>::NotEnoughInstances.into());
        }

        self.last_execution = Some(block);
        self.execution_count = self.execution_count.saturating_add(1);

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

    pub fn revoke(self, origin: OriginFor<T>, permission_id: H256) -> DispatchResult {
        let caller = ensure_signed_or_root(origin)?;

        let delegator = self.delegator.clone();
        let recipients = match &self.scope {
            PermissionScope::Curator(CuratorScope { recipient, .. })
            | PermissionScope::Namespace(NamespaceScope { recipient, .. }) => {
                vec![recipient.clone()]
            }
            PermissionScope::Emission(EmissionScope { recipients, .. }) => {
                recipients.keys().cloned().collect()
            }
        };

        // `who` will not be present if the origin is a root key
        if let Some(caller) = &caller {
            match &self.revocation {
                RevocationTerms::RevocableByDelegator if caller == &delegator => {
                    // Allowed to revoke
                }
                RevocationTerms::RevocableByArbiters {
                    accounts,
                    required_votes,
                } if accounts.contains(caller) => {
                    let votes = RevocationTracking::<T>::get(permission_id)
                        .into_iter()
                        .filter(|id| id != caller)
                        .filter(|id| accounts.contains(id))
                        .count();
                    if votes.saturating_add(1) < *required_votes as usize {
                        return RevocationTracking::<T>::mutate(permission_id, |votes| {
                            votes
                                .try_insert(caller.clone())
                                .map_err(|_| Error::<T>::TooManyRevokers)?;
                            Ok(())
                        });
                    }

                    // Allowed to revoke
                }
                RevocationTerms::RevocableAfter(block)
                    if caller == &delegator
                        && <frame_system::Pallet<T>>::block_number() >= *block =>
                {
                    // Allowed to revoke
                }
                _ => {
                    ensure!(
                        recipients.contains(caller),
                        Error::<T>::NotAuthorizedToRevoke
                    );

                    if recipients.len() > 1 {
                        remove_recipient_from_indices::<T>(&delegator, caller, permission_id);

                        Permissions::<T>::mutate(permission_id, |permission| {
                            if let Some(permission) = permission {
                                #[allow(clippy::single_match)]
                                match &mut permission.scope {
                                    PermissionScope::Emission(EmissionScope {
                                        recipients, ..
                                    }) => {
                                        recipients.remove(caller);
                                    }
                                    _ => {}
                                }
                            }
                        });

                        <Pallet<T>>::deposit_event(Event::PermissionRevoked {
                            delegator,
                            revoked_by: Some(caller.clone()),
                            permission_id,
                        });

                        return Ok(());
                    }
                }
            };
        }

        for child_id in &self.children {
            let Some(child) = Permissions::<T>::get(child_id) else {
                continue;
            };

            let revoker = if caller.is_none() {
                RawOrigin::Root
            } else {
                RawOrigin::Signed(child.delegator.clone())
            };

            child.revoke(revoker.into(), *child_id)?;
        }

        self.cleanup(permission_id)?;

        <Pallet<T>>::deposit_event(Event::PermissionRevoked {
            delegator,
            revoked_by: caller,
            permission_id,
        });

        Ok(())
    }

    fn cleanup(self, permission_id: H256) -> DispatchResult {
        match &self.scope {
            PermissionScope::Curator(CuratorScope { recipient, .. })
            | PermissionScope::Namespace(NamespaceScope { recipient, .. }) => {
                remove_permission_from_indices::<T>(
                    &self.delegator,
                    core::iter::once(recipient),
                    permission_id,
                );
            }
            PermissionScope::Emission(EmissionScope { recipients, .. }) => {
                remove_permission_from_indices::<T>(
                    &self.delegator,
                    recipients.keys(),
                    permission_id,
                );
            }
        };

        Permissions::<T>::remove(permission_id);
        RevocationTracking::<T>::remove(permission_id);
        let _ = EnforcementTracking::<T>::clear_prefix(permission_id, u32::MAX, None);

        match self.scope {
            PermissionScope::Emission(emission) => {
                emission.cleanup(permission_id, &self.last_execution, &self.delegator);
            }
            PermissionScope::Curator(curator) => {
                curator.cleanup(permission_id, &self.last_execution, &self.delegator);
            }
            PermissionScope::Namespace(namespace) => {
                namespace.cleanup(permission_id, &self.last_execution, &self.delegator);
            }
        }

        Ok(())
    }

    pub fn is_updatable(&self) -> bool {
        let current_block = frame_system::Pallet::<T>::block_number();

        match &self.revocation {
            RevocationTerms::RevocableByDelegator => true,
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
    pub recipient: T::AccountId,
    /// Set of namespace paths this permission delegates access to
    pub paths: BoundedBTreeMap<
        Option<PermissionId>,
        BoundedBTreeSet<NamespacePath, T::MaxNamespacesPerPermission>,
        T::MaxNamespacesPerPermission,
    >,
}

impl<T: Config> NamespaceScope<T> {
    /// Cleanup operations when permission is revoked or expired
    fn cleanup(
        &self,
        permission_id: polkadot_sdk::sp_core::H256,
        _last_execution: &Option<crate::BlockNumberFor<T>>,
        _delegator: &T::AccountId,
    ) {
        for pid in self.paths.keys().cloned().flatten() {
            Permissions::<T>::mutate_extant(pid, |parent| {
                parent.children.remove(&permission_id);
            });
        }
    }
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
    /// Can be revoked by the delegator at any time
    RevocableByDelegator,
    /// Can be revoked by third party arbiters
    RevocableByArbiters {
        accounts: BoundedVec<T::AccountId, T::MaxRevokersPerPermission>,
        required_votes: u32,
    },
    /// Time-based revocation
    RevocableAfter(BlockNumberFor<T>),
}

impl<T: Config> RevocationTerms<T> {
    /// Checks if the child revocation terms are weaker than the parent.
    pub fn is_weaker(parent: &Self, child: &Self) -> bool {
        match (parent, child) {
            (_, RevocationTerms::RevocableByDelegator) => true,

            (RevocationTerms::RevocableAfter(a), RevocationTerms::RevocableAfter(b)) if a >= b => {
                true
            }

            (RevocationTerms::Irrevocable, RevocationTerms::RevocableAfter(_)) => true,

            (RevocationTerms::Irrevocable, RevocationTerms::Irrevocable) => true,

            _ => false,
        }
    }
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

    info!(
        target: "auto_permission_execution",
        "executing auto permission execution for {} permissions",
        permissions.len()
    );

    for (permission_id, contract) in Permissions::<T>::iter() {
        #[allow(clippy::single_match)]
        match &contract.scope {
            PermissionScope::Emission(emission_scope) => {
                trace!(target: "auto_permission_execution", "executing auto permission execution for permission {permission_id:?}");
                if let Err(err) = emission::do_auto_distribution(
                    emission_scope,
                    permission_id,
                    current_block,
                    &contract,
                ) {
                    error!(
                        "failed to auto distribute emissions for permission {permission_id:?}: {err:?}"
                    );
                }
            }
            _ => (),
        }

        if contract.is_expired(current_block) {
            expired.push((permission_id, contract));
        }
    }

    for (permission_id, contract) in expired {
        let delegator = contract.delegator.clone();

        if let Err(err) = contract.cleanup(permission_id) {
            error!("failed to cleanup expired permission {permission_id:?}: {err:?}");
        }

        <Pallet<T>>::deposit_event(Event::PermissionExpired {
            delegator,
            permission_id,
        });
    }
}

/// Update storage indices when creating a new permission
pub(crate) fn add_permission_indices<'a, T: Config>(
    delegator: &T::AccountId,
    recipients: impl Iterator<Item = &'a T::AccountId>,
    permission_id: PermissionId,
) -> Result<(), DispatchError> {
    for recipient in recipients {
        // Update (delegator, recipient) -> [permission_id] mapping
        PermissionsByParticipants::<T>::try_mutate(
            (delegator.clone(), recipient.clone()),
            |permissions| -> Result<(), DispatchError> {
                permissions
                    .try_insert(permission_id)
                    .map_err(|_| Error::<T>::TooManyTargets)?;
                Ok(())
            },
        )?;

        // Update recipient -> [permission_id] mapping
        PermissionsByRecipient::<T>::try_mutate(
            recipient.clone(),
            |permissions| -> Result<(), DispatchError> {
                permissions
                    .try_insert(permission_id)
                    .map_err(|_| Error::<T>::TooManyTargets)?;
                Ok(())
            },
        )?;
    }

    // Update delegator -> [permission_id] mapping
    PermissionsByDelegator::<T>::try_mutate(
        delegator.clone(),
        |permissions| -> Result<(), DispatchError> {
            permissions
                .try_insert(permission_id)
                .map_err(|_| Error::<T>::TooManyTargets)?;
            Ok(())
        },
    )?;

    Ok(())
}

/// Remove a permission recipient from storage indices
fn remove_recipient_from_indices<T: Config>(
    delegator: &T::AccountId,
    recipient: &T::AccountId,
    permission_id: PermissionId,
) {
    PermissionsByParticipants::<T>::mutate_exists(
        (delegator.clone(), recipient.clone()),
        |permissions| {
            if let Some(p) = permissions {
                p.remove(&permission_id);
                if p.is_empty() {
                    *permissions = None;
                }
            }
        },
    );

    PermissionsByRecipient::<T>::mutate_exists(recipient, |permissions| {
        if let Some(p) = permissions {
            p.remove(&permission_id);
            if p.is_empty() {
                *permissions = None;
            }
        }
    });
}

/// Remove a permission from storage indices
pub(crate) fn remove_permission_from_indices<'a, T: Config>(
    delegator: &T::AccountId,
    recipients: impl Iterator<Item = &'a T::AccountId>,
    permission_id: PermissionId,
) {
    for recipient in recipients {
        PermissionsByParticipants::<T>::mutate_exists(
            (delegator.clone(), recipient.clone()),
            |permissions| {
                if let Some(p) = permissions {
                    p.remove(&permission_id);
                    if p.is_empty() {
                        *permissions = None;
                    }
                }
            },
        );

        PermissionsByRecipient::<T>::mutate_exists(recipient, |permissions| {
            if let Some(p) = permissions {
                p.remove(&permission_id);
                if p.is_empty() {
                    *permissions = None;
                }
            }
        });
    }

    PermissionsByDelegator::<T>::mutate_exists(delegator, |permissions| {
        if let Some(p) = permissions {
            p.remove(&permission_id);
            if p.is_empty() {
                *permissions = None;
            }
        }
    });
}
