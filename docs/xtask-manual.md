# Torus XTask Manual

The `xtask` crate provides a set of development tools that simplify working with the Torus blockchain. This manual documents how to use these tools effectively for local development, testing, and network simulation.

## Overview

The `xtask` tool provides three main commands:

- `run` for launching Torus nodes with customizable configurations,
- `generate-spec` for creating chain specification files for different network types,
- `coverage` for generating code coverage reports for the pallets.

These commands help streamline the development process, allowing you to quickly set up testing environments and validate your changes.

## Running Nodes

The `run` command launches Torus nodes with various configuration options. It offers two primary modes: local and replica.

### Local Node

The local mode starts a node using either a provided chain spec file or the default development configuration:

```sh
cargo xtask run local [OPTIONS]
```

Common options include:

- `--alice` to start the node with Alice's predefined account and keys,
- `--bob` to start the node with Bob's predefined account and keys,
- `--node-validator true` to run the node as a validator (can produce blocks),
- `-c, --chain-spec <FILE>` to use a specific chain specification file,
- `--account-suri <SURI>` to use a custom account seed in the format `<mnemonic>//<seed>`.

The Alice and Bob accounts come with predefined keys and network configurations. Alice uses TCP port 30341 and RPC port 9951, while Bob uses TCP port 30342 and RPC port 9952. When you run nodes using the `--alice` and `--bob` flags, they are automatically configured to connect to each other, simplifying local network setup. This can also be customized by settings other flags, take a look at `xargs/src/flags.rs`.

### Replica Node

The replica mode creates and runs a node that replicates the mainnet state:

```sh
cargo xtask run replica [OPTIONS]
```

Key options include:

- `--sudo <ADDRESS>` to set a custom sudo key in SS58 address format,
- `--api-url <URL>` to specify a custom WebSocket API endpoint for fetching chain state,
- `-o, --output <FILE>` to write the generated chain spec to a file.

This mode is particularly useful for testing runtime upgrades or simulating behaviors that depend on the current mainnet state.

## Generating Chain Specifications

The `generate-spec` command creates chain specification files that define the initial state of a blockchain. Torus xtask provides two subcommands for spec generation: `gen-new` and `gen-replica`.

### Creating a New Chain

The `gen-new` subcommand creates a fresh chain specification with customizable parameters:

```sh
cargo xtask generate-spec gen-new [OPTIONS] -o <OUTPUT_FILE>
```

Important options include:

- `--name <NAME>` to set a custom chain name,
- `--sudo <ADDRESS>` to set the sudo key in SS58 address format,
- `--aura <ADDRESS>` to add an Aura authority key (can be repeated),
- `--gran <ADDRESS>` to add a GRANDPA authority key (can be repeated),
- `--balance "<ADDRESS>=<AMOUNT>"` to set initial account balances (can be repeated),
- `-c, --base-chain-spec <FILE>` to use a specific chain spec as base (defaults to dev).

For example, to create a new chain with custom authorities and initial balances:

```sh
cargo xtask generate-spec gen-new \
  --name "My Test Network" \
  --sudo 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --aura 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --gran 5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu \
  --balance "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY=1000000000000000000" \
  -o my-chain-spec.json
```

### Creating a Mainnet Replica

The `gen-replica` subcommand creates a chain specification that replicates the current mainnet state:

```sh
cargo xtask generate-spec gen-replica [OPTIONS] -o <OUTPUT_FILE>
```

Key options include:

- `--api-url <URL>` to specify a custom WebSocket API endpoint for fetching chain state (defaults to "wss://api.torus.network")
- `--sudo <ADDRESS>` to override the sudo key in the generated spec.

This command fetches the entire state from a running node and creates a chain specification from it. You can then modify specific genesis values like the sudo key while preserving the rest of the state.

Example usage:

```sh
cargo xtask generate-spec gen-replica \
  --sudo 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --api-url "wss://my-custom-node.torus.network" \
  -o replica-spec.json
```

## Setting Genesis State Values

When creating a chain specification, you can customize various genesis state values through command-line options.

