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

use std::{pin::Pin, sync::Arc, time::Duration};

use fc_rpc::StorageOverrideHandler;
use futures::FutureExt;
use polkadot_sdk::{
    sc_client_api::{Backend, BlockBackend},
    sc_executor::WasmExecutor,
    sc_network_sync::strategy::warp::WarpSyncProvider,
    sc_service::{error::Error as ServiceError, Configuration, TaskManager, WarpSyncConfig},
    sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker},
    sc_transaction_pool_api::OffchainTransactionPoolFactory,
    sp_consensus_aura::sr25519::AuthorityPair,
    sp_runtime::traits::Block as BlockT,
    *,
};
use torus_runtime::{apis::RuntimeApi, configs::eth::TransactionConverter, opaque::Block};

use crate::cli::{
    eth::{
        db_config_dir, new_frontier_partial, spawn_frontier_tasks, EthConfiguration,
        FrontierPartialComponents,
    },
    Consensus,
};

type HostFunctions = sp_io::SubstrateHostFunctions;

pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, WasmExecutor<HostFunctions>>;
pub type FullBackend = sc_service::TFullBackend<Block>;

type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

type BasicImportQueue = sc_consensus::DefaultImportQueue<Block>;
type BoxBlockImport = sc_consensus::BoxBlockImport<Block>;

type GrandpaBlockImport =
    sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;
type GrandpaLinkHalf = sc_consensus_grandpa::LinkHalf<Block, FullClient, FullSelectChain>;

/// Assembly of PartialComponents (enough to run chain ops subcommands)
pub type Service = sc_service::PartialComponents<
    FullClient,
    FullBackend,
    FullSelectChain,
    sc_consensus::DefaultImportQueue<Block>,
    sc_transaction_pool::FullPool<Block, FullClient>,
    (Option<Telemetry>, BoxBlockImport, GrandpaLinkHalf),
>;

pub fn new_partial(
    config: &Configuration,
    eth_config: &EthConfiguration,
    consensus: Consensus,
) -> Result<Service, ServiceError> {
    let telemetry = set_telemetry(config)?;

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            sc_service::new_wasm_executor(&config.executor),
        )?;

    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
        client.clone(),
        512,
        &client,
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    let import_queue_builder = match consensus {
        Consensus::Aura => build_aura_grandpa_import_queue,
        Consensus::ManualSeal(_) | Consensus::InstantSeal => build_manual_seal_import_queue,
    };

    let (import_queue, block_import) = import_queue_builder(
        client.clone(),
        config,
        eth_config,
        &task_manager,
        telemetry.as_ref().map(|x| x.handle()),
        grandpa_block_import,
    )?;

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (telemetry, block_import, grandpa_link),
    })
}

fn set_telemetry(
    config: &Configuration,
) -> Result<Option<(TelemetryWorker, Telemetry)>, ServiceError> {
    config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()
        .map_err(Into::into)
}

type InherentDataProviders = (
    sp_consensus_aura::inherents::InherentDataProvider,
    sp_timestamp::InherentDataProvider,
    fp_dynamic_fee::InherentDataProvider,
);

fn aura_data_provider(
    slot_duration: sp_consensus_aura::SlotDuration,
    eth_config: &EthConfiguration,
) -> impl Fn(
    sp_core::H256,
    (),
) -> Pin<
    Box<
        dyn std::future::Future<
                Output = Result<InherentDataProviders, Box<dyn std::error::Error + Send + Sync>>,
            > + Send
            + Sync,
    >,
> {
    let target_gas_price = eth_config.target_gas_price;
    move |_, ()| {
        Box::pin(async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            let slot =
            sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                *timestamp,
                slot_duration,
            );
            let dynamic_fee = fp_dynamic_fee::InherentDataProvider(target_gas_price.into());
            Ok((slot, timestamp, dynamic_fee))
        })
    }
}

