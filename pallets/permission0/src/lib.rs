#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

pub mod benchmarking;
pub mod weights;
pub use weights::*;

pub mod ext;
pub mod migrations;
pub mod permission;

pub use pallet::*;

pub use permission::{
    generate_permission_id, CuratorPermissions, CuratorScope, DistributionControl,
    EmissionAllocation, EmissionScope, EnforcementAuthority, EnforcementReferendum,
    PermissionContract, PermissionDuration, PermissionId, PermissionScope, RevocationTerms,
};

pub use pallet_permission0_api::{generate_root_stream_id, StreamId};

use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, Get, ReservableCurrency},
        BoundedVec,
    },
    frame_system::{self, pallet_prelude::*},
    polkadot_sdk_frame as frame,
    sp_runtime::{traits::Saturating, Percent},
    sp_std::prelude::*,
};

#[frame::pallet]
pub mod pallet {
    use pallet_torus0_api::NamespacePathInner;
    use polkadot_sdk::frame_support::PalletId;

    use super::*;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(3);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        /// Permission0 pallet ID
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        type WeightInfo: WeightInfo;

        type Currency: ReservableCurrency<Self::AccountId>;

        type Torus: pallet_torus0_api::Torus0Api<Self::AccountId, BalanceOf<Self>>;

        /// Maximum number of controllers per permission.
        #[pallet::constant]
        type MaxControllersPerPermission: Get<u32>;

        /// Maximum number of revokers.
        #[pallet::constant]
        type MaxRevokersPerPermission: Get<u32>;

        /// Maximum number of targets per permission.
        #[pallet::constant]
        type MaxTargetsPerPermission: Get<u32>;

        /// Maximum number of delegated streams per permission.
        #[pallet::constant]
        type MaxStreamsPerPermission: Get<u32>;

        /// Minimum threshold for auto-distribution
        #[pallet::constant]
        type MinAutoDistributionThreshold: Get<BalanceOf<Self>>;

