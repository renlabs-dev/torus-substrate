#![doc = r" Generated storage wrapper functions"]
#![doc = r""]
#![doc = r" This module provides ergonomic access to Substrate storage items."]
#![doc = r" Functions are automatically generated from the subxt API metadata."]
#![allow(dead_code)]
use crate::interfaces::mainnet::api;
use crate::interfaces::mainnet::api::runtime_types;
use codec::Decode;
use std::collections::HashMap;
use subxt::{OnlineClient, PolkadotConfig};
#[doc = r" Get storage value"]
pub async fn get_aura_authorities(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().aura().authorities()).await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_aura_current_slot(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::sp_consensus_slots::Slot>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().aura().current_slot()).await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_balances_account_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<
    Option<runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().balances().account(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_balances_account(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<String, runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().balances().account_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_balances_freezes_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<
    Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                (),
                ::core::primitive::u128,
            >,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().balances().freezes(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_balances_freezes(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        String,
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                (),
                ::core::primitive::u128,
            >,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().balances().freezes_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_balances_holds_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<
    Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                runtime_types::torus_runtime::RuntimeHoldReason,
                ::core::primitive::u128,
            >,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().balances().holds(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_balances_holds(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        String,
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::frame_support::traits::tokens::misc::IdAmount<
                runtime_types::torus_runtime::RuntimeHoldReason,
                ::core::primitive::u128,
            >,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage.iter(api::storage().balances().holds_iter()).await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_balances_inactive_issuance(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().balances().inactive_issuance())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_balances_locks_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<
    Option<
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
            runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().balances().locks(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_balances_locks(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        String,
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
            runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage.iter(api::storage().balances().locks_iter()).await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_balances_reserves_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<
    Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::pallet_balances::types::ReserveData<(), ::core::primitive::u128>,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().balances().reserves(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_balances_reserves(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        String,
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::pallet_balances::types::ReserveData<(), ::core::primitive::u128>,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().balances().reserves_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_balances_total_issuance(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().balances().total_issuance())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_emission0_consensus_members_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<runtime_types::pallet_emission0::ConsensusMember>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().emission0().consensus_members(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_emission0_consensus_members(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<String, runtime_types::pallet_emission0::ConsensusMember>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().emission0().consensus_members_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_emission0_emission_recycling_percentage(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::sp_arithmetic::per_things::Percent>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().emission0().emission_recycling_percentage())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_emission0_incentives_ratio(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::sp_arithmetic::per_things::Percent>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().emission0().incentives_ratio())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_emission0_max_allowed_weights(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().emission0().max_allowed_weights())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_emission0_min_allowed_weights(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().emission0().min_allowed_weights())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_emission0_min_stake_per_weight(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().emission0().min_stake_per_weight())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_emission0_pending_emission(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().emission0().pending_emission())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_emission0_weight_control_delegation_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<::subxt::ext::subxt_core::utils::AccountId32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(
            &api::storage()
                .emission0()
                .weight_control_delegation(account_id32),
        )
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_emission0_weight_control_delegation(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, ::subxt::ext::subxt_core::utils::AccountId32>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().emission0().weight_control_delegation_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_ethereum_block_hash_by_u256(
    client: &OnlineClient<PolkadotConfig>,
    u256: runtime_types::primitive_types::U256,
) -> Result<Option<::subxt::ext::subxt_core::utils::H256>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().ethereum().block_hash(u256))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_ethereum_block_hash(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, ::subxt::ext::subxt_core::utils::H256>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().ethereum().block_hash_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = runtime_types::primitive_types::U256::decode(&mut &kv.key_bytes[32..]) {
            result.insert(format!("{:?}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_ethereum_current_block(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<
        runtime_types::ethereum::block::Block<runtime_types::ethereum::transaction::TransactionV2>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().ethereum().current_block())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_ethereum_current_receipts(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<::subxt::ext::subxt_core::alloc::vec::Vec<runtime_types::ethereum::receipt::ReceiptV3>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().ethereum().current_receipts())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_ethereum_current_transaction_statuses(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<::subxt::ext::subxt_core::alloc::vec::Vec<runtime_types::fp_rpc::TransactionStatus>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().ethereum().current_transaction_statuses())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_ethereum_pending(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<
        ::subxt::ext::subxt_core::alloc::vec::Vec<(
            runtime_types::ethereum::transaction::TransactionV2,
            runtime_types::fp_rpc::TransactionStatus,
            runtime_types::ethereum::receipt::ReceiptV3,
        )>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().ethereum().pending()).await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_evm_account_codes_by_h160(
    client: &OnlineClient<PolkadotConfig>,
    h160: ::subxt::ext::subxt_core::utils::H160,
) -> Result<
    Option<::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().evm().account_codes(h160))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_evm_account_codes(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        ::subxt::ext::subxt_core::utils::H160,
        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().evm().account_codes_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::subxt::ext::subxt_core::utils::H160::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_evm_account_codes_metadata_by_h160(
    client: &OnlineClient<PolkadotConfig>,
    h160: ::subxt::ext::subxt_core::utils::H160,
) -> Result<Option<runtime_types::pallet_evm::CodeMetadata>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().evm().account_codes_metadata(h160))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_evm_account_codes_metadata(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<::subxt::ext::subxt_core::utils::H160, runtime_types::pallet_evm::CodeMetadata>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().evm().account_codes_metadata_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::subxt::ext::subxt_core::utils::H160::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage double map value by keys"]
pub async fn get_evm_account_storages_by_h160_by_h256(
    client: &OnlineClient<PolkadotConfig>,
    h160_1: ::subxt::ext::subxt_core::utils::H160,
    h256_2: ::subxt::ext::subxt_core::utils::H256,
) -> Result<Option<::subxt::ext::subxt_core::utils::H256>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().evm().account_storages(h160_1, h256_2))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage double map"]
pub async fn query_map_evm_account_storages(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        ::subxt::ext::subxt_core::utils::H160,
        HashMap<::subxt::ext::subxt_core::utils::H256, ::subxt::ext::subxt_core::utils::H256>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().evm().account_storages_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok((key1, key2)) = <(
            ::subxt::ext::subxt_core::utils::H160,
            ::subxt::ext::subxt_core::utils::H256,
        )>::decode(&mut &kv.key_bytes[32..])
        {
            result
                .entry(key1)
                .or_insert_with(HashMap::new)
                .insert(key2, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_evm_suicided_by_h160(
    client: &OnlineClient<PolkadotConfig>,
    h160: ::subxt::ext::subxt_core::utils::H160,
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().evm().suicided(h160)).await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_evm_suicided(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<::subxt::ext::subxt_core::utils::H160, ()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage.iter(api::storage().evm().suicided_iter()).await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::subxt::ext::subxt_core::utils::H160::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_governance_agent_applications_by_u32(
    client: &OnlineClient<PolkadotConfig>,
    u32: ::core::primitive::u32,
) -> Result<
    Option<runtime_types::pallet_governance::application::AgentApplication>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().agent_applications(u32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_governance_agent_applications(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        ::core::primitive::u32,
        runtime_types::pallet_governance::application::AgentApplication,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().governance().agent_applications_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::core::primitive::u32::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_governance_allocators_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().allocators(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_governance_allocators(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, ()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().governance().allocators_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_governance_curators_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().curators(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_governance_curators(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, ()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().governance().curators_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_governance_dao_treasury_address(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::subxt::ext::subxt_core::utils::AccountId32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().dao_treasury_address())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_governance_global_governance_config(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<runtime_types::pallet_governance::config::GovernanceConfiguration>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().global_governance_config())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_governance_not_delegating_voting_power(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<
        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
            ::subxt::ext::subxt_core::utils::AccountId32,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().not_delegating_voting_power())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_governance_proposals_by_u64(
    client: &OnlineClient<PolkadotConfig>,
    u64: ::core::primitive::u64,
) -> Result<Option<runtime_types::pallet_governance::proposal::Proposal>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().proposals(u64))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_governance_proposals(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<::core::primitive::u64, runtime_types::pallet_governance::proposal::Proposal>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().governance().proposals_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::core::primitive::u64::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_governance_treasury_emission_fee(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::sp_arithmetic::per_things::Percent>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().treasury_emission_fee())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_governance_unrewarded_proposals_by_u64(
    client: &OnlineClient<PolkadotConfig>,
    u64: ::core::primitive::u64,
) -> Result<
    Option<runtime_types::pallet_governance::proposal::UnrewardedProposal>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().unrewarded_proposals(u64))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_governance_unrewarded_proposals(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<::core::primitive::u64, runtime_types::pallet_governance::proposal::UnrewardedProposal>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().governance().unrewarded_proposals_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::core::primitive::u64::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_governance_whitelist_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().governance().whitelist(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_governance_whitelist(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, ()>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().governance().whitelist_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_grandpa_authorities(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
            runtime_types::sp_consensus_grandpa::app::Public,
            ::core::primitive::u64,
        )>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().grandpa().authorities())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_grandpa_current_set_id(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u64>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().grandpa().current_set_id())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_grandpa_next_forced(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u64>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().grandpa().next_forced())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_grandpa_pending_change(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u64>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().grandpa().pending_change())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_grandpa_set_id_session_by_u64(
    client: &OnlineClient<PolkadotConfig>,
    u64: ::core::primitive::u64,
) -> Result<Option<::core::primitive::u32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().grandpa().set_id_session(u64))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_grandpa_set_id_session(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<::core::primitive::u64, ::core::primitive::u32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().grandpa().set_id_session_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::core::primitive::u64::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_grandpa_stalled(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<(::core::primitive::u64, ::core::primitive::u64)>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().grandpa().stalled()).await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_grandpa_state(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<runtime_types::pallet_grandpa::StoredState<::core::primitive::u64>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().grandpa().state()).await?;
    Ok(result)
}
#[doc = r" Get storage double map value by keys"]
pub async fn get_multisig_multisigs_by_account_id32_by_u8_array(
    client: &OnlineClient<PolkadotConfig>,
    account_id32_1: ::subxt::ext::subxt_core::utils::AccountId32,
    u8_array_2: [::core::primitive::u8; 32usize],
) -> Result<
    Option<
        runtime_types::pallet_multisig::Multisig<
            ::core::primitive::u64,
            ::core::primitive::u128,
            ::subxt::ext::subxt_core::utils::AccountId32,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(
            &api::storage()
                .multisig()
                .multisigs(account_id32_1, u8_array_2),
        )
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage double map"]
pub async fn query_map_multisig_multisigs(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        String,
        HashMap<
            [::core::primitive::u8; 32usize],
            runtime_types::pallet_multisig::Multisig<
                ::core::primitive::u64,
                ::core::primitive::u128,
                ::subxt::ext::subxt_core::utils::AccountId32,
            >,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().multisig().multisigs_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok((key1, key2)) = <(
            ::subxt::ext::subxt_core::utils::AccountId32,
            [::core::primitive::u8; 32usize],
        )>::decode(&mut &kv.key_bytes[32..])
        {
            result
                .entry(format!("{}", key1))
                .or_insert_with(HashMap::new)
                .insert(key2, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_sudo_key(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::subxt::ext::subxt_core::utils::AccountId32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().sudo().key()).await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_system_account_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<
    Option<
        runtime_types::frame_system::AccountInfo<
            ::core::primitive::u32,
            runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().account(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_system_account(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        String,
        runtime_types::frame_system::AccountInfo<
            ::core::primitive::u32,
            runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage.iter(api::storage().system().account_iter()).await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_all_extrinsics_len(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().all_extrinsics_len())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_authorized_upgrade(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::frame_system::CodeUpgradeAuthorization>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().authorized_upgrade())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_system_block_hash_by_u64(
    client: &OnlineClient<PolkadotConfig>,
    u64: ::core::primitive::u64,
) -> Result<Option<::subxt::ext::subxt_core::utils::H256>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().block_hash(u64))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_system_block_hash(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<::core::primitive::u64, ::subxt::ext::subxt_core::utils::H256>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().system().block_hash_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::core::primitive::u64::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_block_weight(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<
        runtime_types::frame_support::dispatch::PerDispatchClass<
            runtime_types::sp_weights::weight_v2::Weight,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().block_weight())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_digest(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::sp_runtime::generic::digest::Digest>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().system().digest()).await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_event_count(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().event_count())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_system_event_topics_by_h256(
    client: &OnlineClient<PolkadotConfig>,
    h256: ::subxt::ext::subxt_core::utils::H256,
) -> Result<
    Option<
        ::subxt::ext::subxt_core::alloc::vec::Vec<(::core::primitive::u64, ::core::primitive::u32)>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().event_topics(h256))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_system_event_topics(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        ::subxt::ext::subxt_core::utils::H256,
        ::subxt::ext::subxt_core::alloc::vec::Vec<(::core::primitive::u64, ::core::primitive::u32)>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().system().event_topics_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::subxt::ext::subxt_core::utils::H256::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_events(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<
        ::subxt::ext::subxt_core::alloc::vec::Vec<
            runtime_types::frame_system::EventRecord<
                runtime_types::torus_runtime::RuntimeEvent,
                ::subxt::ext::subxt_core::utils::H256,
            >,
        >,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().system().events()).await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_execution_phase(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::frame_system::Phase>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().execution_phase())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_extrinsic_count(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u32>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().extrinsic_count())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_system_extrinsic_data_by_u32(
    client: &OnlineClient<PolkadotConfig>,
    u32: ::core::primitive::u32,
) -> Result<
    Option<::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().extrinsic_data(u32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_system_extrinsic_data(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    HashMap<
        ::core::primitive::u32,
        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
    >,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().system().extrinsic_data_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) = ::core::primitive::u32::decode(&mut &kv.key_bytes[32..]) {
            result.insert(key, kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_inherents_applied(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::bool>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().inherents_applied())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_last_runtime_upgrade(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().last_runtime_upgrade())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_number(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u64>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().system().number()).await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_parent_hash(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::subxt::ext::subxt_core::utils::H256>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().parent_hash())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_upgraded_to_triple_ref_count(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::bool>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().upgraded_to_triple_ref_count())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_system_upgraded_to_u32_ref_count(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::bool>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().system().upgraded_to_u32_ref_count())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_timestamp_did_update(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::bool>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().timestamp().did_update())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_timestamp_now(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u64>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().timestamp().now()).await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_torus0_agents_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<runtime_types::pallet_torus0::agent::Agent>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().agents(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_torus0_agents(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, runtime_types::pallet_torus0::agent::Agent>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage.iter(api::storage().torus0().agents_iter()).await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_burn(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage.fetch(&api::storage().torus0().burn()).await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_burn_config(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::pallet_torus0::burn::BurnConfiguration>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().burn_config())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_dividends_participation_weight(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::sp_arithmetic::per_things::Percent>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().dividends_participation_weight())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_fee_constraints(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<
    Option<runtime_types::pallet_torus0::fee::ValidatorFeeConstraints>,
    Box<dyn std::error::Error>,
> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().fee_constraints())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_immunity_period(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().immunity_period())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_max_agent_url_length(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().max_agent_url_length())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_max_allowed_agents(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().max_allowed_agents())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_max_allowed_validators(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().max_allowed_validators())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_max_name_length(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().max_name_length())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_max_registrations_per_block(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().max_registrations_per_block())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_min_allowed_stake(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().min_allowed_stake())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_min_name_length(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().min_name_length())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_min_validator_stake(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().min_validator_stake())
        .await?;
    Ok(result)
}
#[doc = r" Get storage map value by key"]
pub async fn get_torus0_registration_block_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<::core::primitive::u64>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().registration_block(account_id32))
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage map"]
pub async fn query_map_torus0_registration_block(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, ::core::primitive::u64>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().torus0().registration_block_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok(key) =
            ::subxt::ext::subxt_core::utils::AccountId32::decode(&mut &kv.key_bytes[32..])
        {
            result.insert(format!("{}", key), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_registrations_this_block(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().registrations_this_block())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_registrations_this_interval(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().registrations_this_interval())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_reward_interval(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u16>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().reward_interval())
        .await?;
    Ok(result)
}
#[doc = r" Get storage double map value by keys"]
pub async fn get_torus0_staked_by_by_account_id32_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32_1: ::subxt::ext::subxt_core::utils::AccountId32,
    account_id32_2: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(
            &api::storage()
                .torus0()
                .staked_by(account_id32_1, account_id32_2),
        )
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage double map"]
pub async fn query_map_torus0_staked_by(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, HashMap<String, ::core::primitive::u128>>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().torus0().staked_by_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok((key1, key2)) = <(
            ::subxt::ext::subxt_core::utils::AccountId32,
            ::subxt::ext::subxt_core::utils::AccountId32,
        )>::decode(&mut &kv.key_bytes[32..])
        {
            result
                .entry(format!("{}", key1))
                .or_insert_with(HashMap::new)
                .insert(format!("{}", key2), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage double map value by keys"]
pub async fn get_torus0_staking_to_by_account_id32_by_account_id32(
    client: &OnlineClient<PolkadotConfig>,
    account_id32_1: ::subxt::ext::subxt_core::utils::AccountId32,
    account_id32_2: ::subxt::ext::subxt_core::utils::AccountId32,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(
            &api::storage()
                .torus0()
                .staking_to(account_id32_1, account_id32_2),
        )
        .await?;
    Ok(result)
}
#[doc = r" Query all entries in storage double map"]
pub async fn query_map_torus0_staking_to(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<HashMap<String, HashMap<String, ::core::primitive::u128>>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let mut result = HashMap::new();
    let mut iter = storage
        .iter(api::storage().torus0().staking_to_iter())
        .await?;
    while let Some(Ok(kv)) = iter.next().await {
        if let Ok((key1, key2)) = <(
            ::subxt::ext::subxt_core::utils::AccountId32,
            ::subxt::ext::subxt_core::utils::AccountId32,
        )>::decode(&mut &kv.key_bytes[32..])
        {
            result
                .entry(format!("{}", key1))
                .or_insert_with(HashMap::new)
                .insert(format!("{}", key2), kv.value);
        }
    }
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_torus0_total_stake(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<::core::primitive::u128>, Box<dyn std::error::Error>> {
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().torus0().total_stake())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_transaction_payment_next_fee_multiplier(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::sp_arithmetic::fixed_point::FixedU128>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().transaction_payment().next_fee_multiplier())
        .await?;
    Ok(result)
}
#[doc = r" Get storage value"]
pub async fn get_transaction_payment_storage_version(
    client: &OnlineClient<PolkadotConfig>,
) -> Result<Option<runtime_types::pallet_transaction_payment::Releases>, Box<dyn std::error::Error>>
{
    let storage = client.storage().at_latest().await?;
    let result = storage
        .fetch(&api::storage().transaction_payment().storage_version())
        .await?;
    Ok(result)
}