/// Build the import queue for the template runtime (aura + grandpa).
fn build_aura_grandpa_import_queue(
    client: Arc<FullClient>,
    config: &Configuration,
    eth_config: &EthConfiguration,
    task_manager: &TaskManager,
    telemetry: Option<TelemetryHandle>,
    grandpa_block_import: GrandpaBlockImport,
) -> Result<(BasicImportQueue, BoxBlockImport), ServiceError> {
    let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

    let import_queue = sc_consensus_aura::import_queue::<AuthorityPair, _, _, _, _, _>(
        sc_consensus_aura::ImportQueueParams {
            block_import: grandpa_block_import.clone(),
            justification_import: Some(Box::new(grandpa_block_import.clone())),
            client,
            create_inherent_data_providers: aura_data_provider(slot_duration, eth_config),
            spawner: &task_manager.spawn_essential_handle(),
            registry: config.prometheus_registry(),
            check_for_equivocation: Default::default(),
            telemetry,
            compatibility_mode: sc_consensus_aura::CompatibilityMode::None,
        },
    )
    .map_err::<ServiceError, _>(Into::into)?;

    Ok((import_queue, Box::new(grandpa_block_import)))
}

/// Build the import queue for the template runtime (manual seal).
fn build_manual_seal_import_queue(
    client: Arc<FullClient>,
    config: &Configuration,
    _eth_config: &EthConfiguration,
    task_manager: &TaskManager,
    _telemetry: Option<TelemetryHandle>,
    _grandpa_block_import: GrandpaBlockImport,
) -> Result<(BasicImportQueue, BoxBlockImport), ServiceError> {
    let import_queue = sc_consensus_manual_seal::import_queue(
        Box::new(client.clone()),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    );

    Ok((import_queue, Box::new(client)))
}

