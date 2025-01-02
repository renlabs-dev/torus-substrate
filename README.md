# Torus

**The Thermodynamic God's Favorite Child**

Node implementation of a Substrate-based protocol manifesting autonomous monetary networks through recursive stake delegation and emergent multi-graph interactions.

♓︎ Built using the [Polkadot-SDK](https://github.com/paritytech/polkadot-sdk)
♓︎ Website: [torus.network](https://torus.network/)

## Prerequisites

All dependencies are managed through Nix:
- [Nix Package Manager](https://determinate.systems/posts/determinate-nix-installer)
- [Direnv](https://direnv.net/) (recommended)

That's all you need! See Quick Start for build instructions.

## Quick Start

### Using Docker

```sh
# Connect to mainnet
docker run --rm -ti -p 9944:9944 -p 9933:9933 ghcr.io/renlabs-dev/torus-substrate:latest --chain=main
```

### Building from Source

#### Using Nix (recommended)

1. **Install Nix**
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
   ```

2. **Install Direnv**
   - Install [Direnv](https://direnv.net/docs/installation.html):
     ```sh
     nix profile install nixpkgs#direnv
     ```
   - Add to your `$HOME/.zshrc` if using zsh:
     ```sh
     eval "$(direnv hook zsh)"
     ```
   - Run in project directory:
     ```sh
     direnv allow
     ```

3. **Build the node**
   ```sh
   cargo build --release
   ```

4. **Connect to mainnet**
   ```sh
   ./target/release/torus-node --chain=main
   ```

### Connecting to a Testnet

Use the `--chain=test` flag

**Using Docker:**
```sh
docker run --rm -ti -p 9944:9944 -p 9933:9933 ghcr.io/renlabs-dev/torus-substrate:latest --chain=test
```

**Using Source:**
```sh
./target/release/torus-node --chain=test
```

### Running a Localnet

```sh
cargo xtask run local --alice --node-validator true
```

> Note: Without specifying a chain spec, `--dev` will be used automatically (sudo account defaults to alice)

With custom chain spec:
```sh
cargo xtask run local -c main.json --alice --node-validator true
```

Connect to this node at `127.0.0.1:9951`


### Handeling Chain Spec

To generate a chain spec, use the following command:

```sh
./target/release/torus-node build-spec --chain dev > main.json
```

You can then edit this file, such as modify balances, sudo key, or aura and grandpa authorities.

> [!TIP]
> Convert the chain spec to raw format before using it with the node.

```sh
# here we build the raw spec based on the main.json file generated above
./target/release/torus-node build-spec --chain=main.json --raw > main.json
```

## Testing

```bash
cargo test
```

## Node Types & Requirements

### System Requirements

Minimum (Full Node):
- CPU: 4 cores
- RAM: 8GB
- Storage: 50GB SSD
- Stable internet connection

Archive Nodes:
- Same requirements but with 500GB+ Storage

### Node Types

**Full Node**
```sh
./target/release/torus-node --chain=main
```

**Archive Node**
```sh
./target/release/torus-node --chain=main --pruning=archive
```

### Useful CLI Flags

- `--name`: Specify your node's name
- `--pruning`: Choose pruning mode (archive, 256, 1024...)
- `--port`: Specify p2p protocol TCP port
- `--ws-port`: Specify WebSocket RPC server TCP port
- `--rpc-port`: Specify HTTP RPC server TCP port

### Telemetry

Connect to mainnet telemetry:
```sh
./target/release/torus-node --chain=main --telemetry-url 'ws://telemetry.torus.network:8001/submit 0'
```

View telemetry at:
- Mainnet: [telemetry.torus.network](https://telemetry.torus.network/)
- Testnet: [telemetry.testnet.torus.network](https://telemetry.testnet.torus.network/)

## EVM Support

Torus implements Ethereum compatibility via the Frontier EVM pallet.

| Network       | Chain ID |
|---------------|----------|
| Torus Mainnet | 21000    |

Explorer is deployed at [blockscout.torus.network](https://blockscout.torus.network/)

## List of Public Endpoints

| Network       | RPC URL                            | WSS URL                          |
| --------------|------------------------------------|----------------------------------|
| Torus Mainnet | `https://api.torus.network`        | `wss://api.torus.network`        |
| Torus Testnet | `https://api.testnet.torus.network`| `wss://api.testnet.torus.network`|

## Runtime Architecture

The Torus Runtime is built using FRAME and includes pallets from:
- Polkadot-SDK
- Frontier
- Custom pallets in `pallets/` directory

Key local pallets are:
- _Torus_ (Core)
- _Emission_ (Tokenomics and Consensus)
- _Governance_ (DAO)

## Documentation

Rustdocs: Coming Soon!
