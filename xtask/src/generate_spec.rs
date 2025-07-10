use std::{
    collections::{BTreeMap, HashMap},
    io::Cursor,
    path::{Path, PathBuf},
};

use parity_scale_codec::Encode;
use polkadot_sdk::{
    frame_remote_externalities::{self, OnlineConfig},
    sc_client_api::StateBackend,
    sc_service::{self, ChainSpec},
    sp_core::{self, crypto::Ss58Codec},
    sp_crypto_hashing,
    sp_runtime::{
        self,
        generic::{Block, Header},
        traits::BlakeTwo256,
        OpaqueExtrinsic,
    },
};
use serde_json::{Number, Value};

use crate::flags;

pub fn generate_spec(cmd: flags::GenerateSpec) {
    match cmd.subcommand {
        flags::GenerateSpecCmd::GenReplica(gen_replica) => {
            generate_replica_spec(gen_replica, cmd.out, cmd.sudo)
        }
        flags::GenerateSpecCmd::GenNew(ref gen_new) => generate_new_spec(gen_new, &cmd),
    }
}

// Types needed for chain state fetching
type OpaqueBlock = Block<Header<u32, BlakeTwo256>, OpaqueExtrinsic>;
pub type Balance = u64;
pub type Nonce = u32;
pub type RefCount = u32;

fn generate_replica_spec(
    gen_replica: flags::GenReplica,
    out: Option<PathBuf>,
    sudo: Option<String>,
) {
    let replica_cmd = flags::Replica {
        output: out,
        sudo,
        api_url: gen_replica.api_url.clone(),
    };

    targetchain_spec(&replica_cmd);
}

pub fn targetchain_spec(flags: &flags::Replica) -> Option<PathBuf> {
    let spec = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(crate_chain_spec(flags));

    let spec = sc_service::chain_ops::build_spec(spec.as_ref() as &dyn ChainSpec, true).unwrap();
    let mut js: Value = serde_json::from_str(&spec).unwrap();

    let genesis = &mut js["genesis"]["raw"]["top"];

    aura(genesis);
    grandpa(genesis);
    sudo(genesis, flags.sudo.as_ref());

    let js = serde_json::to_string_pretty(&js).unwrap();

    match &flags.output {
        Some(chain_path) => {
            std::fs::write(chain_path, js).unwrap();
            Some(chain_path.clone())
        }
        None => {
            println!("{js}");
            None
        }
    }
}

fn sudo(genesis: &mut Value, sudo: Option<&String>) {
    let key = key_name(b"Sudo", b"Key");

    let sudo = sudo
        .map(|sudo| {
            sp_core::ed25519::Public::from_ss58check(sudo)
                .expect("invalid SS58 sudo address")
                .0
        })
        .unwrap_or(KEYS[0]);

    genesis[&key] = Value::String(format!("0x{}", hex::encode(sudo)));
}

/// Known predefined keys
const KEYS: &[[u8; 32]] = &[
    // Alice
    hex_literal::hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"),
    // Bob
    hex_literal::hex!("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"),
];

/// Sets the aura authorities in the genesis state
fn aura(genesis: &mut Value) {
    let key = key_name(b"Aura", b"Authorities");

    let mut buf = Cursor::new(vec![0; KEYS.size_hint()]);
    KEYS.encode_to(&mut buf);

    let buf = &buf.get_ref()[..buf.position() as usize];
    genesis[&key] = Value::String(format!("0x{}", hex::encode(buf)));
}

/// Sets the grandpa authorities in the genesis state
fn grandpa(genesis: &mut Value) {
    // Alice
    let grandpa = vec![(
        sp_core::ed25519::Public::from_ss58check(
            "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu",
        )
        .expect("invalid SS58 grandpa address")
        .0,
        1u64,
    )];

    let key = key_name(b"Grandpa", b"Authorities");

    let mut buf = Cursor::new(vec![0; grandpa.size_hint()]);
    grandpa.encode_to(&mut buf);

    let buf = &buf.get_ref()[..buf.position() as usize];
    genesis[&key] = Value::String(format!("0x{}", hex::encode(buf)));
}

/// Generates the storage key for a given pallet and storage item
fn key_name(pallet: &[u8], key: &[u8]) -> String {
    let mut res = [0; 32];
    res[0..16].copy_from_slice(&sp_crypto_hashing::twox_128(pallet));
    res[16..32].copy_from_slice(&sp_crypto_hashing::twox_128(key));
    format!("0x{}", hex::encode(res))
}