/// Builds a new service for a full client.
pub async fn new_full<Network: sc_network::NetworkBackend<Block, <Block as BlockT>::Hash>>(
    mut config: Configuration,
    eth_config: EthConfiguration,
    consensus: Consensus,
) -> Result<TaskManager, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (mut telemetry, block_import, grandpa_link),
    } = new_partial(&config, &eth_config, consensus)?;

    let mut net_config = sc_network::config::FullNetworkConfiguration::<
        Block,
        <Block as BlockT>::Hash,
        Network,
    >::new(
        &config.network,
        config
            .prometheus_config
            .as_ref()
            .map(|cfg| cfg.registry.clone()),
    );
    let peer_store_handle = net_config.peer_store_handle();
    let metrics = Network::register_notification_metrics(
        config.prometheus_config.as_ref().map(|cfg| &cfg.registry),
    );

    let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
        &client.block_hash(0)?.expect("Genesis block exists; qed"),
        &config.chain_spec,
    );

    let (grandpa_protocol_config, grandpa_notification_service) =
        sc_consensus_grandpa::grandpa_peers_set_config::<_, Network>(
            grandpa_protocol_name.clone(),
            metrics.clone(),
            peer_store_handle,
        );

    let warp_sync_config = if matches!(consensus, Consensus::Aura) {
        net_config.add_notification_protocol(grandpa_protocol_config);
        let warp_sync: Arc<dyn WarpSyncProvider<Block>> =
            Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
                backend.clone(),
                grandpa_link.shared_authority_set().clone(),
                Vec::default(),
            ));
        Some(WarpSyncConfig::WithProvider(warp_sync))
    } else {
        None
    };

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            net_config,
            block_announce_validator_builder: None,
            warp_sync_config,
            block_relay: None,
            metrics,
        })?;

    if config.offchain_worker.enabled {
        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-worker",
            sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
                runtime_api_provider: client.clone(),
                is_validator: config.role.is_authority(),
                keystore: Some(keystore_container.keystore()),
                offchain_db: backend.offchain_storage(),
                transaction_pool: Some(OffchainTransactionPoolFactory::new(
                    transaction_pool.clone(),
                )),
                network_provider: Arc::new(network.clone()),
                enable_http_requests: true,
                custom_extensions: |_| vec![],
            })
            .run(client.clone(), task_manager.spawn_handle())
            .boxed(),
        );
    }

    // let role = config.role;
    // let force_authoring = config.force_authoring;
    // let name = config.network.node_name.clone();

    // let prometheus_registry = config.prometheus_registry().cloned();

    let proposer = sc_basic_authorship::ProposerFactory::new(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool.clone(),
        config.prometheus_registry(),
        telemetry.as_ref().map(|x| x.handle()),
    );

    let mut command_sink = None;
    match consensus {
        Consensus::Aura => {
            let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

            let aura = sc_consensus_aura::start_aura::<AuthorityPair, _, _, _, _, _, _, _, _, _, _>(
                sc_consensus_aura::StartAuraParams {
                    slot_duration,
                    client: client.clone(),
                    select_chain,
                    block_import,
                    proposer_factory: proposer,
                    sync_oracle: sync_service.clone(),
                    justification_sync_link: sync_service.clone(),
                    create_inherent_data_providers: aura_data_provider(slot_duration, &eth_config),
                    force_authoring: config.force_authoring,
                    backoff_authoring_blocks: Option::<()>::None,
                    keystore: keystore_container.keystore(),
                    block_proposal_slot_portion: sc_consensus_aura::SlotProportion::new(
                        2f32 / 3f32,
                    ),
                    max_block_proposal_slot_portion: None,
                    telemetry: telemetry.as_ref().map(|x| x.handle()),
                    compatibility_mode: sc_consensus_aura::CompatibilityMode::None,
                },
            )?;

            task_manager.spawn_essential_handle().spawn_blocking(
                "aura",
                Some("block-authoring"),
                aura,
            );

            // if the node isn't actively participating in consensus then it doesn't
            // need a keystore, regardless of which protocol we use below.
            let keystore = if config.role.is_authority() {
                Some(keystore_container.keystore())
            } else {
                None
            };

            let grandpa_config = sc_consensus_grandpa::Config {
                // FIXME #1578 make this available through chainspec
                gossip_duration: Duration::from_millis(333),
                justification_generation_period: 512,
                name: Some(config.network.node_name.clone()),
                observer_enabled: false,
                keystore,
                local_role: config.role,
                telemetry: telemetry.as_ref().map(|x| x.handle()),
                protocol_name: grandpa_protocol_name,
            };

            // start the full GRANDPA voter
            // NOTE: non-authorities could run the GRANDPA observer protocol, but at
            // this point the full voter should provide better guarantees of block
            // and vote data availability than the observer. The observer has not
            // been tested extensively yet and having most nodes in a network run it
            // could lead to finality stalls.
            let grandpa_voter =
                sc_consensus_grandpa::run_grandpa_voter(sc_consensus_grandpa::GrandpaParams {
                    config: grandpa_config,
                    link: grandpa_link,
                    network: network.clone(),
                    sync: sync_service.clone(),
                    notification_service: grandpa_notification_service,
                    voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
                    prometheus_registry: config.prometheus_registry().cloned(),
                    shared_voter_state: sc_consensus_grandpa::SharedVoterState::empty(),
                    telemetry: telemetry.as_ref().map(|x| x.handle()),
                    offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(
                        transaction_pool.clone(),
                    ),
                })?;

            task_manager.spawn_essential_handle().spawn_blocking(
                "grandpa-voter",
                None,
                grandpa_voter,
            );
        }
        Consensus::InstantSeal => {
            let params = sc_consensus_manual_seal::InstantSealParams {
                block_import: client.clone(),
                env: proposer,
                client: client.clone(),
                pool: transaction_pool.clone(),
                select_chain,
                consensus_data_provider: None,
                create_inherent_data_providers: move |_, ()| async move {
                    Ok(sp_timestamp::InherentDataProvider::from_system_time())
                },
            };

            let authorship_future = sc_consensus_manual_seal::run_instant_seal(params);

            task_manager.spawn_essential_handle().spawn_blocking(
                "instant-seal",
                None,
                authorship_future,
            );
        }
        Consensus::ManualSeal(block_time) => {
            let (mut sink, commands_stream) = futures::channel::mpsc::channel(1024);

            command_sink = Some(sink.clone());

            task_manager
                .spawn_handle()
                .spawn("block_authoring", None, async move {
                    loop {
                        futures_timer::Delay::new(std::time::Duration::from_millis(block_time))
                            .await;
                        sink.try_send(sc_consensus_manual_seal::EngineCommand::SealNewBlock {
                            create_empty: true,
                            finalize: true,
                            parent_hash: None,
                            sender: None,
                        })
                        .unwrap();
                    }
                });

            let params = sc_consensus_manual_seal::ManualSealParams {
                block_import: client.clone(),
                env: proposer,
                client: client.clone(),
                pool: transaction_pool.clone(),
                select_chain,
                commands_stream: Box::pin(commands_stream),
                consensus_data_provider: None,
                create_inherent_data_providers: move |_, ()| async move {
                    Ok(sp_timestamp::InherentDataProvider::from_system_time())
                },
            };
            let authorship_future = sc_consensus_manual_seal::run_manual_seal(params);

            task_manager.spawn_essential_handle().spawn_blocking(
                "manual-seal",
                None,
                authorship_future,
            );
        }
    }

    let FrontierPartialComponents {
        filter_pool,
        fee_history_cache,
        fee_history_cache_limit,
    } = new_frontier_partial(&eth_config)?;

    let frontier_backend = Arc::new(fc_db::kv::Backend::open(
        client.clone(),
        &config.database,
        &db_config_dir(&config),
    )?);

    let storage_override = Arc::new(StorageOverrideHandler::new(client.clone()));

    // Sinks for pubsub notifications.
    // Every time a new subscription is created, a new mpsc channel is added to the
    // sink pool. The MappingSyncWorker sends through the channel on block
    // import and the subscription emits a notification to the subscriber on
    // receiving a message through this channel. This way we avoid race
    // conditions when using native substrate block import notification stream.
    let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
        fc_mapping_sync::EthereumBlockNotification<Block>,
    > = Default::default();
    let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);

    // for ethereum-compatibility rpc.
    config.rpc.id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let network = network.clone();
        let sync_service = sync_service.clone();

        let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
        let frontier_backend = Arc::clone(&frontier_backend);
        let filter_pool = filter_pool.clone();
        let fee_history_cache = fee_history_cache.clone();
        let storage_override = storage_override.clone();
        let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
            task_manager.spawn_handle(),
            storage_override.clone(),
            eth_config.eth_log_block_cache,
            eth_config.eth_statuses_cache,
            config.prometheus_registry().cloned(),
        ));
        let pubsub_notification_sinks = pubsub_notification_sinks.clone();

        Box::new(move |executor| {
            let eth_deps = crate::rpc::eth::EthDeps {
                client: client.clone(),
                pool: pool.clone(),
                graph: pool.pool().clone(),
                converter: Some(TransactionConverter::<Block>::default()),
                is_authority: config.role.is_authority(),
                enable_dev_signer: eth_config.enable_dev_signer,
                network: network.clone(),
                sync: sync_service.clone(),
                frontier_backend: frontier_backend.clone(),
                storage_override: storage_override.clone(),
                block_data_cache: block_data_cache.clone(),
                filter_pool: filter_pool.clone(),
                max_past_logs: eth_config.max_past_logs,
                fee_history_cache: fee_history_cache.clone(),
                fee_history_cache_limit,
                execute_gas_limit_multiplier: eth_config.execute_gas_limit_multiplier,
                forced_parent_hashes: None,
                pending_create_inherent_data_providers: aura_data_provider(
                    slot_duration,
                    &eth_config,
                ),
            };

            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                command_sink: command_sink.clone(),
                eth: eth_deps,
            };

            crate::rpc::create_full(deps, executor, Arc::clone(&pubsub_notification_sinks))
                .map_err(Into::into)
        })
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_extensions_builder,
        backend: backend.clone(),
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        config,
        telemetry: telemetry.as_mut(),
    })?;

    spawn_frontier_tasks(
        &task_manager,
        client,
        backend,
        frontier_backend,
        filter_pool,
        storage_override,
        fee_history_cache,
        fee_history_cache_limit,
        sync_service,
        pubsub_notification_sinks,
    )
    .await;

    network_starter.start_network();
    Ok(task_manager)
}
