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
    CuratorPermissions, CuratorScope, DistributionControl, EnforcementAuthority,
    EnforcementReferendum, PermissionContract, PermissionDuration, PermissionId, PermissionScope,
    RevocationTerms, StreamAllocation, StreamScope, generate_permission_id,
};

pub use pallet_permission0_api::{StreamId, generate_root_stream_id};

use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, Get, ReservableCurrency},
    },
    frame_system::{self, pallet_prelude::*},
    polkadot_sdk_frame as frame,
    sp_runtime::{Percent, traits::Saturating},
    sp_std::prelude::*,
};

#[frame::pallet]
pub mod pallet {
    use pallet_torus0_api::NamespacePathInner;
    use polkadot_sdk::{frame_support::PalletId, sp_core::TryCollect};

    use super::*;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(6);

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

        /// Maximum number of recipients per permission.
        #[pallet::constant]
        type MaxRecipientsPerPermission: Get<u32>;

        /// Maximum number of delegated streams per permission.
        #[pallet::constant]
        type MaxStreamsPerPermission: Get<u32>;

        /// Minimum threshold for auto-distribution
        #[pallet::constant]
        type MinAutoDistributionThreshold: Get<BalanceOf<Self>>;

        /// Maximum number of namespaces a single permission can delegate.
        #[pallet::constant]
        type MaxNamespacesPerPermission: Get<u32>;

        /// Maximum number of curator subpermissions a single permission can delegate.
        #[pallet::constant]
        type MaxCuratorSubpermissionsPerPermission: Get<u32>;

        /// Maximum number of children a single permission can have.
        #[pallet::constant]
        type MaxChildrenPerPermission: Get<u32>;

        /// Max operations a bulk extrinsic can perform per extrinsic call.
        #[pallet::constant]
        type MaxBulkOperationsPerCall: Get<u32>;
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

    /// Mapping from (delegator, recipient) to permission IDs
    #[pallet::storage]
    pub type PermissionsByParticipants<T: Config> = StorageMap<
        _,
        Identity,
        (T::AccountId, T::AccountId),
        BoundedBTreeSet<PermissionId, T::MaxRecipientsPerPermission>,
        ValueQuery,
    >;

    /// Permissions delegated by a specific account
    #[pallet::storage]
    pub type PermissionsByDelegator<T: Config> = StorageMap<
        _,
        Identity,
        T::AccountId,
        BoundedBTreeSet<PermissionId, T::MaxRecipientsPerPermission>,
        ValueQuery,
    >;

    /// Permissions received by a specific account
    #[pallet::storage]
    pub type PermissionsByRecipient<T: Config> = StorageMap<
        _,
        Identity,
        T::AccountId,
        BoundedBTreeSet<PermissionId, T::MaxRecipientsPerPermission>,
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
        /// Permission delegated from delegator to recipient with ID
        PermissionDelegated {
            delegator: T::AccountId,
            permission_id: PermissionId,
        },
        /// Permission revoked with ID
        PermissionRevoked {
            delegator: T::AccountId,
            revoked_by: Option<T::AccountId>,
            permission_id: PermissionId,
        },
        /// Permission expired with ID
        PermissionExpired {
            delegator: T::AccountId,
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
        /// An stream distribution happened
        StreamDistribution {
            permission_id: PermissionId,
            stream_id: Option<StreamId>,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
            reason: permission::stream::DistributionReason,
        },
        /// Accumulated emission for stream
        AccumulatedEmission {
            permission_id: PermissionId,
            stream_id: StreamId,
            amount: BalanceOf<T>,
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
        /// Invalid stream weight set to recipient
        InvalidRecipientWeight,
        /// No recipients specified
        NoRecipientsSpecified,
        /// Invalid threshold
        InvalidThreshold,
        /// No accumulated amount
        NoAccumulatedAmount,
        /// Not authorized to revoke
        NotAuthorizedToRevoke,
        /// Total allocation exceeded 100%
        TotalAllocationExceeded,
        /// Not the recipient of the permission
        NotPermissionRecipient,
        /// Not the delegator of the permission
        NotPermissionDelegator,
        /// Too many streams
        TooManyStreams,
        /// Too many recipients
        TooManyRecipients,
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
        /// Fixed amount streams can only be triggered once, manually or at a block
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
        /// Tried delegating unknown namespace.
        NamespaceDoesNotExist,
        /// Namespace path provided contains illegal character or is malformatted.
        NamespacePathIsInvalid,
        /// Exceeded amount of total namespaces allowed in a single permission.
        TooManyNamespaces,
        /// Not authorized to edit a permission.
        NotAuthorizedToEdit,
        /// Permission is not editable
        NotEditable,
        /// Namespace creation was disabled by a curator.
        NamespaceCreationDisabled,
        /// Deriving a permission from multiple parents is still forbidden.
        MultiParentForbidden,
        /// Not enough instances available to delegate/execute a permission.
        /// This might mean the execution requires more than the available instances
        /// or that all instances are locked behind redelegations.
        NotEnoughInstances,
        /// Too many children for a permission.
        TooManyChildren,
        /// Stream managers must have up to two entries and always contain the delegator,
        InvalidStreamManagers,
        /// Revocation terms are too strong for a permission re-delegation.
        RevocationTermsTooStrong,
        /// Too many curator permissions being delegated in a single permission.
        TooManyCuratorPermissions,
        /// Namespace delegation depth exceeded the maximum allowed limit.
        DelegationDepthExceeded,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(current_block: BlockNumberFor<T>) {
            permission::do_auto_permission_execution::<T>(current_block);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Delegate a permission for stream delegation
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::delegate_stream_permission())]
        pub fn delegate_stream_permission(
            origin: OriginFor<T>,
            recipients: BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
            allocation: StreamAllocation<T>,
            distribution: DistributionControl<T>,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
            enforcement: EnforcementAuthority<T>,
            recipient_manager: Option<T::AccountId>,
            weight_setter: Option<T::AccountId>,
        ) -> DispatchResult {
            let delegator = ensure_signed(origin)?;

            ext::stream_impl::delegate_stream_permission_impl::<T>(
                delegator,
                recipients,
                allocation,
                distribution,
                duration,
                revocation,
                enforcement,
                recipient_manager,
                weight_setter,
            )?;

            Ok(())
        }

        /// Revoke a permission. The caller must meet revocation constraints or be a root key.
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
            ext::stream_impl::toggle_permission_accumulation_impl::<T>(
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
        /// Only the delegator or root can set enforcement authority
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
                ensure!(
                    who == &contract.delegator,
                    Error::<T>::NotPermissionDelegator
                );
            }

            contract.update_enforcement(permission_id, enforcement)
        }

