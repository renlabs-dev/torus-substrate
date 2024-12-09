use std::{borrow::Cow, net::IpAddr};

mod flags;
mod run;

fn main() {
    let flags = flags::Run::from_env_or_exit();
    run::run(flags);
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
#[derive(Clone, Default)]
struct Account<'a> {
    pub(crate) suri: Cow<'a, str>,
    pub(crate) aura_address: Cow<'a, str>,
    pub(crate) grandpa_address: Cow<'a, str>,
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
