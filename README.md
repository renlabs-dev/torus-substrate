# Torus Network Node

Torus is a self-assembling and evolving peer-to-peer blockchain organism, with a
stake-driven network built in Rust with the Substrate framework. Torus powers an
innovative ecosystem of agents and incentives, incorporating diverse
technologies into its collective body.

## Quick start

TODO

## Building from source

### Dependencies

The recommended way to install dependencies is [using Nix](docs/nix.md).

If you don't want to use Nix, you should have the following dependencies:

- [Rust](https://www.rust-lang.org/)
- [protoc](https://github.com/protocolbuffers/protobuf)

You can install Rust using [Rustup]:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install `protoc`:

```sh
# on Ubuntu
sudo apt install protobuf-compiler
```

<!--
# on Arch
sudo pacman -S protobuf
# on macOS
brew install protobuf
-->

[Rustup]: https://rustup.rs/

## Building and running the node

```sh
cargo build -p torus-node --release
```

To run a node connected to the Torus testnet:

```sh
cargo run --bin torus-node --release -- --chain data/testnet/spec.json
```

## Docker

```sh
mkdir -p torus-node
cd torus-node
wget https://raw.githubusercontent.com/renlabs-dev/torus-substrate/refs/heads/main/docker-compose.yml -O docker-compose.yml
docker compose up -d
```

## Development

Check your code with:

```sh
cargo clippy
```

Run all tests with:

```sh
cargo test
```

Running a local dev node:

```sh
cargo xtask run local --alice
```
