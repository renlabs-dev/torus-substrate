use codec::{Decode, Encode};
use fp_evm::weight_per_gas;
use pallet_ethereum::PostLogContent;
use pallet_evm::{EnsureAddressOrigin, HashedAddressMapping};
use polkadot_sdk::{
    frame_support::{parameter_types, traits::FindAuthor},
    frame_system::RawOrigin,
    pallet_aura,
    polkadot_sdk_frame::prelude::{TransactionValidity, TransactionValidityError},
    sp_core::{ConstU32, H160, U256},
    sp_runtime::{
        traits::{BlakeTwo256, Block as BlockT, DispatchInfoOf, Dispatchable, PostDispatchInfoOf},
        AccountId32, ConsensusEngineId, DispatchResultWithInfo, Permill, RuntimeAppPublic,
    },
    sp_weights::Weight,
};

use super::{
    precompiles::FrontierPrecompiles, Aura, Balances, BaseFee, Block, Runtime, RuntimeCall,
    RuntimeEvent, RuntimeOrigin, Timestamp, UncheckedExtrinsic, NORMAL_DISPATCH_RATIO,
};

impl pallet_evm_chain_id::Config for Runtime {}

pub struct FindAuthorTruncated<F>(core::marker::PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
    fn find_author<'a, I>(digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        if let Some(author_index) = F::find_author(digests) {
            let authority_id =
                pallet_aura::Authorities::<Runtime>::get()[author_index as usize].clone();
            return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]));
        }
        None
    }
}

pub struct EnsureCuratorAddressTruncated;

impl EnsureAddressOrigin<RuntimeOrigin> for EnsureCuratorAddressTruncated {
    type Success = AccountId32;

    fn try_address_origin(
        address: &H160,
        origin: RuntimeOrigin,
    ) -> Result<AccountId32, RuntimeOrigin> {
        <RuntimeOrigin as Into<Result<RawOrigin<AccountId32>, RuntimeOrigin>>>::into(origin)
            .and_then(|o| match o {
                RawOrigin::Signed(who) => {
                    let address_matches = AsRef::<[u8; 32]>::as_ref(&who)[0..20] == address[0..20];

                    if !address_matches {
                        return Err(RuntimeOrigin::from(RawOrigin::Signed(who)));
                    }

                    // TODO:
                    if pallet_governance::Curators::<Runtime>::contains_key(&who) {
                        return Err(RuntimeOrigin::from(RawOrigin::Signed(who)));
                    }

                    Ok(who)
                }
                r => Err(RuntimeOrigin::from(r)),
            })
    }
}

const WEIGHT_MILLISECS_PER_BLOCK: u64 = 2000;

const BLOCK_GAS_LIMIT: u64 = 75_000_000;
const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;
/// The maximum storage growth per block in bytes.
const MAX_STORAGE_GROWTH: u64 = 400 * 1024;

parameter_types! {
    pub BlockGasLimit: U256 = U256::from(BLOCK_GAS_LIMIT);
    pub const GasLimitPovSizeRatio: u64 = BLOCK_GAS_LIMIT.saturating_div(MAX_POV_SIZE);
    pub const GasLimitStorageGrowthRatio: u64 = BLOCK_GAS_LIMIT.saturating_div(MAX_STORAGE_GROWTH);
    pub PrecompilesValue: FrontierPrecompiles = FrontierPrecompiles;
    pub WeightPerGas: Weight = Weight::from_parts(weight_per_gas(BLOCK_GAS_LIMIT, NORMAL_DISPATCH_RATIO, WEIGHT_MILLISECS_PER_BLOCK), 0);
    pub SuicideQuickClearLimit: u32 = 0;
    pub EthereumChainId: u64 = 21000;
}

impl pallet_evm::Config for Runtime {
    type AccountProvider = pallet_evm::FrameSystemAccountProvider<Self>;
    type FeeCalculator = BaseFee;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type CallOrigin = EnsureCuratorAddressTruncated;
    type WithdrawOrigin = EnsureCuratorAddressTruncated;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type PrecompilesType = FrontierPrecompiles;
    type PrecompilesValue = PrecompilesValue;
    type ChainId = EthereumChainId;
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type OnChargeTransaction = ();
    type OnCreate = ();
    type FindAuthor = FindAuthorTruncated<Aura>;
    type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
    type SuicideQuickClearLimit = SuicideQuickClearLimit;
    type GasLimitStorageGrowthRatio = GasLimitStorageGrowthRatio;
    type Timestamp = Timestamp;
    type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}

parameter_types! {
    pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StateRoot = pallet_ethereum::IntermediateStateRoot<Self::Version>;
    type PostLogContent = PostBlockAndTxnHashes;
    type ExtraDataLength = ConstU32<30>;
}

parameter_types! {
    pub BoundDivision: U256 = U256::from(1024);
}

impl pallet_dynamic_fee::Config for Runtime {
    type MinGasPriceBoundDivisor = BoundDivision;
}

parameter_types! {
    pub DefaultBaseFeePerGas: U256 = U256::from(1_000_000_000);
    pub DefaultElasticity: Permill = Permill::from_parts(125_000);
}
pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
    fn lower() -> Permill {
        Permill::zero()
    }
    fn ideal() -> Permill {
        Permill::from_parts(500_000)
    }
    fn upper() -> Permill {
        Permill::from_parts(1_000_000)
    }
}
impl pallet_base_fee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Threshold = BaseFeeThreshold;
    type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
    type DefaultElasticity = DefaultElasticity;
}

impl fp_self_contained::SelfContainedCall for RuntimeCall {
    type SignedInfo = H160;

    fn is_self_contained(&self) -> bool {
        match self {
            RuntimeCall::Ethereum(call) => call.is_self_contained(),
            _ => false,
        }
    }

    fn check_self_contained(&self) -> Option<Result<Self::SignedInfo, TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) => call.check_self_contained(),
            _ => None,
        }
    }

    fn validate_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<TransactionValidity> {
        match self {
            RuntimeCall::Ethereum(call) => call.validate_self_contained(info, dispatch_info, len),
            _ => None,
        }
    }

    fn pre_dispatch_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<Result<(), TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) => {
                call.pre_dispatch_self_contained(info, dispatch_info, len)
            }
            _ => None,
        }
    }

    fn apply_self_contained(
        self,
        info: Self::SignedInfo,
    ) -> Option<DispatchResultWithInfo<PostDispatchInfoOf<Self>>> {
        match self {
            call @ RuntimeCall::Ethereum(pallet_ethereum::Call::transact { .. }) => {
                Some(call.dispatch(RuntimeOrigin::from(
                    pallet_ethereum::RawOrigin::EthereumTransaction(info),
                )))
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct TransactionConverter;

impl fp_rpc::ConvertTransaction<<Block as BlockT>::Extrinsic> for TransactionConverter {
    fn convert_transaction(
        &self,
        transaction: pallet_ethereum::Transaction,
    ) -> <Block as BlockT>::Extrinsic {
        let extrinsic = UncheckedExtrinsic::new_unsigned(
            pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
        );
        let encoded = extrinsic.encode();
        <Block as BlockT>::Extrinsic::decode(&mut &encoded[..])
            .expect("Encoded extrinsic is always valid")
    }
}