        /// Delegate a permission for curator delegation
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::delegate_curator_permission())]
        pub fn delegate_curator_permission(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            flags: BoundedBTreeMap<
                Option<PermissionId>,
                u32,
                T::MaxCuratorSubpermissionsPerPermission,
            >,
            cooldown: Option<BlockNumberFor<T>>,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
            instances: u32,
        ) -> DispatchResult {
            let flags = flags
                .into_iter()
                .map(|(pid, flags)| (pid, CuratorPermissions::from_bits_truncate(flags)))
                .try_collect()?;

            ext::curator_impl::delegate_curator_permission_impl::<T>(
                origin, recipient, flags, cooldown, duration, revocation, instances,
            )?;

            Ok(())
        }

        /// Delegate a permission over namespaces
        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::delegate_namespace_permission())]
        pub fn delegate_namespace_permission(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            paths: BoundedBTreeMap<
                Option<PermissionId>,
                BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
                T::MaxNamespacesPerPermission,
            >,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
            instances: u32,
        ) -> DispatchResult {
            ext::namespace_impl::delegate_namespace_permission_impl::<T>(
                origin, recipient, paths, duration, revocation, instances,
            )?;

            Ok(())
        }

        /// Delegate a permission over namespaces to multiple recipients.
        /// Note: this extrinsic creates _multiple_ permissions with the same
        /// properties.
        #[pallet::call_index(10)]
        #[pallet::weight({
            T::WeightInfo::delegate_namespace_permission()
                .saturating_mul(recipients.len() as u64)
        })]
        pub fn bulk_delegate_namespace_permission(
            origin: OriginFor<T>,
            recipients: BoundedBTreeSet<T::AccountId, T::MaxBulkOperationsPerCall>,
            paths: BoundedBTreeMap<
                Option<PermissionId>,
                BoundedBTreeSet<NamespacePathInner, T::MaxNamespacesPerPermission>,
                T::MaxNamespacesPerPermission,
            >,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
            instances: u32,
        ) -> DispatchResult {
            for recipient in recipients {
                ext::namespace_impl::delegate_namespace_permission_impl::<T>(
                    origin.clone(),
                    recipient,
                    paths.clone(),
                    duration.clone(),
                    revocation.clone(),
                    instances,
                )?;
            }

            Ok(())
        }

        /// Allows Delegator/Recipient to edit stream permission
        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::delegate_curator_permission())]
        pub fn update_stream_permission(
            origin: OriginFor<T>,
            permission_id: PermissionId,
            new_recipients: Option<
                BoundedBTreeMap<T::AccountId, u16, T::MaxRecipientsPerPermission>,
            >,
            new_streams: Option<BoundedBTreeMap<StreamId, Percent, T::MaxStreamsPerPermission>>,
            new_distribution_control: Option<DistributionControl<T>>,
            new_recipient_manager: Option<Option<T::AccountId>>,
            new_weight_setter: Option<Option<T::AccountId>>,
        ) -> DispatchResult {
            ext::stream_impl::update_stream_permission(
                origin,
                permission_id,
                new_recipients,
                new_streams,
                new_distribution_control,
                new_recipient_manager,
                new_weight_setter,
            )?;

            Ok(())
        }

        /// Allows a delegator to update the number of instances of a permission
        #[pallet::call_index(9)]
        #[pallet::weight(T::WeightInfo::update_namespace_permission())]
        pub fn update_namespace_permission(
            origin: OriginFor<T>,
            permission_id: PermissionId,
            max_instances: u32,
        ) -> DispatchResult {
            ext::namespace_impl::update_namespace_permission::<T>(
                origin,
                permission_id,
                max_instances,
            )?;

            Ok(())
        }
    }
}

/// Get total allocated percentage for a delegator
fn get_total_allocated_percentage<T: Config>(
    delegator: &T::AccountId,
    stream: &StreamId,
) -> Percent {
    AccumulatedStreamAmounts::<T>::iter_key_prefix((delegator, stream))
        .filter_map(Permissions::<T>::get)
        .map(|contract| match contract.scope {
            PermissionScope::Stream(StreamScope {
                allocation: StreamAllocation::Streams(streams),
                ..
            }) => streams.get(stream).copied().unwrap_or_default(),
            _ => Percent::zero(),
        })
        .fold(Percent::zero(), |acc, percentage| {
            acc.saturating_add(percentage)
        })
}
