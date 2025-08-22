use std::{
    borrow::Cow,
    net::Ipv4Addr,
    path::{Path, PathBuf},
    process::{exit, Child, Command, Stdio},
    time::Duration,
};

use crate::{flags::Workbench, run::ops::run_node, torus_node, Node};

pub(super) fn run(workbench: Workbench) {
    if !std::fs::exists("tmp/workbench").unwrap() {
        std::fs::create_dir_all("tmp/workbench").unwrap();
    }

    compile_node();

    let spec_path = if let Some(chain_spec) = workbench.chain_spec {
        assert_exists(chain_spec)
    } else {
        generate_spec()
    };

    build_mcp(&spec_path);

    install_mcp();

    let children = run_nodes(&spec_path);

    std::thread::sleep(Duration::from_secs(2));

    run_claude();

    std::thread::sleep(Duration::from_secs(1));

    for mut ele in children {
        let _ = ele.kill();
    }
}

fn compile_node() {
    println!("Building node...");
    let mut cmd = std::process::Command::new("cargo");
    cmd.args(["b", "-r", "-p", "torus-node"]);
    let output = cmd.output().unwrap();

    if !output.status.success() {
        exit(output.status.code().unwrap_or(-1));
    }
}

fn assert_exists(path: PathBuf) -> PathBuf {
    if !std::fs::exists(&path).unwrap() {
        eprintln!("File {path:?} does not exist");
        exit(-1);
    }

    path
}

fn generate_spec() -> PathBuf {
    println!("Generating dev spec...");

    let chain_spec: PathBuf = "tmp/workbench/spec.json".into();
    if std::fs::exists(&chain_spec).unwrap() {
        std::fs::remove_file(&chain_spec).unwrap();
    }

    let output = torus_node!("build-spec", "--dev", "--disable-default-bootnode")
        .output()
        .unwrap();
    if !output.status.success() {
        eprintln!(
            "Could not generate dev spec:\n{}",
            String::from_utf8_lossy(&output.stderr[..])
        );
        exit(-1);
    }

    std::fs::write(&chain_spec, output.stdout).unwrap();

    chain_spec
}

fn build_mcp(spec: &Path) {
    println!("Building mcp...");
    #[allow(clippy::zombie_processes)]
    let mut child_net = run_node(
        &"tmp/workbench/mcpnet",
        &spec.as_os_str(),
        &ALICE_NODE,
        &[],
        false,
    )
    .stdout(Stdio::null())
    .stdin(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .unwrap();

    std::thread::sleep(Duration::from_secs(1));

    let mut build_cmd = Command::new("just");
    build_cmd.arg("build-devnet-mcp");
    let output = build_cmd.output().unwrap();
    if !output.status.success() {
        eprintln!(
            "Could not build mcp:\n{}",
            String::from_utf8_lossy(&output.stderr[..])
        );
        exit(-1);
    }

    child_net.kill().unwrap();
}

fn install_mcp() {
    println!("Installing mcp");

    let home_dir = std::env::home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();
    let home_dir = home_dir.strip_suffix("/").unwrap_or(home_dir);
    let claude_cmd = format!("{home_dir}/.claude/local/claude");

    let mut cmd = Command::new(&claude_cmd);
    cmd.args(["mcp", "remove", "-s", "user", "torus-mcp"]);
    let _ = cmd.output().unwrap();

    let mut cmd = Command::new(&claude_cmd);
    cmd.args(["mcp", "add", "torus-mcp"]);
    cmd.arg(format!(
        "{}/target/release/torus-mcp",
        env!("CARGO_MANIFEST_DIR").strip_suffix("/xtask").unwrap()
    ));
    cmd.args(["-s", "user"]);

    let output = cmd.output().unwrap();
    if !output.status.success() {
        eprintln!(
            "Could not install mcp:\n{}",
            String::from_utf8_lossy(&output.stderr[..])
        );
        exit(-1);
    }
}

fn run_nodes(spec: &Path) -> Vec<Child> {
    let alice = run_node(&"tmp/workbench/alice", &spec, &ALICE_NODE, &[], false)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let bob = run_node(
        &"tmp/workbench/bob",
        &spec,
        &BOB_NODE,
        &[ALICE_NODE.bootnode_uri(Ipv4Addr::LOCALHOST.into())],
        false,
    )
    .stdout(Stdio::null())
    .stdin(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .unwrap();

    vec![alice, bob]
}

fn run_claude() {
    let home_dir = std::env::home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();
    let home_dir = home_dir.strip_suffix("/").unwrap_or(home_dir);
    let claude_cmd = format!("{home_dir}/.claude/local/claude");

    let mut cmd = Command::new(&claude_cmd);
    // cmd.arg("--debug");
    let _ = cmd.spawn().unwrap().wait().unwrap();

    println!("Goodbye");
}

static ALICE_NODE: Node<'static> = Node {
    name: Some(Cow::Borrowed("Alice")),
    id: Some(Cow::Borrowed(
        "12D3KooWBorpca6RKiebVjeFJA5o9iVWnZpg98yQbYqRC6f8CnLw",
    )),
    key: Some(Cow::Borrowed(
        "2756181a3b9bca683a35b51a0a5d75ee536738680bcb9066c68be1db305a1ac5",
    )),
    tcp_port: 30341,
    rpc_port: 9944,
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
