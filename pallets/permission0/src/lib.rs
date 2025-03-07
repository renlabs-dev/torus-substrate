#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;
pub use weights::*;

pub mod ext;
pub mod permission;

use permission::{
    generate_permission_id, DistributionControl, EmissionAllocation, EmissionScope,
    PermissionContract, PermissionDuration, PermissionId, PermissionScope, RevocationTerms,
};
use polkadot_sdk::{
    frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        sp_runtime::traits::Saturating,
        traits::Currency,
        traits::{Get, ReservableCurrency},
        BoundedVec,
    },
    frame_system::{self, pallet_prelude::*},
    polkadot_sdk_frame as frame,
    sp_runtime::Percent,
    sp_std::prelude::*,
};

#[frame::pallet]
pub mod pallet {
    use super::*;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config(with_default)]
    pub trait Config: polkadot_sdk::frame_system::Config {
        #[pallet::no_default_bounds]
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        type WeightInfo: WeightInfo;

        type Currency: ReservableCurrency<Self::AccountId>;

        type Torus: pallet_torus0_api::Torus0Api<
            Self::AccountId,
            <Self::Currency as Currency<Self::AccountId>>::Balance,
            <Self::Currency as Currency<Self::AccountId>>::NegativeImbalance,
        >;

        /// Maximum number of targets per permission.
        #[pallet::constant]
        #[pallet::no_default_bounds]
        type MaxTargetsPerPermission: Get<u32>;

        /// Minimum threshold for auto-distribution
        #[pallet::constant]
        #[pallet::no_default_bounds]
        type MinAutoDistributionThreshold: Get<BalanceOf<Self>>;
    }

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

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
    >;

    /// Permissions granted by a specific account
    #[pallet::storage]
    pub type PermissionsByGrantor<T: Config> =
        StorageMap<_, Identity, T::AccountId, BoundedVec<PermissionId, T::MaxTargetsPerPermission>>;

    /// Permissions received by a specific account
    #[pallet::storage]
    pub type PermissionsByGrantee<T: Config> =
        StorageMap<_, Identity, T::AccountId, BoundedVec<PermissionId, T::MaxTargetsPerPermission>>;

    /// Accumulated amounts for each permission contract
    #[pallet::storage]
    pub type AccumulatedAmounts<T: Config> = StorageMap<_, Identity, PermissionId, BalanceOf<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Permission granted from grantor to grantee with ID
        PermissionGranted {
            grantor: T::AccountId,
            grantee: T::AccountId,
            permission_id: PermissionId,
            percentage: Option<Percent>,
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
            amount: BalanceOf<T>,
        },
        /// Auto-distribution executed
        AutoDistributionExecuted {
            grantor: T::AccountId,
            grantee: T::AccountId,
            permission_id: PermissionId,
            amount: BalanceOf<T>,
        },
        /// Permission expired with ID
        PermissionExpired {
            grantor: T::AccountId,
            grantee: T::AccountId,
            permission_id: PermissionId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The agent is not registered
        NotRegisteredAgent,
        /// Permission not found
        PermissionNotFound,
        /// Self-permission is not allowed
        SelfPermissionNotAllowed,
        /// Invalid percentage (out of range)
        InvalidPercentage,
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
        /// Too many targets
        TooManyTargets,
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
        /// Unsupported permission type
        UnsupportedPermissionType,
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
        #[pallet::weight(T::WeightInfo::grant_permission())]
        pub fn grant_permission(
            origin: OriginFor<T>,
            grantee: T::AccountId,
            allocation: EmissionAllocation<T>,
            targets: Vec<(T::AccountId, u16)>,
            distribution: DistributionControl<T>,
            duration: PermissionDuration<T>,
            revocation: RevocationTerms<T>,
            parent_id: Option<PermissionId>,
        ) -> DispatchResult {
            let grantor = ensure_signed(origin)?;

            ext::grant_permission_impl::<T>(
                grantor,
                grantee,
                allocation,
                targets,
                distribution,
                duration,
                revocation,
                parent_id,
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
    }
}

/// Get total allocated percentage for a grantor
fn get_total_allocated_percentage<T: Config>(grantor: &T::AccountId) -> Percent {
    PermissionsByGrantor::<T>::get(grantor)
        .iter()
        .flatten()
        .filter_map(Permissions::<T>::get)
        .map(|contract| match contract.scope {
            PermissionScope::Emission(EmissionScope {
                allocation: EmissionAllocation::Percentage(percentage),
                ..
            }) => percentage,
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
    // Update (grantor, grantee) -> permission_id mapping
    PermissionsByParticipants::<T>::try_mutate(
        (grantor.clone(), grantee.clone()),
        |permissions| -> Result<(), DispatchError> {
            match permissions {
                Some(ids) => {
                    ids.try_push(permission_id)
                        .map_err(|_| Error::<T>::TooManyTargets)?;
                }
                None => {
                    let mut new_ids =
                        BoundedVec::<PermissionId, T::MaxTargetsPerPermission>::default();
                    new_ids
                        .try_push(permission_id)
                        .map_err(|_| Error::<T>::TooManyTargets)?;
                    *permissions = Some(new_ids);
                }
            }
            Ok(())
        },
    )?;

    // Update grantor -> permission_id mapping
    PermissionsByGrantor::<T>::try_mutate(
        grantor.clone(),
        |permissions| -> Result<(), DispatchError> {
            match permissions {
                Some(ids) => {
                    ids.try_push(permission_id)
                        .map_err(|_| Error::<T>::TooManyTargets)?;
                }
                None => {
                    let mut new_ids =
                        BoundedVec::<PermissionId, T::MaxTargetsPerPermission>::default();
                    new_ids
                        .try_push(permission_id)
                        .map_err(|_| Error::<T>::TooManyTargets)?;
                    *permissions = Some(new_ids);
                }
            }
            Ok(())
        },
    )?;

    // Update grantee -> permission_id mapping
    PermissionsByGrantee::<T>::try_mutate(
        grantee.clone(),
        |permissions| -> Result<(), DispatchError> {
            match permissions {
                Some(ids) => {
                    ids.try_push(permission_id)
                        .map_err(|_| Error::<T>::TooManyTargets)?;
                }
                None => {
                    let mut new_ids =
                        BoundedVec::<PermissionId, T::MaxTargetsPerPermission>::default();
                    new_ids
                        .try_push(permission_id)
                        .map_err(|_| Error::<T>::TooManyTargets)?;
                    *permissions = Some(new_ids);
                }
            }
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
        if let Some(ids) = permissions {
            ids.retain(|id| *id != permission_id);
        }
    });

    PermissionsByGrantor::<T>::mutate(&grantor, |permissions| {
        if let Some(ids) = permissions {
            ids.retain(|id| *id != permission_id);
        }
    });

    PermissionsByGrantee::<T>::mutate(&grantee, |permissions| {
        if let Some(ids) = permissions {
            ids.retain(|id| *id != permission_id);
        }
    });
}