/// Fetches the chain state from a running node
async fn crate_chain_spec(flags: &flags::Replica) -> Box<dyn ChainSpec> {
    let mut chain_spec = sc_service::GenericChainSpec::<sc_service::NoExtension>::from_json_bytes(
        include_bytes!("../../node/specs/main.json"),
    )
    .unwrap();

    let api = flags
        .api_url
        .clone()
        .unwrap_or_else(|| "wss://api.torus.network".to_string());

    let mode = frame_remote_externalities::Builder::<OpaqueBlock>::new().mode(
        frame_remote_externalities::Mode::Online(OnlineConfig {
            at: None,
            state_snapshot: None,
            pallets: vec![],
            transport: frame_remote_externalities::Transport::Uri(api),
            child_trie: true,
            hashed_prefixes: vec![],
            hashed_keys: vec![],
        }),
    );

    let ext = mode.build().await.unwrap();

    let mut top = BTreeMap::new();
    let children_default = HashMap::new();

    let mut last_key = vec![];

    while let Some(key) = ext.backend.next_storage_key(&last_key).unwrap() {
        let val = ext.backend.storage(&key).unwrap();
        top.insert(key.clone(), val.unwrap());
        last_key = key;
    }

    chain_spec.set_storage(sp_runtime::Storage {
        top,
        children_default,
    });

    Box::new(chain_spec)
}

/// Information of an account.
#[derive(Debug, parity_scale_codec::Decode, parity_scale_codec::Encode)]
pub struct AccountInfo {
    /// The number of transactions this account has sent.
    pub nonce: Nonce,
    /// The number of other modules that currently depend on this account's
    /// existence. The account cannot be reaped until this is zero.
    pub consumers: RefCount,
    /// The number of other modules that allow this account to exist. The
    /// account may not be reaped until this and `sufficients` are both
    /// zero.
    pub providers: RefCount,
    /// The number of modules that allow this account to exist for their own
    /// purposes only. The account may not be reaped until this and
    /// `providers` are both zero.
    pub sufficients: RefCount,
    /// The additional data that belongs to this account. Used to store the
    /// balance(s) in a lot of chains.
    pub data: AccountData,
}

/// All balance information for an account.
#[derive(Debug, parity_scale_codec::Decode, parity_scale_codec::Encode)]
pub struct AccountData {
    /// Non-reserved part of the balance which the account holder may be able to
    /// control.
    ///
    /// This is the only balance that matters in terms of most operations on
    /// tokens.
    pub free: Balance,
    /// Balance which is has active holds on it and may not be used at all.
    ///
    /// This is the sum of all individual holds together with any sums still
    /// under the (deprecated) reserves API.
    pub reserved: Balance,
    /// The amount that `free + reserved` may not drop below when reducing the
    /// balance, except for actions where the account owner cannot
    /// reasonably benefit from the balance reduction, such as slashing.
    pub frozen: Balance,
    /// Extra information about this account. The MSB is a flag indicating
    /// whether the new ref- counting logic is in place for this account.
    pub flags: ExtraFlags,
}

#[derive(Debug, parity_scale_codec::Decode, parity_scale_codec::Encode)]
pub struct ExtraFlags(pub(crate) u128);

fn generate_new_spec(gen_new: &flags::GenNew, cmd: &flags::GenerateSpec) {
    let chain_spec = cmd
        .base_chain_spec
        .clone()
        .unwrap_or_else(|| Path::new("dev").to_path_buf());

    let out = crate::torus_node!("build-spec", "--chain", chain_spec)
        .stderr(std::io::stderr())
        .output()
        .expect("failed to run torus node");

    let mut json: Value = serde_json::from_slice(&out.stdout).expect("failed to parse spec file");

    if let Some(name) = &gen_new.name {
        json["name"] = Value::String(name.clone());
    }

    customize_spec(&mut json, cmd);

    let serialized = serde_json::to_string_pretty(&json).expect("failed to generate spec file");
    let Some(output_path) = &cmd.out else {
        println!("{serialized}");
        return;
    };

    std::fs::write(output_path, serialized).expect("failed to write resulting spec file");
}

/// Function to customize a spec file based on the provided flags
pub fn customize_spec(json: &mut Value, cmd: &flags::GenerateSpec) {
    let patch = &mut json["genesis"]["runtimeGenesis"]["patch"];

    if !cmd.aura.is_empty() {
        let aura_keys = &mut patch["aura"]["authorities"]
            .as_array_mut()
            .expect("missing aura keys");
        aura_keys.clear();

        for aura in &cmd.aura {
            aura_keys.push(aura.clone().into());
        }
    }

    if !cmd.gran.is_empty() {
        let gran_keys = patch["grandpa"]["authorities"]
            .as_array_mut()
            .expect("missing grandpa keys");
        gran_keys.clear();

        for gran in &cmd.gran {
            gran_keys.push([Value::from(gran.clone()), 1i32.into()].into());
        }
    }

    if !cmd.balance.is_empty() {
        let balances = &mut patch["balances"]["balances"]
            .as_array_mut()
            .expect("missing balances array");
        balances.clear(); // Clear existing balances for gen-new

        for balance in &cmd.balance {
            let (account, amount) = balance
                .split_once('=')
                .expect("malformed balance entry, format: <account>=<amount>");
            let amount: u128 = amount.parse().expect("balance amount must be a number");

            balances.push([Value::from(account), Number::from_u128(amount).into()].into());
        }
    }

    if let Some(sudo) = &cmd.sudo {
        patch["sudo"]["key"] = sudo.clone().into();
    }
}
