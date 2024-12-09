#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

use core::marker::PhantomData;

use polkadot_sdk::{
    frame_support::Identity,
    polkadot_sdk_frame::{
        self as frame,
        prelude::{StorageMap, StorageValue},
    },
    sp_core::ConstU32,
    sp_runtime::BoundedVec,
};

mod macros;

// pub use pallet::*;

// mod foo;

// #[frame::pallet]
// pub mod pallet {

//     use frame::{
//         prelude::{BlockNumberFor, Weight},
//         traits::Hooks,
//     };
//     use polkadot_sdk::frame_support::traits::IsType;

//     use super::*;

//     #[pallet::config]
//     pub trait Config: polkadot_sdk::frame_system::Config {
//         /// The events emitted on proposal changes.
//         type RuntimeEvent: From<Event<Self>>
//             + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;
//     }

//     #[pallet::pallet]
//     pub struct Pallet<T>(_);

//     #[pallet::storage]
//     type StorageFoo<T: Config> = StorageValue<_, ()>;

//     #[pallet::hooks]
//     impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
//         fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
//             let _block_number: u64 = block_number
//                 .try_into()
//                 .ok()
//                 .expect("blockchain won't pass 2 ^ 64 blocks");
//             Weight::zero()
//         }
//     }

//     #[pallet::event]
//     #[pallet::generate_deposit(pub(crate) fn deposit_event)]
//     pub enum Event<T: Config> {
//         /// A new application has been created.
//         ApplicationCreated(u64),
//     }

//     #[pallet::error]
//     pub enum Error<T> {
//         /// The proposal is already finished. Do not retry.
//         ProposalIsFinished,
//     }
// }

pub struct Pallet<T>(PhantomData<T>);

impl<T: polkadot_sdk::frame_system::Config> polkadot_sdk::frame_support::traits::PalletInfoAccess
    for Pallet<T>
{
    fn index() -> usize {
        <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::index::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is implemented by the runtime")
    }

    fn name() -> &'static str {
        <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is implemented by the runtime")
    }

    fn name_hash() -> [u8; 16] {
        <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name_hash::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is implemented by the runtime")
    }

    fn module_name() -> &'static str {
        <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::module_name::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is implemented by the runtime")
    }

    fn crate_version() -> polkadot_sdk::frame_support::traits::CrateVersion {
        polkadot_sdk::frame_support::traits::CrateVersion {
            major: 0u16,
            minor: 1u8,
            patch: 0u8,
        }
    }
}

impl<T: polkadot_sdk::frame_system::Config> polkadot_sdk::frame_support::traits::OnGenesis
    for Pallet<T>
{
    fn on_genesis() {
        let storage_version: polkadot_sdk::frame_support::traits::StorageVersion =
            core::default::Default::default();
        storage_version.put::<Self>();
    }
}

impl<T: polkadot_sdk::frame_system::Config>
    polkadot_sdk::frame_support::traits::Hooks<
        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T>
{
}

mod hooks {
    use super::*;

    impl<T: polkadot_sdk::frame_system::Config>
        polkadot_sdk::frame_support::traits::OnFinalize<
            polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        > for Pallet<T>
    {
        fn on_finalize(n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>) {
            polkadot_sdk::frame_support::__private::sp_tracing::enter_span!(
                polkadot_sdk::frame_support::__private::sp_tracing::trace_span!("on_finalize")
            );
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_finalize(n)
        }
    }

