// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use eth::EthDeps;
use futures::channel::mpsc;
use jsonrpsee::RpcModule;
use polkadot_sdk::{
    pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer},
    sc_consensus_manual_seal::{
        rpc::{ManualSeal, ManualSealApiServer},
        EngineCommand,
    },
    sc_rpc::SubscriptionTaskExecutor,
    sc_transaction_pool::ChainApi,
    sc_transaction_pool_api::TransactionPool,
    sp_inherents::CreateInherentDataProviders,
    sp_runtime::traits::Block as BlockT,
    substrate_frame_rpc_system::{System, SystemApiServer},
};
use std::sync::Arc;
use torus_runtime::{interface::Hash, opaque::Block};

use crate::service::FullClient;

/// ETH related RPC calls.
pub mod eth;

/// Full client dependencies.
pub struct FullDeps<P, A: ChainApi, CT, CIDP> {
    /// The client instance to use.
    pub client: Arc<FullClient>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Manual seal command sink
    pub command_sink: Option<mpsc::Sender<EngineCommand<Hash>>>,
    /// Ethereum-compatibility specific dependencies.
    pub eth: EthDeps<P, A, CT, CIDP>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<P, A, CT, CIDP>(
    deps: FullDeps<P, A, CT, CIDP>,
    subscription_task_executor: SubscriptionTaskExecutor,
    pubsub_notification_sinks: Arc<
        fc_mapping_sync::EthereumBlockNotificationSinks<
            fc_mapping_sync::EthereumBlockNotification<Block>,
        >,
    >,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    P: TransactionPool<Block = Block> + 'static,
    A: ChainApi<Block = Block> + 'static,
    CIDP: CreateInherentDataProviders<Block, ()> + Send + 'static,
    CT: fp_rpc::ConvertTransaction<<Block as BlockT>::Extrinsic> + Send + Sync + 'static,
{
    let mut io = RpcModule::new(());
    let FullDeps {
        client,
        pool,
        command_sink,
        eth,
    } = deps;

    io.merge(System::new(client.clone(), pool).into_rpc())?;
    io.merge(TransactionPayment::new(client).into_rpc())?;

    if let Some(command_sink) = command_sink {
        io.merge(
            // We provide the rpc handler with the sending end of the channel to allow the rpc
            // send EngineCommands to the background block authorship task.
            ManualSeal::new(command_sink).into_rpc(),
        )?;
    }

    // Ethereum compatibility RPCs
    let io = eth::create_eth::<_, _, _, _>(
        io,
        eth,
        subscription_task_executor,
        pubsub_notification_sinks,
    )?;

    Ok(io)
}
