use std::path::PathBuf;
xflags::xflags! {
    src "src/flags.rs"

    cmd xtask {
        /// Runs a substrate node.
        cmd run {
            /// Overrides the base path of the node. If not set,
            /// a temporary directory will be used.
            optional -p, --path path: PathBuf

            /// Initiates the node with the Alice account.
            /// The Alice node listens on TCP 30341 and RPC 9951.
            optional --alice
            /// Initiates the node with the Bob account.
            /// The Bob node listens on TCP 30342 and RPC 9952.
            optional --bob

            /// If set, the node will be set to 0 Out/In peers.
            optional --isolated

            /// Overrides the default node name.
            optional --node-name node_name: String
            /// Overrides the default node key.
            optional --node-key node_key: String
            /// Overrides whether this node should run as a validator.
            optional --node-validator node_validator: bool

            /// Overrides the default TCP port.
            optional --tcp-port tcp_port: u16
            /// Overrides the default RPC port.
            optional --rpc-port rpc_port: u16
            /// Overrides the default bootnode list.
            repeated --bootnodes bootnodes: String

            /// Starts a local node based on the given chain spec.
            cmd local {
                /// The chain spec file. If missing, it will search
                /// for a `spec.json` file on the current directory.
                optional -c, --chain-spec chain_spec: PathBuf

                /// The account SURI. The pattern is: `<mnemonic>//<seed>`.
                optional --account-suri account_suri: String
            }
            /// Runs a mainnet replica with the latest state.
            cmd replica {
                /// Also write the generated spec file to disk.
                optional -o, --output output: PathBuf

                /// The Sudo address to use. SS58 Address.
                optional --sudo sudo: String

                /// The API URL to use for fetching chain state.
                optional --api-url api_url: String
            }
        }

        /// Generates a new spec file for the test net.
        cmd generate-spec {
            /// The base chain spec to use. Default dev will be used when empty.
            optional -c, --base-chain-spec base_chain_spec: PathBuf

            /// Output file for the chain spec.
            required -o, --out output: PathBuf

            repeated --gran gran_keys: String

            repeated --aura gran_keys: String

            repeated --balance balances: String

            optional --sudo sudo_key: String
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Run(Run),
    GenerateSpec(GenerateSpec),
}

#[derive(Debug)]
pub struct Run {
    pub path: Option<PathBuf>,
    pub alice: bool,
    pub bob: bool,
    pub isolated: bool,
    pub node_name: Option<String>,
    pub node_key: Option<String>,
    pub node_validator: Option<bool>,
    pub tcp_port: Option<u16>,
    pub rpc_port: Option<u16>,
    pub bootnodes: Vec<String>,
    pub subcommand: RunCmd,
}

#[derive(Debug)]
pub enum RunCmd {
    Local(Local),
    Replica(Replica),
}

#[derive(Debug)]
pub struct Local {
    pub chain_spec: Option<PathBuf>,
    pub account_suri: Option<String>,
}

#[derive(Debug)]
pub struct Replica {
    pub output: Option<PathBuf>,
    pub sudo: Option<String>,
    pub api_url: Option<String>,
}

#[derive(Debug)]
pub struct GenerateSpec {
    pub base_chain_spec: Option<PathBuf>,
    pub out: PathBuf,
    pub gran: Vec<String>,
    pub aura: Vec<String>,
    pub balance: Vec<String>,
    pub sudo: Option<String>,
}

impl Xtask {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
