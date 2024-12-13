use std::{collections::BTreeMap, sync::Arc};

use jsonrpsee::RpcModule;
use polkadot_sdk::{
    sc_network::service::traits::NetworkService,
    sc_network_sync::SyncingService,
    sc_rpc::SubscriptionTaskExecutor,
    sc_transaction_pool::{ChainApi, Pool},
    sc_transaction_pool_api::TransactionPool,
    sp_core::H256,
    sp_inherents::CreateInherentDataProviders,
    sp_runtime::traits::Block as BlockT,
};
// Frontier
pub use fc_rpc::EthBlockDataCacheTask;
pub use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool};
use fc_storage::StorageOverride;
use fp_rpc::ConvertTransaction;
use torus_runtime::interface::Block;

use crate::service::{FullBackend, FullClient};

/// Extra dependencies for Ethereum compatibility.
pub struct EthDeps<P, A: ChainApi, CT, CIDP> {
    /// The client instance to use.
    pub client: Arc<FullClient>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Graph pool instance.
    pub graph: Arc<Pool<A>>,
    /// Ethereum transaction converter.
    pub converter: Option<CT>,
    /// The Node authority flag
    pub is_authority: bool,
    /// Whether to enable dev signer
    pub enable_dev_signer: bool,
    /// Network service
    pub network: Arc<dyn NetworkService>,
    /// Chain syncing service
    pub sync: Arc<SyncingService<Block>>,
    /// Frontier Backend.
    pub frontier_backend: Arc<dyn fc_api::Backend<Block>>,
    /// Ethereum data access overrides.
    pub storage_override: Arc<dyn StorageOverride<Block>>,
    /// Cache for Ethereum block data.
    pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
    /// EthFilterApi pool.
    pub filter_pool: Option<FilterPool>,
    /// Maximum number of logs in a query.
    pub max_past_logs: u32,
    /// Fee history cache.
    pub fee_history_cache: FeeHistoryCache,
    /// Maximum fee history cache size.
    pub fee_history_cache_limit: FeeHistoryCacheLimit,
    /// Maximum allowed gas limit will be ` block.gas_limit * execute_gas_limit_multiplier` when
    /// using eth_call/eth_estimateGas.
    pub execute_gas_limit_multiplier: u64,
    /// Mandated parent hashes for a given block hash.
    pub forced_parent_hashes: Option<BTreeMap<H256, H256>>,
    /// Something that can create the inherent data providers for pending state
    pub pending_create_inherent_data_providers: CIDP,
}

/// A struct that implements ETH config with default values;
pub struct DefaultEthConfig;

impl fc_rpc::EthConfig<Block, FullClient> for DefaultEthConfig {
    type EstimateGasAdapter = ();
    type RuntimeStorageOverride = fc_rpc::frontier_backend_client::SystemAccountId20StorageOverride<
        Block,
        FullClient,
        FullBackend,
    >;
}

type PubsubNotificationSinks = Arc<
    fc_mapping_sync::EthereumBlockNotificationSinks<
        fc_mapping_sync::EthereumBlockNotification<Block>,
    >,
>;

/// Instantiate Ethereum-compatible RPC extensions.
pub fn create_eth<P, A, CT, CIDP>(
    mut io: RpcModule<()>,
    deps: EthDeps<P, A, CT, CIDP>,
    subscription_task_executor: SubscriptionTaskExecutor,
    pubsub_notification_sinks: PubsubNotificationSinks,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    P: TransactionPool<Block = Block> + 'static,
    A: ChainApi<Block = Block> + 'static,
    CT: ConvertTransaction<<Block as BlockT>::Extrinsic> + Send + Sync + 'static,
    CIDP: CreateInherentDataProviders<Block, ()> + Send + 'static,
{
    use fc_rpc::{
        pending::AuraConsensusDataProvider, Debug, DebugApiServer, Eth, EthApiServer, EthDevSigner,
        EthFilter, EthFilterApiServer, EthPubSub, EthPubSubApiServer, EthSigner, Net, NetApiServer,
        Web3, Web3ApiServer,
    };
    use fc_rpc::{TxPool, TxPoolApiServer};

    let EthDeps {
        client,
        pool,
        graph,
        converter,
        is_authority,
        enable_dev_signer,
        network,
        sync,
        frontier_backend,
        storage_override,
        block_data_cache,
        filter_pool,
        max_past_logs,
        fee_history_cache,
        fee_history_cache_limit,
        execute_gas_limit_multiplier,
        forced_parent_hashes,
        pending_create_inherent_data_providers,
    } = deps;

    let mut signers = Vec::new();
    if enable_dev_signer {
        signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
    }

    io.merge(
        Eth::<Block, FullClient, P, CT, _, A, CIDP, DefaultEthConfig>::new(
            client.clone(),
            pool.clone(),
            graph.clone(),
            converter,
            sync.clone(),
            signers,
            storage_override.clone(),
            frontier_backend.clone(),
            is_authority,
            block_data_cache.clone(),
            fee_history_cache,
            fee_history_cache_limit,
            execute_gas_limit_multiplier,
            forced_parent_hashes,
            pending_create_inherent_data_providers,
            Some(Box::new(AuraConsensusDataProvider::new(client.clone()))),
        )
        .replace_config::<DefaultEthConfig>()
        .into_rpc(),
    )?;

    if let Some(filter_pool) = filter_pool {
        io.merge(
            EthFilter::new(
                client.clone(),
                frontier_backend.clone(),
                graph.clone(),
                filter_pool,
                500_usize, // max stored filters
                max_past_logs,
                block_data_cache.clone(),
            )
            .into_rpc(),
        )?;
    }

    io.merge(
        EthPubSub::new(
            pool,
            client.clone(),
            sync,
            subscription_task_executor,
            storage_override.clone(),
            pubsub_notification_sinks,
        )
        .into_rpc(),
    )?;

    io.merge(
        Net::new(
            client.clone(),
            network,
            // Whether to format the `peer_count` response as Hex (default) or not.
            true,
        )
        .into_rpc(),
    )?;

    io.merge(Web3::new(client.clone()).into_rpc())?;

    io.merge(
        Debug::new(
            client.clone(),
            frontier_backend,
            storage_override,
            block_data_cache,
        )
        .into_rpc(),
    )?;

    io.merge(TxPool::new(client, graph).into_rpc())?;

    Ok(io)
}
