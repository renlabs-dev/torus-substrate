#[macro_export]
macro_rules! storage_info {
    ($($module:ident),*$(,)?) => {
        impl<T: polkadot_sdk::frame_system::Config> Pallet<T> {
            #[doc(hidden)]
            pub fn storage_metadata(
            ) -> polkadot_sdk::frame_support::__private::metadata_ir::PalletStorageMetadataIR
            {
                let mut entries = alloc::vec::Vec::new();
                $(
                    entries.append(&mut $module::_storage_metadata::<T>());
                )+

                polkadot_sdk::frame_support::__private::metadata_ir::PalletStorageMetadataIR {
                    prefix: <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<Pallet<T>>().expect("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`."),
                    entries,
                }
            }
        }

        impl<T: polkadot_sdk::frame_system::Config> polkadot_sdk::frame_support::traits::StorageInfoTrait for Pallet<T> {
            fn storage_info() -> Vec<polkadot_sdk::polkadot_sdk_frame::traits::StorageInfo> {
                let mut storages = alloc::vec::Vec::new();
                $(
                    storages.append(&mut $module::_storage_info::<T>());
                )+
                storages
            }
        }
    };
}

#[macro_export]
macro_rules! error {
        ($($(#[$attr:meta])* $name:ident),*$(,)?) => {
            #[derive(
                polkadot_sdk::frame_support::__private::codec::Encode,
                polkadot_sdk::frame_support::__private::codec::Decode,
                polkadot_sdk::frame_support::__private::scale_info::TypeInfo,
                polkadot_sdk::frame_support::PalletError,
            )]
            #[scale_info(skip_type_params(T), capture_docs = "always")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error<T> {
                #[doc(hidden)]
                #[codec(skip)]
                __Ignore(
                    core::marker::PhantomData<T>,
                    polkadot_sdk::frame_support::Never,
                ),
                $(
                    $(#[$attr])*
                    $name
                ),*
            }
        };
    }

#[macro_export]
macro_rules! storage {
        ($($(#[$attr:meta])* $vis:vis $name:ident = $ty:ident<$($ty_mod:ty),+>;)*) => {
            paste::paste! {
                $(
                    // This is the storage type itself.
                    $(#[$attr])*
                    $vis type $name<T> = $ty<[<_ Storage $name Prefix>]<T>, $($ty_mod),+>;

                    // The storage tag. This is used to define the storage details
                    // like what pallet prefix and name to use when hashing the key.
                    // This allows reusing types like `StorageValue` without having
                    // to redefine them every time.
                    #[doc(hidden)]
                    pub struct [<_ Storage $name Prefix>]<T>(core::marker::PhantomData<T>);

                    impl<T: polkadot_sdk::frame_system::Config> polkadot_sdk::frame_support::traits::StorageInstance
                        for [<_ Storage $name Prefix>]<T> {
                        fn pallet_prefix() -> &'static str {
                            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is implemented by the runtime")
                        }

                        fn pallet_prefix_hash() -> [u8; 16] {
                            <<T as polkadot_sdk::frame_system::Config>::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo>::name_hash::<Self>().expect("Pallet is part of the runtime because pallet `Config` trait is implemented by the runtime")
                        }

                        const STORAGE_PREFIX: &'static str = stringify!($name);

                        fn storage_prefix_hash() -> [u8; 16] {
                            polkadot_sdk::sp_io::hashing::twox_128(Self::STORAGE_PREFIX.as_bytes())
                        }

                        fn prefix_hash() -> [u8; 32] {
                            let mut final_key = [0u8; 32];
                            final_key[..16].copy_from_slice(&Self::pallet_prefix_hash());
                            final_key[16..].copy_from_slice(&Self::storage_prefix_hash());

                            final_key
                        }
                    }
                )*
            }

             // The storage information used by substrate.
             #[doc(hidden)]
             pub(crate) fn _storage_info<T: polkadot_sdk::frame_system::Config>() ->
                 Vec<polkadot_sdk::polkadot_sdk_frame::traits::StorageInfo>
             {
                 let mut storages = alloc::vec::Vec::new();
                 $(
                     storages.append(&mut <$name<T> as polkadot_sdk::frame_support::traits::StorageInfoTrait>::storage_info());
                 )*
                 storages
             }

             // The storage metadata used by substrate.
             #[doc(hidden)]
             pub(crate) fn _storage_metadata<T: polkadot_sdk::frame_system::Config>() ->
                 Vec<polkadot_sdk::frame_support::__private::metadata_ir::StorageEntryMetadataIR>
             {
                 let mut metadata = alloc::vec::Vec::new();
                 $(
                     <$name<T> as polkadot_sdk::frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(alloc::vec::Vec::new(), &mut metadata);
                 )*
                 metadata
             }
        };
    }
