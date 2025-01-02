use crate::{
    configs::{currency, WEIGHT_REF_TIME_PER_SECOND},
    TransactionPayment,
};
use codec::{Decode, Encode};
use pallet_ethereum::PostLogContent;
use pallet_evm::{FeeCalculator, HashedAddressMapping};
use polkadot_sdk::{
    frame_support::{parameter_types, traits::FindAuthor},
    frame_system, pallet_aura,
    polkadot_sdk_frame::prelude::{TransactionValidity, TransactionValidityError},
    sp_core::{ConstU32, H160, U256},
    sp_runtime::{
        traits::{BlakeTwo256, Block as BlockT, DispatchInfoOf, Dispatchable, PostDispatchInfoOf},
        ConsensusEngineId, DispatchResultWithInfo, FixedPointNumber, RuntimeAppPublic,
    },
    sp_weights::Weight,
};

use super::{
    precompiles::FrontierPrecompiles, Aura, Balances, Runtime, RuntimeCall, RuntimeEvent,
    RuntimeOrigin, Timestamp, UncheckedExtrinsic, NORMAL_DISPATCH_RATIO,
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

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 2000ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 2 * 0.75 ~= 60_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND / GAS_PER_SECOND;

const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

/// Maximum weight per block
pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND, u64::MAX)
    .saturating_mul(2)
    .set_proof_size(MAX_POV_SIZE);

parameter_types! {
    pub BlockGasLimit: U256
        = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
    pub const GasLimitPovSizeRatio: u64 = 16;
    /// The amount of gas per storage (in bytes): BLOCK_GAS_LIMIT / BLOCK_STORAGE_LIMIT
    /// The current definition of BLOCK_STORAGE_LIMIT is 160 KB, resulting in a value of 366.
    pub GasLimitStorageGrowthRatio: u64 = 366;
    pub PrecompilesValue: FrontierPrecompiles = FrontierPrecompiles;
    pub EthereumChainId: u64 = 21000;
    pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
}

pub struct TransactionPaymentAsGasPrice;
impl FeeCalculator for TransactionPaymentAsGasPrice {
    fn min_gas_price() -> (U256, Weight) {
        // note: transaction-payment differs from EIP-1559 in that its tip and length fees are not
        //       scaled by the multiplier, which means its multiplier will be overstated when
        //       applied to an ethereum transaction
        // note: transaction-payment uses both a congestion modifier (next_fee_multiplier, which is
        //       updated once per block in on_finalize) and a 'WeightToFee' implementation. Our
        //       runtime implements this as a 'ConstantModifier', so we can get away with a simple
        //       multiplication here.
        // It is imperative that `saturating_mul_int` be performed as late as possible in the
        // expression since it involves fixed point multiplication with a division by a fixed
        // divisor. This leads to truncation and subsequent precision loss if performed too early.
        // This can lead to min_gas_price being same across blocks even if the multiplier changes.
        // There's still some precision loss when the final `gas_price` (used_gas * min_gas_price)
        // is computed in frontier, but that's currently unavoidable.
        let min_gas_price = TransactionPayment::next_fee_multiplier()
            .saturating_mul_int((currency::WEIGHT_FEE).saturating_mul(WEIGHT_PER_GAS as u128));
        (
            min_gas_price.into(),
            <Runtime as frame_system::Config>::DbWeight::get().reads(1),
        )
    }
}

impl pallet_evm::Config for Runtime {
    type FeeCalculator = TransactionPaymentAsGasPrice;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas; // todo: check
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type CallOrigin = pallet_evm::EnsureAddressTruncated;
    type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type PrecompilesType = FrontierPrecompiles;
    type PrecompilesValue = PrecompilesValue;
    type ChainId = EthereumChainId;
    type OnChargeTransaction = ();
    type BlockGasLimit = BlockGasLimit;
    type FindAuthor = FindAuthorTruncated<Aura>;
    type OnCreate = ();
    type GasLimitPovSizeRatio = GasLimitPovSizeRatio; // todo: check
    type SuicideQuickClearLimit = ConstU32<0>;
    type GasLimitStorageGrowthRatio = GasLimitStorageGrowthRatio; // todo: check
    type Timestamp = Timestamp;
    type AccountProvider = pallet_evm::FrameSystemAccountProvider<Self>;
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
pub struct TransactionConverter<B>(core::marker::PhantomData<B>);

impl<B> Default for TransactionConverter<B> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Block: BlockT> fp_rpc::ConvertTransaction<Block::Extrinsic> for TransactionConverter<Block> {
    fn convert_transaction(&self, transaction: pallet_ethereum::Transaction) -> Block::Extrinsic {
        let extrinsic = UncheckedExtrinsic::new_unsigned(
            pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
        );
        let encoded = extrinsic.encode();
        Block::Extrinsic::decode(&mut &encoded[..]).expect("Encoded extrinsic is always valid")
    }
}
