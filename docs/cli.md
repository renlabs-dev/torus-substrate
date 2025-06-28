# torus-substrate Command-Line Interface (`cli/`)

> A thin, user-friendly shell around the `client` library crate that exposes everyday account and staking operations as simple terminal commands.

---

## 1 · Purpose

The CLI lets operators, validators, and scripts interact with a **running Torus chain** without touching raw RPC calls or Subxt. It aims to be:

* **Stable** – subcommand names & flags change only in major versions.
* **Scriptable** – every action has a `--json` mode with deterministic output.
* **Portable** – a single static binary works against Linux/macOS/Windows.
* **Discoverable** – `--help` output forms its own quick reference.

---

## 2 · Crate location & entrypoints

| Item                     | Location                         |
| ------------------------ | -------------------------------- |
| Workspace member         | `cli/`                           |
| Cargo package            | `torus_cli`                      |
| Binary name              | `torus` (produced at build time) |
| Entry point              | `cli/src/main.rs`                |
| Arg/flag definitions     | `cli/src/args.rs`                |
| Command implementations  | `cli/src/commands/*`             |
| Dev-only extras (faucet) | gated behind `dev-tools` feature |

---

## 3 · Directory layout inside `cli/`

```text
cli/
├── Cargo.toml              # crate manifest
└── src/
    ├── main.rs            # starts the async runtime & dispatches commands
    ├── args.rs            # top-level clap structs (Cli, GlobalOpts, Commands)
    └── commands/
        ├── balance.rs     # free-balance, staked-balance, show, …
        ├── transfer.rs    # transfer, transfer-stake
        ├── staking.rs     # stake, unstake
        └── faucet.rs      # run-faucet (dev only)
```

Each `commands/*` module is a **thin adapter**: convert CLI args → call `client` functions → render output.

---

## 4 · Global options

| Flag            | Default               | Purpose                                 |
| --------------- | --------------------- | --------------------------------------- |
| `--rpc <url>`   | `ws://127.0.0.1:9944` | WebSocket endpoint of the chain node.   |
| `--json`        | *off*                 | Emit machine-readable JSON to `stdout`. |

All subcommands inherit these.

---

## 5 · Subcommands

| Group        | Subcommands                                                   |
| ------------ | ------------------------------------------------------------- |
| **Balance**  | `free-balance`, `staked-balance`, `get-staked`, `show`        |
| **Transfer** | `transfer`, `transfer-stake`                                  |
| **Staking**  | `stake`, `unstake`                                            |
| **Dev**      | `run-faucet` (requires `--features dev-tools` *or* dev build) |

Run `torus <group> --help` to see flags & examples.

---

## 6 · JSON vs. pretty output

* When `--json` is **absent**, the CLI prints human-oriented tables and colored text.
* When `--json` is **present**, **only** structured JSON is written to `stdout`; progress bars and logs go to `stderr` so you can pipe safely:

```bash
torus balance show 5FHneW…        # pretty table

torus --json balance show 5FHneW… | jq .free
```

JSON schemas live under `docs/json/` and are versioned.

---

## 7 · Building & running

```bash
# Build the binary with default features
cargo build -p cli --release

# Build with faucet support (dev chain only)
cargo build -p cli --release --features dev-tools

# Quick smoke test against a local node
./target/release/torus --rpc ws://localhost:9944 balance show Alice
```

A **Dockerfile** is available in `docker/torus-cli.dockerfile` for containerised distribution.

---

## 8 · Feature flags

| Feature     | Adds…                                       | When to enable                      |
| ----------- | ------------------------------------------- | ----------------------------------- |
| `dev-tools` | `run-faucet`, keyring shortcuts (`//Alice`) | local dev networks / testnet faucet |
| *none*      | Production-safe command set                 | everyday mainnet use                |

---

## 9 · Testing

* **Unit tests** (`cargo test -p cli`) mock the `client` trait layer.
* **Snapshot tests** use \[`assert_cmd`] and \[`insta`]: call the binary and snapshot stdout (`pretty`) & JSON.
* **E2E tests** in `tests/` spin up a containerised node via `testcontainers`.

CI runs all three on every PR.

---

## 10 · Release & versioning

* CLI is **co-versioned** with the node/runtime: tag `vX.Y.Z` represents *both* binaries.
* Changelog entries live in `CHANGELOG.md` under *CLI* heading.
* Binaries are uploaded to the GitHub release along with SHA-256 checksums.

---

## 11 · Future work

* Multi-network profiles (`--network mainnet/testnet/custom`)
* Offline signing (`torus tx --offline …`)
* Plugin system for third-party pallets

Pull requests welcome!