The consensus in Torus uses two types of authority keys. Aura keys (specified with `--aura`) are used for block production, and each validator needs an Aura authority key. GRANDPA keys (specified with `--gran`) are used for block finalization, and each validator needs a GRANDPA authority key. These keys must be in SS58 address format. When you set these keys, the corresponding accounts are granted the authority to participate in consensus.

You can set initial account balances using the `--balance` flag with the format `"<ADDRESS>=<AMOUNT>"`. The amount is specified in the smallest unit (planck), so you'll need to add the appropriate number of zeros for the desired token amount. For example, to set an account balance to 1 TORUS token (assuming 12 decimals), use `--balance "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY=1000000000000"`. You can specify multiple balances by repeating the `--balance` flag.

The sudo key is a special account that has administrative privileges on the chain. You can set it using the `--sudo` flag with an SS58 address. This account will be able to execute privileged functions through the Sudo pallet.

## Local Network Simulation

One of the most powerful features of the xtask tool is its ability to simulate a local Torus network. This is particularly useful for testing changes to the runtime or pallets.

The typical workflow is to first generate a chain spec file with desired properties, then launch multiple nodes using that spec file, and finally connect the nodes to form a network.

Here's an example of setting up a two-node local network:

```sh
# Generate a chain spec
cargo xtask generate-spec gen-new --sudo 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY -o local-spec.json

# Start Alice node as validator
cargo xtask run local --chain-spec local-spec.json --alice --node-validator true &

# Start Bob node as validator
cargo xtask run local --chain-spec local-spec.json --bob --node-validator true &
```

The nodes will automatically connect to each other and start producing blocks since they're both configured as validators.

## Code Coverage

The `coverage` command generates code coverage reports for the Torus pallets:

```sh
cargo xtask coverage [--html]
```

By default, this command produces a coverage report in the Cobertura XML format at `target/cov.xml`. If you specify the `--html` flag, it will generate an HTML report instead, which you can view at `target/llvm-cov/html/index.html`.

The coverage analysis focuses on the core pallets (`emission0`, `governance`, `torus0`) and excludes certain files like test utilities, weights, migrations, and benchmarks from the report. This command is valuable for ensuring that your tests adequately cover the codebase, particularly when making changes to the pallets.

## Internal Implementation

The xtask tools use several techniques to simplify working with Torus. It provides pre-configured accounts (Alice and Bob) that have predefined keys and network settings to enable quick setup. By default, node data is stored in temporary directories that are cleaned up automatically. For replica specs, the tool fetches the entire state from a running node and then modifies specific genesis fields. The tool also handles inserting the necessary keys and configuring the nodes, so you don't need to perform these steps manually.

## Advanced Usage

You can customize various node parameters to fit your specific needs:

```sh
cargo xtask run local [OPTIONS] --chain-spec my-spec.json \
  --node-name "Custom Node" \
  --node-key <KEY> \
  --tcp-port 30444 \
  --rpc-port 9966
```

For testing in isolation, you can prevent the node from connecting to other peers with the `--isolated` flag:

```sh
cargo xtask run local --chain-spec my-spec.json --isolated
```

You can also specify custom bootnodes for your node to connect to:

```sh
cargo xtask run local --chain-spec my-spec.json \
  --bootnodes "/ip4/192.168.1.1/tcp/30333/p2p/<node-id>" \
  --bootnodes "/ip4/192.168.1.2/tcp/30333/p2p/<node-id>"
```

## Troubleshooting

When working with xtask, you might encounter some common issues. If a node fails to start, ensure your chain spec is valid and correctly formatted, and check that all keys are valid SS58 addresses. If nodes don't connect, make sure your TCP ports are accessible and not blocked by a firewall. When running multiple nodes on the same machine, use different port numbers. If no blocks are produced, verify that at least one node is configured as a validator and has the correct authority keys.

For debugging purposes, you can examine the temporary node directories to check logs and configuration files. The path is printed when the node starts. You can also run a node without using xtask to get more detailed logs:

```sh
./target/release/torus-node -d /tmp/node-data --chain my-spec.json --validator
```
