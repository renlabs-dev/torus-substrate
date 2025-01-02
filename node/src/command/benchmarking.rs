//! Setup code for [`super::command`] which would otherwise bloat that module.
//!
//! Should only be used for benchmarking as it may break in other contexts.

use crate::service::FullClient;

use polkadot_sdk::{
    sc_cli::Result,
    sc_client_api::BlockBackend,
    sp_core::{Encode, Pair},
    sp_inherents::{InherentData, InherentDataProvider},
    sp_keyring::Sr25519Keyring,
    sp_runtime::{OpaqueExtrinsic, SaturatedConversion},
    *,
};
use torus_runtime::interface::{AccountId, Balance, BalancesCall, SystemCall};

use std::{sync::Arc, time::Duration};

/// Generates extrinsics for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder {
    client: Arc<FullClient>,
}

impl RemarkBuilder {
    /// Creates a new [`Self`] from the given client.
    pub fn new(client: Arc<FullClient>) -> Self {
        Self { client }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder {
    fn pallet(&self) -> &str {
        "system"
    }

    fn extrinsic(&self) -> &str {
        "remark"
    }

    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = Sr25519Keyring::Bob.pair();
        let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
            self.client.as_ref(),
            acc,
            SystemCall::remark { remark: vec![] }.into(),
            nonce,
        )
        .into();

        Ok(extrinsic)
    }
}

/// Generates `Balances::TransferKeepAlive` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct TransferKeepAliveBuilder {
    client: Arc<FullClient>,
    dest: AccountId,
    value: Balance,
}

impl TransferKeepAliveBuilder {
    /// Creates a new [`Self`] from the given client.
    pub fn new(client: Arc<FullClient>, dest: AccountId, value: Balance) -> Self {
        Self {
            client,
            dest,
            value,
        }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for TransferKeepAliveBuilder {
    fn pallet(&self) -> &str {
        "balances"
    }

    fn extrinsic(&self) -> &str {
        "transfer_keep_alive"
    }

    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = Sr25519Keyring::Bob.pair();
        let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
            self.client.as_ref(),
            acc,
            BalancesCall::transfer_keep_alive {
                dest: self.dest.clone().into(),
                value: self.value,
            }
            .into(),
            nonce,
        )
        .into();

        Ok(extrinsic)
    }
}

/// Create a transaction using the given `call`.
///
/// Note: Should only be used for benchmarking.
pub fn create_benchmark_extrinsic(
    client: &FullClient,
    sender: sp_core::sr25519::Pair,
    call: torus_runtime::RuntimeCall,
    nonce: u32,
) -> torus_runtime::interface::UncheckedExtrinsic {
    let genesis_hash = client
        .block_hash(0)
        .ok()
        .flatten()
        .expect("Genesis block exists; qed");
    let best_hash = client.chain_info().best_hash;
    let best_block = client.chain_info().best_number;

    let period = torus_runtime::configs::BlockHashCount::get()
        .checked_next_power_of_two()
        .map(|c| c / 2)
        .unwrap_or(2);
    let tx_ext: torus_runtime::SignedExtra = (
        frame_system::CheckNonZeroSender::<torus_runtime::Runtime>::new(),
        frame_system::CheckSpecVersion::<torus_runtime::Runtime>::new(),
        frame_system::CheckTxVersion::<torus_runtime::Runtime>::new(),
        frame_system::CheckGenesis::<torus_runtime::Runtime>::new(),
        frame_system::CheckEra::<torus_runtime::Runtime>::from(sp_runtime::generic::Era::mortal(
            period,
            best_block.saturated_into(),
        )),
        frame_system::CheckNonce::<torus_runtime::Runtime>::from(nonce),
        frame_system::CheckWeight::<torus_runtime::Runtime>::new(),
        pallet_transaction_payment::ChargeTransactionPayment::<torus_runtime::Runtime>::from(0),
        frame_metadata_hash_extension::CheckMetadataHash::<torus_runtime::Runtime>::new(false),
    );

    let raw_payload = torus_runtime::SignedPayload::from_raw(
        call.clone(),
        tx_ext.clone(),
        (
            (),
            torus_runtime::VERSION.spec_version,
            torus_runtime::VERSION.transaction_version,
            genesis_hash,
            best_hash,
            (),
            (),
            (),
            None,
        ),
    );
    let signature = raw_payload.using_encoded(|e| sender.sign(e));

    torus_runtime::interface::UncheckedExtrinsic::new_signed(
        call,
        sp_runtime::AccountId32::from(sender.public()).into(),
        torus_runtime::interface::Signature::Sr25519(signature),
        tx_ext,
    )
}

/// Generates inherent data for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub fn inherent_benchmark_data() -> Result<InherentData> {
    let mut inherent_data = InherentData::new();
    let d = Duration::from_millis(0);
    let timestamp = sp_timestamp::InherentDataProvider::new(d.into());

    futures::executor::block_on(timestamp.provide_inherent_data(&mut inherent_data))
        .map_err(|e| format!("creating inherent data: {:?}", e))?;
    Ok(inherent_data)
}
