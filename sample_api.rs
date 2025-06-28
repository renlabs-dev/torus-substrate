#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 1usize] = ["Governance"];
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
        pub fn governance(&self) -> governance::constants::ConstantsApi {
            governance::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn governance(&self) -> governance::storage::StorageApi {
            governance::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn governance(&self) -> governance::calls::TransactionApi {
            governance::calls::TransactionApi
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
                11u8, 233u8, 209u8, 167u8, 36u8, 104u8, 165u8, 31u8, 182u8, 145u8, 231u8, 142u8,
                138u8, 96u8, 231u8, 211u8, 233u8, 204u8, 112u8, 132u8, 210u8, 94u8, 115u8, 163u8,
                60u8, 136u8, 87u8, 189u8, 22u8, 4u8, 142u8, 196u8,
            ]
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
                pub struct AddCurator {
                    pub key: add_curator::Key,
                }
                pub mod add_curator {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AddCurator {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "add_curator";
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
                pub struct RemoveCurator {
                    pub key: remove_curator::Key,
                }
                pub mod remove_curator {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveCurator {
                    const PALLET: &'static str = "Governance";
                    const CALL: &'static str = "remove_curator";
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
                pub fn add_curator(
                    &self,
                    key: types::add_curator::Key,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AddCurator>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "add_curator",
                        types::AddCurator { key },
                        [
                            201u8, 5u8, 5u8, 25u8, 149u8, 29u8, 39u8, 211u8, 29u8, 180u8, 130u8,
                            238u8, 234u8, 74u8, 184u8, 239u8, 179u8, 249u8, 189u8, 123u8, 87u8,
                            248u8, 83u8, 97u8, 176u8, 108u8, 8u8, 198u8, 183u8, 82u8, 109u8, 188u8,
                        ],
                    )
                }
                pub fn remove_curator(
                    &self,
                    key: types::remove_curator::Key,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveCurator>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Governance",
                        "remove_curator",
                        types::RemoveCurator { key },
                        [
                            248u8, 209u8, 173u8, 52u8, 9u8, 70u8, 94u8, 2u8, 153u8, 216u8, 42u8,
                            157u8, 69u8, 59u8, 131u8, 133u8, 54u8, 248u8, 147u8, 174u8, 16u8,
                            128u8, 5u8, 198u8, 38u8, 156u8, 170u8, 114u8, 173u8, 3u8, 162u8, 38u8,
                        ],
                    )
                }
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
                            51u8, 43u8, 139u8, 221u8, 83u8, 191u8, 41u8, 153u8, 95u8, 149u8, 80u8,
                            203u8, 121u8, 14u8, 161u8, 176u8, 190u8, 123u8, 19u8, 112u8, 32u8,
                            133u8, 102u8, 135u8, 202u8, 223u8, 75u8, 106u8, 21u8, 21u8, 17u8,
                            109u8,
                        ],
                    )
                }
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
             ...
                pub mod treasury_emission_fee {
                    use super::runtime_types;
                    pub type TreasuryEmissionFee =
                        runtime_types::sp_arithmetic::per_things::Percent;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
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
                            145u8, 6u8, 157u8, 129u8, 42u8, 32u8, 254u8, 121u8, 248u8, 68u8, 7u8,
                            124u8, 69u8, 58u8, 16u8, 174u8, 206u8, 177u8, 102u8, 244u8, 124u8,
                            200u8, 93u8, 225u8, 4u8, 211u8, 65u8, 38u8, 147u8, 21u8, 233u8, 53u8,
                        ],
                    )
                }
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
                            145u8, 6u8, 157u8, 129u8, 42u8, 32u8, 254u8, 121u8, 248u8, 68u8, 7u8,
                            124u8, 69u8, 58u8, 16u8, 174u8, 206u8, 177u8, 102u8, 244u8, 124u8,
                            200u8, 93u8, 225u8, 4u8, 211u8, 65u8, 38u8, 147u8, 21u8, 233u8, 53u8,
                        ],
                    )
                }
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
                pub fn curators_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::curators::Curators,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Curators",
                        (),
                        [
                            122u8, 134u8, 173u8, 24u8, 32u8, 12u8, 201u8, 159u8, 27u8, 239u8,
                            239u8, 92u8, 103u8, 249u8, 223u8, 130u8, 160u8, 197u8, 28u8, 148u8,
                            240u8, 199u8, 211u8, 74u8, 16u8, 156u8, 37u8, 167u8, 161u8, 209u8,
                            13u8, 52u8,
                        ],
                    )
                }
                pub fn curators(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::curators::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::curators::Param0,
                    >,
                    types::curators::Curators,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Governance",
                        "Curators",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            122u8, 134u8, 173u8, 24u8, 32u8, 12u8, 201u8, 159u8, 27u8, 239u8,
                            239u8, 92u8, 103u8, 249u8, 223u8, 130u8, 160u8, 197u8, 28u8, 148u8,
                            240u8, 199u8, 211u8, 74u8, 16u8, 156u8, 37u8, 167u8, 161u8, 209u8,
                            13u8, 52u8,
                        ],
                    )
                }
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
                    #[codec(index = 0)]
                    add_curator {
                        key: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    remove_curator {
                        key: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                  ...
                    #[codec(index = 10)]
                    add_global_params_proposal {
                        data: runtime_types::pallet_governance::proposal::GlobalParamsData,
                        metadata: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 11)]
                    add_global_custom_proposal {
                        metadata: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 12)]
                    add_dao_treasury_transfer_proposal {
                        value: ::core::primitive::u128,
                        destination_key: ::subxt::ext::subxt_core::utils::AccountId32,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 13)]
                    vote_proposal {
                        proposal_id: ::core::primitive::u64,
                        agree: ::core::primitive::bool,
                    },
                    #[codec(index = 14)]
                    remove_vote_proposal { proposal_id: ::core::primitive::u64 },
                    #[codec(index = 15)]
                    enable_vote_delegation,
                    #[codec(index = 16)]
                    disable_vote_delegation,
                    #[codec(index = 17)]
                    add_emission_proposal {
                        recycling_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                        treasury_percentage: runtime_types::sp_arithmetic::per_things::Percent,
                        incentives_ratio: runtime_types::sp_arithmetic::per_things::Percent,
                        data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 18)]
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
                  ...n proposal"]
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
                    pub max_allowed_weights: ::core::primitive::u16,
                    pub min_stake_per_weight: ::core::primitive::u128,
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
        pub mod pallet_transaction_payment {
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
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
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
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
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
                       ...
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
              ...
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
               ...
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
                #[codec(index = 11)]
                Governance(runtime_types::pallet_governance::pallet::Call),
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
                #[codec(index = 11)]
                Governance(runtime_types::pallet_governance::pallet::Error),
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
                #[codec(index = 11)]
                Governance(runtime_types::pallet_governance::pallet::Event),
            }
        }
    }
}