    impl<T: polkadot_sdk::frame_system::Config>
        polkadot_sdk::frame_support::traits::OnIdle<
            polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        > for Pallet<T>
    {
        fn on_idle(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            remaining_weight: polkadot_sdk::frame_support::weights::Weight,
        ) -> polkadot_sdk::frame_support::weights::Weight {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_idle(n, remaining_weight)
        }
    }
    impl<T: polkadot_sdk::frame_system::Config>
        polkadot_sdk::frame_support::traits::OnPoll<
            polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        > for Pallet<T>
    {
        fn on_poll(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            weight: &mut polkadot_sdk::frame_support::weights::WeightMeter,
        ) {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_poll(n, weight);
        }
    }
    impl<T: polkadot_sdk::frame_system::Config>
        polkadot_sdk::frame_support::traits::OnInitialize<
            polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        > for Pallet<T>
    {
        fn on_initialize(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        ) -> polkadot_sdk::frame_support::weights::Weight {
            polkadot_sdk::frame_support::__private::sp_tracing::enter_span!(
                polkadot_sdk::frame_support::__private::sp_tracing::trace_span!("on_initialize")
            );
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_initialize(n)
        }
    }
    impl<T: polkadot_sdk::frame_system::Config>
        polkadot_sdk::frame_support::traits::BeforeAllRuntimeMigrations for Pallet<T>
    {
        fn before_all_runtime_migrations() -> polkadot_sdk::frame_support::weights::Weight {
            use polkadot_sdk::frame_support::storage::unhashed::contains_prefixed_key;
            use polkadot_sdk::frame_support::traits::{Get, PalletInfoAccess};
            polkadot_sdk::frame_support::__private::sp_tracing::enter_span!(
                polkadot_sdk::frame_support::__private::sp_tracing::trace_span!("before_all")
            );
            let pallet_hashed_prefix = <Self as PalletInfoAccess>::name_hash();
            let exists = contains_prefixed_key(&pallet_hashed_prefix);
            if !exists {
                let default_version = polkadot_sdk::frame_support::traits::StorageVersion::new(0);

                polkadot_sdk::polkadot_sdk_frame::log::info!(
                    "🐥 New pallet {:?} detected in the runtime. The pallet has no defined storage version, so the on-chain version is being initialized to {:?}.",
                    <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<Self>().unwrap_or("<unknown pallet name>"),
                    default_version
                );

                default_version.put::<Self>();
                <T as polkadot_sdk::frame_system::Config>::DbWeight::get().reads_writes(1, 1)
            } else {
                <T as polkadot_sdk::frame_system::Config>::DbWeight::get().reads(1)
            }
        }
    }
    impl<T: polkadot_sdk::frame_system::Config>
        polkadot_sdk::frame_support::traits::OnRuntimeUpgrade for Pallet<T>
    {
        fn on_runtime_upgrade() -> polkadot_sdk::frame_support::weights::Weight {
            polkadot_sdk::frame_support::__private::sp_tracing::enter_span!(
                polkadot_sdk::frame_support::__private::sp_tracing::trace_span!(
                    "on_runtime_update"
                )
            );

            polkadot_sdk::polkadot_sdk_frame::log::debug!(
                "✅ no migration for {}",
                <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<Self>().unwrap_or("<unknown pallet name>")
            );

            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_runtime_upgrade()
        }
    }
    impl<T: polkadot_sdk::frame_system::Config>
        polkadot_sdk::frame_support::traits::OffchainWorker<
            polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        > for Pallet<T>
    {
        fn offchain_worker(n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>) {
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::offchain_worker(n)
        }
    }
    impl<T: polkadot_sdk::frame_system::Config> polkadot_sdk::frame_support::traits::IntegrityTest
        for Pallet<T>
    {
        fn integrity_test() {
            polkadot_sdk::frame_support::__private::sp_io::TestExternalities::default()
                .execute_with(|| {
                    <Self as polkadot_sdk::frame_support::traits::Hooks<
                        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
                    >>::integrity_test()
                });
        }
    }
}

impl<T: polkadot_sdk::frame_system::Config> Pallet<T> {
    #[doc(hidden)]
    pub fn pallet_documentation_metadata(
    ) -> polkadot_sdk::frame_support::__private::Vec<&'static str> {
        alloc::vec::Vec::new()
    }
}

impl<T: polkadot_sdk::frame_system::Config> Pallet<T> {
    #[doc(hidden)]
    pub fn pallet_constants_metadata() -> polkadot_sdk::frame_support::__private::Vec<
        polkadot_sdk::frame_support::__private::metadata_ir::PalletConstantMetadataIR,
    > {
        alloc::vec::Vec::new()
    }
}

impl<T: polkadot_sdk::frame_system::Config> Pallet<T> {
    #[doc(hidden)]
    pub fn error_metadata(
    ) -> Option<polkadot_sdk::frame_support::__private::metadata_ir::PalletErrorMetadataIR> {
        None
    }
}

storage!(
    /// Foo
    pub(self) Foo = StorageValue<u16>;

    /// Bar
    pub(crate) Bar = StorageValue<BoundedVec<i32, ConstU32<10>>>;

    pub Boo = StorageMap<Identity, u16, i32>;
);

storage_info!(self);

error!(
    /// Proposal is not enabled.
    ProposalNotEnabled
);

mod internal {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __tt_default_parts_v2_7 {
        {
            $caller:tt your_tt_return = [{
                $my_tt_return:path
            }]
        } => {
            $my_tt_return!{
                $caller tokens = [{
                    +Pallet+Storage
                }]
            }
        };
    }
    pub use __tt_default_parts_v2_7 as tt_default_parts_v2;

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __tt_error_token_1 {
        {
            $caller:tt your_tt_return = [{
                $my_tt_return:path
            }]
        } => {
            $my_tt_return!{
                $caller
            }
        };
    }
    pub use __tt_error_token_1 as tt_error_token;
}

pub use internal::*;
