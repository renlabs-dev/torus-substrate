use std::{borrow::Cow, net::IpAddr};

use polkadot_sdk::sp_keyring;

mod flags;
mod run;

fn main() {
    let cmd = flags::Xtask::from_env_or_exit();
    match cmd.subcommand {
        flags::XtaskCmd::Run(run) => run::run(run),
    }
}

#[derive(Clone)]
pub(crate) struct Node<'a> {
    pub(crate) name: Option<Cow<'a, str>>,
    pub(crate) id: Option<Cow<'a, str>>,
    pub(crate) key: Option<Cow<'a, str>>,
    pub(crate) tcp_port: u16,
    pub(crate) rpc_port: u16,
    pub(crate) validator: bool,
}

impl Node<'_> {
    fn bootnode_uri(&self, addr: IpAddr) -> String {
        format!(
            "/{}/{addr}/tcp/{}/p2p/{}",
            match addr {
                IpAddr::V4(_) => "ip4",
                IpAddr::V6(_) => "ip6",
            },
            self.tcp_port,
            self.id.as_ref().unwrap()
        )
    }
}

impl Default for Node<'_> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            key: Default::default(),
            tcp_port: 30333,
            rpc_port: 9944,
            validator: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct Account<'a> {
    pub(crate) suri: Cow<'a, str>,
    pub(crate) aura_address: sp_keyring::Sr25519Keyring,
    pub(crate) grandpa_address: sp_keyring::Ed25519Keyring,
}

impl<'a> Default for Account<'a> {
    fn default() -> Self {
        Self {
            suri: "".into(),
            aura_address: sp_keyring::Sr25519Keyring::One,
            grandpa_address: sp_keyring::Ed25519Keyring::One,
        }
    }
}

static ALICE_ACCOUNT: Account<'static> = Account {
    suri: Cow::Borrowed(
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
    ),
    aura_address: sp_keyring::Sr25519Keyring::Alice,
    grandpa_address: sp_keyring::Ed25519Keyring::Alice,
};

static BOB_ACCOUNT: Account<'static> = Account {
    suri: Cow::Borrowed(
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob",
    ),
    aura_address: sp_keyring::Sr25519Keyring::Bob,
    grandpa_address: sp_keyring::Ed25519Keyring::Bob,
};

static ALICE_NODE: Node<'static> = Node {
    name: Some(Cow::Borrowed("Alice")),
    id: Some(Cow::Borrowed(
        "12D3KooWBorpca6RKiebVjeFJA5o9iVWnZpg98yQbYqRC6f8CnLw",
    )),
    key: Some(Cow::Borrowed(
        "2756181a3b9bca683a35b51a0a5d75ee536738680bcb9066c68be1db305a1ac5",
    )),
    tcp_port: 30341,
    rpc_port: 9951,
    validator: true,
};

static BOB_NODE: Node<'static> = Node {
    name: Some(Cow::Borrowed("Bob")),
    id: Some(Cow::Borrowed(
        "12D3KooWQh3CeSp2rpUVvPb6jqvmHVCUieoZmKbkUhZ8rPR77vmA",
    )),
    key: Some(Cow::Borrowed(
        "e83fa0787cb280d95c666ead866a2a4bc1ee1e36faa1ed06623595eb3f474681",
    )),
    tcp_port: 30342,
    rpc_port: 9952,
    validator: true,
};
