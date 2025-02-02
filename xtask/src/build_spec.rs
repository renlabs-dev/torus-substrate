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
use serde_json::Value;

pub fn targetchain_spec(flags: &crate::flags::Replica, dir: &Path) -> PathBuf {
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
    // balance(genesis, flags.sudo.as_ref());

    let js = serde_json::to_string_pretty(&js).unwrap();

    let chain_path = flags
        .output
        .clone()
        .unwrap_or_else(|| dir.join("spec.json"));
    std::fs::write(&chain_path, js).unwrap();

    chain_path
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

const KEYS: &[[u8; 32]] = &[
    // Alice
    hex_literal::hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"),
    // Bob
    hex_literal::hex!("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"),
];

fn aura(genesis: &mut Value) {
    let key = key_name(b"Aura", b"Authorities");

    let mut buf = Cursor::new(vec![0; KEYS.size_hint()]);
    KEYS.encode_to(&mut buf);

    let buf = &buf.get_ref()[..buf.position() as usize];
    genesis[&key] = Value::String(format!("0x{}", hex::encode(buf)));
}

fn grandpa(genesis: &mut Value) {
    // Alice
    let grandpa = vec![(
        sp_core::ed25519::Public::from_ss58check(
            "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu",
        )
        .expect("invalid SS58 sudo address")
        .0,
        1u64,
    )];

    let key = key_name(b"Grandpa", b"Authorities");
    dbg!(&key);

    let mut buf = Cursor::new(vec![0; grandpa.size_hint()]);
    grandpa.encode_to(&mut buf);

    let buf = &buf.get_ref()[..buf.position() as usize];
    genesis[&key] = Value::String(format!("0x{}", hex::encode(buf)));
}

// fn balance(genesis: &mut Value, sudo: Option<&String>) {
//     let sudo = sudo
//         .map(|sudo| {
//             sp_core::ed25519::Public::from_ss58check(&sudo)
//                 .expect("invalid SS58 sudo address")
//                 .0
//         })
//         .unwrap_or(KEYS[0]);

//     let mut key = key_name(b"System", b"Account");
//     key.push_str(&hex::encode(
//         [
//             sp_crypto_hashing::blake2_128(&sudo).as_slice(),
//             sudo.as_slice(),
//         ]
//         .concat(),
//     ));

//     let info = AccountInfo {
//         nonce: 0,
//         consumers: 0,
//         providers: 0,
//         sufficients: 0,
//         data: AccountData {
//             free: 1_000_000_000_000_000,
//             reserved: 0,
//             frozen: 0,
//             flags: ExtraFlags(IS_NEW_LOGIC),
//         },
//     };

//     let mut buf = Cursor::new(vec![0; KEYS.size_hint()]);
//     info.encode_to(&mut buf);

//     let buf = &buf.get_ref()[..buf.position() as usize];
//     genesis[&key] = Value::String(format!("0x{}", hex::encode(buf)));
// }

fn key_name(pallet: &[u8], key: &[u8]) -> String {
    let mut res = [0; 32];
    res[0..16].copy_from_slice(&sp_crypto_hashing::twox_128(pallet));
    res[16..32].copy_from_slice(&sp_crypto_hashing::twox_128(key));
    format!("0x{}", hex::encode(res))
}

type OpaqueBlock = Block<Header<u32, BlakeTwo256>, OpaqueExtrinsic>;

// const IS_NEW_LOGIC: u128 = 0x80000000_00000000_00000000_00000000u128;

async fn crate_chain_spec(flags: &crate::flags::Replica) -> Box<dyn ChainSpec> {
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

pub type Balance = u64;
pub type Nonce = u32;
pub type RefCount = u32;

/// Information of an account.
#[derive(Debug, parity_scale_codec::Decode, parity_scale_codec::Encode)]
pub struct AccountInfo {
    /// The number of transactions this account has sent.
    pub nonce: Nonce,
    /// The number of other modules that currently depend on this account's existence. The account
    /// cannot be reaped until this is zero.
    pub consumers: RefCount,
    /// The number of other modules that allow this account to exist. The account may not be reaped
    /// until this and `sufficients` are both zero.
    pub providers: RefCount,
    /// The number of modules that allow this account to exist for their own purposes only. The
    /// account may not be reaped until this and `providers` are both zero.
    pub sufficients: RefCount,
    /// The additional data that belongs to this account. Used to store the balance(s) in a lot of
    /// chains.
    pub data: AccountData,
}

/// All balance information for an account.
#[derive(Debug, parity_scale_codec::Decode, parity_scale_codec::Encode)]
pub struct AccountData {
    /// Non-reserved part of the balance which the account holder may be able to control.
    ///
    /// This is the only balance that matters in terms of most operations on tokens.
    pub free: Balance,
    /// Balance which is has active holds on it and may not be used at all.
    ///
    /// This is the sum of all individual holds together with any sums still under the (deprecated)
    /// reserves API.
    pub reserved: Balance,
    /// The amount that `free + reserved` may not drop below when reducing the balance, except for
    /// actions where the account owner cannot reasonably benefit from the balance reduction, such
    /// as slashing.
    pub frozen: Balance,
    /// Extra information about this account. The MSB is a flag indicating whether the new ref-
    /// counting logic is in place for this account.
    pub flags: ExtraFlags,
}

#[derive(Debug, parity_scale_codec::Decode, parity_scale_codec::Encode)]
pub struct ExtraFlags(pub(crate) u128);