        /// Maximum number of namespaces a single permission can delegate.
        #[pallet::constant]
        type MaxNamespacesPerPermission: Get<u32>;
    }

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Active permission contracts - stored by permission ID
    #[pallet::storage]
    pub type Permissions<T: Config> = StorageMap<_, Identity, PermissionId, PermissionContract<T>>;

    /// Mapping from (grantor, grantee) to permission IDs
    #[pallet::storage]
    pub type PermissionsByParticipants<T: Config> = StorageMap<
        _,
        Identity,
        (T::AccountId, T::AccountId),
        BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
        ValueQuery,
    >;

    /// Permissions granted by a specific account
    #[pallet::storage]
    pub type PermissionsByGrantor<T: Config> = StorageMap<
        _,
        Identity,
        T::AccountId,
        BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
        ValueQuery,
    >;

    /// Permissions received by a specific account
    #[pallet::storage]
    pub type PermissionsByGrantee<T: Config> = StorageMap<
        _,
        Identity,
        T::AccountId,
        BoundedVec<PermissionId, T::MaxTargetsPerPermission>,
        ValueQuery,
    >;

    /// Revocations in progress and the voters
    #[pallet::storage]
    pub type RevocationTracking<T: Config> = StorageMap<
        _,
        Identity,
        PermissionId,
        BoundedBTreeSet<T::AccountId, T::MaxRevokersPerPermission>,
        ValueQuery,
    >;

    /// Enforcement votes in progress and the voters
    #[pallet::storage]
    pub type EnforcementTracking<T: Config> = StorageDoubleMap<
        _,
        Identity,
        PermissionId,
        Identity,
        EnforcementReferendum,
        BoundedBTreeSet<T::AccountId, T::MaxControllersPerPermission>,
        ValueQuery,
    >;

    /// Accumulated amounts for each stream
    #[pallet::storage]
    pub type AccumulatedStreamAmounts<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Identity, T::AccountId>,
            NMapKey<Identity, StreamId>,
            NMapKey<Identity, PermissionId>,
        ),
        BalanceOf<T>,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Permission granted from grantor to grantee with ID
        PermissionGranted {
            grantor: T::AccountId,
            grantee: T::AccountId,
            permission_id: PermissionId,
        },
        /// Permission revoked with ID
        PermissionRevoked {
            grantor: T::AccountId,
            grantee: T::AccountId,
            revoked_by: Option<T::AccountId>,
            permission_id: PermissionId,
        },
        /// Permission executed (manual distribution) with ID
        PermissionExecuted {
            grantor: T::AccountId,
            grantee: T::AccountId,
            permission_id: PermissionId,
            stream_id: Option<StreamId>,
            amount: BalanceOf<T>,
        },
        /// Auto-distribution executed
        AutoDistributionExecuted {
            grantor: T::AccountId,
            grantee: T::AccountId,
            permission_id: PermissionId,
            stream_id: Option<StreamId>,
            amount: BalanceOf<T>,
        },
        /// Permission expired with ID
        PermissionExpired {
            grantor: T::AccountId,
            grantee: T::AccountId,
            permission_id: PermissionId,
        },
        /// Permission accumulation state toggled
        PermissionAccumulationToggled {
            permission_id: PermissionId,
            accumulating: bool,
            toggled_by: Option<T::AccountId>,
        },
        /// Permission was executed by enforcement authority
        PermissionEnforcementExecuted {
            permission_id: PermissionId,
            executed_by: Option<T::AccountId>,
        },
        /// Vote for enforcement action
        EnforcementVoteCast {
            permission_id: PermissionId,
            voter: T::AccountId,
            referendum: EnforcementReferendum,
        },
        /// Enforcement authority set for permission
        EnforcementAuthoritySet {
            permission_id: PermissionId,
            controllers_count: u32,
            required_votes: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The agent is not registered
        NotRegisteredAgent,
        /// Permissions can only be created through extrinsics
        PermissionCreationOutsideExtrinsic,
        /// A permission with the same exact parameters was
        /// already created in the current block
        DuplicatePermissionInBlock,
        /// Permission not found
        PermissionNotFound,
        /// Self-permission is not allowed
        SelfPermissionNotAllowed,
        /// Invalid percentage (out of range)
        InvalidPercentage,
        /// Invalid emission weight set to target
        InvalidTargetWeight,
        /// No targets specified
        NoTargetsSpecified,
        /// Invalid threshold
        InvalidThreshold,
        /// No accumulated amount
        NoAccumulatedAmount,
        /// Not authorized to revoke
        NotAuthorizedToRevoke,
        /// Total allocation exceeded 100%
        TotalAllocationExceeded,
        /// Not the grantee of the permission
        NotPermissionGrantee,
        /// Not the grantor of the permission
        NotPermissionGrantor,
        /// Too many streams
        TooManyStreams,
        /// Too many targets
        TooManyTargets,
        /// Too many revokers
        TooManyRevokers,
        /// Failed to insert into storage
        StorageError,
        /// Invalid amount
        InvalidAmount,
        /// Insufficient balance for operation
        InsufficientBalance,
        /// Invalid distribution interval
        InvalidInterval,
        /// Parent permission not found
        ParentPermissionNotFound,
        /// Invalid distribution method
        InvalidDistributionMethod,
        /// Revokers and required voters must be at least one, and required voters must
        /// be less than the number of revokers
        InvalidNumberOfRevokers,
        /// Fixed amount emissions can only be triggered once, manually or at a block
        FixedAmountCanOnlyBeTriggeredOnce,
        /// Unsupported permission type
        UnsupportedPermissionType,
        /// Not authorized to toggle permission state
        NotAuthorizedToToggle,
        /// Too many controllers
        TooManyControllers,
        /// Invalid number of controllers or required votes
        InvalidNumberOfControllers,
        /// Permission is a duplicate, revoke the previous one
        DuplicatePermission,
        /// Permission is in cooldown, wait a bit.
        PermissionInCooldown,
        /// Curator flags provided are invalid.
        InvalidCuratorPermissions,
        /// Tried granting unknown namespace.
        NamespaceDoesNotExist,
        /// Namespace path provided contains illegal character or is malformatted.
        NamespacePathIsInvalid,
        /// Exceeded amount of total namespaces allowed in a single permission.
        TooManyNamespaces,
        /// Not authorized to edit a stream emission permission.
        NotAuthorizedToEdit,
        /// Stream emission permission is not editable
        NotEditable,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(current_block: BlockNumberFor<T>) {
            permission::do_auto_permission_execution::<T>(current_block);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Grant a permission for emission delegation
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::grant_emission_permission())]
        pub fn grant_emission_permission(
            origin: OriginFor<T>,
            grantee: T::AccountId,
            allocation: EmissionAllocation<T>,
            targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
            distribution: DistributionControl<T>,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
            enforcement: EnforcementAuthority<T>,
        ) -> DispatchResult {
            let grantor = ensure_signed(origin)?;

            ext::emission_impl::grant_emission_permission_impl::<T>(
                grantor,
                grantee,
                allocation,
                targets,
                distribution,
                duration,
                revocation,
                enforcement,
                None,
            )?;

            Ok(())
        }

        /// Revoke a permission. The caller must met revocation constraints or be a root key.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::revoke_permission())]
        pub fn revoke_permission(
            origin: OriginFor<T>,
            permission_id: PermissionId,
        ) -> DispatchResult {
            ext::revoke_permission_impl::<T>(origin, &permission_id)
        }

        /// Execute a manual distribution based on permission
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::execute_permission())]
        pub fn execute_permission(
            origin: OriginFor<T>,
            permission_id: PermissionId,
        ) -> DispatchResult {
            ext::execute_permission_impl::<T>(origin, &permission_id)
        }

        /// Toggle a permission's accumulation state (enabled/disabled)
        /// The caller must be authorized as a controller or be the root key
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::toggle_permission_accumulation())]
        pub fn toggle_permission_accumulation(
            origin: OriginFor<T>,
            permission_id: PermissionId,
            accumulating: bool,
        ) -> DispatchResult {
            ext::emission_impl::toggle_permission_accumulation_impl::<T>(
                origin,
                permission_id,
                accumulating,
            )
        }

        /// Execute a permission through enforcement authority
        /// The caller must be authorized as a controller or be the root key
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::enforcement_execute_permission())]
        pub fn enforcement_execute_permission(
            origin: OriginFor<T>,
            permission_id: PermissionId,
        ) -> DispatchResult {
            ext::enforcement_execute_permission_impl::<T>(origin, permission_id)
        }

        /// Set enforcement authority for a permission
        /// Only the grantor or root can set enforcement authority
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::set_enforcement_authority())]
        pub fn set_enforcement_authority(
            origin: OriginFor<T>,
            permission_id: PermissionId,
            enforcement: EnforcementAuthority<T>,
        ) -> DispatchResult {
            let who = ensure_signed_or_root(origin)?;

            let contract =
                Permissions::<T>::get(permission_id).ok_or(Error::<T>::PermissionNotFound)?;

            if let Some(who) = &who {
                ensure!(who == &contract.grantor, Error::<T>::NotPermissionGrantor);
            }

            contract.update_enforcement(permission_id, enforcement)
        }

        /// Grant a permission for curator delegation
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::grant_curator_permission())]
        pub fn grant_curator_permission(
            origin: OriginFor<T>,
            grantee: T::AccountId,
            flags: u32,
            cooldown: Option<BlockNumberFor<T>>,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
        ) -> DispatchResult {
            ext::curator_impl::grant_curator_permission_impl::<T>(
                origin,
                grantee,
                CuratorPermissions::from_bits_truncate(flags),
                cooldown,
                duration,
                revocation,
            )?;

            Ok(())
        }

        /// Grant a permission over namespaces
        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::grant_curator_permission())]
        pub fn grant_namespace_permission(
            origin: OriginFor<T>,
            grantee: T::AccountId,
            paths: BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
        ) -> DispatchResult {
            ext::namespace_impl::grant_namespace_permission_impl::<T>(
                origin, grantee, paths, duration, revocation,
            )?;

            Ok(())
        }

        /// Allows Grantor/Grantee to edit stream emission permission
        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::grant_curator_permission())]
        pub fn update_emission_permission(
            origin: OriginFor<T>,
            permission_id: PermissionId,
            new_targets: BoundedBTreeMap<T::AccountId, u16, T::MaxTargetsPerPermission>,
            new_streams: Option<BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>>,
            new_distribution_control: Option<DistributionControl<T>>,
        ) -> DispatchResult {
            ext::emission_impl::update_emission_permission(
                origin,
                permission_id,
                new_targets,
                new_streams,
                new_distribution_control,
            )?;

            Ok(())
        }
    }
}

