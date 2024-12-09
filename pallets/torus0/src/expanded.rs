// Recursive expansion of pallet macro
// ====================================

#[doc = r"The `pallet` module in each FRAME pallet hosts the most important items needed
to construct this pallet.

The main components of this pallet are:
- [`Pallet`], which implements all of the dispatchable extrinsics of the pallet, among
other public functions.
	- The subset of the functions that are dispatchable can be identified either in the
	[`dispatchables`] module or in the [`Call`] enum.
- [`storage_types`], which contains the list of all types that are representing a
storage item. Otherwise, all storage items are listed among [*Type Definitions*](#types).
- [`Config`], which contains the configuration trait of this pallet.
- [`Event`] and [`Error`], which are listed among the [*Enums*](#enums).
		"]
pub mod pallet {
    use super::*;
    use frame::{
        prelude::{BlockNumberFor, Weight},
        traits::Hooks,
    };
    use polkadot_sdk::frame_support::traits::IsType;
    #[doc = r"
Configuration trait of this pallet.

The main purpose of this trait is to act as an interface between this pallet and the runtime in
which it is embedded in. A type, function, or constant in this trait is essentially left to be
configured by the runtime that includes this pallet.

Consequently, a runtime that wants to include this pallet must implement this trait."]
    pub trait Config: polkadot_sdk::frame_system::Config {
        #[doc = " The events emitted on proposal changes."]
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;
    }
    #[doc = r"
				The `Pallet` struct, the main type that implements traits and standalone
				functions within the pallet.
			"]
    #[derive(
        polkadot_sdk::frame_support::CloneNoBound,
        polkadot_sdk::frame_support::EqNoBound,
        polkadot_sdk::frame_support::PartialEqNoBound,
        polkadot_sdk::frame_support::RuntimeDebugNoBound,
    )]
    pub struct Pallet<T>(core::marker::PhantomData<(T)>);

    #[allow(type_alias_bounds)]
    #[doc = ""]
    #[doc = "Storage type is [`StorageValue`] with value type `()`."]
    type StorageFoo<T: Config> = StorageValue<_GeneratedPrefixForStorageStorageFoo<T>, ()>;
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            let _block_number: u64 = block_number
                .try_into()
                .ok()
                .expect("blockchain won't pass 2 ^ 64 blocks");
            Weight::zero()
        }
    }
    #[doc = "The `Event` enum of this pallet"]
    #[derive(
        polkadot_sdk::frame_support::CloneNoBound,
        polkadot_sdk::frame_support::EqNoBound,
        polkadot_sdk::frame_support::PartialEqNoBound,
        polkadot_sdk::frame_support::RuntimeDebugNoBound,
        polkadot_sdk::frame_support::__private::codec::Encode,
        polkadot_sdk::frame_support::__private::codec::Decode,
        polkadot_sdk::frame_support::__private::scale_info::TypeInfo,
    )]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        #[doc = " A new application has been created."]
        ApplicationCreated(u64),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            ::core::marker::PhantomData<(T)>,
            polkadot_sdk::frame_support::Never,
        ),
    }
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
            core::marker::PhantomData<(T)>,
            polkadot_sdk::frame_support::Never,
        ),
        #[doc = " The proposal is already finished. Do not retry."]
        ProposalIsFinished,
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_documentation_metadata(
        ) -> polkadot_sdk::frame_support::__private::Vec<&'static str> {
            alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata() -> polkadot_sdk::frame_support::__private::Vec<
            polkadot_sdk::frame_support::__private::metadata_ir::PalletConstantMetadataIR,
        > {
            alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata(
        ) -> Option<polkadot_sdk::frame_support::__private::metadata_ir::PalletErrorMetadataIR>
        {
            Some(
                polkadot_sdk::frame_support::__private::metadata_ir::PalletErrorMetadataIR {
                    ty: polkadot_sdk::frame_support::__private::scale_info::meta_type::<Error<T>>(),
                },
            )
        }
    }
    #[doc = r" Type alias to `Pallet`, to be used by `construct_runtime`."]
    #[doc = r""]
    #[doc = r" Generated by `pallet` attribute macro."]
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> polkadot_sdk::frame_support::traits::GetStorageVersion for Pallet<T> {
        type InCodeStorageVersion = polkadot_sdk::frame_support::traits::NoStorageVersionSet;
        fn in_code_storage_version() -> Self::InCodeStorageVersion {
            core::default::Default::default()
        }
        fn on_chain_storage_version() -> polkadot_sdk::frame_support::traits::StorageVersion {
            polkadot_sdk::frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version: polkadot_sdk::frame_support::traits::StorageVersion =
                core::default::Default::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::index:: <Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn name() -> &'static str {
            < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::name:: <Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn name_hash() -> [u8; 16] {
            < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::name_hash:: <Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn module_name() -> &'static str {
            < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::module_name:: <Self>().expect("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn crate_version() -> polkadot_sdk::frame_support::traits::CrateVersion {
            polkadot_sdk::frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn infos() -> polkadot_sdk::frame_support::__private::Vec<
            polkadot_sdk::frame_support::traits::PalletInfoData,
        > {
            use polkadot_sdk::frame_support::traits::PalletInfoAccess;
            let item = polkadot_sdk::frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            <[_]>::into_vec(
                #[rustc_box]
                alloc::boxed::Box::new([item]),
            )
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> polkadot_sdk::frame_support::__private::Vec<
            polkadot_sdk::frame_support::traits::StorageInfo,
        > {
            #[allow(unused_mut)]
            let mut res = alloc::vec::Vec::new();
            {
                let mut storage_info =  <StorageFoo<T>as polkadot_sdk::frame_support::traits::StorageInfoTrait> ::storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    use polkadot_sdk::frame_support::traits::{
        StorageInfoTrait, TrackedStorageKey, WhitelistedStorageKeys,
    };
    impl<T: Config> WhitelistedStorageKeys for Pallet<T> {
        fn whitelisted_storage_keys(
        ) -> polkadot_sdk::frame_support::__private::Vec<TrackedStorageKey> {
            use polkadot_sdk::frame_support::__private::vec;
            alloc::vec::Vec::new()
        }
    }
    #[doc(hidden)]
    mod warnings {}
    #[allow(unused_imports)]
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_call_part_defined_0 {
            ($pallet_name:ident) => {
                compile_error!(concat!("`",stringify!($pallet_name),"` does not have #[pallet::call] defined, perhaps you should remove `Call` from \
				construct_runtime?",));
            };
        }
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    #[doc = r"Contains a variant per dispatchable extrinsic that this pallet has."]
    #[derive(
        polkadot_sdk::frame_support::RuntimeDebugNoBound,
        polkadot_sdk::frame_support::CloneNoBound,
        polkadot_sdk::frame_support::EqNoBound,
        polkadot_sdk::frame_support::PartialEqNoBound,
        polkadot_sdk::frame_support::__private::codec::Encode,
        polkadot_sdk::frame_support::__private::codec::Decode,
        polkadot_sdk::frame_support::__private::scale_info::TypeInfo,
    )]
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            ::core::marker::PhantomData<(T,)>,
            polkadot_sdk::frame_support::Never,
        ),
    }
    impl<T: Config> Call<T> {}

    impl<T: Config> polkadot_sdk::frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> polkadot_sdk::frame_support::dispatch::DispatchInfo {
            match *self {
                Self::__Ignore(_, _) => {
                    core::panicking::panic_fmt(core::const_format_args!(
                        "internal error: entered unreachable code: {}",
                        core::format_args!("__Ignore cannot be used")
                    ));
                }
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::dispatch::CheckIfFeeless for Call<T> {
        type Origin = polkadot_sdk::frame_system::pallet_prelude::OriginFor<T>;
        #[allow(unused_variables)]
        fn is_feeless(&self, origin: &Self::Origin) -> bool {
            match *self {
                Self::__Ignore(_, _) => {
                    core::panicking::panic_fmt(core::const_format_args!(
                        "internal error: entered unreachable code: {}",
                        core::format_args!("__Ignore cannot be used")
                    ));
                }
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::__Ignore(_, _) => {
                    core::panicking::panic_fmt(core::const_format_args!(
                        "internal error: entered unreachable code: {}",
                        core::format_args!("__PhantomItem cannot be used.")
                    ));
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &[]
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::GetCallIndex for Call<T> {
        fn get_call_index(&self) -> u8 {
            match *self {
                Self::__Ignore(_, _) => {
                    core::panicking::panic_fmt(core::const_format_args!(
                        "internal error: entered unreachable code: {}",
                        core::format_args!("__PhantomItem cannot be used.")
                    ));
                }
            }
        }
        fn get_call_indices() -> &'static [u8] {
            &[]
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::UnfilteredDispatchable for Call<T> {
        type RuntimeOrigin = polkadot_sdk::frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::RuntimeOrigin,
        ) -> polkadot_sdk::frame_support::dispatch::DispatchResultWithPostInfo {
            polkadot_sdk::frame_support::dispatch_context::run_in_context(|| match self {
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    {
                        core::panicking::panic_fmt(core::const_format_args!(
                            "internal error: entered unreachable code: {}",
                            core::format_args!("__PhantomItem cannot be used.")
                        ));
                    };
                }
            })
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::dispatch::Callable<T> for Pallet<T> {
        type RuntimeCall = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn call_functions(
        ) -> polkadot_sdk::frame_support::__private::metadata_ir::PalletCallMetadataIR {
            polkadot_sdk::frame_support::__private::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> core::fmt::Debug for Error<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    core::panicking::panic_fmt(core::const_format_args!(
                        "internal error: entered unreachable code: {}",
                        core::format_args!("`__Ignore` can never be constructed")
                    ));
                }
                Self::ProposalIsFinished => "ProposalIsFinished",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for polkadot_sdk::frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use polkadot_sdk::frame_support::__private::codec::Encode;
            let index =  < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::index:: <Pallet<T>>().expect("Every active module has an index in the runtime; qed")as u8;
            let mut encoded = err.encode();
            encoded.resize(
                polkadot_sdk::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE,
                0,
            );
            polkadot_sdk::frame_support::sp_runtime::DispatchError::Module(polkadot_sdk::frame_support::sp_runtime::ModuleError {
                index,error:TryInto::try_into(encoded).expect("encoded error is resized to be equal to the maximum encoded error size; qed"),message:Some(err.as_str()),
            })
        }
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __tt_error_token_1 {
        {
            $caller:tt your_tt_return = [{
                $my_tt_return:path
            }]
        } => {
            $my_tt_return!{
                $caller error = [{
                    Error
                }]
            }
        };
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_event_part_defined_2 {
            ($pallet_name:ident) => {};
        }
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(crate) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
            let event = <<T as Config>::RuntimeEvent as Into<
                <T as polkadot_sdk::frame_system::Config>::RuntimeEvent,
            >>::into(event);
            <polkadot_sdk::frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata(
        ) -> polkadot_sdk::frame_support::__private::metadata_ir::PalletStorageMetadataIR {
            polkadot_sdk::frame_support::__private::metadata_ir::PalletStorageMetadataIR {
                prefix: < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::name:: <Pallet<T>>().expect("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`."),entries:{
                    #[allow(unused_mut)]
                    let mut entries = alloc::vec::Vec::new();
                    {
                        <StorageFoo<T>as polkadot_sdk::frame_support::storage::StorageEntryMetadataBuilder> ::build_metadata(alloc::vec::Vec::new(), &mut entries,);
                    }entries
                },
            }
        }
    }
    #[doc(hidden)]
    struct _GeneratedPrefixForStorageStorageFoo<T>(core::marker::PhantomData<(T,)>);

    impl<T: Config> polkadot_sdk::frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageStorageFoo<T>
    {
        fn pallet_prefix() -> &'static str {
            < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::name:: <Pallet<T>>().expect("No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
        }
        fn pallet_prefix_hash() -> [u8; 16] {
            < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::name_hash:: <Pallet<T>>().expect("No name_hash found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.")
        }
        const STORAGE_PREFIX: &'static str = "StorageFoo";
        fn storage_prefix_hash() -> [u8; 16] {
            [
                166u8, 26u8, 228u8, 24u8, 128u8, 184u8, 229u8, 46u8, 186u8, 118u8, 69u8, 116u8,
                232u8, 228u8, 147u8, 5u8,
            ]
        }
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_inherent_part_defined_3 {
            ($pallet_name:ident) => {
                compile_error!(concat!(
                    "`",
                    stringify!($pallet_name),
                    "` does not have #[pallet::inherent] defined, perhaps you should \
				remove `Inherent` from construct_runtime?",
                ));
            };
        }
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    #[doc = r" Hidden instance generated to be internally used when module is used without"]
    #[doc = r" instance."]
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<T: Config>
        polkadot_sdk::frame_support::traits::OnFinalize<
            polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        > for Pallet<T>
    {
        fn on_finalize(n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>) {
            let __within_span__ = ({
                use tracing::__macro_support::Callsite as _;
                static __CALLSITE: tracing::callsite::DefaultCallsite = {
                    static META: tracing::Metadata<'static> = {
                        tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            (module_path!()),
                            (tracing::Level::TRACE),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::file!(),
                            ),
                            tracing_core::__macro_support::Option::Some(0u32),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::module_path!(),
                            ),
                            tracing_core::field::FieldSet::new(
                                (&[]),
                                tracing_core::callsite::Identifier((&__CALLSITE)),
                            ),
                            (tracing::metadata::Kind::SPAN),
                        )
                    };
                    tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = tracing::subscriber::Interest::never();
                if (tracing::Level::TRACE) <= tracing::level_filters::STATIC_MAX_LEVEL
                    && (tracing::Level::TRACE) <= tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
                {
                    let meta = __CALLSITE.metadata();
                    tracing::Span::new(meta, &{ (meta.fields()).value_set(&[]) })
                } else {
                    let span = tracing::__macro_support::__disabled_span(__CALLSITE.metadata());
                    if match (tracing::Level::TRACE) {
                        tracing::Level::ERROR => tracing::log::Level::Error,
                        tracing::Level::WARN => tracing::log::Level::Warn,
                        tracing::Level::INFO => tracing::log::Level::Info,
                        tracing::Level::DEBUG => tracing::log::Level::Debug,
                        _ => tracing::log::Level::Trace,
                    } <= tracing::log::STATIC_MAX_LEVEL
                    {
                        if !tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    (__CALLSITE.metadata().fields()).value_set(&[])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            });
            let __tracing_guard__ = __within_span__.enter();
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_finalize(n)
        }
    }
    impl<T: Config>
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
    impl<T: Config>
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
    impl<T: Config>
        polkadot_sdk::frame_support::traits::OnInitialize<
            polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        > for Pallet<T>
    {
        fn on_initialize(
            n: polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
        ) -> polkadot_sdk::frame_support::weights::Weight {
            let __within_span__ = ({
                use tracing::__macro_support::Callsite as _;
                static __CALLSITE: tracing::callsite::DefaultCallsite = {
                    static META: tracing::Metadata<'static> = {
                        tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            (module_path!()),
                            (tracing::Level::TRACE),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::file!(),
                            ),
                            tracing_core::__macro_support::Option::Some(0u32),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::module_path!(),
                            ),
                            tracing_core::field::FieldSet::new(
                                (&[]),
                                tracing_core::callsite::Identifier((&__CALLSITE)),
                            ),
                            (tracing::metadata::Kind::SPAN),
                        )
                    };
                    tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = tracing::subscriber::Interest::never();
                if (tracing::Level::TRACE) <= tracing::level_filters::STATIC_MAX_LEVEL
                    && (tracing::Level::TRACE) <= tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
                {
                    let meta = __CALLSITE.metadata();
                    tracing::Span::new(meta, &{ (meta.fields()).value_set(&[]) })
                } else {
                    let span = tracing::__macro_support::__disabled_span(__CALLSITE.metadata());
                    if match (tracing::Level::TRACE) {
                        tracing::Level::ERROR => tracing::log::Level::Error,
                        tracing::Level::WARN => tracing::log::Level::Warn,
                        tracing::Level::INFO => tracing::log::Level::Info,
                        tracing::Level::DEBUG => tracing::log::Level::Debug,
                        _ => tracing::log::Level::Trace,
                    } <= tracing::log::STATIC_MAX_LEVEL
                    {
                        if !tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    (__CALLSITE.metadata().fields()).value_set(&[])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            });
            let __tracing_guard__ = __within_span__.enter();
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_initialize(n)
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::BeforeAllRuntimeMigrations for Pallet<T> {
        fn before_all_runtime_migrations() -> polkadot_sdk::frame_support::weights::Weight {
            use polkadot_sdk::frame_support::__private::hashing::twox_128;
            use polkadot_sdk::frame_support::storage::unhashed::contains_prefixed_key;
            use polkadot_sdk::frame_support::traits::{Get, PalletInfoAccess};
            let __within_span__ = ({
                use tracing::__macro_support::Callsite as _;
                static __CALLSITE: tracing::callsite::DefaultCallsite = {
                    static META: tracing::Metadata<'static> = {
                        tracing_core::metadata::Metadata::new(
                            "before_all",
                            (module_path!()),
                            (tracing::Level::TRACE),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::file!(),
                            ),
                            tracing_core::__macro_support::Option::Some(0u32),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::module_path!(),
                            ),
                            tracing_core::field::FieldSet::new(
                                (&[]),
                                tracing_core::callsite::Identifier((&__CALLSITE)),
                            ),
                            (tracing::metadata::Kind::SPAN),
                        )
                    };
                    tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = tracing::subscriber::Interest::never();
                if (tracing::Level::TRACE) <= tracing::level_filters::STATIC_MAX_LEVEL
                    && (tracing::Level::TRACE) <= tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
                {
                    let meta = __CALLSITE.metadata();
                    tracing::Span::new(meta, &{ (meta.fields()).value_set(&[]) })
                } else {
                    let span = tracing::__macro_support::__disabled_span(__CALLSITE.metadata());
                    if match (tracing::Level::TRACE) {
                        tracing::Level::ERROR => tracing::log::Level::Error,
                        tracing::Level::WARN => tracing::log::Level::Warn,
                        tracing::Level::INFO => tracing::log::Level::Info,
                        tracing::Level::DEBUG => tracing::log::Level::Debug,
                        _ => tracing::log::Level::Trace,
                    } <= tracing::log::STATIC_MAX_LEVEL
                    {
                        if !tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    (__CALLSITE.metadata().fields()).value_set(&[])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            });
            let __tracing_guard__ = __within_span__.enter();
            let pallet_hashed_prefix = <Self as PalletInfoAccess>::name_hash();
            let exists = contains_prefixed_key(&pallet_hashed_prefix);
            if !exists {
                let default_version = polkadot_sdk::frame_support::traits::StorageVersion::new(0);
                {
                    let lvl = (log::Level::Info);
                    if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
                        log::__private_api::log(log::__private_api::format_args!("🐥 New pallet {:?} detected in the runtime. The pallet has no defined storage version, so the on-chain version is being initialized to {:?}.", < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::name:: <Self>().unwrap_or("<unknown pallet name>"),default_version),lvl, &((polkadot_sdk::frame_support::LOG_TARGET),log::__private_api::module_path!(),log::__private_api::loc()),(),);
                    }
                };
                default_version.put::<Self>();
                <T as polkadot_sdk::frame_system::Config>::DbWeight::get().reads_writes(1, 1)
            } else {
                <T as polkadot_sdk::frame_system::Config>::DbWeight::get().reads(1)
            }
        }
    }
    impl<T: Config> polkadot_sdk::frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> polkadot_sdk::frame_support::weights::Weight {
            let __within_span__ = ({
                use tracing::__macro_support::Callsite as _;
                static __CALLSITE: tracing::callsite::DefaultCallsite = {
                    static META: tracing::Metadata<'static> = {
                        tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            (module_path!()),
                            (tracing::Level::TRACE),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::file!(),
                            ),
                            tracing_core::__macro_support::Option::Some(0u32),
                            tracing_core::__macro_support::Option::Some(
                                tracing_core::__macro_support::module_path!(),
                            ),
                            tracing_core::field::FieldSet::new(
                                (&[]),
                                tracing_core::callsite::Identifier((&__CALLSITE)),
                            ),
                            (tracing::metadata::Kind::SPAN),
                        )
                    };
                    tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = tracing::subscriber::Interest::never();
                if (tracing::Level::TRACE) <= tracing::level_filters::STATIC_MAX_LEVEL
                    && (tracing::Level::TRACE) <= tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
                {
                    let meta = __CALLSITE.metadata();
                    tracing::Span::new(meta, &{ (meta.fields()).value_set(&[]) })
                } else {
                    let span = tracing::__macro_support::__disabled_span(__CALLSITE.metadata());
                    if match (tracing::Level::TRACE) {
                        tracing::Level::ERROR => tracing::log::Level::Error,
                        tracing::Level::WARN => tracing::log::Level::Warn,
                        tracing::Level::INFO => tracing::log::Level::Info,
                        tracing::Level::DEBUG => tracing::log::Level::Debug,
                        _ => tracing::log::Level::Trace,
                    } <= tracing::log::STATIC_MAX_LEVEL
                    {
                        if !tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    (__CALLSITE.metadata().fields()).value_set(&[])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            });
            let __tracing_guard__ = __within_span__.enter();
            {
                let lvl = (log::Level::Debug);
                if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
                    log::__private_api::log(log::__private_api::format_args!("✅ no migration for {}", < <T as polkadot_sdk::frame_system::Config> ::PalletInfo as polkadot_sdk::frame_support::traits::PalletInfo> ::name:: <Self>().unwrap_or("<unknown pallet name>"),),lvl, &((polkadot_sdk::frame_support::LOG_TARGET),log::__private_api::module_path!(),log::__private_api::loc()),(),);
                }
            };
            <Self as polkadot_sdk::frame_support::traits::Hooks<
                polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_runtime_upgrade()
        }
    }
    impl<T: Config>
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
    impl<T: Config> polkadot_sdk::frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            polkadot_sdk::frame_support::__private::sp_io::TestExternalities::default()
                .execute_with(|| {
                    <Self as polkadot_sdk::frame_support::traits::Hooks<
                        polkadot_sdk::frame_system::pallet_prelude::BlockNumberFor<T>,
                    >>::integrity_test()
                });
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_genesis_config_defined_4 {
            ($pallet_name:ident) => {
                compile_error!(concat!(
                    "`",
                    stringify!($pallet_name),
                    "` does not have #[pallet::genesis_config] defined, perhaps you should \
								remove `Config` from construct_runtime?",
                ));
            };
        }
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_std_enabled_for_genesis_4 {
            ($pallet_name:ident, $pallet_path:expr) => {};
        }
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_origin_part_defined_5 {
            ($pallet_name:ident) => {
                compile_error!(concat!(
                    "`",
                    stringify!($pallet_name),
                    "` does not have #[pallet::origin] defined, perhaps you should \
				remove `Origin` from construct_runtime?",
                ));
            };
        }
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! __is_validate_unsigned_part_defined_6 {
            ($pallet_name:ident) => {
                compile_error!(concat!(
                    "`",
                    stringify!($pallet_name),
                    "` does not have #[pallet::validate_unsigned] defined, perhaps you should \
				remove `ValidateUnsigned` from construct_runtime?",
                ));
            };
        }
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __tt_default_parts_7 {
        {
            $caller:tt your_tt_return = [{
                $my_tt_return:path
            }]
        } => {
            $my_tt_return!{
                $caller tokens = [{
                    expanded::{
                        Pallet,Storage,Event<T> ,Error<T> ,
                    }
                }]
            }
        };
    }
    pub use __tt_default_parts_7 as tt_default_parts;
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __tt_extra_parts_7 {
        {
            $caller:tt your_tt_return = [{
                $my_tt_return:path
            }]
        } => {
            $my_tt_return!{
                $caller tokens = [{
                    expanded::{
                        Error<T> ,
                    }
                }]
            }
        };
    }
    pub use __tt_extra_parts_7 as tt_extra_parts;
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
                    +Pallet+Storage+Event<T> +Error<T>
                }]
            }
        };
    }
    pub use __tt_default_parts_v2_7 as tt_default_parts_v2;
    #[doc = r" Auto-generated docs-only module listing all (public and private) defined storage types"]
    #[doc = r" for this pallet."]
    #[doc = r""]
    #[doc = r" # Warning: Doc-Only"]
    #[doc = r""]
    #[doc = r" Members of this module cannot be used directly and are only provided for documentation"]
    #[doc = r" purposes."]
    #[doc = r""]
    #[doc = r" To see the actual storage type, find a struct with the same name at the root of the"]
    #[doc = r" pallet, in the list of [*Type Definitions*](../index.html#types)."]
    #[cfg(doc)]
    pub mod storage_types {
        use super::*;
        #[doc = r""]
        #[doc = r" # Warning: Doc-Only"]
        #[doc = r""]
        #[doc = r" This type is automatically generated, and is doc-only. See the real version in"]
        #[doc = "[`pallet::StorageFoo`]."]
        pub struct StorageFoo();
    }
    #[doc = r" Auto-generated docs-only module listing all defined dispatchables for this pallet."]
    #[doc = r""]
    #[doc = r" # Warning: Doc-Only"]
    #[doc = r""]
    #[doc = r" Members of this module cannot be used directly and are only provided for documentation"]
    #[doc = r" purposes. To see the real version of each dispatchable, look for them in [`Pallet`] or"]
    #[doc = r" [`Call`]."]
    #[cfg(doc)]
    pub mod dispatchables {
        use super::*;
    }
}
