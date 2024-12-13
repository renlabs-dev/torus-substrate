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

use polkadot_sdk::{
    sc_service::{ChainType, Properties},
    *,
};
use serde_json::{json, Value};
use torus_runtime::WASM_BINARY;

/// This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

fn props() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenDecimals".to_string(), 18.into());
    properties.insert("tokenSymbol".to_string(), "TOR".into());
    properties
}

pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.expect("Development wasm not available"),
        Default::default(),
    )
    .with_name("Torus")
    .with_id("torus")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis())
    .with_properties(props())
    .build())
}

/// Configure initial storage state for FRAME pallets.
fn testnet_genesis() -> Value {
    use polkadot_sdk::{
        polkadot_sdk_frame::traits::Get,
        sp_keyring::{Ed25519Keyring, Sr25519Keyring},
    };

    use torus_runtime::{
        interface::{Balance, MinimumBalance},
        BalancesConfig, SudoConfig,
    };

    let endowment = <MinimumBalance as Get<Balance>>::get().max(1) * 1000;
    let balances = Sr25519Keyring::iter()
        .map(|a| (a.to_account_id(), endowment))
        .collect::<Vec<_>>();

    let aura = [Sr25519Keyring::Alice, Sr25519Keyring::Bob];
    let grandpa = [Ed25519Keyring::Alice, Ed25519Keyring::Bob];

    json!({
        "balances": BalancesConfig { balances },
        "sudo": SudoConfig { key: Some(Sr25519Keyring::Alice.to_account_id()) },
        "aura": {
            "authorities": aura.iter().map(|x| (x.public().to_string())).collect::<Vec<_>>(),
        },
        "grandpa": {
            "authorities": grandpa.iter().map(|x| (x.public().to_string(), 1)).collect::<Vec<_>>(),
        },
    })
}
