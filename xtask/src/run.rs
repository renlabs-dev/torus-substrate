use std::{borrow::Cow, net::Ipv4Addr};

use super::*;

pub(super) fn run(mut r: flags::Run) {
    let (mut node, mut account) = match (r.alice, r.bob) {
        (true, false) => {
            if r.bootnodes.is_empty() {
                r.bootnodes
                    .push(BOB_NODE.bootnode_uri(Ipv4Addr::LOCALHOST.into()));
            }
            (ALICE_NODE.clone(), ALICE_ACCOUNT.clone())
        }
        (false, true) => {
            if r.bootnodes.is_empty() {
                r.bootnodes
                    .push(ALICE_NODE.bootnode_uri(Ipv4Addr::LOCALHOST.into()));
            }
            (BOB_NODE.clone(), BOB_ACCOUNT.clone())
        }
        (false, false) => (Node::default(), Account::default()),
        _ => panic!("select only one of: --alice, --bob"),
    };

    node.name = r.node_name.map(Into::into).or(node.name);
    node.validator = r.node_validator.unwrap_or(node.validator);
    node.tcp_port = r.tcp_port.unwrap_or(node.tcp_port);
    node.rpc_port = r.rpc_port.unwrap_or(node.rpc_port);
    if let Some(node_key) = r.node_key {
        let node_id = ops::key_inspect_node_cmd(&node_key);
        node.key = Some(node_key.into());
        node.id = Some(node_id.into());
    }

    let path = r.path.unwrap_or_else(|| {
        tempfile::Builder::new()
            .prefix("torus-node-data")
            .suffix(node.name.as_ref().unwrap_or(&Cow::Borrowed("")).as_ref())
            .tempdir()
            .expect("failed to create tempdir")
            .into_path()
    });

    match (path.exists(), path.is_dir()) {
        (true, false) => panic!("provided path must be a directory"),
        (false, false) => std::fs::create_dir(&path).unwrap(),
        _ => {}
    }

    let (chain_spec, _local_seal) = match &r.subcommand {
        flags::RunCmd::Local(local) => {
            let chain_path = local
                .chain_spec
                .as_ref()
                .map(|s| s.to_str().expect("invalid string"))
                .unwrap_or_else(|| "dev");

            account.suri = local
                .account_suri
                .as_ref()
                .map(Into::into)
                .unwrap_or(account.suri);

            (chain_path, true)
        }
    };

    ops::key_insert_cmd(&path, &chain_spec, &account.suri, "aura");
    ops::key_insert_cmd(&path, &chain_spec, &account.suri, "gran");

    let _run = ops::run_node(&path, &chain_spec, &node, &r.bootnodes, r.isolated)
        .spawn()
        .unwrap()
        .wait();
}

#[allow(dead_code)]
mod ops {
    use super::*;
    use std::{
        ffi::OsStr,
        io::Write,
        process::{Command, Stdio},
    };

    macro_rules! torus_node {
        ($($arg:expr),*) => {{
            let mut cmd = Command::new("cargo");
            cmd.args(["run", "--release", "--package", "torus-node", "--"]);
            $(cmd.arg($arg);)*
            cmd
        }};
    }

    pub fn build_chain_spec(chain_spec: &str) -> Command {
        torus_node!(
            "build-spec",
            "--raw",
            "--chain",
            chain_spec,
            "--disable-default-bootnode"
        )
    }

    pub fn key_generate() -> Command {
        torus_node!(
            "key",
            "generate",
            "--scheme",
            "sr25519",
            "--output-type",
            "json"
        )
    }

    pub fn key_insert_cmd(
        base_path: &dyn AsRef<OsStr>,
        chain_spec: &dyn AsRef<OsStr>,
        suri: &str,
        key_type: &str,
    ) {
        let scheme = match key_type {
            "aura" => "sr25519",
            "gran" => "ed25519",
            _ => panic!(),
        };

        #[rustfmt::skip]
        torus_node!(
            "key", "insert",
            "--base-path", base_path,
            "--chain", chain_spec,
            "--scheme", scheme,
            "--suri", suri,
            "--key-type", key_type
        )
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to run key insert");
    }

    pub fn key_inspect_cmd(suri: &str) -> Command {
        torus_node!(
            "key",
            "inspect",
            "--scheme",
            "ed25519",
            "--output-type",
            "json",
            suri
        )
    }

    pub fn key_inspect_node_cmd(key: &str) -> String {
        let mut child = torus_node!("key", "inspect-node-key")
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to inspect node key");
        child
            .stdin
            .as_mut()
            .expect("missing stdin")
            .write_all(key.as_bytes())
            .expect("failed to write node key");
        let output = child.wait_with_output().expect("inspect-node-key failed");
        String::from_utf8(output.stdout).expect("invalid node id")
    }

    pub fn run_node(
        base_path: &dyn AsRef<OsStr>,
        chain_spec: &dyn AsRef<OsStr>,
        node: &Node<'_>,
        bootnodes: &[String],
        isolated: bool,
    ) -> Command {
        #[rustfmt::skip]
        let mut cmd = torus_node!(
            "--base-path", base_path,
            "--chain", chain_spec,
            "--unsafe-rpc-external",
            "--rpc-cors", "all",
            "--port", node.tcp_port.to_string(),
            "--rpc-port", node.rpc_port.to_string(),
            "--allow-private-ipv4",
            "--discover-local",
            "--rpc-max-response-size","100"
        );

        if !bootnodes.is_empty() {
            cmd.arg("--bootnodes").args(bootnodes);
        }

        if node.validator {
            cmd.args([
                "--validator",
                "--force-authoring",
                "--consensus",
                "manual-seal-8000",
            ]);
        }

        if let Some(name) = &node.name {
            cmd.args(["--name", name]);
        }

        if let Some(node_key) = &node.key {
            cmd.args(["--node-key", node_key]);
        }

        if isolated {
            cmd.args(["--in-peers", "0", "--out-peers", "0"]);
        }

        cmd
    }
}
