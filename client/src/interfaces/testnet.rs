#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 15usize] = [
        "System",
        "Timestamp",
        "Aura",
        "Grandpa",
        "Balances",
        "TransactionPayment",
        "Sudo",
        "Multisig",
        "Ethereum",
        "EVM",
        "Governance",
        "Torus0",
        "Emission0",
        "Permission0",
        "Faucet",
    ];
    pub static RUNTIME_APIS: [&str; 0usize] = [];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::torus_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::torus_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::torus_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use subxt::ext::subxt_core::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {}
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn aura(&self) -> aura::constants::ConstantsApi {
            aura::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn multisig(&self) -> multisig::constants::ConstantsApi {
            multisig::constants::ConstantsApi
        }
        pub fn governance(&self) -> governance::constants::ConstantsApi {
            governance::constants::ConstantsApi
        }
        pub fn torus0(&self) -> torus0::constants::ConstantsApi {
            torus0::constants::ConstantsApi
        }
        pub fn emission0(&self) -> emission0::constants::ConstantsApi {
            emission0::constants::ConstantsApi
        }
        pub fn permission0(&self) -> permission0::constants::ConstantsApi {
            permission0::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi {
            multisig::storage::StorageApi
        }
        pub fn ethereum(&self) -> ethereum::storage::StorageApi {
            ethereum::storage::StorageApi
        }
        pub fn evm(&self) -> evm::storage::StorageApi {
            evm::storage::StorageApi
        }
        pub fn governance(&self) -> governance::storage::StorageApi {
            governance::storage::StorageApi
        }
        pub fn torus0(&self) -> torus0::storage::StorageApi {
            torus0::storage::StorageApi
        }
        pub fn emission0(&self) -> emission0::storage::StorageApi {
            emission0::storage::StorageApi
        }
        pub fn permission0(&self) -> permission0::storage::StorageApi {
            permission0::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi {
            multisig::calls::TransactionApi
        }
        pub fn ethereum(&self) -> ethereum::calls::TransactionApi {
            ethereum::calls::TransactionApi
        }
        pub fn evm(&self) -> evm::calls::TransactionApi {
            evm::calls::TransactionApi
        }
        pub fn governance(&self) -> governance::calls::TransactionApi {
            governance::calls::TransactionApi
        }
        pub fn torus0(&self) -> torus0::calls::TransactionApi {
            torus0::calls::TransactionApi
        }
        pub fn emission0(&self) -> emission0::calls::TransactionApi {
            emission0::calls::TransactionApi
        }
        pub fn permission0(&self) -> permission0::calls::TransactionApi {
            permission0::calls::TransactionApi
        }
        pub fn faucet(&self) -> faucet::calls::TransactionApi {
            faucet::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::ext::subxt_core::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                128u8, 117u8, 28u8, 52u8, 201u8, 68u8, 229u8, 86u8, 39u8, 69u8, 221u8, 157u8,
                220u8, 163u8, 38u8, 27u8, 219u8, 210u8, 245u8, 129u8, 96u8, 48u8, 36u8, 112u8,
                219u8, 28u8, 234u8, 130u8, 190u8, 75u8, 194u8, 54u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "Can be executed by every `origin`."]
                pub struct Remark {
                    pub remark: remark::Remark,
                }
                pub mod remark {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub struct SetHeapPages {
                    pub pages: set_heap_pages::Pages,
                }
                pub mod set_heap_pages {
                    use super::runtime_types;
                    pub type Pages = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the new runtime code."]
                pub struct SetCode {
                    pub code: set_code::Code,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                #[doc = "version!"]
                pub struct SetCodeWithoutChecks {
                    pub code: set_code_without_checks::Code,
                }
                pub mod set_code_without_checks {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set some items of storage."]
                pub struct SetStorage {
                    pub items: set_storage::Items,
                }
                pub mod set_storage {
                    use super::runtime_types;
                    pub type Items = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Kill some items from storage."]
                pub struct KillStorage {
                    pub keys: kill_storage::Keys,
                }
                pub mod kill_storage {
                    use super::runtime_types;
                    pub type Keys = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub struct KillPrefix {
                    pub prefix: kill_prefix::Prefix,
                    pub subkeys: kill_prefix::Subkeys,
                }
                pub mod kill_prefix {
                    use super::runtime_types;
                    pub type Prefix =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Subkeys = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Make some on-chain remark and emit event."]
                pub struct RemarkWithEvent {
                    pub remark: remark_with_event::Remark,
                }
                pub mod remark_with_event {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DoTask {
                    pub task: do_task::Task,
                }
                pub mod do_task {
                    use super::runtime_types;
                    pub type Task = runtime_types::torus_runtime::RuntimeTask;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DoTask {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "do_task";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub struct AuthorizeUpgrade {
                    pub code_hash: authorize_upgrade::CodeHash,
                }
                pub mod authorize_upgrade {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub struct AuthorizeUpgradeWithoutChecks {
                    pub code_hash: authorize_upgrade_without_checks::CodeHash,
                }
                pub mod authorize_upgrade_without_checks {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                #[doc = ""]
                #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                #[doc = "remains unchanged and that the spec version has increased."]
                #[doc = ""]
                #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                #[doc = ""]
                #[doc = "All origins are allowed."]
                pub struct ApplyAuthorizedUpgrade {
                    pub code: apply_authorized_upgrade::Code,
                }
                pub mod apply_authorized_upgrade {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "apply_authorized_upgrade";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "Can be executed by every `origin`."]
                pub fn remark(
                    &self,
                    remark: types::remark::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Remark>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: types::set_heap_pages::Pages,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetHeapPages>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                pub fn set_code(
                    &self,
                    code: types::set_code::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                #[doc = "version!"]
                pub fn set_code_without_checks(
                    &self,
                    code: types::set_code_without_checks::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCodeWithoutChecks>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: types::set_storage::Items,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: types::kill_storage::Keys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: types::kill_prefix::Prefix,
                    subkeys: types::kill_prefix::Subkeys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillPrefix>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: types::remark_with_event::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemarkWithEvent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
                pub fn do_task(
                    &self,
                    task: types::do_task::Task,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::DoTask>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "do_task",
                        types::DoTask { task },
                        [
                            199u8, 103u8, 231u8, 124u8, 182u8, 26u8, 42u8, 190u8, 140u8, 155u8,
                            96u8, 212u8, 167u8, 213u8, 125u8, 28u8, 209u8, 70u8, 241u8, 190u8,
                            219u8, 72u8, 151u8, 74u8, 16u8, 245u8, 77u8, 29u8, 3u8, 20u8, 65u8,
                            158u8,
                        ],
                    )
                }
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub fn authorize_upgrade(
                    &self,
                    code_hash: types::authorize_upgrade::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgrade>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade",
                        types::AuthorizeUpgrade { code_hash },
                        [
                            4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
                            254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
                            58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
                            172u8,
                        ],
                    )
                }
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub fn authorize_upgrade_without_checks(
                    &self,
                    code_hash: types::authorize_upgrade_without_checks::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::AuthorizeUpgradeWithoutChecks,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade_without_checks",
                        types::AuthorizeUpgradeWithoutChecks { code_hash },
                        [
                            126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
                            136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
                            197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
                            233u8,
                        ],
                    )
                }
                #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                #[doc = ""]
                #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                #[doc = "remains unchanged and that the spec version has increased."]
                #[doc = ""]
                #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                #[doc = ""]
                #[doc = "All origins are allowed."]
                pub fn apply_authorized_upgrade(
                    &self,
                    code: types::apply_authorized_upgrade::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ApplyAuthorizedUpgrade,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "apply_authorized_upgrade",
                        types::ApplyAuthorizedUpgrade { code },
                        [
                            232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8,
                            156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8,
                            32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::ext::subxt_core::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Hash = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [`Task`] has started executing"]
            pub struct TaskStarted {
                pub task: task_started::Task,
            }
            pub mod task_started {
                use super::runtime_types;
                pub type Task = runtime_types::torus_runtime::RuntimeTask;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TaskStarted {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "TaskStarted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [`Task`] has finished executing."]
            pub struct TaskCompleted {
                pub task: task_completed::Task,
            }
            pub mod task_completed {
                use super::runtime_types;
                pub type Task = runtime_types::torus_runtime::RuntimeTask;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TaskCompleted {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "TaskCompleted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [`Task`] failed during execution."]
            pub struct TaskFailed {
                pub task: task_failed::Task,
                pub err: task_failed::Err,
            }
            pub mod task_failed {
                use super::runtime_types;
                pub type Task = runtime_types::torus_runtime::RuntimeTask;
                pub type Err = runtime_types::sp_runtime::DispatchError;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TaskFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "TaskFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An upgrade was authorized."]
            pub struct UpgradeAuthorized {
                pub code_hash: upgrade_authorized::CodeHash,
                pub check_version: upgrade_authorized::CheckVersion,
            }
            pub mod upgrade_authorized {
                use super::runtime_types;
                pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                pub type CheckVersion = ::core::primitive::bool;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for UpgradeAuthorized {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod extrinsic_count {
                    use super::runtime_types;
                    pub type ExtrinsicCount = ::core::primitive::u32;
                }
                pub mod inherents_applied {
                    use super::runtime_types;
                    pub type InherentsApplied = ::core::primitive::bool;
                }
                pub mod block_weight {
                    use super::runtime_types;
                    pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >;
                }
                pub mod all_extrinsics_len {
                    use super::runtime_types;
                    pub type AllExtrinsicsLen = ::core::primitive::u32;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod extrinsic_data {
                    use super::runtime_types;
                    pub type ExtrinsicData =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod number {
                    use super::runtime_types;
                    pub type Number = ::core::primitive::u64;
                }
                pub mod parent_hash {
                    use super::runtime_types;
                    pub type ParentHash = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod digest {
                    use super::runtime_types;
                    pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
                }
                pub mod events {
                    use super::runtime_types;
                    pub type Events = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::torus_runtime::RuntimeEvent,
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    >;
                }
                pub mod event_count {
                    use super::runtime_types;
                    pub type EventCount = ::core::primitive::u32;
                }
                pub mod event_topics {
                    use super::runtime_types;
                    pub type EventTopics = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod last_runtime_upgrade {
                    use super::runtime_types;
                    pub type LastRuntimeUpgrade =
                        runtime_types::frame_system::LastRuntimeUpgradeInfo;
                }
                pub mod upgraded_to_u32_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToU32RefCount = ::core::primitive::bool;
                }
                pub mod upgraded_to_triple_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToTripleRefCount = ::core::primitive::bool;
                }
                pub mod execution_phase {
                    use super::runtime_types;
                    pub type ExecutionPhase = runtime_types::frame_system::Phase;
                }
                pub mod authorized_upgrade {
                    use super::runtime_types;
                    pub type AuthorizedUpgrade =
                        runtime_types::frame_system::CodeUpgradeAuthorization;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        (),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_count::ExtrinsicCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicCount",
                        (),
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                #[doc = " Whether all inherents have been applied."]
                pub fn inherents_applied(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inherents_applied::InherentsApplied,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "InherentsApplied",
                        (),
                        [
                            132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8,
                            223u8, 101u8, 55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8,
                            109u8, 34u8, 65u8, 185u8, 150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_weight::BlockWeight,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockWeight",
                        (),
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::all_extrinsics_len::AllExtrinsicsLen,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        (),
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_hash::BlockHash,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        (),
                        [
                            231u8, 203u8, 53u8, 62u8, 34u8, 38u8, 27u8, 62u8, 10u8, 209u8, 96u8,
                            2u8, 207u8, 136u8, 240u8, 67u8, 183u8, 74u8, 239u8, 218u8, 18u8, 200u8,
                            211u8, 134u8, 3u8, 164u8, 96u8, 74u8, 67u8, 204u8, 133u8, 127u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::block_hash::Param0,
                    >,
                    types::block_hash::BlockHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            231u8, 203u8, 53u8, 62u8, 34u8, 38u8, 27u8, 62u8, 10u8, 209u8, 96u8,
                            2u8, 207u8, 136u8, 240u8, 67u8, 183u8, 74u8, 239u8, 218u8, 18u8, 200u8,
                            211u8, 134u8, 3u8, 164u8, 96u8, 74u8, 67u8, 204u8, 133u8, 127u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_data::ExtrinsicData,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        (),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::extrinsic_data::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::extrinsic_data::Param0,
                    >,
                    types::extrinsic_data::ExtrinsicData,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::number::Number,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Number",
                        (),
                        [
                            2u8, 189u8, 173u8, 80u8, 15u8, 128u8, 71u8, 71u8, 14u8, 91u8, 177u8,
                            204u8, 54u8, 135u8, 13u8, 212u8, 9u8, 23u8, 219u8, 228u8, 150u8, 137u8,
                            199u8, 181u8, 58u8, 241u8, 247u8, 70u8, 19u8, 237u8, 23u8, 77u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::parent_hash::ParentHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ParentHash",
                        (),
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::digest::Digest,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Digest",
                        (),
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::events::Events,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Events",
                        (),
                        [
                            61u8, 77u8, 73u8, 123u8, 246u8, 49u8, 67u8, 225u8, 189u8, 2u8, 170u8,
                            186u8, 161u8, 109u8, 179u8, 245u8, 127u8, 221u8, 56u8, 63u8, 92u8,
                            202u8, 170u8, 154u8, 226u8, 158u8, 56u8, 23u8, 195u8, 225u8, 226u8,
                            214u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_count::EventCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventCount",
                        (),
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_topics::EventTopics,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        (),
                        [
                            190u8, 220u8, 184u8, 246u8, 192u8, 219u8, 183u8, 210u8, 216u8, 1u8,
                            239u8, 142u8, 255u8, 35u8, 134u8, 39u8, 114u8, 27u8, 34u8, 194u8, 90u8,
                            54u8, 113u8, 119u8, 85u8, 117u8, 23u8, 81u8, 186u8, 94u8, 34u8, 89u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::event_topics::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::event_topics::Param0,
                    >,
                    types::event_topics::EventTopics,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            190u8, 220u8, 184u8, 246u8, 192u8, 219u8, 183u8, 210u8, 216u8, 1u8,
                            239u8, 142u8, 255u8, 35u8, 134u8, 39u8, 114u8, 27u8, 34u8, 194u8, 90u8,
                            54u8, 113u8, 119u8, 85u8, 117u8, 23u8, 81u8, 186u8, 94u8, 34u8, 89u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::last_runtime_upgrade::LastRuntimeUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        (),
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        (),
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        (),
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::execution_phase::ExecutionPhase,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExecutionPhase",
                        (),
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
                #[doc = " `Some` if a code upgrade has been authorized."]
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorized_upgrade::AuthorizedUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AuthorizedUpgrade",
                        (),
                        [
                            165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
                            169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
                            214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockWeights,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockLength,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_weights::RuntimeDbWeight,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                #[doc = " Get the chain's in-code version."]
                pub fn version(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_version::RuntimeVersion,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "[`Config::MinimumPeriod`]."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _None_."]
                #[doc = ""]
                #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                #[doc = "block to execute any other calls."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub struct Set {
                    #[codec(compact)]
                    pub now: set::Now,
                }
                pub mod set {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "[`Config::MinimumPeriod`]."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _None_."]
                #[doc = ""]
                #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                #[doc = "block to execute any other calls."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub fn set(
                    &self,
                    now: types::set::Now,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Set>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod now {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                pub mod did_update {
                    use super::runtime_types;
                    pub type DidUpdate = ::core::primitive::bool;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::now::Now,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "Now",
                        (),
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                #[doc = " Whether the timestamp has been updated in this block."]
                #[doc = ""]
                #[doc = " This value is updated to `true` upon successful submission of a timestamp by a node."]
                #[doc = " It is then checked at the end of each block execution in the `on_finalize` hook."]
                pub fn did_update(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::did_update::DidUpdate,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "DidUpdate",
                        (),
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks."]
                #[doc = ""]
                #[doc = " Be aware that this is different to the *expected* period that the block production"]
                #[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
                #[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
                #[doc = " period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >;
                }
                pub mod current_slot {
                    use super::runtime_types;
                    pub type CurrentSlot = runtime_types::sp_consensus_slots::Slot;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorities::Authorities,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Aura",
                        "Authorities",
                        (),
                        [
                            95u8, 52u8, 203u8, 53u8, 254u8, 107u8, 134u8, 122u8, 95u8, 253u8, 51u8,
                            137u8, 142u8, 106u8, 237u8, 248u8, 159u8, 80u8, 41u8, 233u8, 137u8,
                            133u8, 13u8, 217u8, 176u8, 88u8, 132u8, 199u8, 241u8, 47u8, 125u8,
                            27u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_slot::CurrentSlot,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Aura",
                        "CurrentSlot",
                        (),
                        [
                            112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
                            236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
                            201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
                            43u8, 57u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The slot duration Aura should run with, expressed in milliseconds."]
                #[doc = " The effective value of this type should not change while the chain is running."]
                #[doc = ""]
                #[doc = " For backwards compatibility either use [`MinimumPeriodTimesTwo`] or a const."]
                pub fn slot_duration(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Aura",
                        "SlotDuration",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_grandpa::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_grandpa::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub struct ReportEquivocation {
                    pub equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                        report_equivocation::EquivocationProof,
                    >,
                    pub key_owner_proof: report_equivocation::KeyOwnerProof,
                }
                pub mod report_equivocation {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::ext::subxt_core::utils::H256,
                            ::core::primitive::u64,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ReportEquivocation {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub struct ReportEquivocationUnsigned {
                    pub equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                        report_equivocation_unsigned::EquivocationProof,
                    >,
                    pub key_owner_proof: report_equivocation_unsigned::KeyOwnerProof,
                }
                pub mod report_equivocation_unsigned {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::ext::subxt_core::utils::H256,
                            ::core::primitive::u64,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation_unsigned";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub struct NoteStalled {
                    pub delay: note_stalled::Delay,
                    pub best_finalized_block_number: note_stalled::BestFinalizedBlockNumber,
                }
                pub mod note_stalled {
                    use super::runtime_types;
                    pub type Delay = ::core::primitive::u64;
                    pub type BestFinalizedBlockNumber = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for NoteStalled {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "note_stalled";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: types::report_equivocation::EquivocationProof,
                    key_owner_proof: types::report_equivocation::KeyOwnerProof,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ReportEquivocation>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "report_equivocation",
                        types::ReportEquivocation {
                            equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            134u8, 142u8, 87u8, 229u8, 16u8, 200u8, 253u8, 196u8, 11u8, 170u8, 0u8,
                            151u8, 39u8, 200u8, 169u8, 14u8, 77u8, 63u8, 38u8, 180u8, 140u8, 113u8,
                            248u8, 220u8, 62u8, 243u8, 63u8, 98u8, 24u8, 123u8, 191u8, 41u8,
                        ],
                    )
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: types::report_equivocation_unsigned::EquivocationProof,
                    key_owner_proof: types::report_equivocation_unsigned::KeyOwnerProof,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ReportEquivocationUnsigned,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        types::ReportEquivocationUnsigned {
                            equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            249u8, 221u8, 85u8, 143u8, 2u8, 211u8, 205u8, 249u8, 24u8, 206u8,
                            251u8, 140u8, 49u8, 54u8, 30u8, 125u8, 108u8, 46u8, 173u8, 184u8, 65u8,
                            139u8, 139u8, 12u8, 20u8, 27u8, 149u8, 225u8, 113u8, 56u8, 249u8, 44u8,
                        ],
                    )
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: types::note_stalled::Delay,
                    best_finalized_block_number: types::note_stalled::BestFinalizedBlockNumber,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::NoteStalled>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "note_stalled",
                        types::NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            172u8, 89u8, 201u8, 164u8, 105u8, 69u8, 86u8, 125u8, 143u8, 174u8,
                            42u8, 253u8, 45u8, 160u8, 140u8, 155u8, 198u8, 91u8, 125u8, 108u8,
                            158u8, 47u8, 233u8, 185u8, 109u8, 227u8, 106u8, 207u8, 95u8, 189u8,
                            190u8, 53u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: new_authorities::AuthoritySet,
            }
            pub mod new_authorities {
                use super::runtime_types;
                pub type AuthoritySet = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::ext::subxt_core::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::ext::subxt_core::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod state {
                    use super::runtime_types;
                    pub type State =
                        runtime_types::pallet_grandpa::StoredState<::core::primitive::u64>;
                }
                pub mod pending_change {
                    use super::runtime_types;
                    pub type PendingChange =
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u64>;
                }
                pub mod next_forced {
                    use super::runtime_types;
                    pub type NextForced = ::core::primitive::u64;
                }
                pub mod stalled {
                    use super::runtime_types;
                    pub type Stalled = (::core::primitive::u64, ::core::primitive::u64);
                }
                pub mod current_set_id {
                    use super::runtime_types;
                    pub type CurrentSetId = ::core::primitive::u64;
                }
                pub mod set_id_session {
                    use super::runtime_types;
                    pub type SetIdSession = ::core::primitive::u32;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::state::State,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "State",
                        (),
                        [
                            52u8, 94u8, 52u8, 200u8, 52u8, 34u8, 254u8, 53u8, 83u8, 6u8, 129u8,
                            34u8, 8u8, 49u8, 75u8, 153u8, 118u8, 3u8, 28u8, 182u8, 64u8, 234u8,
                            152u8, 44u8, 147u8, 222u8, 17u8, 17u8, 61u8, 0u8, 186u8, 122u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pending_change::PendingChange,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "PendingChange",
                        (),
                        [
                            195u8, 146u8, 73u8, 229u8, 76u8, 128u8, 45u8, 145u8, 57u8, 243u8, 61u8,
                            227u8, 173u8, 96u8, 145u8, 126u8, 239u8, 128u8, 232u8, 110u8, 227u8,
                            92u8, 89u8, 20u8, 252u8, 235u8, 112u8, 37u8, 102u8, 145u8, 24u8, 126u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_forced::NextForced,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "NextForced",
                        (),
                        [
                            66u8, 193u8, 103u8, 170u8, 125u8, 104u8, 224u8, 91u8, 124u8, 113u8,
                            65u8, 233u8, 30u8, 79u8, 109u8, 123u8, 40u8, 7u8, 115u8, 162u8, 181u8,
                            225u8, 47u8, 48u8, 240u8, 29u8, 131u8, 206u8, 142u8, 22u8, 136u8,
                            231u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::stalled::Stalled,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "Stalled",
                        (),
                        [
                            194u8, 42u8, 49u8, 169u8, 34u8, 43u8, 158u8, 240u8, 232u8, 208u8, 15u8,
                            10u8, 135u8, 180u8, 99u8, 216u8, 83u8, 250u8, 0u8, 148u8, 173u8, 169u8,
                            105u8, 136u8, 3u8, 136u8, 125u8, 87u8, 49u8, 173u8, 223u8, 56u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_set_id::CurrentSetId,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "CurrentSetId",
                        (),
                        [
                            234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
                            207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
                            246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
                            115u8, 73u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::set_id_session::SetIdSession,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "SetIdSession",
                        (),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::set_id_session::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::set_id_session::Param0,
                    >,
                    types::set_id_session::SetIdSession,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "SetIdSession",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                #[doc = " The current list of authorities."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorities::Authorities,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "Authorities",
                        (),
                        [
                            192u8, 157u8, 98u8, 244u8, 104u8, 38u8, 195u8, 114u8, 183u8, 62u8,
                            247u8, 18u8, 31u8, 152u8, 246u8, 206u8, 97u8, 13u8, 118u8, 211u8,
                            104u8, 54u8, 150u8, 152u8, 126u8, 170u8, 228u8, 158u8, 108u8, 129u8,
                            134u8, 44u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of nominators for each validator."]
                pub fn max_nominators(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxNominators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of entries to keep in the set id to session index mapping."]
                #[doc = ""]
                #[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
                #[doc = " value should relate to the bonding duration of whatever staking system is"]
                #[doc = " being used (if any). If equivocation handling is not enabled then this value"]
                #[doc = " can be zero."]
                pub fn max_set_id_session_entries(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxSetIdSessionEntries",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub struct TransferAllowDeath {
                    pub dest: transfer_allow_death::Dest,
                    #[codec(compact)]
                    pub value: transfer_allow_death::Value,
                }
                pub mod transfer_allow_death {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub struct ForceTransfer {
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub value: force_transfer::Value,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Source = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub struct TransferKeepAlive {
                    pub dest: transfer_keep_alive::Dest,
                    #[codec(compact)]
                    pub value: transfer_keep_alive::Value,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub struct TransferAll {
                    pub dest: transfer_all::Dest,
                    pub keep_alive: transfer_all::KeepAlive,
                }
                pub mod transfer_all {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub struct ForceUnreserve {
                    pub who: force_unreserve::Who,
                    pub amount: force_unreserve::Amount,
                }
                pub mod force_unreserve {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibility of churn)."]
                pub struct UpgradeAccounts {
                    pub who: upgrade_accounts::Who,
                }
                pub mod upgrade_accounts {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub struct ForceSetBalance {
                    pub who: force_set_balance::Who,
                    #[codec(compact)]
                    pub new_free: force_set_balance::NewFree,
                }
                pub mod force_set_balance {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type NewFree = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Adjust the total issuance in a saturating way."]
                #[doc = ""]
                #[doc = "Can only be called by root and always needs a positive `delta`."]
                #[doc = ""]
                #[doc = "# Example"]
                pub struct ForceAdjustTotalIssuance {
                    pub direction: force_adjust_total_issuance::Direction,
                    #[codec(compact)]
                    pub delta: force_adjust_total_issuance::Delta,
                }
                pub mod force_adjust_total_issuance {
                    use super::runtime_types;
                    pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
                    pub type Delta = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_adjust_total_issuance";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Burn the specified liquid free balance from the origin account."]
                #[doc = ""]
                #[doc = "If the origin's account ends up below the existential deposit as a result"]
                #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                #[doc = ""]
                #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                pub struct Burn {
                    #[codec(compact)]
                    pub value: burn::Value,
                    pub keep_alive: burn::KeepAlive,
                }
                pub mod burn {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Burn {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "burn";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub fn transfer_allow_death(
                    &self,
                    dest: types::transfer_allow_death::Dest,
                    value: types::transfer_allow_death::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAllowDeath>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub fn force_transfer(
                    &self,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    value: types::force_transfer::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceTransfer>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: types::transfer_keep_alive::Dest,
                    value: types::transfer_keep_alive::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub fn transfer_all(
                    &self,
                    dest: types::transfer_all::Dest,
                    keep_alive: types::transfer_all::KeepAlive,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAll>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: types::force_unreserve::Who,
                    amount: types::force_unreserve::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceUnreserve>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibility of churn)."]
                pub fn upgrade_accounts(
                    &self,
                    who: types::upgrade_accounts::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpgradeAccounts>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn force_set_balance(
                    &self,
                    who: types::force_set_balance::Who,
                    new_free: types::force_set_balance::NewFree,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSetBalance>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
                #[doc = "Adjust the total issuance in a saturating way."]
                #[doc = ""]
                #[doc = "Can only be called by root and always needs a positive `delta`."]
                #[doc = ""]
                #[doc = "# Example"]
                pub fn force_adjust_total_issuance(
                    &self,
                    direction: types::force_adjust_total_issuance::Direction,
                    delta: types::force_adjust_total_issuance::Delta,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ForceAdjustTotalIssuance,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_adjust_total_issuance",
                        types::ForceAdjustTotalIssuance { direction, delta },
                        [
                            208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
                            190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8,
                            176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8, 150u8,
                            202u8,
                        ],
                    )
                }
                #[doc = "Burn the specified liquid free balance from the origin account."]
                #[doc = ""]
                #[doc = "If the origin's account ends up below the existential deposit as a result"]
                #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                #[doc = ""]
                #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                pub fn burn(
                    &self,
                    value: types::burn::Value,
                    keep_alive: types::burn::KeepAlive,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Burn>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "burn",
                        types::Burn { value, keep_alive },
                        [
                            176u8, 64u8, 7u8, 109u8, 16u8, 44u8, 145u8, 125u8, 147u8, 152u8, 130u8,
                            114u8, 221u8, 201u8, 150u8, 162u8, 118u8, 71u8, 52u8, 92u8, 240u8,
                            116u8, 203u8, 98u8, 5u8, 22u8, 43u8, 102u8, 94u8, 208u8, 101u8, 57u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Free = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The `TotalIssuance` was forcefully changed."]
            pub struct TotalIssuanceForced {
                pub old: total_issuance_forced::Old,
                pub new: total_issuance_forced::New,
            }
            pub mod total_issuance_forced {
                use super::runtime_types;
                pub type Old = ::core::primitive::u128;
                pub type New = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TotalIssuanceForced {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "TotalIssuanceForced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod total_issuance {
                    use super::runtime_types;
                    pub type TotalIssuance = ::core::primitive::u128;
                }
                pub mod inactive_issuance {
                    use super::runtime_types;
                    pub type InactiveIssuance = ::core::primitive::u128;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account =
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod locks {
                    use super::runtime_types;
                    pub type Locks =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod reserves {
                    use super::runtime_types;
                    pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            (),
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod holds {
                    use super::runtime_types;
                    pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            runtime_types::torus_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod freezes {
                    use super::runtime_types;
                    pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::total_issuance::TotalIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "TotalIssuance",
                        (),
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inactive_issuance::InactiveIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "InactiveIssuance",
                        (),
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        (),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn locks_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::locks::Locks,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        (),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn locks(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::locks::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::locks::Param0,
                    >,
                    types::locks::Locks,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reserves::Reserves,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        (),
                        [
                            144u8, 205u8, 22u8, 82u8, 201u8, 157u8, 72u8, 92u8, 42u8, 112u8, 242u8,
                            185u8, 235u8, 124u8, 50u8, 39u8, 108u8, 226u8, 79u8, 102u8, 116u8,
                            242u8, 243u8, 134u8, 189u8, 237u8, 249u8, 240u8, 147u8, 135u8, 37u8,
                            57u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn reserves(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::reserves::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::reserves::Param0,
                    >,
                    types::reserves::Reserves,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            144u8, 205u8, 22u8, 82u8, 201u8, 157u8, 72u8, 92u8, 42u8, 112u8, 242u8,
                            185u8, 235u8, 124u8, 50u8, 39u8, 108u8, 226u8, 79u8, 102u8, 116u8,
                            242u8, 243u8, 134u8, 189u8, 237u8, 249u8, 240u8, 147u8, 135u8, 37u8,
                            57u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::holds::Holds,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        (),
                        [
                            37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
                            117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
                            37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::holds::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::holds::Param0,
                    >,
                    types::holds::Holds,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
                            117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
                            37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::freezes::Freezes,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        (),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::freezes::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::freezes::Param0,
                    >,
                    types::freezes::Freezes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: transaction_fee_paid::Who,
                pub actual_fee: transaction_fee_paid::ActualFee,
                pub tip: transaction_fee_paid::Tip,
            }
            pub mod transaction_fee_paid {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type ActualFee = ::core::primitive::u128;
                pub type Tip = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod next_fee_multiplier {
                    use super::runtime_types;
                    pub type NextFeeMultiplier =
                        runtime_types::sp_arithmetic::fixed_point::FixedU128;
                }
                pub mod storage_version {
                    use super::runtime_types;
                    pub type StorageVersion = runtime_types::pallet_transaction_payment::Releases;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_fee_multiplier::NextFeeMultiplier,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        (),
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::storage_version::StorageVersion,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        (),
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee multiplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multiplied by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u8,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the Sudo pallet."]
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                pub struct Sudo {
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo::Call>,
                }
                pub mod sudo {
                    use super::runtime_types;
                    pub type Call = runtime_types::torus_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub struct SudoUncheckedWeight {
                    pub call:
                        ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_unchecked_weight::Call>,
                    pub weight: sudo_unchecked_weight::Weight,
                }
                pub mod sudo_unchecked_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::torus_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                pub struct SetKey {
                    pub new: set_key::New,
                }
                pub mod set_key {
                    use super::runtime_types;
                    pub type New = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub struct SudoAs {
                    pub who: sudo_as::Who,
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_as::Call>,
                }
                pub mod sudo_as {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Call = runtime_types::torus_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Permanently removes the sudo key."]
                #[doc = ""]
                #[doc = "**This cannot be un-done.**"]
                pub struct RemoveKey;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "remove_key";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                pub fn sudo(
                    &self,
                    call: types::sudo::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Sudo>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            252u8, 97u8, 212u8, 199u8, 247u8, 12u8, 182u8, 200u8, 233u8, 206u8,
                            129u8, 197u8, 18u8, 60u8, 208u8, 164u8, 86u8, 251u8, 95u8, 151u8,
                            235u8, 97u8, 115u8, 170u8, 232u8, 237u8, 10u8, 140u8, 108u8, 76u8,
                            249u8, 173u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: types::sudo_unchecked_weight::Call,
                    weight: types::sudo_unchecked_weight::Weight,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoUncheckedWeight>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                            weight,
                        },
                        [
                            225u8, 80u8, 10u8, 189u8, 41u8, 22u8, 195u8, 198u8, 183u8, 231u8,
                            119u8, 160u8, 11u8, 236u8, 28u8, 104u8, 254u8, 57u8, 239u8, 146u8,
                            239u8, 131u8, 122u8, 74u8, 159u8, 42u8, 20u8, 132u8, 121u8, 156u8,
                            68u8, 126u8,
                        ],
                    )
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                pub fn set_key(
                    &self,
                    new: types::set_key::New,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn sudo_as(
                    &self,
                    who: types::sudo_as::Who,
                    call: types::sudo_as::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoAs>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            109u8, 210u8, 247u8, 43u8, 238u8, 223u8, 203u8, 108u8, 5u8, 123u8,
                            226u8, 34u8, 105u8, 28u8, 188u8, 8u8, 188u8, 51u8, 66u8, 29u8, 113u8,
                            4u8, 12u8, 100u8, 40u8, 143u8, 240u8, 197u8, 241u8, 81u8, 89u8, 21u8,
                        ],
                    )
                }
                #[doc = "Permanently removes the sudo key."]
                #[doc = ""]
                #[doc = "**This cannot be un-done.**"]
                pub fn remove_key(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "remove_key",
                        types::RemoveKey {},
                        [
                            133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
                            25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8,
                            191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8, 211u8,
                            79u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A sudo call just took place."]
            pub struct Sudid {
                pub sudo_result: sudid::SudoResult,
            }
            pub mod sudid {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The sudo key has been updated."]
            pub struct KeyChanged {
                pub old: key_changed::Old,
                pub new: key_changed::New,
            }
            pub mod key_changed {
                use super::runtime_types;
                pub type Old = ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
                pub type New = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The key was permanently removed."]
            pub struct KeyRemoved;
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyRemoved {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyRemoved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
            pub struct SudoAsDone {
                pub sudo_result: sudo_as_done::SudoResult,
            }
            pub mod sudo_as_done {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod key {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::key::Key,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sudo",
                        "Key",
                        (),
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod multisig {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_multisig::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_multisig::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                #[doc = "multi-signature, but do not participate in the approval process."]
                #[doc = "- `call`: The call to be executed."]
                #[doc = ""]
                #[doc = "Result is equivalent to the dispatched result."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
                pub struct AsMultiThreshold1 {
                    pub other_signatories: as_multi_threshold1::OtherSignatories,
                    pub call:
                        ::subxt::ext::subxt_core::alloc::boxed::Box<as_multi_threshold1::Call>,
                }
                pub mod as_multi_threshold1 {
                    use super::runtime_types;
                    pub type OtherSignatories = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type Call = runtime_types::torus_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AsMultiThreshold1 {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "as_multi_threshold_1";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                #[doc = ""]
                #[doc = "If there are enough, then dispatch the call."]
                #[doc = ""]
                #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                #[doc = "is cancelled."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                #[doc = "transaction index) of the first approval transaction."]
                #[doc = "- `call`: The call to be executed."]
                #[doc = ""]
                #[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
                #[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
                #[doc = ""]
                #[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
                #[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
                #[doc = "may be found in the deposited `MultisigExecuted` event."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S + Z + Call)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                #[doc = "- One event."]
                #[doc = "- The weight of the `call`."]
                #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                pub struct AsMulti {
                    pub threshold: as_multi::Threshold,
                    pub other_signatories: as_multi::OtherSignatories,
                    pub maybe_timepoint: as_multi::MaybeTimepoint,
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<as_multi::Call>,
                    pub max_weight: as_multi::MaxWeight,
                }
                pub mod as_multi {
                    use super::runtime_types;
                    pub type Threshold = ::core::primitive::u16;
                    pub type OtherSignatories = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type MaybeTimepoint = ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                    >;
                    pub type Call = runtime_types::torus_runtime::RuntimeCall;
                    pub type MaxWeight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "as_multi";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                #[doc = ""]
                #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                #[doc = "is cancelled."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                #[doc = "transaction index) of the first approval transaction."]
                #[doc = "- `call_hash`: The hash of the call to be executed."]
                #[doc = ""]
                #[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                #[doc = "- One event."]
                #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                pub struct ApproveAsMulti {
                    pub threshold: approve_as_multi::Threshold,
                    pub other_signatories: approve_as_multi::OtherSignatories,
                    pub maybe_timepoint: approve_as_multi::MaybeTimepoint,
                    pub call_hash: approve_as_multi::CallHash,
                    pub max_weight: approve_as_multi::MaxWeight,
                }
                pub mod approve_as_multi {
                    use super::runtime_types;
                    pub type Threshold = ::core::primitive::u16;
                    pub type OtherSignatories = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type MaybeTimepoint = ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                    >;
                    pub type CallHash = [::core::primitive::u8; 32usize];
                    pub type MaxWeight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ApproveAsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "approve_as_multi";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
                #[doc = "for this operation will be unreserved on success."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
                #[doc = "transaction for this dispatch."]
                #[doc = "- `call_hash`: The hash of the call to be executed."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- One event."]
                #[doc = "- I/O: 1 read `O(S)`, one remove."]
                #[doc = "- Storage: removes one item."]
                pub struct CancelAsMulti {
                    pub threshold: cancel_as_multi::Threshold,
                    pub other_signatories: cancel_as_multi::OtherSignatories,
                    pub timepoint: cancel_as_multi::Timepoint,
                    pub call_hash: cancel_as_multi::CallHash,
                }
                pub mod cancel_as_multi {
                    use super::runtime_types;
                    pub type Threshold = ::core::primitive::u16;
                    pub type OtherSignatories = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type Timepoint =
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>;
                    pub type CallHash = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for CancelAsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "cancel_as_multi";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                #[doc = "multi-signature, but do not participate in the approval process."]
                #[doc = "- `call`: The call to be executed."]
                #[doc = ""]
                #[doc = "Result is equivalent to the dispatched result."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
                pub fn as_multi_threshold_1(
                    &self,
                    other_signatories: types::as_multi_threshold1::OtherSignatories,
                    call: types::as_multi_threshold1::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AsMultiThreshold1>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Multisig",
                        "as_multi_threshold_1",
                        types::AsMultiThreshold1 {
                            other_signatories,
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            48u8, 102u8, 95u8, 242u8, 54u8, 52u8, 61u8, 33u8, 74u8, 112u8, 157u8,
                            17u8, 3u8, 135u8, 122u8, 68u8, 201u8, 148u8, 145u8, 123u8, 186u8,
                            216u8, 111u8, 163u8, 133u8, 93u8, 55u8, 213u8, 83u8, 251u8, 184u8, 6u8,
                        ],
                    )
                }
                #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                #[doc = ""]
                #[doc = "If there are enough, then dispatch the call."]
                #[doc = ""]
                #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                #[doc = "is cancelled."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                #[doc = "transaction index) of the first approval transaction."]
                #[doc = "- `call`: The call to be executed."]
                #[doc = ""]
                #[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
                #[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
                #[doc = ""]
                #[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
                #[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
                #[doc = "may be found in the deposited `MultisigExecuted` event."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S + Z + Call)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                #[doc = "- One event."]
                #[doc = "- The weight of the `call`."]
                #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                pub fn as_multi(
                    &self,
                    threshold: types::as_multi::Threshold,
                    other_signatories: types::as_multi::OtherSignatories,
                    maybe_timepoint: types::as_multi::MaybeTimepoint,
                    call: types::as_multi::Call,
                    max_weight: types::as_multi::MaxWeight,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AsMulti>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Multisig",
                        "as_multi",
                        types::AsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                            max_weight,
                        },
                        [
                            222u8, 121u8, 58u8, 86u8, 248u8, 120u8, 50u8, 3u8, 177u8, 53u8, 20u8,
                            170u8, 146u8, 174u8, 173u8, 103u8, 63u8, 246u8, 76u8, 12u8, 32u8, 52u8,
                            211u8, 87u8, 236u8, 129u8, 28u8, 234u8, 14u8, 199u8, 122u8, 32u8,
                        ],
                    )
                }
                #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                #[doc = ""]
                #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                #[doc = "is cancelled."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                #[doc = "transaction index) of the first approval transaction."]
                #[doc = "- `call_hash`: The hash of the call to be executed."]
                #[doc = ""]
                #[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                #[doc = "- One event."]
                #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                pub fn approve_as_multi(
                    &self,
                    threshold: types::approve_as_multi::Threshold,
                    other_signatories: types::approve_as_multi::OtherSignatories,
                    maybe_timepoint: types::approve_as_multi::MaybeTimepoint,
                    call_hash: types::approve_as_multi::CallHash,
                    max_weight: types::approve_as_multi::MaxWeight,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ApproveAsMulti>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Multisig",
                        "approve_as_multi",
                        types::ApproveAsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call_hash,
                            max_weight,
                        },
                        [
                            54u8, 141u8, 48u8, 156u8, 12u8, 82u8, 142u8, 38u8, 79u8, 125u8, 32u8,
                            202u8, 3u8, 230u8, 157u8, 221u8, 206u8, 76u8, 163u8, 225u8, 18u8,
                            253u8, 165u8, 17u8, 21u8, 65u8, 103u8, 79u8, 236u8, 68u8, 10u8, 21u8,
                        ],
                    )
                }
                #[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
                #[doc = "for this operation will be unreserved on success."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
                #[doc = "transaction for this dispatch."]
                #[doc = "- `call_hash`: The hash of the call to be executed."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- One event."]
                #[doc = "- I/O: 1 read `O(S)`, one remove."]
                #[doc = "- Storage: removes one item."]
                pub fn cancel_as_multi(
                    &self,
                    threshold: types::cancel_as_multi::Threshold,
                    other_signatories: types::cancel_as_multi::OtherSignatories,
                    timepoint: types::cancel_as_multi::Timepoint,
                    call_hash: types::cancel_as_multi::CallHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::CancelAsMulti>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Multisig",
                        "cancel_as_multi",
                        types::CancelAsMulti {
                            threshold,
                            other_signatories,
                            timepoint,
                            call_hash,
                        },
                        [
                            118u8, 81u8, 25u8, 77u8, 172u8, 129u8, 41u8, 32u8, 104u8, 194u8, 106u8,
                            92u8, 195u8, 252u8, 140u8, 31u8, 177u8, 250u8, 247u8, 73u8, 206u8,
                            153u8, 131u8, 168u8, 96u8, 45u8, 216u8, 234u8, 173u8, 37u8, 226u8,
                            20u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A new multisig operation has begun."]
            pub struct NewMultisig {
                pub approving: new_multisig::Approving,
                pub multisig: new_multisig::Multisig,
                pub call_hash: new_multisig::CallHash,
            }
            pub mod new_multisig {
                use super::runtime_types;
                pub type Approving = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Multisig = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A multisig operation has been approved by someone."]
            pub struct MultisigApproval {
                pub approving: multisig_approval::Approving,
                pub timepoint: multisig_approval::Timepoint,
                pub multisig: multisig_approval::Multisig,
                pub call_hash: multisig_approval::CallHash,
            }
            pub mod multisig_approval {
                use super::runtime_types;
                pub type Approving = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Timepoint =
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>;
                pub type Multisig = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A multisig operation has been executed."]
            pub struct MultisigExecuted {
                pub approving: multisig_executed::Approving,
                pub timepoint: multisig_executed::Timepoint,
                pub multisig: multisig_executed::Multisig,
                pub call_hash: multisig_executed::CallHash,
                pub result: multisig_executed::Result,
            }
            pub mod multisig_executed {
                use super::runtime_types;
                pub type Approving = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Timepoint =
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>;
                pub type Multisig = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
                pub type Result =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A multisig operation has been cancelled."]
            pub struct MultisigCancelled {
                pub cancelling: multisig_cancelled::Cancelling,
                pub timepoint: multisig_cancelled::Timepoint,
                pub multisig: multisig_cancelled::Multisig,
                pub call_hash: multisig_cancelled::CallHash,
            }
            pub mod multisig_cancelled {
                use super::runtime_types;
                pub type Cancelling = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Timepoint =
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>;
                pub type Multisig = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod multisigs {
                    use super::runtime_types;
                    pub type Multisigs = runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u64,
                        ::core::primitive::u128,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param1 = [::core::primitive::u8; 32usize];
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The set of open multisig operations."]
                pub fn multisigs_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::multisigs::Multisigs,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Multisig",
                        "Multisigs",
                        (),
                        [
                            69u8, 190u8, 134u8, 80u8, 236u8, 248u8, 25u8, 153u8, 154u8, 71u8,
                            192u8, 101u8, 159u8, 179u8, 0u8, 228u8, 93u8, 125u8, 99u8, 229u8,
                            142u8, 117u8, 215u8, 149u8, 134u8, 13u8, 139u8, 251u8, 220u8, 251u8,
                            2u8, 255u8,
                        ],
                    )
                }
                #[doc = " The set of open multisig operations."]
                pub fn multisigs_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::multisigs::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::multisigs::Param0,
                    >,
                    types::multisigs::Multisigs,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Multisig",
                        "Multisigs",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            69u8, 190u8, 134u8, 80u8, 236u8, 248u8, 25u8, 153u8, 154u8, 71u8,
                            192u8, 101u8, 159u8, 179u8, 0u8, 228u8, 93u8, 125u8, 99u8, 229u8,
                            142u8, 117u8, 215u8, 149u8, 134u8, 13u8, 139u8, 251u8, 220u8, 251u8,
                            2u8, 255u8,
                        ],
                    )
                }
                #[doc = " The set of open multisig operations."]
                pub fn multisigs(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::multisigs::Param0>,
                    _1: impl ::core::borrow::Borrow<types::multisigs::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::multisigs::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::multisigs::Param1,
                        >,
                    ),
                    types::multisigs::Multisigs,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Multisig",
                        "Multisigs",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            69u8, 190u8, 134u8, 80u8, 236u8, 248u8, 25u8, 153u8, 154u8, 71u8,
                            192u8, 101u8, 159u8, 179u8, 0u8, 228u8, 93u8, 125u8, 99u8, 229u8,
                            142u8, 117u8, 215u8, 149u8, 134u8, 13u8, 139u8, 251u8, 220u8, 251u8,
                            2u8, 255u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The base amount of currency needed to reserve for creating a multisig execution or to"]
                #[doc = " store a dispatch call for later."]
                #[doc = ""]
                #[doc = " This is held for an additional storage item whose value size is"]
                #[doc = " `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is"]
                #[doc = " `32 + sizeof(AccountId)` bytes."]
                pub fn deposit_base(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Multisig",
                        "DepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of currency needed per unit threshold when creating a multisig execution."]
                #[doc = ""]
                #[doc = " This is held for adding 32 bytes more into a pre-existing storage value."]
                pub fn deposit_factor(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Multisig",
                        "DepositFactor",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum amount of signatories allowed in the multisig."]
                pub fn max_signatories(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Multisig",
                        "MaxSignatories",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod ethereum {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_ethereum::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_ethereum::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transact an Ethereum transaction."]
                pub struct Transact {
                    pub transaction: transact::Transaction,
                }
                pub mod transact {
                    use super::runtime_types;
                    pub type Transaction = runtime_types::ethereum::transaction::TransactionV2;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Transact {
                    const PALLET: &'static str = "Ethereum";
                    const CALL: &'static str = "transact";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transact an Ethereum transaction."]
                pub fn transact(
                    &self,
                    transaction: types::transact::Transaction,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Transact>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Ethereum",
                        "transact",
                        types::Transact { transaction },
                        [
                            124u8, 9u8, 75u8, 222u8, 225u8, 49u8, 255u8, 53u8, 207u8, 220u8, 198u8,
                            31u8, 26u8, 150u8, 238u8, 140u8, 230u8, 77u8, 248u8, 1u8, 97u8, 222u8,
                            9u8, 32u8, 217u8, 160u8, 195u8, 4u8, 69u8, 210u8, 251u8, 109u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_ethereum::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An ethereum transaction was successfully executed."]
            pub struct Executed {
                pub from: executed::From,
                pub to: executed::To,
                pub transaction_hash: executed::TransactionHash,
                pub exit_reason: executed::ExitReason,
                pub extra_data: executed::ExtraData,
            }
            pub mod executed {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::H160;
                pub type To = ::subxt::ext::subxt_core::utils::H160;
                pub type TransactionHash = ::subxt::ext::subxt_core::utils::H256;
                pub type ExitReason = runtime_types::evm_core::error::ExitReason;
                pub type ExtraData =
                    ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Executed {
                const PALLET: &'static str = "Ethereum";
                const EVENT: &'static str = "Executed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod pending {
                    use super::runtime_types;
                    pub type Pending = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        runtime_types::ethereum::transaction::TransactionV2,
                        runtime_types::fp_rpc::TransactionStatus,
                        runtime_types::ethereum::receipt::ReceiptV3,
                    )>;
                }
                pub mod current_block {
                    use super::runtime_types;
                    pub type CurrentBlock = runtime_types::ethereum::block::Block<
                        runtime_types::ethereum::transaction::TransactionV2,
                    >;
                }
                pub mod current_receipts {
                    use super::runtime_types;
                    pub type CurrentReceipts = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::ethereum::receipt::ReceiptV3,
                    >;
                }
                pub mod current_transaction_statuses {
                    use super::runtime_types;
                    pub type CurrentTransactionStatuses = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::fp_rpc::TransactionStatus,
                    >;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param0 = runtime_types::primitive_types::U256;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current building block's transactions and receipts."]
                pub fn pending(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pending::Pending,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Ethereum",
                        "Pending",
                        (),
                        [
                            249u8, 60u8, 121u8, 166u8, 91u8, 128u8, 146u8, 87u8, 240u8, 165u8,
                            236u8, 61u8, 65u8, 140u8, 14u8, 203u8, 169u8, 102u8, 126u8, 247u8,
                            245u8, 3u8, 166u8, 188u8, 144u8, 74u8, 13u8, 2u8, 244u8, 49u8, 223u8,
                            198u8,
                        ],
                    )
                }
                #[doc = " The current Ethereum block."]
                pub fn current_block(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_block::CurrentBlock,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Ethereum",
                        "CurrentBlock",
                        (),
                        [
                            54u8, 128u8, 41u8, 16u8, 65u8, 25u8, 184u8, 85u8, 192u8, 220u8, 208u8,
                            92u8, 166u8, 132u8, 223u8, 50u8, 252u8, 112u8, 236u8, 217u8, 108u8,
                            166u8, 131u8, 224u8, 141u8, 59u8, 248u8, 42u8, 197u8, 96u8, 240u8,
                            88u8,
                        ],
                    )
                }
                #[doc = " The current Ethereum receipts."]
                pub fn current_receipts(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_receipts::CurrentReceipts,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Ethereum",
                        "CurrentReceipts",
                        (),
                        [
                            97u8, 46u8, 228u8, 135u8, 133u8, 148u8, 98u8, 3u8, 128u8, 26u8, 83u8,
                            12u8, 33u8, 135u8, 88u8, 205u8, 147u8, 176u8, 13u8, 113u8, 148u8, 48u8,
                            31u8, 200u8, 105u8, 224u8, 201u8, 225u8, 157u8, 108u8, 55u8, 209u8,
                        ],
                    )
                }
                #[doc = " The current transaction statuses."]
                pub fn current_transaction_statuses(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_transaction_statuses::CurrentTransactionStatuses,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Ethereum",
                        "CurrentTransactionStatuses",
                        (),
                        [
                            29u8, 20u8, 106u8, 243u8, 226u8, 102u8, 121u8, 20u8, 222u8, 53u8, 99u8,
                            68u8, 173u8, 238u8, 167u8, 165u8, 192u8, 192u8, 230u8, 46u8, 231u8,
                            88u8, 144u8, 159u8, 3u8, 171u8, 72u8, 125u8, 68u8, 66u8, 125u8, 165u8,
                        ],
                    )
                }
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_hash::BlockHash,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Ethereum",
                        "BlockHash",
                        (),
                        [
                            131u8, 87u8, 201u8, 82u8, 203u8, 241u8, 176u8, 149u8, 39u8, 243u8,
                            227u8, 1u8, 86u8, 62u8, 6u8, 231u8, 55u8, 6u8, 212u8, 96u8, 207u8,
                            73u8, 56u8, 204u8, 215u8, 227u8, 48u8, 249u8, 67u8, 137u8, 139u8, 76u8,
                        ],
                    )
                }
                pub fn block_hash(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::block_hash::Param0,
                    >,
                    types::block_hash::BlockHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Ethereum",
                        "BlockHash",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            131u8, 87u8, 201u8, 82u8, 203u8, 241u8, 176u8, 149u8, 39u8, 243u8,
                            227u8, 1u8, 86u8, 62u8, 6u8, 231u8, 55u8, 6u8, 212u8, 96u8, 207u8,
                            73u8, 56u8, 204u8, 215u8, 227u8, 48u8, 249u8, 67u8, 137u8, 139u8, 76u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod evm {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_evm::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_evm::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Withdraw balance from EVM into currency/balances pallet."]
                pub struct Withdraw {
                    pub address: withdraw::Address,
                    pub value: withdraw::Value,
                }
                pub mod withdraw {
                    use super::runtime_types;
                    pub type Address = ::subxt::ext::subxt_core::utils::H160;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Withdraw {
                    const PALLET: &'static str = "EVM";
                    const CALL: &'static str = "withdraw";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Issue an EVM call operation. This is similar to a message call transaction in Ethereum."]
                pub struct Call {
                    pub source: call::Source,
                    pub target: call::Target,
                    pub input: call::Input,
                    pub value: call::Value,
                    pub gas_limit: call::GasLimit,
                    pub max_fee_per_gas: call::MaxFeePerGas,
                    pub max_priority_fee_per_gas: call::MaxPriorityFeePerGas,
                    pub nonce: call::Nonce,
                    pub access_list: call::AccessList,
                }
                pub mod call {
                    use super::runtime_types;
                    pub type Source = ::subxt::ext::subxt_core::utils::H160;
                    pub type Target = ::subxt::ext::subxt_core::utils::H160;
                    pub type Input =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Value = runtime_types::primitive_types::U256;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type MaxFeePerGas = runtime_types::primitive_types::U256;
                    pub type MaxPriorityFeePerGas =
                        ::core::option::Option<runtime_types::primitive_types::U256>;
                    pub type Nonce = ::core::option::Option<runtime_types::primitive_types::U256>;
                    pub type AccessList = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::utils::H160,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Call {
                    const PALLET: &'static str = "EVM";
                    const CALL: &'static str = "call";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Issue an EVM create operation. This is similar to a contract creation transaction in"]
                #[doc = "Ethereum."]
                pub struct Create {
                    pub source: create::Source,
                    pub init: create::Init,
                    pub value: create::Value,
                    pub gas_limit: create::GasLimit,
                    pub max_fee_per_gas: create::MaxFeePerGas,
                    pub max_priority_fee_per_gas: create::MaxPriorityFeePerGas,
                    pub nonce: create::Nonce,
                    pub access_list: create::AccessList,
                }
                pub mod create {
                    use super::runtime_types;
                    pub type Source = ::subxt::ext::subxt_core::utils::H160;
                    pub type Init =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Value = runtime_types::primitive_types::U256;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type MaxFeePerGas = runtime_types::primitive_types::U256;
                    pub type MaxPriorityFeePerGas =
                        ::core::option::Option<runtime_types::primitive_types::U256>;
                    pub type Nonce = ::core::option::Option<runtime_types::primitive_types::U256>;
                    pub type AccessList = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::utils::H160,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Create {
                    const PALLET: &'static str = "EVM";
                    const CALL: &'static str = "create";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Issue an EVM create2 operation."]
                pub struct Create2 {
                    pub source: create2::Source,
                    pub init: create2::Init,
                    pub salt: create2::Salt,
                    pub value: create2::Value,
                    pub gas_limit: create2::GasLimit,
                    pub max_fee_per_gas: create2::MaxFeePerGas,
                    pub max_priority_fee_per_gas: create2::MaxPriorityFeePerGas,
                    pub nonce: create2::Nonce,
                    pub access_list: create2::AccessList,
                }
                pub mod create2 {
                    use super::runtime_types;
                    pub type Source = ::subxt::ext::subxt_core::utils::H160;
                    pub type Init =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Salt = ::subxt::ext::subxt_core::utils::H256;
                    pub type Value = runtime_types::primitive_types::U256;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type MaxFeePerGas = runtime_types::primitive_types::U256;
                    pub type MaxPriorityFeePerGas =
                        ::core::option::Option<runtime_types::primitive_types::U256>;
                    pub type Nonce = ::core::option::Option<runtime_types::primitive_types::U256>;
                    pub type AccessList = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::utils::H160,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Create2 {
                    const PALLET: &'static str = "EVM";
                    const CALL: &'static str = "create2";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Withdraw balance from EVM into currency/balances pallet."]
                pub fn withdraw(
                    &self,
                    address: types::withdraw::Address,
                    value: types::withdraw::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Withdraw>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "EVM",
                        "withdraw",
                        types::Withdraw { address, value },
                        [
                            62u8, 162u8, 234u8, 15u8, 176u8, 61u8, 183u8, 203u8, 241u8, 10u8,
                            202u8, 26u8, 45u8, 116u8, 38u8, 44u8, 32u8, 57u8, 208u8, 55u8, 182u8,
                            92u8, 136u8, 133u8, 216u8, 255u8, 25u8, 132u8, 242u8, 34u8, 43u8, 64u8,
                        ],
                    )
                }
                #[doc = "Issue an EVM call operation. This is similar to a message call transaction in Ethereum."]
                pub fn call(
                    &self,
                    source: types::call::Source,
                    target: types::call::Target,
                    input: types::call::Input,
                    value: types::call::Value,
                    gas_limit: types::call::GasLimit,
                    max_fee_per_gas: types::call::MaxFeePerGas,
                    max_priority_fee_per_gas: types::call::MaxPriorityFeePerGas,
                    nonce: types::call::Nonce,
                    access_list: types::call::AccessList,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Call>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "EVM",
                        "call",
                        types::Call {
                            source,
                            target,
                            input,
                            value,
                            gas_limit,
                            max_fee_per_gas,
                            max_priority_fee_per_gas,
                            nonce,
                            access_list,
                        },
                        [
                            121u8, 179u8, 103u8, 152u8, 89u8, 27u8, 36u8, 13u8, 114u8, 246u8,
                            222u8, 197u8, 249u8, 250u8, 241u8, 66u8, 219u8, 123u8, 126u8, 144u8,
                            144u8, 213u8, 165u8, 25u8, 248u8, 129u8, 86u8, 34u8, 105u8, 145u8,
                            85u8, 85u8,
                        ],
                    )
                }
                #[doc = "Issue an EVM create operation. This is similar to a contract creation transaction in"]
                #[doc = "Ethereum."]
                pub fn create(
                    &self,
                    source: types::create::Source,
                    init: types::create::Init,
                    value: types::create::Value,
                    gas_limit: types::create::GasLimit,
                    max_fee_per_gas: types::create::MaxFeePerGas,
                    max_priority_fee_per_gas: types::create::MaxPriorityFeePerGas,
                    nonce: types::create::Nonce,
                    access_list: types::create::AccessList,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Create>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "EVM",
                        "create",
                        types::Create {
                            source,
                            init,
                            value,
                            gas_limit,
                            max_fee_per_gas,
                            max_priority_fee_per_gas,
                            nonce,
                            access_list,
                        },
                        [
                            231u8, 52u8, 103u8, 5u8, 29u8, 96u8, 200u8, 245u8, 151u8, 231u8, 111u8,
                            150u8, 185u8, 126u8, 12u8, 42u8, 169u8, 92u8, 68u8, 130u8, 36u8, 11u8,
                            234u8, 211u8, 199u8, 200u8, 45u8, 10u8, 53u8, 91u8, 226u8, 145u8,
                        ],
                    )
                }
                #[doc = "Issue an EVM create2 operation."]
                pub fn create2(
                    &self,
                    source: types::create2::Source,
                    init: types::create2::Init,
                    salt: types::create2::Salt,
                    value: types::create2::Value,
                    gas_limit: types::create2::GasLimit,
                    max_fee_per_gas: types::create2::MaxFeePerGas,
                    max_priority_fee_per_gas: types::create2::MaxPriorityFeePerGas,
                    nonce: types::create2::Nonce,
                    access_list: types::create2::AccessList,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Create2>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "EVM",
                        "create2",
                        types::Create2 {
                            source,
                            init,
                            salt,
                            value,
                            gas_limit,
                            max_fee_per_gas,
                            max_priority_fee_per_gas,
                            nonce,
                            access_list,
                        },
                        [
                            73u8, 157u8, 32u8, 232u8, 164u8, 93u8, 191u8, 129u8, 171u8, 104u8,
                            212u8, 108u8, 167u8, 5u8, 61u8, 171u8, 247u8, 97u8, 122u8, 162u8,
                            102u8, 152u8, 224u8, 130u8, 94u8, 112u8, 115u8, 68u8, 249u8, 215u8,
                            233u8, 115u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_evm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Ethereum events from contracts."]
            pub struct Log {
                pub log: log::Log,
            }
            pub mod log {
                use super::runtime_types;
                pub type Log = runtime_types::ethereum::log::Log;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Log {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Log";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A contract has been created at given address."]
            pub struct Created {
                pub address: created::Address,
            }
            pub mod created {
                use super::runtime_types;
                pub type Address = ::subxt::ext::subxt_core::utils::H160;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Created {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A contract was attempted to be created, but the execution failed."]
            pub struct CreatedFailed {
                pub address: created_failed::Address,
            }
            pub mod created_failed {
                use super::runtime_types;
                pub type Address = ::subxt::ext::subxt_core::utils::H160;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for CreatedFailed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "CreatedFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A contract has been executed successfully with states applied."]
            pub struct Executed {
                pub address: executed::Address,
            }
            pub mod executed {
                use super::runtime_types;
                pub type Address = ::subxt::ext::subxt_core::utils::H160;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Executed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Executed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A contract has been executed with errors. States are reverted with only gas fees applied."]
            pub struct ExecutedFailed {
                pub address: executed_failed::Address,
            }
            pub mod executed_failed {
                use super::runtime_types;
                pub type Address = ::subxt::ext::subxt_core::utils::H160;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExecutedFailed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "ExecutedFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account_codes {
                    use super::runtime_types;
                    pub type AccountCodes =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H160;
                }
                pub mod account_codes_metadata {
                    use super::runtime_types;
                    pub type AccountCodesMetadata = runtime_types::pallet_evm::CodeMetadata;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H160;
                }
                pub mod account_storages {
                    use super::runtime_types;
                    pub type AccountStorages = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H160;
                    pub type Param1 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod suicided {
                    use super::runtime_types;
                    pub type Suicided = ();
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H160;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn account_codes_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account_codes::AccountCodes,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "AccountCodes",
                        (),
                        [
                            49u8, 73u8, 188u8, 164u8, 3u8, 40u8, 187u8, 216u8, 70u8, 119u8, 176u8,
                            187u8, 76u8, 24u8, 49u8, 174u8, 54u8, 98u8, 208u8, 255u8, 38u8, 214u8,
                            120u8, 116u8, 130u8, 139u8, 44u8, 102u8, 115u8, 222u8, 63u8, 56u8,
                        ],
                    )
                }
                pub fn account_codes(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account_codes::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account_codes::Param0,
                    >,
                    types::account_codes::AccountCodes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "AccountCodes",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            49u8, 73u8, 188u8, 164u8, 3u8, 40u8, 187u8, 216u8, 70u8, 119u8, 176u8,
                            187u8, 76u8, 24u8, 49u8, 174u8, 54u8, 98u8, 208u8, 255u8, 38u8, 214u8,
                            120u8, 116u8, 130u8, 139u8, 44u8, 102u8, 115u8, 222u8, 63u8, 56u8,
                        ],
                    )
                }
                pub fn account_codes_metadata_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account_codes_metadata::AccountCodesMetadata,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "AccountCodesMetadata",
                        (),
                        [
                            17u8, 83u8, 22u8, 15u8, 158u8, 242u8, 39u8, 174u8, 61u8, 230u8, 0u8,
                            161u8, 173u8, 242u8, 155u8, 156u8, 149u8, 108u8, 47u8, 129u8, 190u8,
                            223u8, 25u8, 235u8, 168u8, 86u8, 49u8, 118u8, 132u8, 93u8, 100u8,
                            173u8,
                        ],
                    )
                }
                pub fn account_codes_metadata(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account_codes_metadata::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account_codes_metadata::Param0,
                    >,
                    types::account_codes_metadata::AccountCodesMetadata,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "AccountCodesMetadata",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            17u8, 83u8, 22u8, 15u8, 158u8, 242u8, 39u8, 174u8, 61u8, 230u8, 0u8,
                            161u8, 173u8, 242u8, 155u8, 156u8, 149u8, 108u8, 47u8, 129u8, 190u8,
                            223u8, 25u8, 235u8, 168u8, 86u8, 49u8, 118u8, 132u8, 93u8, 100u8,
                            173u8,
                        ],
                    )
                }
                pub fn account_storages_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account_storages::AccountStorages,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "AccountStorages",
                        (),
                        [
                            63u8, 69u8, 109u8, 3u8, 190u8, 233u8, 39u8, 122u8, 94u8, 37u8, 74u8,
                            90u8, 197u8, 191u8, 12u8, 119u8, 165u8, 61u8, 217u8, 15u8, 36u8, 167u8,
                            211u8, 120u8, 169u8, 97u8, 13u8, 38u8, 148u8, 224u8, 167u8, 199u8,
                        ],
                    )
                }
                pub fn account_storages_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account_storages::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account_storages::Param0,
                    >,
                    types::account_storages::AccountStorages,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "AccountStorages",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            63u8, 69u8, 109u8, 3u8, 190u8, 233u8, 39u8, 122u8, 94u8, 37u8, 74u8,
                            90u8, 197u8, 191u8, 12u8, 119u8, 165u8, 61u8, 217u8, 15u8, 36u8, 167u8,
                            211u8, 120u8, 169u8, 97u8, 13u8, 38u8, 148u8, 224u8, 167u8, 199u8,
                        ],
                    )
                }
                pub fn account_storages(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account_storages::Param0>,
                    _1: impl ::core::borrow::Borrow<types::account_storages::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::account_storages::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::account_storages::Param1,
                        >,
                    ),
                    types::account_storages::AccountStorages,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "AccountStorages",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            63u8, 69u8, 109u8, 3u8, 190u8, 233u8, 39u8, 122u8, 94u8, 37u8, 74u8,
                            90u8, 197u8, 191u8, 12u8, 119u8, 165u8, 61u8, 217u8, 15u8, 36u8, 167u8,
                            211u8, 120u8, 169u8, 97u8, 13u8, 38u8, 148u8, 224u8, 167u8, 199u8,
                        ],
                    )
                }
                pub fn suicided_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::suicided::Suicided,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "Suicided",
                        (),
                        [
                            5u8, 137u8, 180u8, 131u8, 216u8, 217u8, 148u8, 127u8, 9u8, 159u8, 14u8,
                            25u8, 56u8, 99u8, 55u8, 151u8, 140u8, 143u8, 188u8, 172u8, 33u8, 91u8,
                            42u8, 59u8, 104u8, 94u8, 215u8, 41u8, 224u8, 118u8, 190u8, 249u8,
                        ],
                    )
                }
                pub fn suicided(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::suicided::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::suicided::Param0,
                    >,
                    types::suicided::Suicided,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "EVM",
                        "Suicided",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            5u8, 137u8, 180u8, 131u8, 216u8, 217u8, 148u8, 127u8, 9u8, 159u8, 14u8,
                            25u8, 56u8, 99u8, 55u8, 151u8, 140u8, 143u8, 188u8, 172u8, 33u8, 91u8,
                            42u8, 59u8, 104u8, 94u8, 215u8, 41u8, 224u8, 118u8, 190u8, 249u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod governance {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_governance::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_governance::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Adds a new allocator to the list. Only available for the root key."]
                pub struct AddAllocator {
                    pub key: add_allocator::Key,
                }
                pub mod add_allocator {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddAllocator {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "add_allocator";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Removes an existing allocator from the list. Only available for the"]
                #[doc = "root key."]
                pub struct RemoveAllocator {
                    pub key: remove_allocator::Key,
                }
                pub mod remove_allocator {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveAllocator {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "remove_allocator";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Forcefully adds a new agent to the whitelist. Only available for the"]
                #[doc = "root key or curators."]
                pub struct AddToWhitelist {
                    pub key: add_to_whitelist::Key,
                }
                pub mod add_to_whitelist {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddToWhitelist {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "add_to_whitelist";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Forcefully removes an agent from the whitelist. Only available for"]
                #[doc = "the root key or curators."]
                pub struct RemoveFromWhitelist {
                    pub key: remove_from_whitelist::Key,
                }
                pub mod remove_from_whitelist {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveFromWhitelist {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "remove_from_whitelist";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Accepts an agent application. Only available for the root key or"]
                #[doc = "curators."]
                pub struct AcceptApplication {
                    pub application_id: accept_application::ApplicationId,
                }
                pub mod accept_application {
                    use super::runtime_types;
                    pub type ApplicationId = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AcceptApplication {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "accept_application";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Denies an agent application. Only available for the root key or"]
                #[doc = "curators."]
                pub struct DenyApplication {
                    pub application_id: deny_application::ApplicationId,
                }
                pub mod deny_application {
                    use super::runtime_types;
                    pub type ApplicationId = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DenyApplication {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "deny_application";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Sets a penalty factor to the given agent emissions. Only available"]
                #[doc = "for the root key or curators."]
                pub struct PenalizeAgent {
                    pub agent_key: penalize_agent::AgentKey,
                    pub percentage: penalize_agent::Percentage,
                }
                pub mod penalize_agent {
                    use super::runtime_types;
                    pub type AgentKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Percentage = ::core::primitive::u8;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for PenalizeAgent {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "penalize_agent";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Submits a new agent application on behalf of a given key."]
                pub struct SubmitApplication {
                    pub agent_key: submit_application::AgentKey,
                    pub metadata: submit_application::Metadata,
                    pub removing: submit_application::Removing,
                }
                pub mod submit_application {
                    use super::runtime_types;
                    pub type AgentKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Metadata =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Removing = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SubmitApplication {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "submit_application";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Creates a new global parameters proposal."]
                pub struct AddGlobalParamsProposal {
                    pub data: add_global_params_proposal::Data,
                    pub metadata: add_global_params_proposal::Metadata,
                }
                pub mod add_global_params_proposal {
                    use super::runtime_types;
                    pub type Data = runtime_types::pallet_governance::proposal::GlobalParamsData;
                    pub type Metadata =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddGlobalParamsProposal {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "add_global_params_proposal";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Creates a new custom global proposal."]
                pub struct AddGlobalCustomProposal {
                    pub metadata: add_global_custom_proposal::Metadata,
                }
                pub mod add_global_custom_proposal {
                    use super::runtime_types;
                    pub type Metadata =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddGlobalCustomProposal {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "add_global_custom_proposal";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Creates a proposal moving funds from the treasury account to the"]
                #[doc = "given key."]
                pub struct AddDaoTreasuryTransferProposal {
                    pub value: add_dao_treasury_transfer_proposal::Value,
                    pub destination_key: add_dao_treasury_transfer_proposal::DestinationKey,
                    pub data: add_dao_treasury_transfer_proposal::Data,
                }
                pub mod add_dao_treasury_transfer_proposal {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type DestinationKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddDaoTreasuryTransferProposal {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "add_dao_treasury_transfer_proposal";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Casts a vote for an open proposal."]
                pub struct VoteProposal {
                    pub proposal_id: vote_proposal::ProposalId,
                    pub agree: vote_proposal::Agree,
                }
                pub mod vote_proposal {
                    use super::runtime_types;
                    pub type ProposalId = ::core::primitive::u64;
                    pub type Agree = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for VoteProposal {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "vote_proposal";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Removes a casted vote for an open proposal."]
                pub struct RemoveVoteProposal {
                    pub proposal_id: remove_vote_proposal::ProposalId,
                }
                pub mod remove_vote_proposal {
                    use super::runtime_types;
                    pub type ProposalId = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveVoteProposal {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "remove_vote_proposal";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Enables vote power delegation."]
                pub struct EnableVoteDelegation;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for EnableVoteDelegation {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "enable_vote_delegation";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Disables vote power delegation."]
                pub struct DisableVoteDelegation;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DisableVoteDelegation {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "disable_vote_delegation";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Creates a new emission percentage proposal."]
                pub struct AddEmissionProposal {
                    pub recycling_percentage: add_emission_proposal::RecyclingPercentage,
                    pub treasury_percentage: add_emission_proposal::TreasuryPercentage,
                    pub incentives_ratio: add_emission_proposal::IncentivesRatio,
                    pub data: add_emission_proposal::Data,
                }
                pub mod add_emission_proposal {
                    use super::runtime_types;
                    pub type RecyclingPercentage =
                        runtime_types::sp_arithmetic::per_things::Percent;
                    pub type TreasuryPercentage = runtime_types::sp_arithmetic::per_things::Percent;
                    pub type IncentivesRatio = runtime_types::sp_arithmetic::per_things::Percent;
                    pub type Data =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddEmissionProposal {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "add_emission_proposal";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Forcefully sets emission percentages. Only available for the root"]
                #[doc = "key."]
                pub struct SetEmissionParams {
                    pub recycling_percentage: set_emission_params::RecyclingPercentage,
                    pub treasury_percentage: set_emission_params::TreasuryPercentage,
                }
                pub mod set_emission_params {
                    use super::runtime_types;
                    pub type RecyclingPercentage =
                        runtime_types::sp_arithmetic::per_things::Percent;
                    pub type TreasuryPercentage = runtime_types::sp_arithmetic::per_things::Percent;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetEmissionParams {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "set_emission_params";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Adds a new allocator to the list. Only available for the root key."]
                pub fn add_allocator(
                    &self,
                    key: types::add_allocator::Key,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AddAllocator>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "add_allocator",
                        types::AddAllocator { key },
                        [
                            185u8, 239u8, 249u8, 115u8, 42u8, 214u8, 202u8, 58u8, 208u8, 95u8,
                            70u8, 28u8, 30u8, 68u8, 56u8, 74u8, 95u8, 98u8, 209u8, 15u8, 119u8,
                            127u8, 0u8, 147u8, 103u8, 81u8, 0u8, 42u8, 78u8, 115u8, 111u8, 104u8,
                        ],
                    )
                }
                #[doc = "Removes an existing allocator from the list. Only available for the"]
                #[doc = "root key."]
                pub fn remove_allocator(
                    &self,
                    key: types::remove_allocator::Key,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveAllocator>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "remove_allocator",
                        types::RemoveAllocator { key },
                        [
                            138u8, 200u8, 103u8, 25u8, 90u8, 93u8, 237u8, 66u8, 152u8, 244u8, 27u8,
                            131u8, 221u8, 17u8, 238u8, 55u8, 174u8, 213u8, 225u8, 255u8, 172u8,
                            239u8, 80u8, 172u8, 103u8, 35u8, 61u8, 28u8, 142u8, 122u8, 244u8, 69u8,
                        ],
                    )
                }
                #[doc = "Forcefully adds a new agent to the whitelist. Only available for the"]
                #[doc = "root key or curators."]
                pub fn add_to_whitelist(
                    &self,
                    key: types::add_to_whitelist::Key,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AddToWhitelist>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "add_to_whitelist",
                        types::AddToWhitelist { key },
                        [
                            145u8, 160u8, 68u8, 51u8, 52u8, 199u8, 121u8, 159u8, 77u8, 179u8, 26u8,
                            223u8, 120u8, 81u8, 23u8, 170u8, 208u8, 151u8, 74u8, 226u8, 160u8,
                            66u8, 62u8, 251u8, 180u8, 0u8, 198u8, 178u8, 77u8, 246u8, 180u8, 158u8,
                        ],
                    )
                }
                #[doc = "Forcefully removes an agent from the whitelist. Only available for"]
                #[doc = "the root key or curators."]
                pub fn remove_from_whitelist(
                    &self,
                    key: types::remove_from_whitelist::Key,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveFromWhitelist>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "remove_from_whitelist",
                        types::RemoveFromWhitelist { key },
                        [
                            143u8, 60u8, 232u8, 97u8, 57u8, 212u8, 215u8, 246u8, 184u8, 176u8,
                            222u8, 164u8, 96u8, 245u8, 246u8, 222u8, 235u8, 250u8, 90u8, 62u8,
                            163u8, 194u8, 59u8, 132u8, 79u8, 136u8, 247u8, 153u8, 82u8, 200u8,
                            123u8, 160u8,
                        ],
                    )
                }
                #[doc = "Accepts an agent application. Only available for the root key or"]
                #[doc = "curators."]
                pub fn accept_application(
                    &self,
                    application_id: types::accept_application::ApplicationId,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AcceptApplication>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "accept_application",
                        types::AcceptApplication { application_id },
                        [
                            158u8, 23u8, 11u8, 5u8, 244u8, 237u8, 147u8, 18u8, 191u8, 246u8, 228u8,
                            172u8, 125u8, 114u8, 91u8, 46u8, 32u8, 87u8, 21u8, 158u8, 153u8, 17u8,
                            167u8, 185u8, 19u8, 242u8, 178u8, 191u8, 85u8, 139u8, 1u8, 122u8,
                        ],
                    )
                }
                #[doc = "Denies an agent application. Only available for the root key or"]
                #[doc = "curators."]
                pub fn deny_application(
                    &self,
                    application_id: types::deny_application::ApplicationId,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::DenyApplication>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "deny_application",
                        types::DenyApplication { application_id },
                        [
                            3u8, 35u8, 31u8, 189u8, 19u8, 229u8, 3u8, 195u8, 171u8, 129u8, 3u8,
                            104u8, 196u8, 200u8, 94u8, 211u8, 19u8, 1u8, 89u8, 118u8, 66u8, 113u8,
                            61u8, 94u8, 76u8, 239u8, 81u8, 209u8, 175u8, 136u8, 87u8, 81u8,
                        ],
                    )
                }
                #[doc = "Sets a penalty factor to the given agent emissions. Only available"]
                #[doc = "for the root key or curators."]
                pub fn penalize_agent(
                    &self,
                    agent_key: types::penalize_agent::AgentKey,
                    percentage: types::penalize_agent::Percentage,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::PenalizeAgent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "penalize_agent",
                        types::PenalizeAgent {
                            agent_key,
                            percentage,
                        },
                        [
                            178u8, 246u8, 110u8, 98u8, 211u8, 244u8, 246u8, 253u8, 238u8, 44u8,
                            36u8, 44u8, 42u8, 34u8, 226u8, 45u8, 41u8, 172u8, 217u8, 48u8, 36u8,
                            247u8, 224u8, 30u8, 152u8, 220u8, 167u8, 233u8, 109u8, 12u8, 1u8,
                            244u8,
                        ],
                    )
                }
                #[doc = "Submits a new agent application on behalf of a given key."]
                pub fn submit_application(
                    &self,
                    agent_key: types::submit_application::AgentKey,
                    metadata: types::submit_application::Metadata,
                    removing: types::submit_application::Removing,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SubmitApplication>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "submit_application",
                        types::SubmitApplication {
                            agent_key,
                            metadata,
                            removing,
                        },
                        [
                            90u8, 74u8, 176u8, 200u8, 162u8, 165u8, 39u8, 151u8, 138u8, 43u8,
                            134u8, 179u8, 193u8, 179u8, 215u8, 156u8, 218u8, 123u8, 242u8, 110u8,
                            53u8, 206u8, 146u8, 31u8, 44u8, 117u8, 100u8, 108u8, 226u8, 254u8,
                            38u8, 98u8,
                        ],
                    )
                }
                #[doc = "Creates a new global parameters proposal."]
                pub fn add_global_params_proposal(
                    &self,
                    data: types::add_global_params_proposal::Data,
                    metadata: types::add_global_params_proposal::Metadata,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::AddGlobalParamsProposal,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "add_global_params_proposal",
                        types::AddGlobalParamsProposal { data, metadata },
                        [
                            248u8, 18u8, 13u8, 160u8, 144u8, 9u8, 109u8, 130u8, 112u8, 148u8,
                            205u8, 195u8, 196u8, 178u8, 202u8, 72u8, 90u8, 248u8, 37u8, 190u8,
                            27u8, 128u8, 194u8, 89u8, 242u8, 180u8, 162u8, 47u8, 2u8, 174u8, 54u8,
                            181u8,
                        ],
                    )
                }
                #[doc = "Creates a new custom global proposal."]
                pub fn add_global_custom_proposal(
                    &self,
                    metadata: types::add_global_custom_proposal::Metadata,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::AddGlobalCustomProposal,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "add_global_custom_proposal",
                        types::AddGlobalCustomProposal { metadata },
                        [
                            39u8, 161u8, 177u8, 122u8, 116u8, 162u8, 240u8, 121u8, 52u8, 12u8,
                            245u8, 36u8, 174u8, 191u8, 86u8, 6u8, 193u8, 252u8, 91u8, 168u8, 65u8,
                            210u8, 152u8, 32u8, 170u8, 1u8, 90u8, 104u8, 147u8, 10u8, 236u8, 150u8,
                        ],
                    )
                }
                #[doc = "Creates a proposal moving funds from the treasury account to the"]
                #[doc = "given key."]
                pub fn add_dao_treasury_transfer_proposal(
                    &self,
                    value: types::add_dao_treasury_transfer_proposal::Value,
                    destination_key: types::add_dao_treasury_transfer_proposal::DestinationKey,
                    data: types::add_dao_treasury_transfer_proposal::Data,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::AddDaoTreasuryTransferProposal,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "add_dao_treasury_transfer_proposal",
                        types::AddDaoTreasuryTransferProposal {
                            value,
                            destination_key,
                            data,
                        },
                        [
                            50u8, 69u8, 216u8, 230u8, 244u8, 243u8, 204u8, 93u8, 137u8, 242u8,
                            60u8, 95u8, 133u8, 236u8, 232u8, 109u8, 187u8, 226u8, 248u8, 205u8,
                            161u8, 155u8, 221u8, 20u8, 223u8, 58u8, 109u8, 82u8, 89u8, 8u8, 194u8,
                            51u8,
                        ],
                    )
                }
                #[doc = "Casts a vote for an open proposal."]
                pub fn vote_proposal(
                    &self,
                    proposal_id: types::vote_proposal::ProposalId,
                    agree: types::vote_proposal::Agree,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::VoteProposal>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "vote_proposal",
                        types::VoteProposal { proposal_id, agree },
                        [
                            11u8, 193u8, 8u8, 164u8, 55u8, 154u8, 190u8, 167u8, 17u8, 190u8, 74u8,
                            98u8, 3u8, 190u8, 136u8, 41u8, 160u8, 174u8, 13u8, 195u8, 163u8, 205u8,
                            232u8, 136u8, 134u8, 51u8, 17u8, 30u8, 17u8, 81u8, 216u8, 99u8,
                        ],
                    )
                }
                #[doc = "Removes a casted vote for an open proposal."]
                pub fn remove_vote_proposal(
                    &self,
                    proposal_id: types::remove_vote_proposal::ProposalId,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveVoteProposal>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "remove_vote_proposal",
                        types::RemoveVoteProposal { proposal_id },
                        [
                            23u8, 230u8, 183u8, 84u8, 129u8, 33u8, 62u8, 197u8, 56u8, 160u8, 164u8,
                            182u8, 106u8, 8u8, 213u8, 139u8, 15u8, 151u8, 197u8, 240u8, 218u8,
                            67u8, 231u8, 36u8, 7u8, 4u8, 234u8, 239u8, 33u8, 31u8, 91u8, 129u8,
                        ],
                    )
                }
                #[doc = "Enables vote power delegation."]
                pub fn enable_vote_delegation(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::EnableVoteDelegation>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "enable_vote_delegation",
                        types::EnableVoteDelegation {},
                        [
                            228u8, 231u8, 137u8, 108u8, 148u8, 143u8, 38u8, 0u8, 161u8, 200u8,
                            186u8, 63u8, 209u8, 196u8, 19u8, 213u8, 107u8, 14u8, 183u8, 89u8,
                            243u8, 171u8, 149u8, 114u8, 96u8, 49u8, 49u8, 148u8, 0u8, 191u8, 55u8,
                            47u8,
                        ],
                    )
                }
                #[doc = "Disables vote power delegation."]
                pub fn disable_vote_delegation(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::DisableVoteDelegation,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "disable_vote_delegation",
                        types::DisableVoteDelegation {},
                        [
                            61u8, 31u8, 181u8, 99u8, 56u8, 239u8, 154u8, 3u8, 164u8, 75u8, 2u8,
                            225u8, 105u8, 238u8, 254u8, 133u8, 236u8, 235u8, 2u8, 113u8, 237u8,
                            102u8, 205u8, 12u8, 236u8, 162u8, 73u8, 224u8, 213u8, 95u8, 129u8,
                            72u8,
                        ],
                    )
                }
                #[doc = "Creates a new emission percentage proposal."]
                pub fn add_emission_proposal(
                    &self,
                    recycling_percentage: types::add_emission_proposal::RecyclingPercentage,
                    treasury_percentage: types::add_emission_proposal::TreasuryPercentage,
                    incentives_ratio: types::add_emission_proposal::IncentivesRatio,
                    data: types::add_emission_proposal::Data,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AddEmissionProposal>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "add_emission_proposal",
                        types::AddEmissionProposal {
                            recycling_percentage,
                            treasury_percentage,
                            incentives_ratio,
                            data,
                        },
                        [
                            86u8, 137u8, 122u8, 40u8, 253u8, 161u8, 153u8, 63u8, 45u8, 150u8,
                            116u8, 253u8, 217u8, 129u8, 55u8, 122u8, 12u8, 68u8, 61u8, 155u8, 11u8,
                            2u8, 149u8, 212u8, 39u8, 39u8, 206u8, 246u8, 43u8, 121u8, 200u8, 238u8,
                        ],
                    )
                }
                #[doc = "Forcefully sets emission percentages. Only available for the root"]
                #[doc = "key."]
                pub fn set_emission_params(
                    &self,
                    recycling_percentage: types::set_emission_params::RecyclingPercentage,
                    treasury_percentage: types::set_emission_params::TreasuryPercentage,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetEmissionParams>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "set_emission_params",
                        types::SetEmissionParams {
                            recycling_percentage,
                            treasury_percentage,
                        },
                        [
                            98u8, 20u8, 103u8, 241u8, 31u8, 208u8, 28u8, 89u8, 38u8, 204u8, 113u8,
                            254u8, 185u8, 66u8, 165u8, 30u8, 232u8, 168u8, 55u8, 98u8, 212u8,
                            150u8, 39u8, 241u8, 182u8, 119u8, 216u8, 156u8, 186u8, 0u8, 160u8,
                            190u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_governance::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A new proposal has been created."]
            pub struct ProposalCreated(pub proposal_created::Field0);
            pub mod proposal_created {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ProposalCreated {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ProposalCreated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A proposal has been accepted."]
            pub struct ProposalAccepted(pub proposal_accepted::Field0);
            pub mod proposal_accepted {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ProposalAccepted {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ProposalAccepted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A proposal has been refused."]
            pub struct ProposalRefused(pub proposal_refused::Field0);
            pub mod proposal_refused {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ProposalRefused {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ProposalRefused";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A proposal has expired."]
            pub struct ProposalExpired(pub proposal_expired::Field0);
            pub mod proposal_expired {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ProposalExpired {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ProposalExpired";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A vote has been cast on a proposal."]
            pub struct ProposalVoted(
                pub proposal_voted::Field0,
                pub proposal_voted::Field1,
                pub proposal_voted::Field2,
            );
            pub mod proposal_voted {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u64;
                pub type Field1 = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Field2 = ::core::primitive::bool;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ProposalVoted {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ProposalVoted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A vote has been unregistered from a proposal."]
            pub struct ProposalVoteUnregistered(
                pub proposal_vote_unregistered::Field0,
                pub proposal_vote_unregistered::Field1,
            );
            pub mod proposal_vote_unregistered {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u64;
                pub type Field1 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ProposalVoteUnregistered {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ProposalVoteUnregistered";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An agent account has been added to the whitelist."]
            pub struct WhitelistAdded(pub whitelist_added::Field0);
            pub mod whitelist_added {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for WhitelistAdded {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "WhitelistAdded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An agent account has been removed from the whitelist."]
            pub struct WhitelistRemoved(pub whitelist_removed::Field0);
            pub mod whitelist_removed {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for WhitelistRemoved {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "WhitelistRemoved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A new application has been created."]
            pub struct ApplicationCreated(pub application_created::Field0);
            pub mod application_created {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ApplicationCreated {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ApplicationCreated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An application has been accepted."]
            pub struct ApplicationAccepted(pub application_accepted::Field0);
            pub mod application_accepted {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ApplicationAccepted {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ApplicationAccepted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An application has been denied."]
            pub struct ApplicationDenied(pub application_denied::Field0);
            pub mod application_denied {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ApplicationDenied {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ApplicationDenied";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An application has expired."]
            pub struct ApplicationExpired(pub application_expired::Field0);
            pub mod application_expired {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ApplicationExpired {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "ApplicationExpired";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A penalty was applied to an agent."]
            pub struct PenaltyApplied {
                pub curator: penalty_applied::Curator,
                pub agent: penalty_applied::Agent,
                pub penalty: penalty_applied::Penalty,
            }
            pub mod penalty_applied {
                use super::runtime_types;
                pub type Curator = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Agent = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Penalty = runtime_types::sp_arithmetic::per_things::Percent;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PenaltyApplied {
                const PALLET: &'static str = "Governance";
                const EVENT: &'static str = "PenaltyApplied";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod proposals {
                    use super::runtime_types;
                    pub type Proposals = runtime_types::pallet_governance::proposal::Proposal;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod unrewarded_proposals {
                    use super::runtime_types;
                    pub type UnrewardedProposals =
                        runtime_types::pallet_governance::proposal::UnrewardedProposal;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod not_delegating_voting_power {
                    use super::runtime_types;
                    pub type NotDelegatingVotingPower =
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >;
                }
                pub mod global_governance_config {
                    use super::runtime_types;
                    pub type GlobalGovernanceConfig =
                        runtime_types::pallet_governance::config::GovernanceConfiguration;
                }
                pub mod dao_treasury_address {
                    use super::runtime_types;
                    pub type DaoTreasuryAddress = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod agent_applications {
                    use super::runtime_types;
                    pub type AgentApplications =
                        runtime_types::pallet_governance::application::AgentApplication;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod whitelist {
                    use super::runtime_types;
                    pub type Whitelist = ();
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod allocators {
                    use super::runtime_types;
                    pub type Allocators = ();
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod treasury_emission_fee {
                    use super::runtime_types;
                    pub type TreasuryEmissionFee =
                        runtime_types::sp_arithmetic::per_things::Percent;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Map of past and present proposals indexed by their incrementing ID."]
                pub fn proposals_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::proposals::Proposals,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Proposals",
                        (),
                        [
                            242u8, 56u8, 33u8, 10u8, 246u8, 39u8, 184u8, 197u8, 104u8, 49u8, 249u8,
                            114u8, 22u8, 17u8, 211u8, 162u8, 100u8, 66u8, 217u8, 20u8, 234u8, 96u8,
                            142u8, 154u8, 72u8, 93u8, 141u8, 243u8, 221u8, 158u8, 94u8, 170u8,
                        ],
                    )
                }
                #[doc = " Map of past and present proposals indexed by their incrementing ID."]
                pub fn proposals(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::proposals::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::proposals::Param0,
                    >,
                    types::proposals::Proposals,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Proposals",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            242u8, 56u8, 33u8, 10u8, 246u8, 39u8, 184u8, 197u8, 104u8, 49u8, 249u8,
                            114u8, 22u8, 17u8, 211u8, 162u8, 100u8, 66u8, 217u8, 20u8, 234u8, 96u8,
                            142u8, 154u8, 72u8, 93u8, 141u8, 243u8, 221u8, 158u8, 94u8, 170u8,
                        ],
                    )
                }
                #[doc = " Queue of proposals to be rewarded after closing."]
                pub fn unrewarded_proposals_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::unrewarded_proposals::UnrewardedProposals,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "UnrewardedProposals",
                        (),
                        [
                            115u8, 132u8, 50u8, 54u8, 240u8, 128u8, 218u8, 102u8, 24u8, 109u8,
                            206u8, 173u8, 167u8, 220u8, 52u8, 242u8, 51u8, 12u8, 172u8, 194u8,
                            168u8, 202u8, 220u8, 167u8, 53u8, 161u8, 33u8, 203u8, 120u8, 205u8,
                            250u8, 215u8,
                        ],
                    )
                }
                #[doc = " Queue of proposals to be rewarded after closing."]
                pub fn unrewarded_proposals(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::unrewarded_proposals::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::unrewarded_proposals::Param0,
                    >,
                    types::unrewarded_proposals::UnrewardedProposals,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "UnrewardedProposals",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            115u8, 132u8, 50u8, 54u8, 240u8, 128u8, 218u8, 102u8, 24u8, 109u8,
                            206u8, 173u8, 167u8, 220u8, 52u8, 242u8, 51u8, 12u8, 172u8, 194u8,
                            168u8, 202u8, 220u8, 167u8, 53u8, 161u8, 33u8, 203u8, 120u8, 205u8,
                            250u8, 215u8,
                        ],
                    )
                }
                #[doc = " List of keys that are NOT delegating their voting power. By default, all"]
                #[doc = " keys delegate their voting power."]
                pub fn not_delegating_voting_power(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::not_delegating_voting_power::NotDelegatingVotingPower,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "NotDelegatingVotingPower",
                        (),
                        [
                            28u8, 206u8, 199u8, 30u8, 61u8, 157u8, 107u8, 244u8, 237u8, 100u8,
                            167u8, 44u8, 125u8, 179u8, 49u8, 128u8, 217u8, 151u8, 123u8, 3u8, 9u8,
                            130u8, 225u8, 64u8, 75u8, 48u8, 249u8, 158u8, 101u8, 20u8, 253u8,
                            123u8,
                        ],
                    )
                }
                #[doc = " Global governance configuration files."]
                pub fn global_governance_config(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::global_governance_config::GlobalGovernanceConfig,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "GlobalGovernanceConfig",
                        (),
                        [
                            243u8, 71u8, 183u8, 133u8, 127u8, 58u8, 172u8, 228u8, 141u8, 18u8,
                            171u8, 8u8, 63u8, 58u8, 35u8, 177u8, 71u8, 239u8, 12u8, 148u8, 53u8,
                            143u8, 224u8, 30u8, 45u8, 147u8, 52u8, 127u8, 231u8, 206u8, 65u8, 26u8,
                        ],
                    )
                }
                #[doc = " The treasury address to which the treasury emission percentages and"]
                #[doc = " other funds go to. A proposal can be created withdrawing the funds to a"]
                #[doc = " key."]
                pub fn dao_treasury_address(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::dao_treasury_address::DaoTreasuryAddress,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "DaoTreasuryAddress",
                        (),
                        [
                            7u8, 254u8, 38u8, 167u8, 254u8, 110u8, 184u8, 208u8, 105u8, 92u8,
                            148u8, 9u8, 107u8, 217u8, 20u8, 65u8, 159u8, 243u8, 144u8, 18u8, 158u8,
                            43u8, 34u8, 100u8, 174u8, 20u8, 224u8, 213u8, 189u8, 189u8, 69u8,
                            240u8,
                        ],
                    )
                }
                #[doc = " A map of agent applications, past and present."]
                pub fn agent_applications_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::agent_applications::AgentApplications,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "AgentApplications",
                        (),
                        [
                            10u8, 102u8, 23u8, 131u8, 138u8, 196u8, 205u8, 135u8, 137u8, 25u8, 2u8,
                            46u8, 94u8, 2u8, 110u8, 76u8, 37u8, 215u8, 112u8, 5u8, 134u8, 233u8,
                            108u8, 86u8, 188u8, 200u8, 225u8, 150u8, 180u8, 47u8, 54u8, 103u8,
                        ],
                    )
                }
                #[doc = " A map of agent applications, past and present."]
                pub fn agent_applications(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::agent_applications::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::agent_applications::Param0,
                    >,
                    types::agent_applications::AgentApplications,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "AgentApplications",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            10u8, 102u8, 23u8, 131u8, 138u8, 196u8, 205u8, 135u8, 137u8, 25u8, 2u8,
                            46u8, 94u8, 2u8, 110u8, 76u8, 37u8, 215u8, 112u8, 5u8, 134u8, 233u8,
                            108u8, 86u8, 188u8, 200u8, 225u8, 150u8, 180u8, 47u8, 54u8, 103u8,
                        ],
                    )
                }
                #[doc = " List of whitelisted keys. Keys listed here are allowed to register"]
                #[doc = " agents."]
                pub fn whitelist_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::whitelist::Whitelist,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Whitelist",
                        (),
                        [
                            126u8, 167u8, 137u8, 166u8, 238u8, 45u8, 127u8, 160u8, 135u8, 139u8,
                            136u8, 73u8, 129u8, 86u8, 64u8, 46u8, 104u8, 161u8, 62u8, 173u8, 113u8,
                            163u8, 23u8, 249u8, 141u8, 27u8, 188u8, 136u8, 33u8, 232u8, 173u8,
                            203u8,
                        ],
                    )
                }
                #[doc = " List of whitelisted keys. Keys listed here are allowed to register"]
                #[doc = " agents."]
                pub fn whitelist(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::whitelist::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::whitelist::Param0,
                    >,
                    types::whitelist::Whitelist,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Whitelist",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            126u8, 167u8, 137u8, 166u8, 238u8, 45u8, 127u8, 160u8, 135u8, 139u8,
                            136u8, 73u8, 129u8, 86u8, 64u8, 46u8, 104u8, 161u8, 62u8, 173u8, 113u8,
                            163u8, 23u8, 249u8, 141u8, 27u8, 188u8, 136u8, 33u8, 232u8, 173u8,
                            203u8,
                        ],
                    )
                }
                #[doc = " List of allocator keys, which are the default validators on the network."]
                pub fn allocators_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::allocators::Allocators,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Allocators",
                        (),
                        [
                            87u8, 38u8, 0u8, 204u8, 39u8, 210u8, 232u8, 75u8, 58u8, 70u8, 171u8,
                            171u8, 140u8, 224u8, 137u8, 99u8, 164u8, 244u8, 92u8, 13u8, 132u8,
                            51u8, 148u8, 242u8, 17u8, 144u8, 124u8, 29u8, 233u8, 237u8, 186u8,
                            43u8,
                        ],
                    )
                }
                #[doc = " List of allocator keys, which are the default validators on the network."]
                pub fn allocators(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::allocators::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::allocators::Param0,
                    >,
                    types::allocators::Allocators,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Allocators",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            87u8, 38u8, 0u8, 204u8, 39u8, 210u8, 232u8, 75u8, 58u8, 70u8, 171u8,
                            171u8, 140u8, 224u8, 137u8, 99u8, 164u8, 244u8, 92u8, 13u8, 132u8,
                            51u8, 148u8, 242u8, 17u8, 144u8, 124u8, 29u8, 233u8, 237u8, 186u8,
                            43u8,
                        ],
                    )
                }
                #[doc = " Fee taken from emission distribution and deposited into"]
                #[doc = " [`DaoTreasuryAddress`]."]
                pub fn treasury_emission_fee(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::treasury_emission_fee::TreasuryEmissionFee,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "TreasuryEmissionFee",
                        (),
                        [
                            28u8, 90u8, 191u8, 200u8, 20u8, 143u8, 151u8, 112u8, 207u8, 187u8,
                            22u8, 49u8, 79u8, 170u8, 167u8, 249u8, 57u8, 58u8, 114u8, 19u8, 83u8,
                            166u8, 122u8, 255u8, 34u8, 148u8, 150u8, 17u8, 136u8, 198u8, 47u8,
                            235u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn pallet_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_support::PalletId,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "PalletId",
                        [
                            56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
                            161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
                            129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
                        ],
                    )
                }
                pub fn min_application_data_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "MinApplicationDataLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_application_data_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "MaxApplicationDataLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn application_expiration(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "ApplicationExpiration",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn max_penalty_percentage(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_arithmetic::per_things::Percent,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "MaxPenaltyPercentage",
                        [
                            40u8, 171u8, 69u8, 196u8, 34u8, 184u8, 50u8, 128u8, 139u8, 192u8, 63u8,
                            231u8, 249u8, 200u8, 252u8, 73u8, 244u8, 170u8, 51u8, 177u8, 106u8,
                            47u8, 114u8, 234u8, 84u8, 104u8, 62u8, 118u8, 227u8, 50u8, 225u8,
                            122u8,
                        ],
                    )
                }
                pub fn default_treasury_emission_fee(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_arithmetic::per_things::Percent,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultTreasuryEmissionFee",
                        [
                            40u8, 171u8, 69u8, 196u8, 34u8, 184u8, 50u8, 128u8, 139u8, 192u8, 63u8,
                            231u8, 249u8, 200u8, 252u8, 73u8, 244u8, 170u8, 51u8, 177u8, 106u8,
                            47u8, 114u8, 234u8, 84u8, 104u8, 62u8, 118u8, 227u8, 50u8, 225u8,
                            122u8,
                        ],
                    )
                }
                pub fn default_proposal_cost(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultProposalCost",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_proposal_expiration(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultProposalExpiration",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn default_agent_application_cost(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultAgentApplicationCost",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_agent_application_expiration(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultAgentApplicationExpiration",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn default_proposal_reward_treasury_allocation(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_arithmetic::per_things::Percent,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultProposalRewardTreasuryAllocation",
                        [
                            40u8, 171u8, 69u8, 196u8, 34u8, 184u8, 50u8, 128u8, 139u8, 192u8, 63u8,
                            231u8, 249u8, 200u8, 252u8, 73u8, 244u8, 170u8, 51u8, 177u8, 106u8,
                            47u8, 114u8, 234u8, 84u8, 104u8, 62u8, 118u8, 227u8, 50u8, 225u8,
                            122u8,
                        ],
                    )
                }
                pub fn default_max_proposal_reward_treasury_allocation(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultMaxProposalRewardTreasuryAllocation",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_proposal_reward_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Governance",
                        "DefaultProposalRewardInterval",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod torus0 {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_torus0::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_torus0::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Adds stakes from origin to the agent key."]
                pub struct AddStake {
                    pub agent_key: add_stake::AgentKey,
                    pub amount: add_stake::Amount,
                }
                pub mod add_stake {
                    use super::runtime_types;
                    pub type AgentKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddStake {
                    const PALLET: &'static str = "Torus0";
                    const CALL: &'static str = "add_stake";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Removes stakes from origin to the agent key."]
                pub struct RemoveStake {
                    pub agent_key: remove_stake::AgentKey,
                    pub amount: remove_stake::Amount,
                }
                pub mod remove_stake {
                    use super::runtime_types;
                    pub type AgentKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveStake {
                    const PALLET: &'static str = "Torus0";
                    const CALL: &'static str = "remove_stake";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transfers origin's stakes from an agent to another."]
                pub struct TransferStake {
                    pub agent_key: transfer_stake::AgentKey,
                    pub new_agent_key: transfer_stake::NewAgentKey,
                    pub amount: transfer_stake::Amount,
                }
                pub mod transfer_stake {
                    use super::runtime_types;
                    pub type AgentKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type NewAgentKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferStake {
                    const PALLET: &'static str = "Torus0";
                    const CALL: &'static str = "transfer_stake";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Registers a new agent on behalf of an arbitrary key."]
                pub struct RegisterAgent {
                    pub agent_key: register_agent::AgentKey,
                    pub name: register_agent::Name,
                    pub url: register_agent::Url,
                    pub metadata: register_agent::Metadata,
                }
                pub mod register_agent {
                    use super::runtime_types;
                    pub type AgentKey = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Name =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Url = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Metadata =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RegisterAgent {
                    const PALLET: &'static str = "Torus0";
                    const CALL: &'static str = "register_agent";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Unregister origin's key agent."]
                pub struct UnregisterAgent;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UnregisterAgent {
                    const PALLET: &'static str = "Torus0";
                    const CALL: &'static str = "unregister_agent";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Updates origin's key agent metadata."]
                pub struct UpdateAgent {
                    pub name: update_agent::Name,
                    pub url: update_agent::Url,
                    pub metadata: update_agent::Metadata,
                    pub staking_fee: update_agent::StakingFee,
                    pub weight_control_fee: update_agent::WeightControlFee,
                }
                pub mod update_agent {
                    use super::runtime_types;
                    pub type Name =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Url = ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Metadata = ::core::option::Option<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                    pub type StakingFee =
                        ::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>;
                    pub type WeightControlFee =
                        ::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpdateAgent {
                    const PALLET: &'static str = "Torus0";
                    const CALL: &'static str = "update_agent";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Updates origin's key agent metadata."]
                pub struct SetAgentUpdateCooldown {
                    pub new_cooldown: set_agent_update_cooldown::NewCooldown,
                }
                pub mod set_agent_update_cooldown {
                    use super::runtime_types;
                    pub type NewCooldown = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetAgentUpdateCooldown {
                    const PALLET: &'static str = "Torus0";
                    const CALL: &'static str = "set_agent_update_cooldown";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Adds stakes from origin to the agent key."]
                pub fn add_stake(
                    &self,
                    agent_key: types::add_stake::AgentKey,
                    amount: types::add_stake::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AddStake>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Torus0",
                        "add_stake",
                        types::AddStake { agent_key, amount },
                        [
                            121u8, 38u8, 54u8, 17u8, 63u8, 145u8, 243u8, 55u8, 59u8, 139u8, 1u8,
                            161u8, 75u8, 65u8, 252u8, 14u8, 180u8, 158u8, 162u8, 190u8, 4u8, 116u8,
                            1u8, 179u8, 250u8, 142u8, 130u8, 8u8, 81u8, 11u8, 248u8, 249u8,
                        ],
                    )
                }
                #[doc = "Removes stakes from origin to the agent key."]
                pub fn remove_stake(
                    &self,
                    agent_key: types::remove_stake::AgentKey,
                    amount: types::remove_stake::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveStake>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Torus0",
                        "remove_stake",
                        types::RemoveStake { agent_key, amount },
                        [
                            233u8, 29u8, 115u8, 1u8, 59u8, 255u8, 233u8, 49u8, 208u8, 123u8, 197u8,
                            138u8, 14u8, 249u8, 49u8, 64u8, 148u8, 115u8, 197u8, 239u8, 110u8,
                            191u8, 40u8, 188u8, 141u8, 175u8, 28u8, 112u8, 129u8, 167u8, 58u8,
                            66u8,
                        ],
                    )
                }
                #[doc = "Transfers origin's stakes from an agent to another."]
                pub fn transfer_stake(
                    &self,
                    agent_key: types::transfer_stake::AgentKey,
                    new_agent_key: types::transfer_stake::NewAgentKey,
                    amount: types::transfer_stake::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferStake>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Torus0",
                        "transfer_stake",
                        types::TransferStake {
                            agent_key,
                            new_agent_key,
                            amount,
                        },
                        [
                            156u8, 151u8, 191u8, 7u8, 21u8, 36u8, 213u8, 199u8, 80u8, 219u8, 157u8,
                            234u8, 186u8, 95u8, 46u8, 247u8, 46u8, 13u8, 232u8, 26u8, 72u8, 12u8,
                            41u8, 17u8, 152u8, 117u8, 34u8, 118u8, 29u8, 23u8, 113u8, 110u8,
                        ],
                    )
                }
                #[doc = "Registers a new agent on behalf of an arbitrary key."]
                pub fn register_agent(
                    &self,
                    agent_key: types::register_agent::AgentKey,
                    name: types::register_agent::Name,
                    url: types::register_agent::Url,
                    metadata: types::register_agent::Metadata,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RegisterAgent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Torus0",
                        "register_agent",
                        types::RegisterAgent {
                            agent_key,
                            name,
                            url,
                            metadata,
                        },
                        [
                            22u8, 219u8, 52u8, 137u8, 14u8, 162u8, 6u8, 211u8, 174u8, 197u8, 212u8,
                            223u8, 215u8, 175u8, 245u8, 184u8, 243u8, 152u8, 90u8, 119u8, 232u8,
                            58u8, 242u8, 153u8, 211u8, 9u8, 109u8, 86u8, 239u8, 198u8, 164u8,
                            182u8,
                        ],
                    )
                }
                #[doc = "Unregister origin's key agent."]
                pub fn unregister_agent(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UnregisterAgent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Torus0",
                        "unregister_agent",
                        types::UnregisterAgent {},
                        [
                            6u8, 176u8, 124u8, 167u8, 12u8, 211u8, 188u8, 90u8, 86u8, 0u8, 30u8,
                            75u8, 142u8, 247u8, 237u8, 110u8, 64u8, 60u8, 81u8, 35u8, 67u8, 92u8,
                            164u8, 1u8, 6u8, 141u8, 79u8, 129u8, 114u8, 48u8, 191u8, 92u8,
                        ],
                    )
                }
                #[doc = "Updates origin's key agent metadata."]
                pub fn update_agent(
                    &self,
                    name: types::update_agent::Name,
                    url: types::update_agent::Url,
                    metadata: types::update_agent::Metadata,
                    staking_fee: types::update_agent::StakingFee,
                    weight_control_fee: types::update_agent::WeightControlFee,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpdateAgent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Torus0",
                        "update_agent",
                        types::UpdateAgent {
                            name,
                            url,
                            metadata,
                            staking_fee,
                            weight_control_fee,
                        },
                        [
                            28u8, 23u8, 34u8, 208u8, 45u8, 23u8, 30u8, 85u8, 251u8, 2u8, 238u8,
                            68u8, 53u8, 102u8, 44u8, 30u8, 58u8, 68u8, 206u8, 40u8, 249u8, 0u8,
                            131u8, 95u8, 198u8, 219u8, 137u8, 23u8, 124u8, 18u8, 35u8, 159u8,
                        ],
                    )
                }
                #[doc = "Updates origin's key agent metadata."]
                pub fn set_agent_update_cooldown(
                    &self,
                    new_cooldown: types::set_agent_update_cooldown::NewCooldown,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::SetAgentUpdateCooldown,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Torus0",
                        "set_agent_update_cooldown",
                        types::SetAgentUpdateCooldown { new_cooldown },
                        [
                            205u8, 244u8, 69u8, 229u8, 205u8, 68u8, 116u8, 34u8, 203u8, 37u8,
                            121u8, 211u8, 255u8, 232u8, 115u8, 144u8, 196u8, 31u8, 48u8, 84u8,
                            148u8, 244u8, 115u8, 65u8, 204u8, 193u8, 109u8, 184u8, 103u8, 41u8,
                            224u8, 92u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_torus0::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Event created when stake has been transferred from the coldkey"]
            #[doc = "account onto the key staking account"]
            pub struct StakeAdded(
                pub stake_added::Field0,
                pub stake_added::Field1,
                pub stake_added::Field2,
            );
            pub mod stake_added {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Field1 = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Field2 = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for StakeAdded {
                const PALLET: &'static str = "Torus0";
                const EVENT: &'static str = "StakeAdded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Event created when stake has been removed from the key staking"]
            #[doc = "account onto the coldkey account"]
            pub struct StakeRemoved(
                pub stake_removed::Field0,
                pub stake_removed::Field1,
                pub stake_removed::Field2,
            );
            pub mod stake_removed {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Field1 = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Field2 = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for StakeRemoved {
                const PALLET: &'static str = "Torus0";
                const EVENT: &'static str = "StakeRemoved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Event created when a new agent account has been registered to the"]
            #[doc = "chain"]
            pub struct AgentRegistered(pub agent_registered::Field0);
            pub mod agent_registered {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AgentRegistered {
                const PALLET: &'static str = "Torus0";
                const EVENT: &'static str = "AgentRegistered";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Event created when a agent account has been deregistered from the"]
            #[doc = "chain"]
            pub struct AgentUnregistered(pub agent_unregistered::Field0);
            pub mod agent_unregistered {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AgentUnregistered {
                const PALLET: &'static str = "Torus0";
                const EVENT: &'static str = "AgentUnregistered";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Event created when the agent's updated information is added to the"]
            #[doc = "network"]
            pub struct AgentUpdated(pub agent_updated::Field0);
            pub mod agent_updated {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AgentUpdated {
                const PALLET: &'static str = "Torus0";
                const EVENT: &'static str = "AgentUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod max_allowed_validators {
                    use super::runtime_types;
                    pub type MaxAllowedValidators = ::core::primitive::u16;
                }
                pub mod burn {
                    use super::runtime_types;
                    pub type Burn = ::core::primitive::u128;
                }
                pub mod registrations_this_interval {
                    use super::runtime_types;
                    pub type RegistrationsThisInterval = ::core::primitive::u16;
                }
                pub mod min_validator_stake {
                    use super::runtime_types;
                    pub type MinValidatorStake = ::core::primitive::u128;
                }
                pub mod immunity_period {
                    use super::runtime_types;
                    pub type ImmunityPeriod = ::core::primitive::u16;
                }
                pub mod reward_interval {
                    use super::runtime_types;
                    pub type RewardInterval = ::core::primitive::u16;
                }
                pub mod agents {
                    use super::runtime_types;
                    pub type Agents = runtime_types::pallet_torus0::agent::Agent;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod max_name_length {
                    use super::runtime_types;
                    pub type MaxNameLength = ::core::primitive::u16;
                }
                pub mod min_name_length {
                    use super::runtime_types;
                    pub type MinNameLength = ::core::primitive::u16;
                }
                pub mod max_agent_url_length {
                    use super::runtime_types;
                    pub type MaxAgentUrlLength = ::core::primitive::u16;
                }
                pub mod max_allowed_agents {
                    use super::runtime_types;
                    pub type MaxAllowedAgents = ::core::primitive::u16;
                }
                pub mod registrations_this_block {
                    use super::runtime_types;
                    pub type RegistrationsThisBlock = ::core::primitive::u16;
                }
                pub mod max_registrations_per_block {
                    use super::runtime_types;
                    pub type MaxRegistrationsPerBlock = ::core::primitive::u16;
                }
                pub mod staking_to {
                    use super::runtime_types;
                    pub type StakingTo = ::core::primitive::u128;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param1 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod staked_by {
                    use super::runtime_types;
                    pub type StakedBy = ::core::primitive::u128;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param1 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod total_stake {
                    use super::runtime_types;
                    pub type TotalStake = ::core::primitive::u128;
                }
                pub mod min_allowed_stake {
                    use super::runtime_types;
                    pub type MinAllowedStake = ::core::primitive::u128;
                }
                pub mod dividends_participation_weight {
                    use super::runtime_types;
                    pub type DividendsParticipationWeight =
                        runtime_types::sp_arithmetic::per_things::Percent;
                }
                pub mod fee_constraints {
                    use super::runtime_types;
                    pub type FeeConstraints =
                        runtime_types::pallet_torus0::fee::ValidatorFeeConstraints;
                }
                pub mod burn_config {
                    use super::runtime_types;
                    pub type BurnConfig = runtime_types::pallet_torus0::burn::BurnConfiguration;
                }
                pub mod agent_update_cooldown {
                    use super::runtime_types;
                    pub type AgentUpdateCooldown = ::core::primitive::u64;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Max allowed of validators. This is used then calculating emissions, only"]
                #[doc = " the top staked agents up to this value will have their weights"]
                #[doc = " considered."]
                pub fn max_allowed_validators(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::max_allowed_validators::MaxAllowedValidators,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxAllowedValidators",
                        (),
                        [
                            227u8, 197u8, 85u8, 131u8, 74u8, 66u8, 99u8, 26u8, 210u8, 152u8, 231u8,
                            123u8, 22u8, 14u8, 75u8, 25u8, 19u8, 80u8, 151u8, 103u8, 188u8, 243u8,
                            16u8, 255u8, 109u8, 193u8, 202u8, 216u8, 9u8, 134u8, 132u8, 255u8,
                        ],
                    )
                }
                #[doc = " Amount of tokens to burn from a payer key when registering new agents."]
                pub fn burn(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::burn::Burn,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "Burn",
                        (),
                        [
                            3u8, 120u8, 184u8, 9u8, 148u8, 75u8, 3u8, 101u8, 66u8, 202u8, 17u8,
                            189u8, 92u8, 156u8, 209u8, 55u8, 221u8, 122u8, 207u8, 183u8, 245u8,
                            153u8, 79u8, 175u8, 17u8, 206u8, 76u8, 173u8, 237u8, 180u8, 83u8,
                            146u8,
                        ],
                    )
                }
                #[doc = " Number of agent registrations that happened in the last"]
                #[doc = " [`BurnConfiguration::target_registrations_interval`] blocks."]
                pub fn registrations_this_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::registrations_this_interval::RegistrationsThisInterval,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "RegistrationsThisInterval",
                        (),
                        [
                            118u8, 90u8, 11u8, 140u8, 176u8, 111u8, 7u8, 81u8, 70u8, 196u8, 145u8,
                            118u8, 195u8, 44u8, 197u8, 206u8, 212u8, 170u8, 98u8, 149u8, 151u8,
                            216u8, 107u8, 129u8, 43u8, 203u8, 88u8, 154u8, 213u8, 189u8, 71u8,
                            32u8,
                        ],
                    )
                }
                #[doc = " Minimum required stake for an agent to be considered a validator."]
                pub fn min_validator_stake(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::min_validator_stake::MinValidatorStake,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MinValidatorStake",
                        (),
                        [
                            168u8, 90u8, 97u8, 102u8, 221u8, 186u8, 251u8, 26u8, 49u8, 37u8, 102u8,
                            93u8, 196u8, 6u8, 242u8, 182u8, 168u8, 184u8, 6u8, 97u8, 178u8, 157u8,
                            11u8, 199u8, 130u8, 40u8, 123u8, 91u8, 252u8, 40u8, 221u8, 76u8,
                        ],
                    )
                }
                #[doc = " Number of blocks in which an agent is immune to pruning after"]
                #[doc = " registration."]
                pub fn immunity_period(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::immunity_period::ImmunityPeriod,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "ImmunityPeriod",
                        (),
                        [
                            230u8, 228u8, 145u8, 124u8, 8u8, 14u8, 88u8, 68u8, 93u8, 76u8, 144u8,
                            150u8, 169u8, 180u8, 180u8, 132u8, 182u8, 194u8, 108u8, 208u8, 213u8,
                            157u8, 106u8, 156u8, 171u8, 137u8, 202u8, 156u8, 157u8, 26u8, 52u8,
                            216u8,
                        ],
                    )
                }
                #[doc = " Number of blocks between emissions."]
                pub fn reward_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reward_interval::RewardInterval,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "RewardInterval",
                        (),
                        [
                            232u8, 163u8, 159u8, 56u8, 63u8, 173u8, 169u8, 238u8, 15u8, 17u8,
                            130u8, 228u8, 41u8, 85u8, 72u8, 202u8, 17u8, 151u8, 178u8, 155u8, 18u8,
                            234u8, 50u8, 64u8, 62u8, 51u8, 228u8, 117u8, 20u8, 193u8, 50u8, 41u8,
                        ],
                    )
                }
                #[doc = " Known registered network agents indexed by the owner's key."]
                pub fn agents_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::agents::Agents,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "Agents",
                        (),
                        [
                            255u8, 188u8, 168u8, 152u8, 228u8, 64u8, 223u8, 49u8, 223u8, 64u8,
                            11u8, 29u8, 9u8, 197u8, 194u8, 166u8, 254u8, 200u8, 9u8, 98u8, 14u8,
                            13u8, 102u8, 113u8, 64u8, 62u8, 16u8, 147u8, 131u8, 25u8, 204u8, 52u8,
                        ],
                    )
                }
                #[doc = " Known registered network agents indexed by the owner's key."]
                pub fn agents(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::agents::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::agents::Param0,
                    >,
                    types::agents::Agents,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "Agents",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            255u8, 188u8, 168u8, 152u8, 228u8, 64u8, 223u8, 49u8, 223u8, 64u8,
                            11u8, 29u8, 9u8, 197u8, 194u8, 166u8, 254u8, 200u8, 9u8, 98u8, 14u8,
                            13u8, 102u8, 113u8, 64u8, 62u8, 16u8, 147u8, 131u8, 25u8, 204u8, 52u8,
                        ],
                    )
                }
                #[doc = " Maximum number of characters allowed in an agent name."]
                pub fn max_name_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::max_name_length::MaxNameLength,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxNameLength",
                        (),
                        [
                            97u8, 239u8, 253u8, 96u8, 147u8, 40u8, 242u8, 31u8, 217u8, 245u8,
                            167u8, 222u8, 136u8, 230u8, 117u8, 99u8, 210u8, 24u8, 230u8, 109u8,
                            165u8, 239u8, 152u8, 200u8, 213u8, 93u8, 255u8, 205u8, 225u8, 49u8,
                            114u8, 21u8,
                        ],
                    )
                }
                #[doc = " Minimum number of characters required in an agent name."]
                pub fn min_name_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::min_name_length::MinNameLength,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MinNameLength",
                        (),
                        [
                            48u8, 196u8, 143u8, 97u8, 143u8, 143u8, 25u8, 146u8, 94u8, 133u8, 13u8,
                            19u8, 205u8, 197u8, 240u8, 90u8, 78u8, 175u8, 158u8, 145u8, 45u8,
                            118u8, 62u8, 12u8, 37u8, 205u8, 157u8, 36u8, 144u8, 71u8, 226u8, 230u8,
                        ],
                    )
                }
                #[doc = " Maximum number of characters allowed in an agent URL."]
                pub fn max_agent_url_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::max_agent_url_length::MaxAgentUrlLength,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxAgentUrlLength",
                        (),
                        [
                            147u8, 46u8, 243u8, 6u8, 194u8, 167u8, 76u8, 0u8, 198u8, 168u8, 158u8,
                            249u8, 142u8, 57u8, 77u8, 118u8, 189u8, 91u8, 56u8, 220u8, 30u8, 221u8,
                            42u8, 27u8, 199u8, 112u8, 240u8, 192u8, 53u8, 239u8, 122u8, 230u8,
                        ],
                    )
                }
                #[doc = " Maximum number of agents registered at one time. Registering when this"]
                #[doc = " number is met means new comers will cause pruning of old agents."]
                pub fn max_allowed_agents(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::max_allowed_agents::MaxAllowedAgents,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxAllowedAgents",
                        (),
                        [
                            212u8, 94u8, 169u8, 15u8, 240u8, 143u8, 21u8, 156u8, 1u8, 174u8, 183u8,
                            124u8, 100u8, 147u8, 53u8, 64u8, 190u8, 186u8, 149u8, 200u8, 251u8,
                            181u8, 57u8, 194u8, 68u8, 69u8, 97u8, 81u8, 194u8, 67u8, 42u8, 123u8,
                        ],
                    )
                }
                #[doc = " Number of agent registrations that happened this block."]
                pub fn registrations_this_block(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::registrations_this_block::RegistrationsThisBlock,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "RegistrationsThisBlock",
                        (),
                        [
                            87u8, 222u8, 95u8, 202u8, 125u8, 45u8, 101u8, 83u8, 106u8, 122u8,
                            246u8, 95u8, 76u8, 232u8, 187u8, 217u8, 228u8, 227u8, 166u8, 108u8,
                            1u8, 252u8, 163u8, 241u8, 140u8, 141u8, 85u8, 156u8, 170u8, 222u8,
                            172u8, 174u8,
                        ],
                    )
                }
                #[doc = " Maximum amount of agent registrations per block, tracked by"]
                #[doc = " [`RegistrationsThisBlock`]."]
                pub fn max_registrations_per_block(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::max_registrations_per_block::MaxRegistrationsPerBlock,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxRegistrationsPerBlock",
                        (),
                        [
                            30u8, 223u8, 209u8, 99u8, 68u8, 64u8, 210u8, 8u8, 59u8, 57u8, 208u8,
                            159u8, 25u8, 159u8, 66u8, 187u8, 230u8, 39u8, 251u8, 199u8, 95u8, 48u8,
                            197u8, 140u8, 240u8, 110u8, 57u8, 138u8, 36u8, 244u8, 1u8, 255u8,
                        ],
                    )
                }
                pub fn staking_to_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::staking_to::StakingTo,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "StakingTo",
                        (),
                        [
                            146u8, 118u8, 130u8, 67u8, 15u8, 28u8, 221u8, 198u8, 149u8, 75u8, 66u8,
                            189u8, 75u8, 153u8, 180u8, 161u8, 110u8, 148u8, 201u8, 33u8, 184u8,
                            250u8, 171u8, 144u8, 182u8, 224u8, 39u8, 175u8, 37u8, 41u8, 166u8,
                            16u8,
                        ],
                    )
                }
                pub fn staking_to_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::staking_to::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::staking_to::Param0,
                    >,
                    types::staking_to::StakingTo,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "StakingTo",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            146u8, 118u8, 130u8, 67u8, 15u8, 28u8, 221u8, 198u8, 149u8, 75u8, 66u8,
                            189u8, 75u8, 153u8, 180u8, 161u8, 110u8, 148u8, 201u8, 33u8, 184u8,
                            250u8, 171u8, 144u8, 182u8, 224u8, 39u8, 175u8, 37u8, 41u8, 166u8,
                            16u8,
                        ],
                    )
                }
                pub fn staking_to(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::staking_to::Param0>,
                    _1: impl ::core::borrow::Borrow<types::staking_to::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::staking_to::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::staking_to::Param1,
                        >,
                    ),
                    types::staking_to::StakingTo,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "StakingTo",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            146u8, 118u8, 130u8, 67u8, 15u8, 28u8, 221u8, 198u8, 149u8, 75u8, 66u8,
                            189u8, 75u8, 153u8, 180u8, 161u8, 110u8, 148u8, 201u8, 33u8, 184u8,
                            250u8, 171u8, 144u8, 182u8, 224u8, 39u8, 175u8, 37u8, 41u8, 166u8,
                            16u8,
                        ],
                    )
                }
                pub fn staked_by_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::staked_by::StakedBy,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "StakedBy",
                        (),
                        [
                            15u8, 177u8, 30u8, 202u8, 143u8, 1u8, 74u8, 96u8, 112u8, 98u8, 100u8,
                            144u8, 171u8, 144u8, 118u8, 124u8, 251u8, 91u8, 209u8, 99u8, 196u8,
                            143u8, 108u8, 96u8, 100u8, 247u8, 85u8, 169u8, 19u8, 154u8, 0u8, 147u8,
                        ],
                    )
                }
                pub fn staked_by_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::staked_by::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::staked_by::Param0,
                    >,
                    types::staked_by::StakedBy,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "StakedBy",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            15u8, 177u8, 30u8, 202u8, 143u8, 1u8, 74u8, 96u8, 112u8, 98u8, 100u8,
                            144u8, 171u8, 144u8, 118u8, 124u8, 251u8, 91u8, 209u8, 99u8, 196u8,
                            143u8, 108u8, 96u8, 100u8, 247u8, 85u8, 169u8, 19u8, 154u8, 0u8, 147u8,
                        ],
                    )
                }
                pub fn staked_by(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::staked_by::Param0>,
                    _1: impl ::core::borrow::Borrow<types::staked_by::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::staked_by::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::staked_by::Param1,
                        >,
                    ),
                    types::staked_by::StakedBy,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "StakedBy",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            15u8, 177u8, 30u8, 202u8, 143u8, 1u8, 74u8, 96u8, 112u8, 98u8, 100u8,
                            144u8, 171u8, 144u8, 118u8, 124u8, 251u8, 91u8, 209u8, 99u8, 196u8,
                            143u8, 108u8, 96u8, 100u8, 247u8, 85u8, 169u8, 19u8, 154u8, 0u8, 147u8,
                        ],
                    )
                }
                #[doc = " The total amount of stake in the network."]
                pub fn total_stake(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::total_stake::TotalStake,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "TotalStake",
                        (),
                        [
                            216u8, 150u8, 145u8, 207u8, 111u8, 220u8, 156u8, 4u8, 35u8, 94u8,
                            223u8, 172u8, 253u8, 203u8, 42u8, 240u8, 136u8, 43u8, 16u8, 202u8,
                            48u8, 213u8, 46u8, 248u8, 59u8, 98u8, 98u8, 124u8, 154u8, 100u8, 89u8,
                            78u8,
                        ],
                    )
                }
                #[doc = " Minimum amount of stake in tokens a key has to deposit in an agent."]
                pub fn min_allowed_stake(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::min_allowed_stake::MinAllowedStake,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "MinAllowedStake",
                        (),
                        [
                            95u8, 136u8, 254u8, 192u8, 170u8, 15u8, 199u8, 24u8, 60u8, 216u8, 30u8,
                            197u8, 237u8, 10u8, 46u8, 19u8, 23u8, 145u8, 240u8, 79u8, 47u8, 124u8,
                            201u8, 60u8, 13u8, 169u8, 71u8, 136u8, 227u8, 72u8, 30u8, 108u8,
                        ],
                    )
                }
                #[doc = " The weight dividends have when finding agents to prune. 100% meaning it"]
                #[doc = " is taking fully into account."]
                pub fn dividends_participation_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::dividends_participation_weight::DividendsParticipationWeight,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "DividendsParticipationWeight",
                        (),
                        [
                            200u8, 171u8, 104u8, 232u8, 161u8, 238u8, 115u8, 12u8, 241u8, 70u8,
                            80u8, 231u8, 204u8, 114u8, 172u8, 225u8, 193u8, 213u8, 168u8, 125u8,
                            11u8, 86u8, 237u8, 121u8, 29u8, 112u8, 78u8, 199u8, 72u8, 210u8, 250u8,
                            45u8,
                        ],
                    )
                }
                #[doc = " Constraints defining validation of agent fees."]
                pub fn fee_constraints(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::fee_constraints::FeeConstraints,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "FeeConstraints",
                        (),
                        [
                            161u8, 137u8, 31u8, 186u8, 219u8, 246u8, 226u8, 190u8, 38u8, 9u8,
                            100u8, 114u8, 52u8, 218u8, 47u8, 116u8, 111u8, 12u8, 113u8, 57u8,
                            186u8, 236u8, 142u8, 253u8, 74u8, 119u8, 110u8, 7u8, 168u8, 215u8,
                            90u8, 173u8,
                        ],
                    )
                }
                #[doc = " [`Burn`] configuration values."]
                pub fn burn_config(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::burn_config::BurnConfig,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "BurnConfig",
                        (),
                        [
                            106u8, 67u8, 203u8, 217u8, 76u8, 71u8, 35u8, 160u8, 111u8, 207u8,
                            178u8, 176u8, 14u8, 128u8, 233u8, 132u8, 243u8, 255u8, 35u8, 65u8,
                            143u8, 246u8, 184u8, 14u8, 191u8, 213u8, 159u8, 29u8, 204u8, 113u8,
                            213u8, 15u8,
                        ],
                    )
                }
                #[doc = " Cooldown (in blocks) in which an agent needs to wait between each `update_agent` call."]
                pub fn agent_update_cooldown(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::agent_update_cooldown::AgentUpdateCooldown,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Torus0",
                        "AgentUpdateCooldown",
                        (),
                        [
                            111u8, 168u8, 125u8, 36u8, 247u8, 65u8, 246u8, 71u8, 36u8, 187u8, 1u8,
                            77u8, 46u8, 43u8, 103u8, 100u8, 198u8, 70u8, 31u8, 141u8, 192u8, 199u8,
                            45u8, 123u8, 86u8, 53u8, 158u8, 66u8, 6u8, 112u8, 96u8, 141u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn default_max_allowed_validators(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMaxAllowedValidators",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_min_validator_stake(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMinValidatorStake",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_immunity_period(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultImmunityPeriod",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_reward_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultRewardInterval",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_min_name_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMinNameLength",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_max_name_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMaxNameLength",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_max_agent_url_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMaxAgentUrlLength",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_max_allowed_agents(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMaxAllowedAgents",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_max_registrations_per_block(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMaxRegistrationsPerBlock",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_min_allowed_stake(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMinAllowedStake",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_min_staking_fee(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u8,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMinStakingFee",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
                pub fn default_min_weight_control_fee(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u8,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMinWeightControlFee",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
                pub fn default_min_burn(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMinBurn",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_max_burn(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMaxBurn",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_adjustment_alpha(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultAdjustmentAlpha",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn default_target_registrations_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultTargetRegistrationsInterval",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn default_target_registrations_per_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultTargetRegistrationsPerInterval",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                pub fn default_max_registrations_per_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultMaxRegistrationsPerInterval",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
                #[doc = " The storage MaxNameLength should be constrained to be no more than"]
                #[doc = " the value of this. This is needed on agent::Agent to set the"]
                #[doc = " `name` field BoundedVec max length."]
                pub fn max_agent_name_length_constraint(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxAgentNameLengthConstraint",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " This is needed on agent::Agent to set the `url` field BoundedVec max"]
                #[doc = " length."]
                pub fn max_agent_url_length_constraint(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxAgentUrlLengthConstraint",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_agent_metadata_length_constraint(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "MaxAgentMetadataLengthConstraint",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn default_dividends_participation_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_arithmetic::per_things::Percent,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultDividendsParticipationWeight",
                        [
                            40u8, 171u8, 69u8, 196u8, 34u8, 184u8, 50u8, 128u8, 139u8, 192u8, 63u8,
                            231u8, 249u8, 200u8, 252u8, 73u8, 244u8, 170u8, 51u8, 177u8, 106u8,
                            47u8, 114u8, 234u8, 84u8, 104u8, 62u8, 118u8, 227u8, 50u8, 225u8,
                            122u8,
                        ],
                    )
                }
                #[doc = " Default Cooldown (in blocks) in which an agent needs to wait between each `update_agent` call."]
                pub fn default_agent_update_cooldown(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Torus0",
                        "DefaultAgentUpdateCooldown",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod emission0 {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_emission0::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_emission0::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct SetWeights {
                    pub weights: set_weights::Weights,
                }
                pub mod set_weights {
                    use super::runtime_types;
                    pub type Weights = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u16,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetWeights {
                    const PALLET: &'static str = "Emission0";
                    const CALL: &'static str = "set_weights";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DelegateWeightControl {
                    pub target: delegate_weight_control::Target,
                }
                pub mod delegate_weight_control {
                    use super::runtime_types;
                    pub type Target = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DelegateWeightControl {
                    const PALLET: &'static str = "Emission0";
                    const CALL: &'static str = "delegate_weight_control";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RegainWeightControl;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RegainWeightControl {
                    const PALLET: &'static str = "Emission0";
                    const CALL: &'static str = "regain_weight_control";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn set_weights(
                    &self,
                    weights: types::set_weights::Weights,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetWeights>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Emission0",
                        "set_weights",
                        types::SetWeights { weights },
                        [
                            99u8, 93u8, 160u8, 199u8, 77u8, 5u8, 193u8, 14u8, 138u8, 243u8, 85u8,
                            142u8, 156u8, 206u8, 128u8, 38u8, 64u8, 167u8, 111u8, 175u8, 23u8,
                            157u8, 219u8, 38u8, 190u8, 196u8, 214u8, 200u8, 98u8, 24u8, 71u8,
                            154u8,
                        ],
                    )
                }
                pub fn delegate_weight_control(
                    &self,
                    target: types::delegate_weight_control::Target,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::DelegateWeightControl,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Emission0",
                        "delegate_weight_control",
                        types::DelegateWeightControl { target },
                        [
                            79u8, 217u8, 227u8, 252u8, 174u8, 179u8, 228u8, 203u8, 57u8, 148u8,
                            247u8, 93u8, 171u8, 21u8, 8u8, 49u8, 238u8, 184u8, 166u8, 94u8, 227u8,
                            51u8, 205u8, 122u8, 210u8, 196u8, 48u8, 5u8, 228u8, 205u8, 226u8, 32u8,
                        ],
                    )
                }
                pub fn regain_weight_control(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RegainWeightControl>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Emission0",
                        "regain_weight_control",
                        types::RegainWeightControl {},
                        [
                            140u8, 239u8, 90u8, 15u8, 184u8, 224u8, 151u8, 91u8, 205u8, 189u8, 4u8,
                            127u8, 184u8, 250u8, 50u8, 63u8, 97u8, 239u8, 137u8, 135u8, 241u8,
                            250u8, 92u8, 78u8, 125u8, 19u8, 72u8, 60u8, 145u8, 19u8, 138u8, 220u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_emission0::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An agent set weights in the network."]
            pub struct WeightsSet(pub weights_set::Field0);
            pub mod weights_set {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for WeightsSet {
                const PALLET: &'static str = "Emission0";
                const EVENT: &'static str = "WeightsSet";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An agent gave weight control to the second agent."]
            pub struct DelegatedWeightControl(
                pub delegated_weight_control::Field0,
                pub delegated_weight_control::Field1,
            );
            pub mod delegated_weight_control {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Field1 = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DelegatedWeightControl {
                const PALLET: &'static str = "Emission0";
                const EVENT: &'static str = "DelegatedWeightControl";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod consensus_members {
                    use super::runtime_types;
                    pub type ConsensusMembers = runtime_types::pallet_emission0::ConsensusMember;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod weight_control_delegation {
                    use super::runtime_types;
                    pub type WeightControlDelegation = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod emission_recycling_percentage {
                    use super::runtime_types;
                    pub type EmissionRecyclingPercentage =
                        runtime_types::sp_arithmetic::per_things::Percent;
                }
                pub mod incentives_ratio {
                    use super::runtime_types;
                    pub type IncentivesRatio = runtime_types::sp_arithmetic::per_things::Percent;
                }
                pub mod pending_emission {
                    use super::runtime_types;
                    pub type PendingEmission = ::core::primitive::u128;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Map of consensus members indexed by their keys. A consensus member is"]
                #[doc = " any agent eligible for emissions in the next epoch. This means"]
                #[doc = " unregistered agents will also receive emissions."]
                pub fn consensus_members_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::consensus_members::ConsensusMembers,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Emission0",
                        "ConsensusMembers",
                        (),
                        [
                            15u8, 101u8, 129u8, 231u8, 175u8, 190u8, 183u8, 45u8, 136u8, 98u8,
                            190u8, 54u8, 144u8, 56u8, 245u8, 205u8, 71u8, 154u8, 87u8, 169u8,
                            126u8, 78u8, 58u8, 109u8, 95u8, 122u8, 177u8, 229u8, 87u8, 31u8, 172u8,
                            37u8,
                        ],
                    )
                }
                #[doc = " Map of consensus members indexed by their keys. A consensus member is"]
                #[doc = " any agent eligible for emissions in the next epoch. This means"]
                #[doc = " unregistered agents will also receive emissions."]
                pub fn consensus_members(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::consensus_members::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::consensus_members::Param0,
                    >,
                    types::consensus_members::ConsensusMembers,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Emission0",
                        "ConsensusMembers",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            15u8, 101u8, 129u8, 231u8, 175u8, 190u8, 183u8, 45u8, 136u8, 98u8,
                            190u8, 54u8, 144u8, 56u8, 245u8, 205u8, 71u8, 154u8, 87u8, 169u8,
                            126u8, 78u8, 58u8, 109u8, 95u8, 122u8, 177u8, 229u8, 87u8, 31u8, 172u8,
                            37u8,
                        ],
                    )
                }
                #[doc = " Map of agents delegating weight control to other agents. Emissions"]
                #[doc = " derived from weight delegation are taxed and the fees go the original"]
                #[doc = " weight setter."]
                pub fn weight_control_delegation_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::weight_control_delegation::WeightControlDelegation,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Emission0",
                        "WeightControlDelegation",
                        (),
                        [
                            189u8, 110u8, 175u8, 121u8, 145u8, 213u8, 232u8, 122u8, 54u8, 12u8,
                            173u8, 83u8, 17u8, 154u8, 227u8, 240u8, 79u8, 213u8, 87u8, 149u8,
                            135u8, 132u8, 37u8, 238u8, 205u8, 97u8, 241u8, 189u8, 80u8, 6u8, 3u8,
                            97u8,
                        ],
                    )
                }
                #[doc = " Map of agents delegating weight control to other agents. Emissions"]
                #[doc = " derived from weight delegation are taxed and the fees go the original"]
                #[doc = " weight setter."]
                pub fn weight_control_delegation(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::weight_control_delegation::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::weight_control_delegation::Param0,
                    >,
                    types::weight_control_delegation::WeightControlDelegation,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Emission0",
                        "WeightControlDelegation",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            189u8, 110u8, 175u8, 121u8, 145u8, 213u8, 232u8, 122u8, 54u8, 12u8,
                            173u8, 83u8, 17u8, 154u8, 227u8, 240u8, 79u8, 213u8, 87u8, 149u8,
                            135u8, 132u8, 37u8, 238u8, 205u8, 97u8, 241u8, 189u8, 80u8, 6u8, 3u8,
                            97u8,
                        ],
                    )
                }
                #[doc = " Percentage of issued tokens to be burned every epoch."]
                pub fn emission_recycling_percentage(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::emission_recycling_percentage::EmissionRecyclingPercentage,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Emission0",
                        "EmissionRecyclingPercentage",
                        (),
                        [
                            93u8, 103u8, 233u8, 213u8, 86u8, 61u8, 71u8, 69u8, 244u8, 52u8, 113u8,
                            83u8, 46u8, 26u8, 214u8, 179u8, 129u8, 174u8, 93u8, 162u8, 44u8, 36u8,
                            175u8, 96u8, 206u8, 101u8, 217u8, 100u8, 226u8, 4u8, 242u8, 255u8,
                        ],
                    )
                }
                #[doc = " Ratio between incentives and dividends on distribution. 50% means they"]
                #[doc = " are distributed equally."]
                pub fn incentives_ratio(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::incentives_ratio::IncentivesRatio,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Emission0",
                        "IncentivesRatio",
                        (),
                        [
                            56u8, 128u8, 249u8, 176u8, 193u8, 2u8, 84u8, 88u8, 235u8, 5u8, 206u8,
                            79u8, 196u8, 11u8, 138u8, 245u8, 8u8, 152u8, 173u8, 51u8, 72u8, 50u8,
                            44u8, 109u8, 158u8, 38u8, 15u8, 4u8, 124u8, 137u8, 7u8, 171u8,
                        ],
                    )
                }
                #[doc = " Amount of tokens accumulated since the last epoch. This increases on"]
                #[doc = " every block. See [`distribute::get_total_emission_per_block`]."]
                pub fn pending_emission(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pending_emission::PendingEmission,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Emission0",
                        "PendingEmission",
                        (),
                        [
                            36u8, 111u8, 76u8, 130u8, 5u8, 228u8, 17u8, 101u8, 55u8, 92u8, 18u8,
                            131u8, 151u8, 228u8, 86u8, 36u8, 98u8, 154u8, 101u8, 59u8, 5u8, 59u8,
                            255u8, 28u8, 35u8, 43u8, 41u8, 14u8, 153u8, 49u8, 131u8, 128u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Tokens emitted in an interval before halving the emissions in NANOs."]
                pub fn halving_interval(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::num::NonZeroU128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Emission0",
                        "HalvingInterval",
                        [
                            62u8, 145u8, 102u8, 227u8, 159u8, 92u8, 27u8, 54u8, 159u8, 228u8,
                            193u8, 99u8, 75u8, 196u8, 26u8, 250u8, 229u8, 230u8, 88u8, 109u8,
                            246u8, 100u8, 152u8, 158u8, 14u8, 25u8, 224u8, 173u8, 224u8, 41u8,
                            105u8, 231u8,
                        ],
                    )
                }
                #[doc = " Max token supply in NANOs."]
                pub fn max_supply(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::num::NonZeroU128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Emission0",
                        "MaxSupply",
                        [
                            62u8, 145u8, 102u8, 227u8, 159u8, 92u8, 27u8, 54u8, 159u8, 228u8,
                            193u8, 99u8, 75u8, 196u8, 26u8, 250u8, 229u8, 230u8, 88u8, 109u8,
                            246u8, 100u8, 152u8, 158u8, 14u8, 25u8, 224u8, 173u8, 224u8, 41u8,
                            105u8, 231u8,
                        ],
                    )
                }
                #[doc = " Emissions per block in NANOs. Not taking into account halving and"]
                #[doc = " recycling."]
                pub fn block_emission(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Emission0",
                        "BlockEmission",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn default_emission_recycling_percentage(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_arithmetic::per_things::Percent,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Emission0",
                        "DefaultEmissionRecyclingPercentage",
                        [
                            40u8, 171u8, 69u8, 196u8, 34u8, 184u8, 50u8, 128u8, 139u8, 192u8, 63u8,
                            231u8, 249u8, 200u8, 252u8, 73u8, 244u8, 170u8, 51u8, 177u8, 106u8,
                            47u8, 114u8, 234u8, 84u8, 104u8, 62u8, 118u8, 227u8, 50u8, 225u8,
                            122u8,
                        ],
                    )
                }
                pub fn default_incentives_ratio(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_arithmetic::per_things::Percent,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Emission0",
                        "DefaultIncentivesRatio",
                        [
                            40u8, 171u8, 69u8, 196u8, 34u8, 184u8, 50u8, 128u8, 139u8, 192u8, 63u8,
                            231u8, 249u8, 200u8, 252u8, 73u8, 244u8, 170u8, 51u8, 177u8, 106u8,
                            47u8, 114u8, 234u8, 84u8, 104u8, 62u8, 118u8, 227u8, 50u8, 225u8,
                            122u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod permission0 {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_permission0::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_permission0::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Grant a permission for emission delegation"]
                pub struct GrantEmissionPermission {
                    pub grantee: grant_emission_permission::Grantee,
                    pub allocation: grant_emission_permission::Allocation,
                    pub targets: grant_emission_permission::Targets,
                    pub distribution: grant_emission_permission::Distribution,
                    pub duration: grant_emission_permission::Duration,
                    pub revocation: grant_emission_permission::Revocation,
                    pub enforcement: grant_emission_permission::Enforcement,
                }
                pub mod grant_emission_permission {
                    use super::runtime_types;
                    pub type Grantee = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Allocation =
                        runtime_types::pallet_permission0::permission::emission::EmissionAllocation;
                    pub type Targets = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u16,
                    )>;
                    pub type Distribution = runtime_types :: pallet_permission0 :: permission :: emission :: DistributionControl ;
                    pub type Duration =
                        runtime_types::pallet_permission0::permission::PermissionDuration;
                    pub type Revocation =
                        runtime_types::pallet_permission0::permission::RevocationTerms;
                    pub type Enforcement =
                        runtime_types::pallet_permission0::permission::EnforcementAuthority;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for GrantEmissionPermission {
                    const PALLET: &'static str = "Permission0";
                    const CALL: &'static str = "grant_emission_permission";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Revoke a permission. The caller must met revocation constraints or be a root key."]
                pub struct RevokePermission {
                    pub permission_id: revoke_permission::PermissionId,
                }
                pub mod revoke_permission {
                    use super::runtime_types;
                    pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RevokePermission {
                    const PALLET: &'static str = "Permission0";
                    const CALL: &'static str = "revoke_permission";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Execute a manual distribution based on permission"]
                pub struct ExecutePermission {
                    pub permission_id: execute_permission::PermissionId,
                }
                pub mod execute_permission {
                    use super::runtime_types;
                    pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ExecutePermission {
                    const PALLET: &'static str = "Permission0";
                    const CALL: &'static str = "execute_permission";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Toggle a permission's accumulation state (enabled/disabled)"]
                #[doc = "The caller must be authorized as a controller or be the root key"]
                pub struct TogglePermissionAccumulation {
                    pub permission_id: toggle_permission_accumulation::PermissionId,
                    pub accumulating: toggle_permission_accumulation::Accumulating,
                }
                pub mod toggle_permission_accumulation {
                    use super::runtime_types;
                    pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                    pub type Accumulating = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TogglePermissionAccumulation {
                    const PALLET: &'static str = "Permission0";
                    const CALL: &'static str = "toggle_permission_accumulation";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Execute a permission through enforcement authority"]
                #[doc = "The caller must be authorized as a controller or be the root key"]
                pub struct EnforcementExecutePermission {
                    pub permission_id: enforcement_execute_permission::PermissionId,
                }
                pub mod enforcement_execute_permission {
                    use super::runtime_types;
                    pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for EnforcementExecutePermission {
                    const PALLET: &'static str = "Permission0";
                    const CALL: &'static str = "enforcement_execute_permission";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set enforcement authority for a permission"]
                #[doc = "Only the grantor or root can set enforcement authority"]
                pub struct SetEnforcementAuthority {
                    pub permission_id: set_enforcement_authority::PermissionId,
                    pub controllers: set_enforcement_authority::Controllers,
                    pub required_votes: set_enforcement_authority::RequiredVotes,
                }
                pub mod set_enforcement_authority {
                    use super::runtime_types;
                    pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                    pub type Controllers = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type RequiredVotes = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetEnforcementAuthority {
                    const PALLET: &'static str = "Permission0";
                    const CALL: &'static str = "set_enforcement_authority";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Grant a permission for curator delegation"]
                pub struct GrantCuratorPermission {
                    pub grantee: grant_curator_permission::Grantee,
                    pub flags: grant_curator_permission::Flags,
                    pub cooldown: grant_curator_permission::Cooldown,
                    pub duration: grant_curator_permission::Duration,
                    pub revocation: grant_curator_permission::Revocation,
                }
                pub mod grant_curator_permission {
                    use super::runtime_types;
                    pub type Grantee = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Flags = ::core::primitive::u32;
                    pub type Cooldown = ::core::option::Option<::core::primitive::u64>;
                    pub type Duration =
                        runtime_types::pallet_permission0::permission::PermissionDuration;
                    pub type Revocation =
                        runtime_types::pallet_permission0::permission::RevocationTerms;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for GrantCuratorPermission {
                    const PALLET: &'static str = "Permission0";
                    const CALL: &'static str = "grant_curator_permission";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Grant a permission for emission delegation"]
                pub fn grant_emission_permission(
                    &self,
                    grantee: types::grant_emission_permission::Grantee,
                    allocation: types::grant_emission_permission::Allocation,
                    targets: types::grant_emission_permission::Targets,
                    distribution: types::grant_emission_permission::Distribution,
                    duration: types::grant_emission_permission::Duration,
                    revocation: types::grant_emission_permission::Revocation,
                    enforcement: types::grant_emission_permission::Enforcement,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::GrantEmissionPermission,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Permission0",
                        "grant_emission_permission",
                        types::GrantEmissionPermission {
                            grantee,
                            allocation,
                            targets,
                            distribution,
                            duration,
                            revocation,
                            enforcement,
                        },
                        [
                            129u8, 121u8, 87u8, 63u8, 18u8, 3u8, 93u8, 199u8, 77u8, 255u8, 153u8,
                            58u8, 161u8, 162u8, 230u8, 53u8, 203u8, 18u8, 156u8, 211u8, 2u8, 112u8,
                            253u8, 144u8, 87u8, 233u8, 214u8, 106u8, 155u8, 112u8, 72u8, 226u8,
                        ],
                    )
                }
                #[doc = "Revoke a permission. The caller must met revocation constraints or be a root key."]
                pub fn revoke_permission(
                    &self,
                    permission_id: types::revoke_permission::PermissionId,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RevokePermission>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Permission0",
                        "revoke_permission",
                        types::RevokePermission { permission_id },
                        [
                            242u8, 35u8, 37u8, 187u8, 105u8, 214u8, 188u8, 52u8, 64u8, 4u8, 170u8,
                            247u8, 49u8, 169u8, 159u8, 203u8, 90u8, 92u8, 245u8, 214u8, 115u8,
                            251u8, 170u8, 114u8, 85u8, 31u8, 27u8, 62u8, 143u8, 29u8, 180u8, 24u8,
                        ],
                    )
                }
                #[doc = "Execute a manual distribution based on permission"]
                pub fn execute_permission(
                    &self,
                    permission_id: types::execute_permission::PermissionId,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ExecutePermission>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Permission0",
                        "execute_permission",
                        types::ExecutePermission { permission_id },
                        [
                            68u8, 161u8, 232u8, 52u8, 8u8, 155u8, 222u8, 131u8, 153u8, 76u8, 160u8,
                            77u8, 67u8, 205u8, 144u8, 134u8, 72u8, 210u8, 83u8, 233u8, 217u8,
                            203u8, 91u8, 127u8, 214u8, 3u8, 129u8, 25u8, 129u8, 109u8, 213u8,
                            230u8,
                        ],
                    )
                }
                #[doc = "Toggle a permission's accumulation state (enabled/disabled)"]
                #[doc = "The caller must be authorized as a controller or be the root key"]
                pub fn toggle_permission_accumulation(
                    &self,
                    permission_id: types::toggle_permission_accumulation::PermissionId,
                    accumulating: types::toggle_permission_accumulation::Accumulating,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::TogglePermissionAccumulation,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Permission0",
                        "toggle_permission_accumulation",
                        types::TogglePermissionAccumulation {
                            permission_id,
                            accumulating,
                        },
                        [
                            221u8, 252u8, 210u8, 85u8, 61u8, 249u8, 146u8, 238u8, 161u8, 166u8,
                            132u8, 201u8, 78u8, 37u8, 142u8, 185u8, 84u8, 93u8, 71u8, 170u8, 153u8,
                            247u8, 117u8, 168u8, 10u8, 232u8, 12u8, 102u8, 87u8, 197u8, 152u8,
                            24u8,
                        ],
                    )
                }
                #[doc = "Execute a permission through enforcement authority"]
                #[doc = "The caller must be authorized as a controller or be the root key"]
                pub fn enforcement_execute_permission(
                    &self,
                    permission_id: types::enforcement_execute_permission::PermissionId,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::EnforcementExecutePermission,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Permission0",
                        "enforcement_execute_permission",
                        types::EnforcementExecutePermission { permission_id },
                        [
                            73u8, 200u8, 158u8, 114u8, 4u8, 15u8, 33u8, 43u8, 81u8, 84u8, 137u8,
                            193u8, 24u8, 102u8, 159u8, 3u8, 202u8, 172u8, 16u8, 17u8, 84u8, 203u8,
                            39u8, 192u8, 5u8, 70u8, 108u8, 33u8, 77u8, 42u8, 99u8, 119u8,
                        ],
                    )
                }
                #[doc = "Set enforcement authority for a permission"]
                #[doc = "Only the grantor or root can set enforcement authority"]
                pub fn set_enforcement_authority(
                    &self,
                    permission_id: types::set_enforcement_authority::PermissionId,
                    controllers: types::set_enforcement_authority::Controllers,
                    required_votes: types::set_enforcement_authority::RequiredVotes,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::SetEnforcementAuthority,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Permission0",
                        "set_enforcement_authority",
                        types::SetEnforcementAuthority {
                            permission_id,
                            controllers,
                            required_votes,
                        },
                        [
                            111u8, 35u8, 118u8, 195u8, 9u8, 131u8, 141u8, 47u8, 202u8, 198u8,
                            208u8, 192u8, 30u8, 191u8, 115u8, 227u8, 86u8, 193u8, 243u8, 42u8,
                            29u8, 111u8, 171u8, 161u8, 117u8, 139u8, 106u8, 109u8, 115u8, 51u8,
                            133u8, 250u8,
                        ],
                    )
                }
                #[doc = "Grant a permission for curator delegation"]
                pub fn grant_curator_permission(
                    &self,
                    grantee: types::grant_curator_permission::Grantee,
                    flags: types::grant_curator_permission::Flags,
                    cooldown: types::grant_curator_permission::Cooldown,
                    duration: types::grant_curator_permission::Duration,
                    revocation: types::grant_curator_permission::Revocation,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::GrantCuratorPermission,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Permission0",
                        "grant_curator_permission",
                        types::GrantCuratorPermission {
                            grantee,
                            flags,
                            cooldown,
                            duration,
                            revocation,
                        },
                        [
                            33u8, 92u8, 243u8, 122u8, 66u8, 9u8, 242u8, 78u8, 61u8, 245u8, 4u8,
                            170u8, 161u8, 143u8, 92u8, 133u8, 10u8, 82u8, 179u8, 242u8, 101u8,
                            253u8, 18u8, 36u8, 212u8, 22u8, 144u8, 248u8, 158u8, 103u8, 203u8,
                            237u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_permission0::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Permission granted from grantor to grantee with ID"]
            pub struct PermissionGranted {
                pub grantor: permission_granted::Grantor,
                pub grantee: permission_granted::Grantee,
                pub permission_id: permission_granted::PermissionId,
            }
            pub mod permission_granted {
                use super::runtime_types;
                pub type Grantor = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Grantee = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PermissionGranted {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "PermissionGranted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Permission revoked with ID"]
            pub struct PermissionRevoked {
                pub grantor: permission_revoked::Grantor,
                pub grantee: permission_revoked::Grantee,
                pub revoked_by: permission_revoked::RevokedBy,
                pub permission_id: permission_revoked::PermissionId,
            }
            pub mod permission_revoked {
                use super::runtime_types;
                pub type Grantor = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Grantee = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type RevokedBy =
                    ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PermissionRevoked {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "PermissionRevoked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Permission executed (manual distribution) with ID"]
            pub struct PermissionExecuted {
                pub grantor: permission_executed::Grantor,
                pub grantee: permission_executed::Grantee,
                pub permission_id: permission_executed::PermissionId,
                pub stream_id: permission_executed::StreamId,
                pub amount: permission_executed::Amount,
            }
            pub mod permission_executed {
                use super::runtime_types;
                pub type Grantor = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Grantee = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                pub type StreamId = ::core::option::Option<::subxt::ext::subxt_core::utils::H256>;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PermissionExecuted {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "PermissionExecuted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Auto-distribution executed"]
            pub struct AutoDistributionExecuted {
                pub grantor: auto_distribution_executed::Grantor,
                pub grantee: auto_distribution_executed::Grantee,
                pub permission_id: auto_distribution_executed::PermissionId,
                pub stream_id: auto_distribution_executed::StreamId,
                pub amount: auto_distribution_executed::Amount,
            }
            pub mod auto_distribution_executed {
                use super::runtime_types;
                pub type Grantor = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Grantee = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                pub type StreamId = ::core::option::Option<::subxt::ext::subxt_core::utils::H256>;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for AutoDistributionExecuted {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "AutoDistributionExecuted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Permission expired with ID"]
            pub struct PermissionExpired {
                pub grantor: permission_expired::Grantor,
                pub grantee: permission_expired::Grantee,
                pub permission_id: permission_expired::PermissionId,
            }
            pub mod permission_expired {
                use super::runtime_types;
                pub type Grantor = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Grantee = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PermissionExpired {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "PermissionExpired";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Permission accumulation state toggled"]
            pub struct PermissionAccumulationToggled {
                pub permission_id: permission_accumulation_toggled::PermissionId,
                pub accumulating: permission_accumulation_toggled::Accumulating,
                pub toggled_by: permission_accumulation_toggled::ToggledBy,
            }
            pub mod permission_accumulation_toggled {
                use super::runtime_types;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                pub type Accumulating = ::core::primitive::bool;
                pub type ToggledBy =
                    ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PermissionAccumulationToggled {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "PermissionAccumulationToggled";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Permission was executed by enforcement authority"]
            pub struct PermissionEnforcementExecuted {
                pub permission_id: permission_enforcement_executed::PermissionId,
                pub executed_by: permission_enforcement_executed::ExecutedBy,
            }
            pub mod permission_enforcement_executed {
                use super::runtime_types;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                pub type ExecutedBy =
                    ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PermissionEnforcementExecuted {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "PermissionEnforcementExecuted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Vote for enforcement action"]
            pub struct EnforcementVoteCast {
                pub permission_id: enforcement_vote_cast::PermissionId,
                pub voter: enforcement_vote_cast::Voter,
                pub referendum: enforcement_vote_cast::Referendum,
            }
            pub mod enforcement_vote_cast {
                use super::runtime_types;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                pub type Voter = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Referendum =
                    runtime_types::pallet_permission0::permission::EnforcementReferendum;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for EnforcementVoteCast {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "EnforcementVoteCast";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Enforcement authority set for permission"]
            pub struct EnforcementAuthoritySet {
                pub permission_id: enforcement_authority_set::PermissionId,
                pub controllers_count: enforcement_authority_set::ControllersCount,
                pub required_votes: enforcement_authority_set::RequiredVotes,
            }
            pub mod enforcement_authority_set {
                use super::runtime_types;
                pub type PermissionId = ::subxt::ext::subxt_core::utils::H256;
                pub type ControllersCount = ::core::primitive::u32;
                pub type RequiredVotes = ::core::primitive::u32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for EnforcementAuthoritySet {
                const PALLET: &'static str = "Permission0";
                const EVENT: &'static str = "EnforcementAuthoritySet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod permissions {
                    use super::runtime_types;
                    pub type Permissions =
                        runtime_types::pallet_permission0::permission::PermissionContract;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod permissions_by_participants {
                    use super::runtime_types;
                    pub type PermissionsByParticipants =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param1 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod permissions_by_grantor {
                    use super::runtime_types;
                    pub type PermissionsByGrantor =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod permissions_by_grantee {
                    use super::runtime_types;
                    pub type PermissionsByGrantee =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::H256,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod revocation_tracking {
                    use super::runtime_types;
                    pub type RevocationTracking =
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod enforcement_tracking {
                    use super::runtime_types;
                    pub type EnforcementTracking =
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param1 =
                        runtime_types::pallet_permission0::permission::EnforcementReferendum;
                }
                pub mod accumulated_stream_amounts {
                    use super::runtime_types;
                    pub type AccumulatedStreamAmounts = ::core::primitive::u128;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param1 = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param2 = ::subxt::ext::subxt_core::utils::H256;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Active permission contracts - stored by permission ID"]
                pub fn permissions_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::permissions::Permissions,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "Permissions",
                        (),
                        [
                            15u8, 210u8, 0u8, 131u8, 219u8, 134u8, 121u8, 227u8, 120u8, 28u8,
                            151u8, 34u8, 220u8, 186u8, 122u8, 121u8, 39u8, 54u8, 195u8, 6u8, 91u8,
                            182u8, 69u8, 230u8, 5u8, 222u8, 212u8, 181u8, 53u8, 195u8, 22u8, 136u8,
                        ],
                    )
                }
                #[doc = " Active permission contracts - stored by permission ID"]
                pub fn permissions(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::permissions::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::permissions::Param0,
                    >,
                    types::permissions::Permissions,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "Permissions",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            15u8, 210u8, 0u8, 131u8, 219u8, 134u8, 121u8, 227u8, 120u8, 28u8,
                            151u8, 34u8, 220u8, 186u8, 122u8, 121u8, 39u8, 54u8, 195u8, 6u8, 91u8,
                            182u8, 69u8, 230u8, 5u8, 222u8, 212u8, 181u8, 53u8, 195u8, 22u8, 136u8,
                        ],
                    )
                }
                #[doc = " Mapping from (grantor, grantee) to permission IDs"]
                pub fn permissions_by_participants_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::permissions_by_participants::PermissionsByParticipants,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "PermissionsByParticipants",
                        (),
                        [
                            51u8, 71u8, 155u8, 4u8, 227u8, 66u8, 196u8, 202u8, 106u8, 10u8, 87u8,
                            187u8, 88u8, 27u8, 210u8, 241u8, 241u8, 79u8, 186u8, 43u8, 142u8,
                            121u8, 21u8, 73u8, 162u8, 207u8, 20u8, 130u8, 248u8, 207u8, 168u8,
                            180u8,
                        ],
                    )
                }
                #[doc = " Mapping from (grantor, grantee) to permission IDs"]
                pub fn permissions_by_participants_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::permissions_by_participants::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::permissions_by_participants::Param0,
                    >,
                    types::permissions_by_participants::PermissionsByParticipants,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "PermissionsByParticipants",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            51u8, 71u8, 155u8, 4u8, 227u8, 66u8, 196u8, 202u8, 106u8, 10u8, 87u8,
                            187u8, 88u8, 27u8, 210u8, 241u8, 241u8, 79u8, 186u8, 43u8, 142u8,
                            121u8, 21u8, 73u8, 162u8, 207u8, 20u8, 130u8, 248u8, 207u8, 168u8,
                            180u8,
                        ],
                    )
                }
                #[doc = " Mapping from (grantor, grantee) to permission IDs"]
                pub fn permissions_by_participants(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::permissions_by_participants::Param0>,
                    _1: impl ::core::borrow::Borrow<types::permissions_by_participants::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::permissions_by_participants::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::permissions_by_participants::Param1,
                        >,
                    ),
                    types::permissions_by_participants::PermissionsByParticipants,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "PermissionsByParticipants",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            51u8, 71u8, 155u8, 4u8, 227u8, 66u8, 196u8, 202u8, 106u8, 10u8, 87u8,
                            187u8, 88u8, 27u8, 210u8, 241u8, 241u8, 79u8, 186u8, 43u8, 142u8,
                            121u8, 21u8, 73u8, 162u8, 207u8, 20u8, 130u8, 248u8, 207u8, 168u8,
                            180u8,
                        ],
                    )
                }
                #[doc = " Permissions granted by a specific account"]
                pub fn permissions_by_grantor_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::permissions_by_grantor::PermissionsByGrantor,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "PermissionsByGrantor",
                        (),
                        [
                            184u8, 69u8, 253u8, 222u8, 225u8, 86u8, 72u8, 121u8, 91u8, 237u8, 71u8,
                            45u8, 27u8, 20u8, 154u8, 98u8, 134u8, 120u8, 250u8, 146u8, 198u8, 87u8,
                            83u8, 162u8, 70u8, 183u8, 2u8, 75u8, 41u8, 102u8, 81u8, 116u8,
                        ],
                    )
                }
                #[doc = " Permissions granted by a specific account"]
                pub fn permissions_by_grantor(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::permissions_by_grantor::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::permissions_by_grantor::Param0,
                    >,
                    types::permissions_by_grantor::PermissionsByGrantor,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "PermissionsByGrantor",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            184u8, 69u8, 253u8, 222u8, 225u8, 86u8, 72u8, 121u8, 91u8, 237u8, 71u8,
                            45u8, 27u8, 20u8, 154u8, 98u8, 134u8, 120u8, 250u8, 146u8, 198u8, 87u8,
                            83u8, 162u8, 70u8, 183u8, 2u8, 75u8, 41u8, 102u8, 81u8, 116u8,
                        ],
                    )
                }
                #[doc = " Permissions received by a specific account"]
                pub fn permissions_by_grantee_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::permissions_by_grantee::PermissionsByGrantee,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "PermissionsByGrantee",
                        (),
                        [
                            203u8, 197u8, 206u8, 216u8, 167u8, 86u8, 180u8, 210u8, 47u8, 175u8,
                            207u8, 137u8, 116u8, 187u8, 93u8, 52u8, 231u8, 44u8, 128u8, 22u8,
                            198u8, 212u8, 24u8, 194u8, 255u8, 237u8, 59u8, 10u8, 49u8, 99u8, 156u8,
                            168u8,
                        ],
                    )
                }
                #[doc = " Permissions received by a specific account"]
                pub fn permissions_by_grantee(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::permissions_by_grantee::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::permissions_by_grantee::Param0,
                    >,
                    types::permissions_by_grantee::PermissionsByGrantee,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "PermissionsByGrantee",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            203u8, 197u8, 206u8, 216u8, 167u8, 86u8, 180u8, 210u8, 47u8, 175u8,
                            207u8, 137u8, 116u8, 187u8, 93u8, 52u8, 231u8, 44u8, 128u8, 22u8,
                            198u8, 212u8, 24u8, 194u8, 255u8, 237u8, 59u8, 10u8, 49u8, 99u8, 156u8,
                            168u8,
                        ],
                    )
                }
                #[doc = " Revocations in progress and the voters"]
                pub fn revocation_tracking_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::revocation_tracking::RevocationTracking,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "RevocationTracking",
                        (),
                        [
                            101u8, 190u8, 75u8, 9u8, 43u8, 101u8, 12u8, 47u8, 93u8, 79u8, 13u8,
                            31u8, 221u8, 205u8, 96u8, 234u8, 136u8, 190u8, 242u8, 102u8, 0u8,
                            172u8, 129u8, 133u8, 141u8, 215u8, 91u8, 218u8, 228u8, 88u8, 38u8,
                            116u8,
                        ],
                    )
                }
                #[doc = " Revocations in progress and the voters"]
                pub fn revocation_tracking(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::revocation_tracking::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::revocation_tracking::Param0,
                    >,
                    types::revocation_tracking::RevocationTracking,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "RevocationTracking",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            101u8, 190u8, 75u8, 9u8, 43u8, 101u8, 12u8, 47u8, 93u8, 79u8, 13u8,
                            31u8, 221u8, 205u8, 96u8, 234u8, 136u8, 190u8, 242u8, 102u8, 0u8,
                            172u8, 129u8, 133u8, 141u8, 215u8, 91u8, 218u8, 228u8, 88u8, 38u8,
                            116u8,
                        ],
                    )
                }
                #[doc = " Enforcement votes in progress and the voters"]
                pub fn enforcement_tracking_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::enforcement_tracking::EnforcementTracking,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "EnforcementTracking",
                        (),
                        [
                            165u8, 93u8, 40u8, 0u8, 140u8, 17u8, 101u8, 215u8, 156u8, 46u8, 201u8,
                            248u8, 165u8, 231u8, 143u8, 71u8, 116u8, 40u8, 32u8, 107u8, 2u8, 85u8,
                            94u8, 141u8, 155u8, 228u8, 123u8, 49u8, 200u8, 203u8, 131u8, 57u8,
                        ],
                    )
                }
                #[doc = " Enforcement votes in progress and the voters"]
                pub fn enforcement_tracking_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::enforcement_tracking::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::enforcement_tracking::Param0,
                    >,
                    types::enforcement_tracking::EnforcementTracking,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "EnforcementTracking",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            165u8, 93u8, 40u8, 0u8, 140u8, 17u8, 101u8, 215u8, 156u8, 46u8, 201u8,
                            248u8, 165u8, 231u8, 143u8, 71u8, 116u8, 40u8, 32u8, 107u8, 2u8, 85u8,
                            94u8, 141u8, 155u8, 228u8, 123u8, 49u8, 200u8, 203u8, 131u8, 57u8,
                        ],
                    )
                }
                #[doc = " Enforcement votes in progress and the voters"]
                pub fn enforcement_tracking(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::enforcement_tracking::Param0>,
                    _1: impl ::core::borrow::Borrow<types::enforcement_tracking::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::enforcement_tracking::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::enforcement_tracking::Param1,
                        >,
                    ),
                    types::enforcement_tracking::EnforcementTracking,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "EnforcementTracking",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            165u8, 93u8, 40u8, 0u8, 140u8, 17u8, 101u8, 215u8, 156u8, 46u8, 201u8,
                            248u8, 165u8, 231u8, 143u8, 71u8, 116u8, 40u8, 32u8, 107u8, 2u8, 85u8,
                            94u8, 141u8, 155u8, 228u8, 123u8, 49u8, 200u8, 203u8, 131u8, 57u8,
                        ],
                    )
                }
                #[doc = " Accumulated amounts for each stream"]
                pub fn accumulated_stream_amounts_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::accumulated_stream_amounts::AccumulatedStreamAmounts,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "AccumulatedStreamAmounts",
                        (),
                        [
                            120u8, 155u8, 99u8, 84u8, 16u8, 116u8, 215u8, 254u8, 204u8, 92u8, 62u8,
                            104u8, 62u8, 120u8, 220u8, 150u8, 7u8, 239u8, 195u8, 200u8, 103u8,
                            132u8, 31u8, 237u8, 248u8, 190u8, 154u8, 207u8, 56u8, 134u8, 117u8,
                            192u8,
                        ],
                    )
                }
                #[doc = " Accumulated amounts for each stream"]
                pub fn accumulated_stream_amounts_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::accumulated_stream_amounts::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::accumulated_stream_amounts::Param0,
                    >,
                    types::accumulated_stream_amounts::AccumulatedStreamAmounts,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "AccumulatedStreamAmounts",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            120u8, 155u8, 99u8, 84u8, 16u8, 116u8, 215u8, 254u8, 204u8, 92u8, 62u8,
                            104u8, 62u8, 120u8, 220u8, 150u8, 7u8, 239u8, 195u8, 200u8, 103u8,
                            132u8, 31u8, 237u8, 248u8, 190u8, 154u8, 207u8, 56u8, 134u8, 117u8,
                            192u8,
                        ],
                    )
                }
                #[doc = " Accumulated amounts for each stream"]
                pub fn accumulated_stream_amounts_iter2(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::accumulated_stream_amounts::Param0>,
                    _1: impl ::core::borrow::Borrow<types::accumulated_stream_amounts::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::accumulated_stream_amounts::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::accumulated_stream_amounts::Param1,
                        >,
                    ),
                    types::accumulated_stream_amounts::AccumulatedStreamAmounts,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "AccumulatedStreamAmounts",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            120u8, 155u8, 99u8, 84u8, 16u8, 116u8, 215u8, 254u8, 204u8, 92u8, 62u8,
                            104u8, 62u8, 120u8, 220u8, 150u8, 7u8, 239u8, 195u8, 200u8, 103u8,
                            132u8, 31u8, 237u8, 248u8, 190u8, 154u8, 207u8, 56u8, 134u8, 117u8,
                            192u8,
                        ],
                    )
                }
                #[doc = " Accumulated amounts for each stream"]
                pub fn accumulated_stream_amounts(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::accumulated_stream_amounts::Param0>,
                    _1: impl ::core::borrow::Borrow<types::accumulated_stream_amounts::Param1>,
                    _2: impl ::core::borrow::Borrow<types::accumulated_stream_amounts::Param2>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::accumulated_stream_amounts::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::accumulated_stream_amounts::Param1,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::accumulated_stream_amounts::Param2,
                        >,
                    ),
                    types::accumulated_stream_amounts::AccumulatedStreamAmounts,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Permission0",
                        "AccumulatedStreamAmounts",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _2.borrow(),
                            ),
                        ),
                        [
                            120u8, 155u8, 99u8, 84u8, 16u8, 116u8, 215u8, 254u8, 204u8, 92u8, 62u8,
                            104u8, 62u8, 120u8, 220u8, 150u8, 7u8, 239u8, 195u8, 200u8, 103u8,
                            132u8, 31u8, 237u8, 248u8, 190u8, 154u8, 207u8, 56u8, 134u8, 117u8,
                            192u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Permission0 pallet ID"]
                pub fn pallet_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_support::PalletId,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Permission0",
                        "PalletId",
                        [
                            56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
                            161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
                            129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
                        ],
                    )
                }
                #[doc = " Maximum number of targets per permission."]
                pub fn max_targets_per_permission(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Permission0",
                        "MaxTargetsPerPermission",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of delegated streams per permission."]
                pub fn max_streams_per_permission(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Permission0",
                        "MaxStreamsPerPermission",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of revokers."]
                pub fn max_revokers_per_permission(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Permission0",
                        "MaxRevokersPerPermission",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of controllers per permission."]
                pub fn max_controllers_per_permission(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Permission0",
                        "MaxControllersPerPermission",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Minimum threshold for auto-distribution"]
                pub fn min_auto_distribution_threshold(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u128,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Permission0",
                        "MinAutoDistributionThreshold",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod faucet {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Errors that can occur in the faucet pallet"]
        pub type Error = runtime_types::pallet_faucet::pallet::Error;
        #[doc = "Callable functions for the faucet pallet"]
        pub type Call = runtime_types::pallet_faucet::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Request tokens from the faucet by performing proof of work"]
                #[doc = ""]
                #[doc = "This extrinsic is only available on testnets. It requires the user to perform"]
                #[doc = "proof-of-work by finding a nonce that, when combined with a recent block hash"]
                #[doc = "and the user's account ID, produces a hash that meets the difficulty requirement."]
                #[doc = ""]
                #[doc = "The account must have a total balance (free + staked) below the threshold to be eligible."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = "* `origin` - Must be None (unsigned)"]
                #[doc = "* `block_number` - A recent block number (within 3 blocks)"]
                #[doc = "* `nonce` - A value that makes the resulting hash meet the difficulty requirement"]
                #[doc = "* `work` - The hash result of the proof of work"]
                #[doc = "* `key` - The account ID that will receive the tokens"]
                #[doc = ""]
                #[doc = "# Weight"]
                #[doc = "* Read operations: 16"]
                #[doc = "* Write operations: 28"]
                #[doc = "* Does not pay fees"]
                pub struct Faucet {
                    pub block_number: faucet::BlockNumber,
                    pub nonce: faucet::Nonce,
                    pub work: faucet::Work,
                    pub key: faucet::Key,
                }
                pub mod faucet {
                    use super::runtime_types;
                    pub type BlockNumber = ::core::primitive::u64;
                    pub type Nonce = ::core::primitive::u64;
                    pub type Work =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Key = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Faucet {
                    const PALLET: &'static str = "Faucet";
                    const CALL: &'static str = "faucet";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Request tokens from the faucet by performing proof of work"]
                #[doc = ""]
                #[doc = "This extrinsic is only available on testnets. It requires the user to perform"]
                #[doc = "proof-of-work by finding a nonce that, when combined with a recent block hash"]
                #[doc = "and the user's account ID, produces a hash that meets the difficulty requirement."]
                #[doc = ""]
                #[doc = "The account must have a total balance (free + staked) below the threshold to be eligible."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = "* `origin` - Must be None (unsigned)"]
                #[doc = "* `block_number` - A recent block number (within 3 blocks)"]
                #[doc = "* `nonce` - A value that makes the resulting hash meet the difficulty requirement"]
                #[doc = "* `work` - The hash result of the proof of work"]
                #[doc = "* `key` - The account ID that will receive the tokens"]
                #[doc = ""]
                #[doc = "# Weight"]
                #[doc = "* Read operations: 16"]
                #[doc = "* Write operations: 28"]
                #[doc = "* Does not pay fees"]
                pub fn faucet(
                    &self,
                    block_number: types::faucet::BlockNumber,
                    nonce: types::faucet::Nonce,
                    work: types::faucet::Work,
                    key: types::faucet::Key,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Faucet>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Faucet",
                        "faucet",
                        types::Faucet {
                            block_number,
                            nonce,
                            work,
                            key,
                        },
                        [
                            190u8, 190u8, 250u8, 200u8, 107u8, 30u8, 125u8, 226u8, 52u8, 30u8,
                            50u8, 158u8, 77u8, 159u8, 189u8, 192u8, 213u8, 18u8, 111u8, 40u8,
                            145u8, 8u8, 11u8, 178u8, 50u8, 209u8, 195u8, 72u8, 231u8, 181u8, 198u8,
                            59u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Events emitted by the faucet pallet"]
        pub type Event = runtime_types::pallet_faucet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Tokens were successfully distributed by the faucet"]
            #[doc = "[account, amount]"]
            pub struct Faucet(pub faucet::Field0, pub faucet::Field1);
            pub mod faucet {
                use super::runtime_types;
                pub type Field0 = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Field1 = ::core::primitive::u128;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Faucet {
                const PALLET: &'static str = "Faucet";
                const EVENT: &'static str = "Faucet";
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_btree_map {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedBTreeMap<_0, _1>(
                    pub ::subxt::ext::subxt_core::utils::KeyedVec<_0, _1>,
                );
            }
            pub mod bounded_btree_set {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedBTreeSet<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeakBoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
        }
        pub mod ethbloom {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Bloom(pub [::core::primitive::u8; 256usize]);
        }
        pub mod ethereum {
            use super::runtime_types;
            pub mod block {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Block<_0> {
                    pub header: runtime_types::ethereum::header::Header,
                    pub transactions: ::subxt::ext::subxt_core::alloc::vec::Vec<_0>,
                    pub ommers: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::ethereum::header::Header,
                    >,
                }
            }
            pub mod header {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Header {
                    pub parent_hash: ::subxt::ext::subxt_core::utils::H256,
                    pub ommers_hash: ::subxt::ext::subxt_core::utils::H256,
                    pub beneficiary: ::subxt::ext::subxt_core::utils::H160,
                    pub state_root: ::subxt::ext::subxt_core::utils::H256,
                    pub transactions_root: ::subxt::ext::subxt_core::utils::H256,
                    pub receipts_root: ::subxt::ext::subxt_core::utils::H256,
                    pub logs_bloom: runtime_types::ethbloom::Bloom,
                    pub difficulty: runtime_types::primitive_types::U256,
                    pub number: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub gas_used: runtime_types::primitive_types::U256,
                    pub timestamp: ::core::primitive::u64,
                    pub extra_data:
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub mix_hash: ::subxt::ext::subxt_core::utils::H256,
                    pub nonce: runtime_types::ethereum_types::hash::H64,
                }
            }
            pub mod log {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Log {
                    pub address: ::subxt::ext::subxt_core::utils::H160,
                    pub topics: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                    pub data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod receipt {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct EIP658ReceiptData {
                    pub status_code: ::core::primitive::u8,
                    pub used_gas: runtime_types::primitive_types::U256,
                    pub logs_bloom: runtime_types::ethbloom::Bloom,
                    pub logs: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::ethereum::log::Log,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ReceiptV3 {
                    #[codec(index = 0)]
                    Legacy(runtime_types::ethereum::receipt::EIP658ReceiptData),
                    #[codec(index = 1)]
                    EIP2930(runtime_types::ethereum::receipt::EIP658ReceiptData),
                    #[codec(index = 2)]
                    EIP1559(runtime_types::ethereum::receipt::EIP658ReceiptData),
                }
            }
            pub mod transaction {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccessListItem {
                    pub address: ::subxt::ext::subxt_core::utils::H160,
                    pub storage_keys: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::H256,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct EIP1559Transaction {
                    pub chain_id: ::core::primitive::u64,
                    pub nonce: runtime_types::primitive_types::U256,
                    pub max_priority_fee_per_gas: runtime_types::primitive_types::U256,
                    pub max_fee_per_gas: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub action: runtime_types::ethereum::transaction::TransactionAction,
                    pub value: runtime_types::primitive_types::U256,
                    pub input: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub access_list: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::ethereum::transaction::AccessListItem,
                    >,
                    pub odd_y_parity: ::core::primitive::bool,
                    pub r: ::subxt::ext::subxt_core::utils::H256,
                    pub s: ::subxt::ext::subxt_core::utils::H256,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct EIP2930Transaction {
                    pub chain_id: ::core::primitive::u64,
                    pub nonce: runtime_types::primitive_types::U256,
                    pub gas_price: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub action: runtime_types::ethereum::transaction::TransactionAction,
                    pub value: runtime_types::primitive_types::U256,
                    pub input: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub access_list: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::ethereum::transaction::AccessListItem,
                    >,
                    pub odd_y_parity: ::core::primitive::bool,
                    pub r: ::subxt::ext::subxt_core::utils::H256,
                    pub s: ::subxt::ext::subxt_core::utils::H256,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct LegacyTransaction {
                    pub nonce: runtime_types::primitive_types::U256,
                    pub gas_price: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub action: runtime_types::ethereum::transaction::TransactionAction,
                    pub value: runtime_types::primitive_types::U256,
                    pub input: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub signature: runtime_types::ethereum::transaction::TransactionSignature,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionAction {
                    #[codec(index = 0)]
                    Call(::subxt::ext::subxt_core::utils::H160),
                    #[codec(index = 1)]
                    Create,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransactionRecoveryId(pub ::core::primitive::u64);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct TransactionSignature {
                    pub v: runtime_types::ethereum::transaction::TransactionRecoveryId,
                    pub r: ::subxt::ext::subxt_core::utils::H256,
                    pub s: ::subxt::ext::subxt_core::utils::H256,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionV2 {
                    #[codec(index = 0)]
                    Legacy(runtime_types::ethereum::transaction::LegacyTransaction),
                    #[codec(index = 1)]
                    EIP2930(runtime_types::ethereum::transaction::EIP2930Transaction),
                    #[codec(index = 2)]
                    EIP1559(runtime_types::ethereum::transaction::EIP1559Transaction),
                }
            }
        }
        pub mod ethereum_types {
            use super::runtime_types;
            pub mod hash {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct H64(pub [::core::primitive::u8; 8usize]);
            }
        }
        pub mod evm_core {
            use super::runtime_types;
            pub mod error {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ExitError {
                    #[codec(index = 0)]
                    StackUnderflow,
                    #[codec(index = 1)]
                    StackOverflow,
                    #[codec(index = 2)]
                    InvalidJump,
                    #[codec(index = 3)]
                    InvalidRange,
                    #[codec(index = 4)]
                    DesignatedInvalid,
                    #[codec(index = 5)]
                    CallTooDeep,
                    #[codec(index = 6)]
                    CreateCollision,
                    #[codec(index = 7)]
                    CreateContractLimit,
                    #[codec(index = 15)]
                    InvalidCode(runtime_types::evm_core::opcode::Opcode),
                    #[codec(index = 8)]
                    OutOfOffset,
                    #[codec(index = 9)]
                    OutOfGas,
                    #[codec(index = 10)]
                    OutOfFund,
                    #[codec(index = 11)]
                    PCUnderflow,
                    #[codec(index = 12)]
                    CreateEmpty,
                    #[codec(index = 13)]
                    Other(::subxt::ext::subxt_core::alloc::string::String),
                    #[codec(index = 14)]
                    MaxNonce,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ExitFatal {
                    #[codec(index = 0)]
                    NotSupported,
                    #[codec(index = 1)]
                    UnhandledInterrupt,
                    #[codec(index = 2)]
                    CallErrorAsFatal(runtime_types::evm_core::error::ExitError),
                    #[codec(index = 3)]
                    Other(::subxt::ext::subxt_core::alloc::string::String),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ExitReason {
                    #[codec(index = 0)]
                    Succeed(runtime_types::evm_core::error::ExitSucceed),
                    #[codec(index = 1)]
                    Error(runtime_types::evm_core::error::ExitError),
                    #[codec(index = 2)]
                    Revert(runtime_types::evm_core::error::ExitRevert),
                    #[codec(index = 3)]
                    Fatal(runtime_types::evm_core::error::ExitFatal),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ExitRevert {
                    #[codec(index = 0)]
                    Reverted,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ExitSucceed {
                    #[codec(index = 0)]
                    Stopped,
                    #[codec(index = 1)]
                    Returned,
                    #[codec(index = 2)]
                    Suicided,
                }
            }
            pub mod opcode {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Opcode(pub ::core::primitive::u8);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod fp_rpc {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct TransactionStatus {
                pub transaction_hash: ::subxt::ext::subxt_core::utils::H256,
                pub transaction_index: ::core::primitive::u32,
                pub from: ::subxt::ext::subxt_core::utils::H160,
                pub to: ::core::option::Option<::subxt::ext::subxt_core::utils::H160>,
                pub contract_address: ::core::option::Option<::subxt::ext::subxt_core::utils::H160>,
                pub logs:
                    ::subxt::ext::subxt_core::alloc::vec::Vec<runtime_types::ethereum::log::Log>,
                pub logs_bloom: runtime_types::ethbloom::Bloom,
            }
        }
        pub mod frame_metadata_hash_extension {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CheckMetadataHash {
                pub mode: runtime_types::frame_metadata_hash_extension::Mode,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Mode {
                #[codec(index = 0)]
                Disabled,
                #[codec(index = 1)]
                Enabled,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub struct IdAmount<_0, _1> {
                            pub id: _0,
                            pub amount: _1,
                        }
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "Can be executed by every `origin`."]
                    remark {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    set_code {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                    #[doc = "version!"]
                    set_code_without_checks {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    do_task {
                        task: runtime_types::torus_runtime::RuntimeTask,
                    },
                    #[codec(index = 9)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 10)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                    #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                    #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade_without_checks {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 11)]
                    #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                    #[doc = ""]
                    #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                    #[doc = "remains unchanged and that the spec version has increased."]
                    #[doc = ""]
                    #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                    #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                    #[doc = ""]
                    #[doc = "All origins are allowed."]
                    apply_authorized_upgrade {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                    #[codec(index = 6)]
                    #[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
                    MultiBlockMigrationsOngoing,
                    #[codec(index = 7)]
                    #[doc = "The specified [`Task`] is not valid."]
                    InvalidTask,
                    #[codec(index = 8)]
                    #[doc = "The specified [`Task`] failed during execution."]
                    FailedTask,
                    #[codec(index = 9)]
                    #[doc = "No upgrade authorized."]
                    NothingAuthorized,
                    #[codec(index = 10)]
                    #[doc = "The submitted code is not authorized."]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::ext::subxt_core::utils::AccountId32,
                        hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "A [`Task`] has started executing"]
                    TaskStarted {
                        task: runtime_types::torus_runtime::RuntimeTask,
                    },
                    #[codec(index = 7)]
                    #[doc = "A [`Task`] has finished executing."]
                    TaskCompleted {
                        task: runtime_types::torus_runtime::RuntimeTask,
                    },
                    #[codec(index = 8)]
                    #[doc = "A [`Task`] failed during execution."]
                    TaskFailed {
                        task: runtime_types::torus_runtime::RuntimeTask,
                        err: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 9)]
                    #[doc = "An upgrade was authorized."]
                    UpgradeAuthorized {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        check_version: ::core::primitive::bool,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeUpgradeAuthorization {
                pub code_hash: ::subxt::ext::subxt_core::utils::H256,
                pub check_version: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    transfer_allow_death {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    force_transfer {
                        source: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibility of churn)."]
                    upgrade_accounts {
                        who: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    force_set_balance {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Adjust the total issuance in a saturating way."]
                    #[doc = ""]
                    #[doc = "Can only be called by root and always needs a positive `delta`."]
                    #[doc = ""]
                    #[doc = "# Example"]
                    force_adjust_total_issuance {
                        direction: runtime_types::pallet_balances::types::AdjustmentDirection,
                        #[codec(compact)]
                        delta: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Burn the specified liquid free balance from the origin account."]
                    #[doc = ""]
                    #[doc = "If the origin's account ends up below the existential deposit as a result"]
                    #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                    #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                    burn {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        keep_alive: ::core::primitive::bool,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                    #[codec(index = 10)]
                    #[doc = "The issuance cannot be modified since it is already deactivated."]
                    IssuanceDeactivated,
                    #[codec(index = 11)]
                    #[doc = "The delta cannot be zero."]
                    DeltaZero,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 21)]
                    #[doc = "The `TotalIssuance` was forcefully changed."]
                    TotalIssuanceForced {
                        old: ::core::primitive::u128,
                        new: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AdjustmentDirection {
                    #[codec(index = 0)]
                    Increase,
                    #[codec(index = 1)]
                    Decrease,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_emission0 {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    set_weights {
                        weights: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::core::primitive::u16,
                        )>,
                    },
                    #[codec(index = 1)]
                    delegate_weight_control {
                        target: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    regain_weight_control,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Agent tried setting more than 2 ^ 32 weights."]
                    WeightSetTooLarge,
                    #[codec(index = 1)]
                    #[doc = "Tried setting weights for an agent that does not exist."]
                    AgentIsNotRegistered,
                    #[codec(index = 2)]
                    #[doc = "Tried setting weights for itself."]
                    CannotSetWeightsForSelf,
                    #[codec(index = 3)]
                    #[doc = "Tried setting weights while delegating weight control."]
                    CannotSetWeightsWhileDelegating,
                    #[codec(index = 4)]
                    #[doc = "Tried delegating weight control to itself."]
                    CannotDelegateWeightControlToSelf,
                    #[codec(index = 5)]
                    #[doc = "Tried regaining weight control without delegating it."]
                    AgentIsNotDelegating,
                    #[codec(index = 6)]
                    #[doc = "Agent does not have enough stake to set weights."]
                    NotEnoughStakeToSetWeights,
                    #[codec(index = 7)]
                    #[doc = "At the current state, agents cannot control their own weight."]
                    WeightControlNotEnabled,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An agent set weights in the network."]
                    WeightsSet(::subxt::ext::subxt_core::utils::AccountId32),
                    #[codec(index = 1)]
                    #[doc = "An agent gave weight control to the second agent."]
                    DelegatedWeightControl(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    ),
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ConsensusMember {
                pub weights: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                    ::subxt::ext::subxt_core::utils::AccountId32,
                    ::core::primitive::u16,
                )>,
                pub last_incentives: ::core::primitive::u16,
                pub last_dividends: ::core::primitive::u16,
            }
        }
        pub mod pallet_ethereum {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transact an Ethereum transaction."]
                    transact {
                        transaction: runtime_types::ethereum::transaction::TransactionV2,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Signature is invalid."]
                    InvalidSignature,
                    #[codec(index = 1)]
                    #[doc = "Pre-log is present, therefore transact is not allowed."]
                    PreLogExists,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An ethereum transaction was successfully executed."]
                    Executed {
                        from: ::subxt::ext::subxt_core::utils::H160,
                        to: ::subxt::ext::subxt_core::utils::H160,
                        transaction_hash: ::subxt::ext::subxt_core::utils::H256,
                        exit_reason: runtime_types::evm_core::error::ExitReason,
                        extra_data:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod pallet_evm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Withdraw balance from EVM into currency/balances pallet."]
                    withdraw {
                        address: ::subxt::ext::subxt_core::utils::H160,
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Issue an EVM call operation. This is similar to a message call transaction in Ethereum."]
                    call {
                        source: ::subxt::ext::subxt_core::utils::H160,
                        target: ::subxt::ext::subxt_core::utils::H160,
                        input: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        max_fee_per_gas: runtime_types::primitive_types::U256,
                        max_priority_fee_per_gas:
                            ::core::option::Option<runtime_types::primitive_types::U256>,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                        access_list: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<
                                ::subxt::ext::subxt_core::utils::H256,
                            >,
                        )>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Issue an EVM create operation. This is similar to a contract creation transaction in"]
                    #[doc = "Ethereum."]
                    create {
                        source: ::subxt::ext::subxt_core::utils::H160,
                        init: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        max_fee_per_gas: runtime_types::primitive_types::U256,
                        max_priority_fee_per_gas:
                            ::core::option::Option<runtime_types::primitive_types::U256>,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                        access_list: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<
                                ::subxt::ext::subxt_core::utils::H256,
                            >,
                        )>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Issue an EVM create2 operation."]
                    create2 {
                        source: ::subxt::ext::subxt_core::utils::H160,
                        init: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        salt: ::subxt::ext::subxt_core::utils::H256,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        max_fee_per_gas: runtime_types::primitive_types::U256,
                        max_priority_fee_per_gas:
                            ::core::option::Option<runtime_types::primitive_types::U256>,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                        access_list: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::utils::H160,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<
                                ::subxt::ext::subxt_core::utils::H256,
                            >,
                        )>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Not enough balance to perform action"]
                    BalanceLow,
                    #[codec(index = 1)]
                    #[doc = "Calculating total fee overflowed"]
                    FeeOverflow,
                    #[codec(index = 2)]
                    #[doc = "Calculating total payment overflowed"]
                    PaymentOverflow,
                    #[codec(index = 3)]
                    #[doc = "Withdraw fee failed"]
                    WithdrawFailed,
                    #[codec(index = 4)]
                    #[doc = "Gas price is too low."]
                    GasPriceTooLow,
                    #[codec(index = 5)]
                    #[doc = "Nonce is invalid"]
                    InvalidNonce,
                    #[codec(index = 6)]
                    #[doc = "Gas limit is too low."]
                    GasLimitTooLow,
                    #[codec(index = 7)]
                    #[doc = "Gas limit is too high."]
                    GasLimitTooHigh,
                    #[codec(index = 8)]
                    #[doc = "The chain id is invalid."]
                    InvalidChainId,
                    #[codec(index = 9)]
                    #[doc = "the signature is invalid."]
                    InvalidSignature,
                    #[codec(index = 10)]
                    #[doc = "EVM reentrancy"]
                    Reentrancy,
                    #[codec(index = 11)]
                    #[doc = "EIP-3607,"]
                    TransactionMustComeFromEOA,
                    #[codec(index = 12)]
                    #[doc = "Undefined error."]
                    Undefined,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Ethereum events from contracts."]
                    Log {
                        log: runtime_types::ethereum::log::Log,
                    },
                    #[codec(index = 1)]
                    #[doc = "A contract has been created at given address."]
                    Created {
                        address: ::subxt::ext::subxt_core::utils::H160,
                    },
                    #[codec(index = 2)]
                    #[doc = "A contract was attempted to be created, but the execution failed."]
                    CreatedFailed {
                        address: ::subxt::ext::subxt_core::utils::H160,
                    },
                    #[codec(index = 3)]
                    #[doc = "A contract has been executed successfully with states applied."]
                    Executed {
                        address: ::subxt::ext::subxt_core::utils::H160,
                    },
                    #[codec(index = 4)]
                    #[doc = "A contract has been executed with errors. States are reverted with only gas fees applied."]
                    ExecutedFailed {
                        address: ::subxt::ext::subxt_core::utils::H160,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeMetadata {
                pub size: ::core::primitive::u64,
                pub hash: ::subxt::ext::subxt_core::utils::H256,
            }
        }
        pub mod pallet_faucet {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Callable functions for the faucet pallet"]
                pub enum Call {
                    #[codec(index = 1)]
                    #[doc = "Request tokens from the faucet by performing proof of work"]
                    #[doc = ""]
                    #[doc = "This extrinsic is only available on testnets. It requires the user to perform"]
                    #[doc = "proof-of-work by finding a nonce that, when combined with a recent block hash"]
                    #[doc = "and the user's account ID, produces a hash that meets the difficulty requirement."]
                    #[doc = ""]
                    #[doc = "The account must have a total balance (free + staked) below the threshold to be eligible."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = "* `origin` - Must be None (unsigned)"]
                    #[doc = "* `block_number` - A recent block number (within 3 blocks)"]
                    #[doc = "* `nonce` - A value that makes the resulting hash meet the difficulty requirement"]
                    #[doc = "* `work` - The hash result of the proof of work"]
                    #[doc = "* `key` - The account ID that will receive the tokens"]
                    #[doc = ""]
                    #[doc = "# Weight"]
                    #[doc = "* Read operations: 16"]
                    #[doc = "* Write operations: 28"]
                    #[doc = "* Does not pay fees"]
                    faucet {
                        block_number: ::core::primitive::u64,
                        nonce: ::core::primitive::u64,
                        work: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        key: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Errors that can occur in the faucet pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The block number provided is invalid (too old or in the future)"]
                    InvalidWorkBlock,
                    #[codec(index = 1)]
                    #[doc = "The proof-of-work does not meet the required difficulty"]
                    InvalidDifficulty,
                    #[codec(index = 2)]
                    #[doc = "The seal (hash) is invalid for the given parameters"]
                    InvalidSeal,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Events emitted by the faucet pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Tokens were successfully distributed by the faucet"]
                    #[doc = "[account, amount]"]
                    Faucet(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
        }
        pub mod pallet_governance {
            use super::runtime_types;
            pub mod application {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AgentApplication {
                    pub id: ::core::primitive::u32,
                    pub payer_key: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub cost: ::core::primitive::u128,
                    pub expires_at: ::core::primitive::u64,
                    pub action: runtime_types::pallet_governance::application::ApplicationAction,
                    pub status: runtime_types::pallet_governance::application::ApplicationStatus,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ApplicationAction {
                    #[codec(index = 0)]
                    Add,
                    #[codec(index = 1)]
                    Remove,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ApplicationStatus {
                    #[codec(index = 0)]
                    Open,
                    #[codec(index = 1)]
                    Resolved { accepted: ::core::primitive::bool },
                    #[codec(index = 2)]
                    Expired,
                }
            }
            pub mod config {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GovernanceConfiguration {
                    pub proposal_cost: ::core::primitive::u128,
                    pub proposal_expiration: ::core::primitive::u64,
                    pub agent_application_cost: ::core::primitive::u128,
                    pub agent_application_expiration: ::core::primitive::u64,
                    pub proposal_reward_treasury_allocation:
                        runtime_types::sp_arithmetic::per_things::Percent,
                    pub max_proposal_reward_treasury_allocation: ::core::primitive::u128,
                    pub proposal_reward_interval: ::core::primitive::u64,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 2)]
                    #[doc = "Adds a new allocator to the list. Only available for the root key."]
                    add_allocator {
                        key: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Removes an existing allocator from the list. Only available for the"]
                    #[doc = "root key."]
                    remove_allocator {
                        key: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Forcefully adds a new agent to the whitelist. Only available for the"]
                    #[doc = "root key or curators."]
                    add_to_whitelist {
                        key: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Forcefully removes an agent from the whitelist. Only available for"]
                    #[doc = "the root key or curators."]
                    remove_from_whitelist {
                        key: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Accepts an agent application. Only available for the root key or"]
                    #[doc = "curators."]
                    accept_application {
                        application_id: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Denies an agent application. Only available for the root key or"]
                    #[doc = "curators."]
                    deny_application {
                        application_id: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Sets a penalty factor to the given agent emissions. Only available"]
                    #[doc = "for the root key or curators."]
                    penalize_agent {
                        agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        percentage: ::core::primitive::u8,
                    },
                    #[codec(index = 9)]
                    #[doc = "Submits a new agent application on behalf of a given key."]
                    submit_application {
                        agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        metadata: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        removing: ::core::primitive::bool,
                    },
                    #[codec(index = 10)]
                    #[doc = "Creates a new global parameters proposal."]
                    add_global_params_proposal {
                        data: runtime_types::pallet_governance::proposal::GlobalParamsData,
                        metadata: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 11)]
                    #[doc = "Creates a new custom global proposal."]
                    add_global_custom_proposal {
                        metadata: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 12)]
                    #[doc = "Creates a proposal moving funds from the treasury account to the"]
                    #[doc = "given key."]
                    add_dao_treasury_transfer_proposal {
                        value: ::core::primitive::u128,
                        destination_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 13)]
                    #[doc = "Casts a vote for an open proposal."]
                    vote_proposal {
                        proposal_id: ::core::primitive::u64,
                        agree: ::core::primitive::bool,
                    },
                    #[codec(index = 14)]
                    #[doc = "Removes a casted vote for an open proposal."]
                    remove_vote_proposal { proposal_id: ::core::primitive::u64 },
                    #[codec(index = 15)]
                    #[doc = "Enables vote power delegation."]
                    enable_vote_delegation,
                    #[codec(index = 16)]
                    #[doc = "Disables vote power delegation."]
                    disable_vote_delegation,
                    #[codec(index = 17)]
                    #[doc = "Creates a new emission percentage proposal."]
                    add_emission_proposal {
                        recycling_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                        treasury_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                        incentives_ratio: runtime_types::sp_arithmetic::per_things::Percent,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 18)]
                    #[doc = "Forcefully sets emission percentages. Only available for the root"]
                    #[doc = "key."]
                    set_emission_params {
                        recycling_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                        treasury_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The proposal is already finished. Do not retry."]
                    ProposalIsFinished,
                    #[codec(index = 1)]
                    #[doc = "Invalid parameters were provided to the finalization process."]
                    InvalidProposalFinalizationParameters,
                    #[codec(index = 2)]
                    #[doc = "Invalid parameters were provided to the voting process."]
                    InvalidProposalVotingParameters,
                    #[codec(index = 3)]
                    #[doc = "Negative proposal cost when setting global or subnet governance"]
                    #[doc = "configuration."]
                    InvalidProposalCost,
                    #[codec(index = 4)]
                    #[doc = "Negative expiration when setting global or subnet governance"]
                    #[doc = "configuration."]
                    InvalidProposalExpiration,
                    #[codec(index = 5)]
                    #[doc = "Key doesn't have enough tokens to create a proposal."]
                    NotEnoughBalanceToPropose,
                    #[codec(index = 6)]
                    #[doc = "Proposal data is empty."]
                    ProposalDataTooSmall,
                    #[codec(index = 7)]
                    #[doc = "Proposal data is bigger than 256 characters."]
                    ProposalDataTooLarge,
                    #[codec(index = 8)]
                    #[doc = "The staked module is already delegating for 2 ^ 32 keys."]
                    ModuleDelegatingForMaxStakers,
                    #[codec(index = 9)]
                    #[doc = "Proposal with given id doesn't exist."]
                    ProposalNotFound,
                    #[codec(index = 10)]
                    #[doc = "Proposal was either accepted, refused or expired and cannot accept"]
                    #[doc = "votes."]
                    ProposalClosed,
                    #[codec(index = 11)]
                    #[doc = "Proposal data isn't composed by valid UTF-8 characters."]
                    InvalidProposalData,
                    #[codec(index = 12)]
                    #[doc = "Invalid value given when transforming a u64 into T::Currency."]
                    InvalidCurrencyConversionValue,
                    #[codec(index = 13)]
                    #[doc = "Dao Treasury doesn't have enough funds to be transferred."]
                    InsufficientDaoTreasuryFunds,
                    #[codec(index = 14)]
                    #[doc = "Key has already voted on given Proposal."]
                    AlreadyVoted,
                    #[codec(index = 15)]
                    #[doc = "Key hasn't voted on given Proposal."]
                    NotVoted,
                    #[codec(index = 16)]
                    #[doc = "Key doesn't have enough stake to vote."]
                    InsufficientStake,
                    #[codec(index = 17)]
                    #[doc = "The voter is delegating its voting power to their staked modules."]
                    #[doc = "Disable voting power delegation."]
                    VoterIsDelegatingVotingPower,
                    #[codec(index = 18)]
                    #[doc = "An internal error occurred, probably relating to the size of the"]
                    #[doc = "bounded sets."]
                    InternalError,
                    #[codec(index = 19)]
                    #[doc = "The application is not in a pending state."]
                    ApplicationNotOpen,
                    #[codec(index = 20)]
                    #[doc = "The application key is already used in another application."]
                    ApplicationKeyAlreadyUsed,
                    #[codec(index = 21)]
                    #[doc = "The account doesn't have enough balance to submit an application."]
                    NotEnoughBalanceToApply,
                    #[codec(index = 22)]
                    #[doc = "The operation can only be performed by the curator."]
                    NotCurator,
                    #[codec(index = 23)]
                    #[doc = "The application with the given ID was not found."]
                    ApplicationNotFound,
                    #[codec(index = 24)]
                    #[doc = "The account is already whitelisted and cannot be added again."]
                    AlreadyWhitelisted,
                    #[codec(index = 25)]
                    #[doc = "The account is not whitelisted and cannot be removed from the"]
                    #[doc = "whitelist."]
                    NotWhitelisted,
                    #[codec(index = 26)]
                    #[doc = "Failed to convert the given value to a balance."]
                    CouldNotConvertToBalance,
                    #[codec(index = 27)]
                    #[doc = "The application data provided does not meet the length requirement"]
                    InvalidApplicationDataLength,
                    #[codec(index = 28)]
                    #[doc = "The key is already a curator."]
                    AlreadyCurator,
                    #[codec(index = 29)]
                    #[doc = "The key is already an allocator."]
                    AlreadyAllocator,
                    #[codec(index = 30)]
                    #[doc = "The key is not an allocator."]
                    NotAllocator,
                    #[codec(index = 31)]
                    #[doc = "Agent not found"]
                    AgentNotFound,
                    #[codec(index = 32)]
                    #[doc = "Invalid agent penalty percentage"]
                    InvalidPenaltyPercentage,
                    #[codec(index = 33)]
                    #[doc = "Invalid minimum name length in proposal"]
                    InvalidMinNameLength,
                    #[codec(index = 34)]
                    #[doc = "Invalid maximum name length in proposal"]
                    InvalidMaxNameLength,
                    #[codec(index = 35)]
                    #[doc = "Invalid maximum allowed agents in proposal"]
                    InvalidMaxAllowedAgents,
                    #[codec(index = 36)]
                    #[doc = "Invalid maximum allowed weights in proposal"]
                    InvalidMaxAllowedWeights,
                    #[codec(index = 37)]
                    #[doc = "Invalid minimum weight control fee in proposal"]
                    InvalidMinWeightControlFee,
                    #[codec(index = 38)]
                    #[doc = "Invalid minimum staking fee in proposal"]
                    InvalidMinStakingFee,
                    #[codec(index = 39)]
                    #[doc = "Invalid params given to Emission proposal"]
                    InvalidEmissionProposalData,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new proposal has been created."]
                    ProposalCreated(::core::primitive::u64),
                    #[codec(index = 1)]
                    #[doc = "A proposal has been accepted."]
                    ProposalAccepted(::core::primitive::u64),
                    #[codec(index = 2)]
                    #[doc = "A proposal has been refused."]
                    ProposalRefused(::core::primitive::u64),
                    #[codec(index = 3)]
                    #[doc = "A proposal has expired."]
                    ProposalExpired(::core::primitive::u64),
                    #[codec(index = 4)]
                    #[doc = "A vote has been cast on a proposal."]
                    ProposalVoted(
                        ::core::primitive::u64,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::bool,
                    ),
                    #[codec(index = 5)]
                    #[doc = "A vote has been unregistered from a proposal."]
                    ProposalVoteUnregistered(
                        ::core::primitive::u64,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    ),
                    #[codec(index = 6)]
                    #[doc = "An agent account has been added to the whitelist."]
                    WhitelistAdded(::subxt::ext::subxt_core::utils::AccountId32),
                    #[codec(index = 7)]
                    #[doc = "An agent account has been removed from the whitelist."]
                    WhitelistRemoved(::subxt::ext::subxt_core::utils::AccountId32),
                    #[codec(index = 8)]
                    #[doc = "A new application has been created."]
                    ApplicationCreated(::core::primitive::u32),
                    #[codec(index = 9)]
                    #[doc = "An application has been accepted."]
                    ApplicationAccepted(::core::primitive::u32),
                    #[codec(index = 10)]
                    #[doc = "An application has been denied."]
                    ApplicationDenied(::core::primitive::u32),
                    #[codec(index = 11)]
                    #[doc = "An application has expired."]
                    ApplicationExpired(::core::primitive::u32),
                    #[codec(index = 12)]
                    #[doc = "A penalty was applied to an agent."]
                    PenaltyApplied {
                        curator: ::subxt::ext::subxt_core::utils::AccountId32,
                        agent: ::subxt::ext::subxt_core::utils::AccountId32,
                        penalty: runtime_types::sp_arithmetic::per_things::Percent,
                    },
                }
            }
            pub mod proposal {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GlobalParamsData {
                    pub min_name_length: ::core::primitive::u16,
                    pub max_name_length: ::core::primitive::u16,
                    pub max_allowed_agents: ::core::primitive::u16,
                    pub min_weight_control_fee: ::core::primitive::u8,
                    pub min_staking_fee: ::core::primitive::u8,
                    pub dividends_participation_weight:
                        runtime_types::sp_arithmetic::per_things::Percent,
                    pub proposal_cost: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Proposal {
                    pub id: ::core::primitive::u64,
                    pub proposer: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub expiration_block: ::core::primitive::u64,
                    pub data: runtime_types::pallet_governance::proposal::ProposalData,
                    pub status: runtime_types::pallet_governance::proposal::ProposalStatus,
                    pub metadata: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub proposal_cost: ::core::primitive::u128,
                    pub creation_block: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ProposalData {
                    #[codec(index = 0)]
                    GlobalParams(runtime_types::pallet_governance::proposal::GlobalParamsData),
                    #[codec(index = 1)]
                    GlobalCustom,
                    #[codec(index = 2)]
                    Emission {
                        recycling_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                        treasury_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                        incentives_ratio: runtime_types::sp_arithmetic::per_things::Percent,
                    },
                    #[codec(index = 3)]
                    TransferDaoTreasury {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ProposalStatus {
                    #[codec(index = 0)]
                    Open {
                        votes_for:
                            runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                                ::subxt::ext::subxt_core::utils::AccountId32,
                            >,
                        votes_against:
                            runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                                ::subxt::ext::subxt_core::utils::AccountId32,
                            >,
                        stake_for: ::core::primitive::u128,
                        stake_against: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    Accepted {
                        block: ::core::primitive::u64,
                        stake_for: ::core::primitive::u128,
                        stake_against: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Refused {
                        block: ::core::primitive::u64,
                        stake_for: ::core::primitive::u128,
                        stake_against: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    Expired,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct UnrewardedProposal {
                    pub block: ::core::primitive::u64,
                    pub votes_for:
                        runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::core::primitive::u128,
                        >,
                    pub votes_against:
                        runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            ::core::primitive::u128,
                        >,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::ext::subxt_core::utils::H256,
                                ::core::primitive::u64,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::ext::subxt_core::utils::H256,
                                ::core::primitive::u64,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u64,
                        best_finalized_block_number: ::core::primitive::u64,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        _0,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                    #[doc = "multi-signature, but do not participate in the approval process."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
                    as_multi_threshold_1 {
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::torus_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "If there are enough, then dispatch the call."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
                    #[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
                    #[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
                    #[doc = "may be found in the deposited `MultisigExecuted` event."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S + Z + Call)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- The weight of the `call`."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::torus_runtime::RuntimeCall,
                        >,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
                    #[doc = "for this operation will be unreserved on success."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
                    #[doc = "transaction for this dispatch."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- One event."]
                    #[doc = "- I/O: 1 read `O(S)`, one remove."]
                    #[doc = "- Storage: removes one item."]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Threshold must be 2 or greater."]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    #[doc = "Call is already approved by this signatory."]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    #[doc = "Call doesn't need any (more) approvals."]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    #[doc = "There are too few signatories in the list."]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    #[doc = "There are too many signatories in the list."]
                    TooManySignatories,
                    #[codec(index = 5)]
                    #[doc = "The signatories were provided out of order; they should be ordered."]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    #[doc = "The sender was contained in the other signatories; it shouldn't be."]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    #[doc = "Multisig operation not found when attempting to cancel."]
                    NotFound,
                    #[codec(index = 8)]
                    #[doc = "Only the account that originally created the multisig is able to cancel it."]
                    NotOwner,
                    #[codec(index = 9)]
                    #[doc = "No timepoint was given, yet the multisig operation is already underway."]
                    NoTimepoint,
                    #[codec(index = 10)]
                    #[doc = "A different timepoint was given to the multisig operation that is underway."]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    #[doc = "A timepoint was given, yet no multisig operation is underway."]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    #[doc = "The maximum weight information provided was too low."]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    #[doc = "The data to be stored is already stored."]
                    AlreadyStored,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new multisig operation has begun."]
                    NewMultisig {
                        approving: ::subxt::ext::subxt_core::utils::AccountId32,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    #[doc = "A multisig operation has been approved by someone."]
                    MultisigApproval {
                        approving: ::subxt::ext::subxt_core::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "A multisig operation has been executed."]
                    MultisigExecuted {
                        approving: ::subxt::ext::subxt_core::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    #[doc = "A multisig operation has been cancelled."]
                    MultisigCancelled {
                        cancelling: ::subxt::ext::subxt_core::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u64>,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: ::core::primitive::u32,
            }
        }
        pub mod pallet_permission0 {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Grant a permission for emission delegation"] grant_emission_permission { grantee : :: subxt :: ext :: subxt_core :: utils :: AccountId32 , allocation : runtime_types :: pallet_permission0 :: permission :: emission :: EmissionAllocation , targets : :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < (:: subxt :: ext :: subxt_core :: utils :: AccountId32 , :: core :: primitive :: u16 ,) > , distribution : runtime_types :: pallet_permission0 :: permission :: emission :: DistributionControl , duration : runtime_types :: pallet_permission0 :: permission :: PermissionDuration , revocation : runtime_types :: pallet_permission0 :: permission :: RevocationTerms , enforcement : runtime_types :: pallet_permission0 :: permission :: EnforcementAuthority , } , # [codec (index = 1)] # [doc = "Revoke a permission. The caller must met revocation constraints or be a root key."] revoke_permission { permission_id : :: subxt :: ext :: subxt_core :: utils :: H256 , } , # [codec (index = 2)] # [doc = "Execute a manual distribution based on permission"] execute_permission { permission_id : :: subxt :: ext :: subxt_core :: utils :: H256 , } , # [codec (index = 3)] # [doc = "Toggle a permission's accumulation state (enabled/disabled)"] # [doc = "The caller must be authorized as a controller or be the root key"] toggle_permission_accumulation { permission_id : :: subxt :: ext :: subxt_core :: utils :: H256 , accumulating : :: core :: primitive :: bool , } , # [codec (index = 4)] # [doc = "Execute a permission through enforcement authority"] # [doc = "The caller must be authorized as a controller or be the root key"] enforcement_execute_permission { permission_id : :: subxt :: ext :: subxt_core :: utils :: H256 , } , # [codec (index = 5)] # [doc = "Set enforcement authority for a permission"] # [doc = "Only the grantor or root can set enforcement authority"] set_enforcement_authority { permission_id : :: subxt :: ext :: subxt_core :: utils :: H256 , controllers : :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < :: subxt :: ext :: subxt_core :: utils :: AccountId32 > , required_votes : :: core :: primitive :: u32 , } , # [codec (index = 6)] # [doc = "Grant a permission for curator delegation"] grant_curator_permission { grantee : :: subxt :: ext :: subxt_core :: utils :: AccountId32 , flags : :: core :: primitive :: u32 , cooldown : :: core :: option :: Option < :: core :: primitive :: u64 > , duration : runtime_types :: pallet_permission0 :: permission :: PermissionDuration , revocation : runtime_types :: pallet_permission0 :: permission :: RevocationTerms , } , }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The agent is not registered"]
                    NotRegisteredAgent,
                    #[codec(index = 1)]
                    #[doc = "Permission not found"]
                    PermissionNotFound,
                    #[codec(index = 2)]
                    #[doc = "Self-permission is not allowed"]
                    SelfPermissionNotAllowed,
                    #[codec(index = 3)]
                    #[doc = "Invalid percentage (out of range)"]
                    InvalidPercentage,
                    #[codec(index = 4)]
                    #[doc = "No targets specified"]
                    NoTargetsSpecified,
                    #[codec(index = 5)]
                    #[doc = "Invalid threshold"]
                    InvalidThreshold,
                    #[codec(index = 6)]
                    #[doc = "No accumulated amount"]
                    NoAccumulatedAmount,
                    #[codec(index = 7)]
                    #[doc = "Not authorized to revoke"]
                    NotAuthorizedToRevoke,
                    #[codec(index = 8)]
                    #[doc = "Total allocation exceeded 100%"]
                    TotalAllocationExceeded,
                    #[codec(index = 9)]
                    #[doc = "Not the grantee of the permission"]
                    NotPermissionGrantee,
                    #[codec(index = 10)]
                    #[doc = "Not the grantor of the permission"]
                    NotPermissionGrantor,
                    #[codec(index = 11)]
                    #[doc = "Too many streams"]
                    TooManyStreams,
                    #[codec(index = 12)]
                    #[doc = "Too many targets"]
                    TooManyTargets,
                    #[codec(index = 13)]
                    #[doc = "Too many revokers"]
                    TooManyRevokers,
                    #[codec(index = 14)]
                    #[doc = "Failed to insert into storage"]
                    StorageError,
                    #[codec(index = 15)]
                    #[doc = "Invalid amount"]
                    InvalidAmount,
                    #[codec(index = 16)]
                    #[doc = "Insufficient balance for operation"]
                    InsufficientBalance,
                    #[codec(index = 17)]
                    #[doc = "Invalid distribution interval"]
                    InvalidInterval,
                    #[codec(index = 18)]
                    #[doc = "Parent permission not found"]
                    ParentPermissionNotFound,
                    #[codec(index = 19)]
                    #[doc = "Invalid distribution method"]
                    InvalidDistributionMethod,
                    #[codec(index = 20)]
                    #[doc = "Revokers and required voters must be at least one, and required voters must"]
                    #[doc = "be less than the number of revokers"]
                    InvalidNumberOfRevokers,
                    #[codec(index = 21)]
                    #[doc = "Fixed amount emissions can only be triggered once, manually or at a block"]
                    FixedAmountCanOnlyBeTriggeredOnce,
                    #[codec(index = 22)]
                    #[doc = "Unsupported permission type"]
                    UnsupportedPermissionType,
                    #[codec(index = 23)]
                    #[doc = "Not authorized to toggle permission state"]
                    NotAuthorizedToToggle,
                    #[codec(index = 24)]
                    #[doc = "Too many controllers"]
                    TooManyControllers,
                    #[codec(index = 25)]
                    #[doc = "Invalid number of controllers or required votes"]
                    InvalidNumberOfControllers,
                    #[codec(index = 26)]
                    #[doc = "Permission is a duplicate, revoke the previous one"]
                    DuplicatePermission,
                    #[codec(index = 27)]
                    #[doc = "Permission is in cooldown, wait a bit."]
                    PermissionInCooldown,
                    #[codec(index = 28)]
                    #[doc = "Curator flags provided are invalid."]
                    InvalidCuratorPermissions,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Permission granted from grantor to grantee with ID"]
                    PermissionGranted {
                        grantor: ::subxt::ext::subxt_core::utils::AccountId32,
                        grantee: ::subxt::ext::subxt_core::utils::AccountId32,
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 1)]
                    #[doc = "Permission revoked with ID"]
                    PermissionRevoked {
                        grantor: ::subxt::ext::subxt_core::utils::AccountId32,
                        grantee: ::subxt::ext::subxt_core::utils::AccountId32,
                        revoked_by:
                            ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 2)]
                    #[doc = "Permission executed (manual distribution) with ID"]
                    PermissionExecuted {
                        grantor: ::subxt::ext::subxt_core::utils::AccountId32,
                        grantee: ::subxt::ext::subxt_core::utils::AccountId32,
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                        stream_id: ::core::option::Option<::subxt::ext::subxt_core::utils::H256>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Auto-distribution executed"]
                    AutoDistributionExecuted {
                        grantor: ::subxt::ext::subxt_core::utils::AccountId32,
                        grantee: ::subxt::ext::subxt_core::utils::AccountId32,
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                        stream_id: ::core::option::Option<::subxt::ext::subxt_core::utils::H256>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Permission expired with ID"]
                    PermissionExpired {
                        grantor: ::subxt::ext::subxt_core::utils::AccountId32,
                        grantee: ::subxt::ext::subxt_core::utils::AccountId32,
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 5)]
                    #[doc = "Permission accumulation state toggled"]
                    PermissionAccumulationToggled {
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                        accumulating: ::core::primitive::bool,
                        toggled_by:
                            ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Permission was executed by enforcement authority"]
                    PermissionEnforcementExecuted {
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                        executed_by:
                            ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Vote for enforcement action"]
                    EnforcementVoteCast {
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                        voter: ::subxt::ext::subxt_core::utils::AccountId32,
                        referendum:
                            runtime_types::pallet_permission0::permission::EnforcementReferendum,
                    },
                    #[codec(index = 8)]
                    #[doc = "Enforcement authority set for permission"]
                    EnforcementAuthoritySet {
                        permission_id: ::subxt::ext::subxt_core::utils::H256,
                        controllers_count: ::core::primitive::u32,
                        required_votes: ::core::primitive::u32,
                    },
                }
            }
            pub mod permission {
                use super::runtime_types;
                pub mod curator {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CuratorPermissions(pub ::core::primitive::u32);
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CuratorScope { pub flags : runtime_types :: pallet_permission0 :: permission :: curator :: CuratorPermissions , pub cooldown : :: core :: option :: Option < :: core :: primitive :: u64 > , }
                }
                pub mod emission {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum DistributionControl {
                        #[codec(index = 0)]
                        Manual,
                        #[codec(index = 1)]
                        Automatic(::core::primitive::u128),
                        #[codec(index = 2)]
                        AtBlock(::core::primitive::u64),
                        #[codec(index = 3)]
                        Interval(::core::primitive::u64),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum EmissionAllocation {
                        #[codec(index = 0)]
                        Streams(
                            runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                                ::subxt::ext::subxt_core::utils::H256,
                                runtime_types::sp_arithmetic::per_things::Percent,
                            >,
                        ),
                        #[codec(index = 1)]
                        FixedAmount(::core::primitive::u128),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct EmissionScope { pub allocation : runtime_types :: pallet_permission0 :: permission :: emission :: EmissionAllocation , pub distribution : runtime_types :: pallet_permission0 :: permission :: emission :: DistributionControl , pub targets : runtime_types :: bounded_collections :: bounded_btree_map :: BoundedBTreeMap < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , :: core :: primitive :: u16 > , pub accumulating : :: core :: primitive :: bool , }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum EnforcementAuthority {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    ControlledBy {
                        controllers: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        required_votes: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum EnforcementReferendum {
                    #[codec(index = 0)]
                    EmissionAccumulation(::core::primitive::bool),
                    #[codec(index = 1)]
                    Execution,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PermissionContract {
                    pub grantor: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub grantee: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub scope: runtime_types::pallet_permission0::permission::PermissionScope,
                    pub duration: runtime_types::pallet_permission0::permission::PermissionDuration,
                    pub revocation: runtime_types::pallet_permission0::permission::RevocationTerms,
                    pub enforcement:
                        runtime_types::pallet_permission0::permission::EnforcementAuthority,
                    pub last_execution: ::core::option::Option<::core::primitive::u64>,
                    pub execution_count: ::core::primitive::u32,
                    pub parent: ::core::option::Option<::subxt::ext::subxt_core::utils::H256>,
                    pub created_at: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum PermissionDuration {
                    #[codec(index = 0)]
                    UntilBlock(::core::primitive::u64),
                    #[codec(index = 1)]
                    Indefinite,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum PermissionScope {
                    #[codec(index = 0)]
                    Emission(
                        runtime_types::pallet_permission0::permission::emission::EmissionScope,
                    ),
                    #[codec(index = 1)]
                    Curator(runtime_types::pallet_permission0::permission::curator::CuratorScope),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum RevocationTerms {
                    #[codec(index = 0)]
                    Irrevocable,
                    #[codec(index = 1)]
                    RevocableByGrantor,
                    #[codec(index = 2)]
                    RevocableByArbiters {
                        accounts: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        required_votes: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    RevocableAfter(::core::primitive::u64),
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    sudo {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::torus_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_unchecked_weight {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::torus_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    set_key {
                        new: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_as {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::torus_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Permanently removes the sudo key."]
                    #[doc = ""]
                    #[doc = "**This cannot be un-done.**"]
                    remove_key,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the Sudo pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account."]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo call just took place."]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The sudo key has been updated."]
                    KeyChanged {
                        old: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                        new: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The key was permanently removed."]
                    KeyRemoved,
                    #[codec(index = 3)]
                    #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "[`Config::MinimumPeriod`]."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _None_."]
                    #[doc = ""]
                    #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                    #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                    #[doc = "block to execute any other calls."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_torus0 {
            use super::runtime_types;
            pub mod agent {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Agent {
                    pub key: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub url: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub metadata: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub weight_penalty_factor: runtime_types::sp_arithmetic::per_things::Percent,
                    pub registration_block: ::core::primitive::u64,
                    pub fees: runtime_types::pallet_torus0::fee::ValidatorFee,
                    pub last_update_block: ::core::primitive::u64,
                }
            }
            pub mod burn {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BurnConfiguration {
                    pub min_burn: ::core::primitive::u128,
                    pub max_burn: ::core::primitive::u128,
                    pub adjustment_alpha: ::core::primitive::u64,
                    pub target_registrations_interval: ::core::primitive::u64,
                    pub target_registrations_per_interval: ::core::primitive::u16,
                    pub max_registrations_per_interval: ::core::primitive::u16,
                }
            }
            pub mod fee {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidatorFee {
                    pub staking_fee: runtime_types::sp_arithmetic::per_things::Percent,
                    pub weight_control_fee: runtime_types::sp_arithmetic::per_things::Percent,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidatorFeeConstraints {
                    pub min_staking_fee: runtime_types::sp_arithmetic::per_things::Percent,
                    pub min_weight_control_fee: runtime_types::sp_arithmetic::per_things::Percent,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Adds stakes from origin to the agent key."]
                    add_stake {
                        agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Removes stakes from origin to the agent key."]
                    remove_stake {
                        agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfers origin's stakes from an agent to another."]
                    transfer_stake {
                        agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        new_agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Registers a new agent on behalf of an arbitrary key."]
                    register_agent {
                        agent_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        url: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        metadata: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Unregister origin's key agent."]
                    unregister_agent,
                    #[codec(index = 5)]
                    #[doc = "Updates origin's key agent metadata."]
                    update_agent {
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        url: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        metadata: ::core::option::Option<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                        staking_fee: ::core::option::Option<
                            runtime_types::sp_arithmetic::per_things::Percent,
                        >,
                        weight_control_fee: ::core::option::Option<
                            runtime_types::sp_arithmetic::per_things::Percent,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Updates origin's key agent metadata."]
                    set_agent_update_cooldown {
                        new_cooldown: ::core::primitive::u64,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The specified agent does not exist."]
                    AgentDoesNotExist,
                    #[codec(index = 1)]
                    #[doc = "Insufficient stake to withdraw the requested amount."]
                    NotEnoughStakeToWithdraw,
                    #[codec(index = 2)]
                    #[doc = "Insufficient balance in the cold key account to stake the requested"]
                    #[doc = "amount."]
                    NotEnoughBalanceToStake,
                    #[codec(index = 3)]
                    #[doc = "The number of agent registrations in this block exceeds the allowed"]
                    #[doc = "limit."]
                    TooManyAgentRegistrationsThisBlock,
                    #[codec(index = 4)]
                    #[doc = "The number of agent registrations in this interval exceeds the"]
                    #[doc = "allowed limit."]
                    TooManyAgentRegistrationsThisInterval,
                    #[codec(index = 5)]
                    #[doc = "The agent is already registered in the active set."]
                    AgentAlreadyRegistered,
                    #[codec(index = 6)]
                    #[doc = "Failed to convert between u128 and T::Balance."]
                    CouldNotConvertToBalance,
                    #[codec(index = 7)]
                    #[doc = "Failed to add balance to the account."]
                    BalanceNotAdded,
                    #[codec(index = 8)]
                    #[doc = "Failed to remove stake from the account."]
                    StakeNotRemoved,
                    #[codec(index = 9)]
                    #[doc = "Invalid shares distribution."]
                    InvalidShares,
                    #[codec(index = 10)]
                    #[doc = "Insufficient balance to register."]
                    NotEnoughBalanceToRegisterAgent,
                    #[codec(index = 11)]
                    #[doc = "Failed to add stake to the account."]
                    StakeNotAdded,
                    #[codec(index = 12)]
                    #[doc = "Failed to remove balance from the account."]
                    BalanceNotRemoved,
                    #[codec(index = 13)]
                    #[doc = "Balance could not be removed from the account."]
                    BalanceCouldNotBeRemoved,
                    #[codec(index = 14)]
                    #[doc = "Insufficient stake to register."]
                    NotEnoughStakeToRegister,
                    #[codec(index = 15)]
                    #[doc = "The entity is still registered and cannot be modified."]
                    StillRegistered,
                    #[codec(index = 16)]
                    #[doc = "Attempted to set max allowed agents to a value less than the current"]
                    #[doc = "number of registered agents."]
                    MaxAllowedAgents,
                    #[codec(index = 17)]
                    #[doc = "Insufficient balance to transfer."]
                    NotEnoughBalanceToTransfer,
                    #[codec(index = 18)]
                    #[doc = "The agent metadata is invalid."]
                    InvalidAgentMetadata,
                    #[codec(index = 19)]
                    #[doc = "The agent metadata is too long."]
                    AgentMetadataTooLong,
                    #[codec(index = 20)]
                    #[doc = "The agent metadata is too long."]
                    AgentMetadataTooShort,
                    #[codec(index = 21)]
                    #[doc = "The minimum burn value is invalid, likely too small."]
                    InvalidMinBurn,
                    #[codec(index = 22)]
                    #[doc = "The maximum burn value is invalid."]
                    InvalidMaxBurn,
                    #[codec(index = 23)]
                    #[doc = "The agent name is too long."]
                    AgentNameTooLong,
                    #[codec(index = 24)]
                    #[doc = "The agent name is too short."]
                    AgentNameTooShort,
                    #[codec(index = 25)]
                    #[doc = "The agent name is invalid. It must be a UTF-8 encoded string."]
                    InvalidAgentName,
                    #[codec(index = 26)]
                    #[doc = "The agent url is too long."]
                    AgentUrlTooLong,
                    #[codec(index = 27)]
                    #[doc = "The agent url is too short."]
                    AgentUrlTooShort,
                    #[codec(index = 28)]
                    #[doc = "The agent ur; is invalid."]
                    InvalidAgentUrl,
                    #[codec(index = 29)]
                    #[doc = "A agent with this name already exists in the subnet."]
                    AgentNameAlreadyExists,
                    #[codec(index = 30)]
                    #[doc = "The stake amount to add or remove is too small. Minimum is 0.5 unit."]
                    StakeTooSmall,
                    #[codec(index = 31)]
                    #[doc = "Key is not present in Whitelist, it needs to be whitelisted by a"]
                    #[doc = "Curator"]
                    AgentKeyNotWhitelisted,
                    #[codec(index = 32)]
                    #[doc = "The amount given is 0"]
                    InvalidAmount,
                    #[codec(index = 33)]
                    #[doc = "The staking fee given is lower than the minimum fee"]
                    InvalidStakingFee,
                    #[codec(index = 34)]
                    #[doc = "The weight control fee given is lower than the minimum fee"]
                    InvalidWeightControlFee,
                    #[codec(index = 35)]
                    #[doc = "The agent already updated recently"]
                    AgentUpdateOnCooldown,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event created when stake has been transferred from the coldkey"]
                    #[doc = "account onto the key staking account"]
                    StakeAdded(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    #[doc = "Event created when stake has been removed from the key staking"]
                    #[doc = "account onto the coldkey account"]
                    StakeRemoved(
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    #[doc = "Event created when a new agent account has been registered to the"]
                    #[doc = "chain"]
                    AgentRegistered(::subxt::ext::subxt_core::utils::AccountId32),
                    #[codec(index = 3)]
                    #[doc = "Event created when a agent account has been deregistered from the"]
                    #[doc = "chain"]
                    AgentUnregistered(::subxt::ext::subxt_core::utils::AccountId32),
                    #[codec(index = 4)]
                    #[doc = "Event created when the agent's updated information is added to the"]
                    #[doc = "network"]
                    AgentUpdated(::subxt::ext::subxt_core::utils::AccountId32),
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct U256(pub [::core::primitive::u64; 4usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Percent(pub ::core::primitive::u8);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Public(pub [::core::primitive::u8; 32usize]);
                }
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Digest {
                        pub logs: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519([::core::primitive::u8; 64usize]),
                #[codec(index = 1)]
                Sr25519([::core::primitive::u8; 64usize]),
                #[codec(index = 2)]
                Ecdsa([::core::primitive::u8; 65usize]),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub impl_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
        pub mod torus_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 7)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 8)]
                Ethereum(runtime_types::pallet_ethereum::pallet::Call),
                #[codec(index = 9)]
                EVM(runtime_types::pallet_evm::pallet::Call),
                #[codec(index = 11)]
                Governance(runtime_types::pallet_governance::pallet::Call),
                #[codec(index = 12)]
                Torus0(runtime_types::pallet_torus0::pallet::Call),
                #[codec(index = 13)]
                Emission0(runtime_types::pallet_emission0::pallet::Call),
                #[codec(index = 14)]
                Permission0(runtime_types::pallet_permission0::pallet::Call),
                #[codec(index = 15)]
                Faucet(runtime_types::pallet_faucet::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Error),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 7)]
                Multisig(runtime_types::pallet_multisig::pallet::Error),
                #[codec(index = 8)]
                Ethereum(runtime_types::pallet_ethereum::pallet::Error),
                #[codec(index = 9)]
                EVM(runtime_types::pallet_evm::pallet::Error),
                #[codec(index = 11)]
                Governance(runtime_types::pallet_governance::pallet::Error),
                #[codec(index = 12)]
                Torus0(runtime_types::pallet_torus0::pallet::Error),
                #[codec(index = 13)]
                Emission0(runtime_types::pallet_emission0::pallet::Error),
                #[codec(index = 14)]
                Permission0(runtime_types::pallet_permission0::pallet::Error),
                #[codec(index = 15)]
                Faucet(runtime_types::pallet_faucet::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 5)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 7)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 8)]
                Ethereum(runtime_types::pallet_ethereum::pallet::Event),
                #[codec(index = 9)]
                EVM(runtime_types::pallet_evm::pallet::Event),
                #[codec(index = 11)]
                Governance(runtime_types::pallet_governance::pallet::Event),
                #[codec(index = 12)]
                Torus0(runtime_types::pallet_torus0::pallet::Event),
                #[codec(index = 13)]
                Emission0(runtime_types::pallet_emission0::pallet::Event),
                #[codec(index = 14)]
                Permission0(runtime_types::pallet_permission0::pallet::Event),
                #[codec(index = 15)]
                Faucet(runtime_types::pallet_faucet::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {}
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeTask {}
        }
    }
}
