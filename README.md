# Torus Network Node

[![](https://dcbadge.limes.pink/api/server/https://discord.gg/torus)](https://discord.gg/torus)

![Docker Build](https://github.com/renlabs-dev/torus-substrate/actions/workflows/build-docker.yml/badge.svg)

Torus is a self-assembling and evolving peer-to-peer organism with a
stake-driven ecosystem of agents and incentives. It's core coordination system
is a blockchain built on Substrate, implemented in this repository.

## Table of Contents

- [Quick Start](#quick-start)
  - [Running a node](#running-a-node)
  - [Building and running from source](#building-and-running-from-source)
- [Network connectivity](#network-connectivity)
- [Local network](#local-network)
- [Documentation](#documentation)
- [Terminology](#terminology)

## Quick Start

### Running a node

There are several ways to run a Torus node, depending on your needs. The fastest way is using Docker:

```sh
docker run -p 9944:9944 -p 30333:30333 ghcr.io/renlabs-dev/torus-substrate:latest torus-node --chain main --sync warp
```

This command will:

- Pull our latest Docker image
- Run a node connected to the Torus mainnet
- Use warp sync for faster synchronization
- Expose the WebSocket RPC port (9944) and p2p port (30333)

Or you could use docker-compose:

```sh
mkdir -p torus-node
cd torus-node
wget https://raw.githubusercontent.com/renlabs-dev/torus-substrate/refs/heads/main/docker-compose.yml -O docker-compose.yml
docker compose up -d
```

### Building and running from source

For detailed instructions on setting up the build environment with Nix and all required dependencies, please refer to our [CONTRIBUTING.md](CONTRIBUTING.md) guide. Once your environment is set up:

```sh
# Build the node
cargo build -p torus-node --release

# Run connected to mainnet
./target/release/torus-node --chain main

# Or run connected to testnet using the included spec file
./target/release/torus-node --chain data/testnet/spec.json

# For faster synchronization, add the warp flag
./target/release/torus-node --chain main --sync warp
```

## Network connectivity

The Torus node operates on two primary ports:

- `9944`: a WebSocket RPC for interacting with the node's API. It's recommended to always have SSL enabled and the node behind a reverse proxy.
- `30333`: Used for node-to-node communication in the network.

## Local network

We have tools to simulate a network locally as well. The `cargo xtask run` command allows you to run a local network from scratch by setting up nodes with fresh spec files:

```sh
cargo xtask generate-spec gen-new --sudo <your Torus key> -o myspec.json
cargo xtask run local --chain myspec.json --alice --node-validator true &
cargo xtask run local --chain myspec.json --bob --node-validator true &
```

The `generate-spec` command has been expanded with subcommands that allow you to create different types of chain specifications:

1. Generate a new empty chain spec with custom parameters:

```sh
cargo xtask generate-spec gen-new --name "My Custom Chain" --sudo <your Torus key> -o myspec.json
```

2. Generate a replica of the mainnet:

```sh
cargo xtask generate-spec gen-replica --sudo <your Torus key> -o myreplica.json
```

You can also pass an `--api-url` flag with a WS endpoint to clone the state from.

Both commands take additional parameters to configure Aura, Grandpa and balances. For example:

```sh
cargo xtask generate-spec gen-new \
  --name "My Chain" \
  --sudo <your Torus key> \
  --gran <grandpa key> \
  --aura <aura key> \
  --balance "<account>=<amount>" \
  -o myspec.json
```

You can then run nodes with the generated specs:

```sh
cargo xtask run local --chain myspec.json --alice --node-validator true &
cargo xtask run local --chain myspec.json --bob --node-validator true &
```

Other options are also available. Check the code or run `cargo xtask generate-spec -h` for more details!

## Documentation

We maintain several documentation files to help you understand and contribute to the Torus network:

- [CONTRIBUTING.md](CONTRIBUTING.md): Guidelines for contributing to the project, including development environment setup with Nix, code style standards, testing requirements, and our pull request process

- [docs/pallet-structure.md](docs/pallet-structure.md): Detailed explanation of our pallet architecture, including the API-first design, storage patterns, and coding conventions

- [docs/linear-emission.md](docs/linear-emission.md): Technical documentation on the linear emission algorithm used for token distribution in the `emission0` pallet

- [docs/xtask-manual.md](docs/xtask-manual.md): Comprehensive guide to the `xtask` command-line tool that simplifies local development, testing, and network simulation

- [pallet_emission0 docs](https://renlabs-dev.github.io/torus-substrate/pallet_emission0): Generated docs for the crate `pallet_emission0`  

- [pallet_governance docs](https://renlabs-dev.github.io/torus-substrate/pallet_governance): Generated docs for the crate `pallet_governance`  

- [pallet_permission0 docs](https://renlabs-dev.github.io/torus-substrate/pallet_permission0): Generated docs for the crate `pallet_permission0`  

- [pallet_torus0 docs](https://renlabs-dev.github.io/torus-substrate/pallet_torus0): Generated docs for the crate `pallet_torus0`  

## Terminology

We have some specific terminology throughout the codebase, so here's a list of the most important ones:

| Term                    | Description                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Agent                   | A participant in the Torus network identified by an account ID, with optional metadata like name and URL. Agents can be validators (who assign weights) or miners (who produce utility). The network tracks their registrations, stake, and emissions. See: Miner, Validator. [Source: [agent.rs](pallets/torus0/src/agent.rs)]                                                                                                              |
| Allocator               | The trusted and relevant validators of the network, chosen by governance. Currently, they're the only agents performing as validators, but this won't always be the case. Allocators have special permissions to perform actions like weight setting. See: Validator, Governance. [Source: [roles.rs](pallets/governance/src/roles.rs)]                                                                                                      |
| Application             | The process by which new participants request to join the Torus network as agents. Applications are reviewed and approved by governance, ensuring only quality participants enter the ecosystem. Successful applications result in agent registration once approved. See: Agent, Governance. [Source: [application.rs](pallets/governance/src/application.rs)]                                                                               |
| Burn                    | A mechanism that destroys tokens when registering new agents, particularly when the network is at capacity. The burn amount adjusts dynamically - it increases when registration activity is high and decreases when low. This creates a natural economic barrier to entry when the network is saturated. [Source: [burn.rs](pallets/torus0/src/burn.rs)]                                                                                    |
| Curator                 | A role responsible for managing the whitelist of agents that can participate in certain network activities. Curators help maintain network quality and might also operate off-chain, for example through popular votes. See: Agent. [Source: [whitelist.rs](pallets/governance/src/whitelist.rs)]                                                                                                                                            |
| Delegation Fee          | A percentage that validators can charge from stakers. When emissions are distributed, this fee is deducted from the staker's share and given to the validator. It incentivizes validators to maintain good performance to attract delegations. See: Stake, Validator. [Source: [fee.rs](pallets/torus0/src/fee.rs)]                                                                                                                          |
| Dividends               | Tokens distributed to validators based on their stake and the performance of agents they've weighted. Dividends reward validators that honestly weigh miners, creating an incentive for accurate assessment. Good validators who support productive miners earn more dividends. See: Incentives, Validator. [Source: [distribute.rs](pallets/emission0/src/distribute.rs)]                                                                   |
| Emission                | The process of creating new tokens and distributing them to network participants based on stake and weight assignments. Emissions occur at regular intervals defined by `RewardInterval` and form the economic engine of the network. See: Incentives, Dividends. [Source: [distribute.rs](pallets/emission0/src/distribute.rs)]                                                                                                             |
| Incentives              | Tokens distributed to miners based on the weights assigned to them by validators. Incentives reward utility and efficiency, with the goal of encouraging miners that produce the most value. They're designed to motivate future work and contributions to the network. See: Dividends, Miner. [Source: [distribute.rs](pallets/emission0/src/distribute.rs)]                                                                                |
| Miner                   | Agents that produce off-chain utility, typically via their API exposed at their URL, or through other means. Miners receive incentives based on the weights assigned to them by validators, which reflect their perceived value to the network. See: Validator, Weights. [Source: [agent.rs](pallets/torus0/src/agent.rs)]                                                                                                                   |
| Proposal                | A cornerstone of the network's governance. Proposals suggest changes to the network's parameters, configurations, or operations, which network participants can vote on. Not all proposals reflect changes in the network, but most do. Approved proposals are executed automatically. See: Governance, Proposal Rewards. [Source: [proposal.rs](pallets/governance/src/proposal.rs)]                                                        |
| Proposal Rewards        | Incentives distributed to participants who vote on proposals, coming directly from the treasury. The reward calculation uses a quadratic formula based on stake (square root of stake), which proportionally rewards smaller accounts more than larger ones. This system democratizes governance by encouraging participation across all account sizes. See: Proposal, Treasury. [Source: [proposal.rs](pallets/governance/src/proposal.rs)] |
| Stake                   | Tokens locked in support of an agent. Higher stakes affect the weights set by the validator and their influence in the network. Stake determines validator status (only top staked agents can be validators) and affects emission distribution rewards. See: Validator, Emission. [Source: [stake.rs](pallets/torus0/src/stake.rs)]                                                                                                          |
| Storage Value           | On-chain state variables that persist across blocks. Configuration storage values are configurable by the network through governance, making directing the chain a public effort. Examples include `PendingEmission`, `ConsensusMembers`, and `RewardInterval`. See: Proposal. [Source: [lib.rs](pallets/torus0/src/lib.rs)]                                                                                                                 |
| Treasury                | A special account controlled by governance that collects fees and a portion of emissions for community use. Treasury funds can be allocated through governance decisions to support network development and improvement. [Source: [proposal.rs](pallets/governance/src/proposal.rs)]                                                                                                                                                         |
| Validator               | Agents that prove the efficiency and utility of miners, typically by periodically checking the miners via their exposed URL (API). Validators must have sufficient stake to participate in consensus and assign weights to miners. Only the top staked agents (limited by `MaxAllowedValidators`) can act as validators. See: Weights, Miner. [Source: [agent.rs](pallets/torus0/src/agent.rs)]                                              |
| Voting Power Delegation | A mechanism that allows agents to delegate their governance voting power to other participants. When an agent delegates voting power, their stake is counted towards the delegate's votes on proposals. This enables passive participants to contribute to governance through trusted representatives. See: Proposal, Stake. [Source: [voting.rs](pallets/governance/src/voting.rs)]                                                         |
| Weight Control          | The system by which validators assign and manage weights to miners. Weight control is a critical responsibility that directly impacts network incentive distribution. Validators can delegate this duty to other validators through weight control delegation in exchange for a fee. See: Weight Control Fee, Weights. [Source: [weight_control.rs](pallets/emission0/src/weight_control.rs)]                                                |
| Weight Control Fee      | A fee paid when delegating weight assignment responsibility to another validator. This allows validators to outsource their assessment duties while still sharing in the rewards. See: Validator, Weight Control, Weights. [Source: [fee.rs](pallets/torus0/src/fee.rs)]                                                                                                                                                                     |
| Weights                 | Numerical values (0 to 65535) assigned by validators to other agents, reflecting the worth that the validator sees in a miner. Weights directly influence emission distribution and determine how incentives are allocated. Honest and accurate weight assignment is crucial for the network's health. See: Validator, Incentives. [Source: [distribute.rs](pallets/emission0/src/distribute.rs)]                                            |