/// Get total allocated percentage for a grantor
fn get_total_allocated_percentage<T: Config>(grantor: &T::AccountId, stream: &StreamId) -> Percent {
    AccumulatedStreamAmounts::<T>::iter_key_prefix((grantor, stream))
        .filter_map(Permissions::<T>::get)
        .map(|contract| match contract.scope {
            PermissionScope::Emission(EmissionScope {
                allocation: EmissionAllocation::Streams(streams),
                ..
            }) => streams.get(stream).copied().unwrap_or_default(),
            _ => Percent::zero(),
        })
        .fold(Percent::zero(), |acc, percentage| {
            acc.saturating_add(percentage)
        })
}

/// Update storage indices when creating a new permission
fn update_permission_indices<T: Config>(
    grantor: &T::AccountId,
    grantee: &T::AccountId,
    permission_id: PermissionId,
) -> Result<(), DispatchError> {
    // Update (grantor, grantee) -> [permission_id] mapping
    PermissionsByParticipants::<T>::try_mutate(
        (grantor.clone(), grantee.clone()),
        |permissions| -> Result<(), DispatchError> {
            permissions
                .try_push(permission_id)
                .map_err(|_| Error::<T>::TooManyTargets)?;
            Ok(())
        },
    )?;

    // Update grantor -> [permission_id] mapping
    PermissionsByGrantor::<T>::try_mutate(
        grantor.clone(),
        |permissions| -> Result<(), DispatchError> {
            permissions
                .try_push(permission_id)
                .map_err(|_| Error::<T>::TooManyTargets)?;
            Ok(())
        },
    )?;

    // Update grantee -> [permission_id] mapping
    PermissionsByGrantee::<T>::try_mutate(
        grantee.clone(),
        |permissions| -> Result<(), DispatchError> {
            permissions
                .try_push(permission_id)
                .map_err(|_| Error::<T>::TooManyTargets)?;
            Ok(())
        },
    )?;

    Ok(())
}

/// Remove a permission from storage indices
fn remove_permission_from_indices<T: Config>(
    grantor: &T::AccountId,
    grantee: &T::AccountId,
    permission_id: PermissionId,
) {
    PermissionsByParticipants::<T>::mutate((grantor.clone(), grantee.clone()), |permissions| {
        permissions.retain(|id| *id != permission_id);
    });

    PermissionsByGrantor::<T>::mutate(grantor, |permissions| {
        permissions.retain(|id| *id != permission_id);
    });

    PermissionsByGrantee::<T>::mutate(grantee, |permissions| {
        permissions.retain(|id| *id != permission_id);
    });
}
