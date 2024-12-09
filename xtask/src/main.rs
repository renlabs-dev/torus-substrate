use std::{borrow::Cow, net::IpAddr};

use polkadot_sdk::sp_keyring::Ed25519Keyring;

mod flags;
mod run;

fn main() {
    let flags = flags::Run::from_env_or_exit();
    run::run(flags);
}

#[derive(Clone)]
pub(crate) struct Node<'a> {
    pub(crate) name: Option<Cow<'a, str>>,
    pub(crate) id: Option<NodeId<'a>>,
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
#[derive(Clone, Default)]
struct Account<'a> {
    pub(crate) suri: Cow<'a, str>,
    pub(crate) aura_address: Cow<'a, str>,
    pub(crate) grandpa_address: Cow<'a, str>,
}

#[derive(Clone)]
enum NodeId<'a> {
    Arbitrary(Cow<'a, str>),
    Keyring(Ed25519Keyring),
}

impl std::fmt::Display for NodeId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeId::Arbitrary(s) => write!(f, "{s}"),
            NodeId::Keyring(keyring) => write!(f, "{}", keyring.to_account_id()),
        }
    }
}

static ALICE_ACCOUNT: Account<'static> = Account {
    suri: Cow::Borrowed(
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
    ),
    aura_address: Cow::Borrowed("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"),
    grandpa_address: Cow::Borrowed("5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"),
};

static BOB_ACCOUNT: Account<'static> = Account {
    suri: Cow::Borrowed(
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob",
    ),
    aura_address: Cow::Borrowed("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"),
    grandpa_address: Cow::Borrowed("5GoNkf6WdbxCFnPdAnYYQyCjAKPJgLNxXwPjwTh6DGg6gN3E"),
};

static BOB_NODE: Node<'static> = Node {
    name: Some(Cow::Borrowed("Bob")),
    id: Some(NodeId::Keyring(Ed25519Keyring::Bob)),
    key: Some(Cow::Borrowed(
        "e83fa0787cb280d95c666ead866a2a4bc1ee1e36faa1ed06623595eb3f474681",
    )),
    tcp_port: 30342,
    rpc_port: 9952,
    validator: true,
};

static ALICE_NODE: Node<'static> = Node {
    name: Some(Cow::Borrowed("Alice")),
    id: Some(NodeId::Keyring(Ed25519Keyring::Alice)),
    key: Some(Cow::Borrowed(
        "2756181a3b9bca683a35b51a0a5d75ee536738680bcb9066c68be1db305a1ac5",
    )),
    tcp_port: 30341,
    rpc_port: 9951,
    validator: true,
};
