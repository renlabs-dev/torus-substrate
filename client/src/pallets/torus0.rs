use std::marker::PhantomData;

use codec::Decode;
use futures::{Stream, StreamExt, TryStreamExt};
use subxt::{
    utils::{AccountId32, H256},
    OnlineClient, PolkadotConfig,
};

use crate::{chain::Chain, storage_dmap, storage_map, storage_value};

pub struct Torus0<C: Chain> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) _pd: PhantomData<C>,
}

impl<C: Chain> Torus0<C> {
    pub fn storage(&self) -> Torus0Storage<C> {
        Torus0Storage {
            client: self.client.clone(),
            block: None,
            _pd: PhantomData,
        }
    }
}

pub struct Torus0Storage<C: Chain> {
    client: OnlineClient<PolkadotConfig>,
    block: Option<H256>,
    _pd: PhantomData<C>,
}

#[cfg(feature = "mainnet")]
impl Torus0Storage<crate::chain::MainNet> {
    storage_value!(mainnet, torus0, max_allowed_validators -> u16);
    storage_value!(mainnet, torus0, burn -> u128);
    storage_value!(mainnet, torus0, registrations_this_interval -> u16);
    storage_value!(mainnet, torus0, min_validator_stake -> u128);
    storage_value!(mainnet, torus0, immunity_period -> u16);
    storage_value!(mainnet, torus0, reward_interval -> u16);
    storage_map!(mainnet, torus0, agents -> <AccountId32, crate::interfaces::mainnet::api::runtime_types::pallet_torus0::agent::Agent>);
    storage_value!(mainnet, torus0, max_name_length -> u16);
    storage_value!(mainnet, torus0, min_name_length -> u16);
    storage_value!(mainnet, torus0, max_agent_url_length -> u16);
    storage_value!(mainnet, torus0, max_allowed_agents -> u16);
    storage_value!(mainnet, torus0, registrations_this_block -> u16);
    storage_value!(mainnet, torus0, max_registrations_per_block -> u16);
    storage_dmap!(mainnet, torus0, staking_to -> <(AccountId32, AccountId32), u128>);
    storage_dmap!(mainnet, torus0, staked_by -> <(AccountId32, AccountId32), u128>);
    storage_value!(mainnet, torus0, total_stake -> u128);
    storage_value!(mainnet, torus0, min_allowed_stake -> u128);
    storage_value!(mainnet, torus0, dividends_participation_weight -> crate::interfaces::mainnet::api::runtime_types::sp_arithmetic::per_things::Percent);
    storage_value!(mainnet, torus0, fee_constraints -> crate::interfaces::mainnet::api::torus0::storage::types::fee_constraints::FeeConstraints);
    storage_value!(mainnet, torus0, burn_config -> crate::interfaces::mainnet::api::torus0::storage::types::burn_config::BurnConfig);
    // storage_value!(mainnet, torus0, agent_update_cooldown -> u64);
}

#[cfg(feature = "testnet")]
impl Torus0Storage<crate::chain::TestNet> {
    storage_value!(testnet, torus0, max_allowed_validators -> u16);
    storage_value!(testnet, torus0, burn -> u128);
    storage_value!(testnet, torus0, registrations_this_interval -> u16);
    storage_value!(testnet, torus0, min_validator_stake -> u128);
    storage_value!(testnet, torus0, immunity_period -> u16);
    storage_value!(testnet, torus0, reward_interval -> u16);
    storage_map!(testnet, torus0, agents -> <AccountId32, crate::interfaces::testnet::api::runtime_types::pallet_torus0::agent::Agent>);
    storage_value!(testnet, torus0, max_name_length -> u16);
    storage_value!(testnet, torus0, min_name_length -> u16);
    storage_value!(testnet, torus0, max_agent_url_length -> u16);
    storage_value!(testnet, torus0, max_allowed_agents -> u16);
    storage_value!(testnet, torus0, registrations_this_block -> u16);
    storage_value!(testnet, torus0, max_registrations_per_block -> u16);
    storage_dmap!(testnet, torus0, staking_to -> <(AccountId32, AccountId32), u128>);
    storage_dmap!(testnet, torus0, staked_by -> <(AccountId32, AccountId32), u128>);
    storage_value!(testnet, torus0, total_stake -> u128);
    storage_value!(testnet, torus0, min_allowed_stake -> u128);
    storage_value!(testnet, torus0, dividends_participation_weight -> crate::interfaces::testnet::api::runtime_types::sp_arithmetic::per_things::Percent);
    storage_value!(testnet, torus0, fee_constraints -> crate::interfaces::testnet::api::torus0::storage::types::fee_constraints::FeeConstraints);
    storage_value!(testnet, torus0, burn_config -> crate::interfaces::testnet::api::torus0::storage::types::burn_config::BurnConfig);
    // storage_value!(testnet, torus0, agent_update_cooldown -> u64);
}
