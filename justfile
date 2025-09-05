default: fmt check test

# Build

build-mainnet:
  cargo build --release --timings --package torus-runtime

build-testnet:
  cargo build --release --features testnet --timings --package torus-runtime

# Development

check: fmt
  cargo clippy --tests

test: check
  SKIP_WASM_BUILD=1 cargo nextest run

fmt:
  cargo fmt

select crates="" depth="2":
  SKIP_WASM_BUILD=1 cargo r -p project-selector -- {{crates}} \
    -b "*polkadot*,*cumulus*,*snowbridge*,*parachain*,*xcm*,pallet*,fp*,*metadata-hash*,bp*,bridge*,fc*,substrate*build*" \
    -a "polkadot-sdk*,pallet*api" \
    -d {{depth}} > rust-project.json

run-localnode profile="--alice":
  cargo xtask run local {{profile}}

# Specs

base_spec_path := "node/specs/base.json"

gen-base-spec:
  cargo run -p torus-node --release -- build-spec --chain dev > "{{base_spec_path}}"

gen-spec-file env: gen-base-spec
  mkdir -p tmp/spec

  node_version=$(cargo run -p torus-node --release -- --version) \
  && scripts/adjust-spec-file.py "{{env}}" "{{base_spec_path}}" \
      --balances-file data/torus-genesis-balances.json \
      --merge-balances \
      --aura-list-file "data/{{env}}/aura.pub.json" \
      --gran-list-file "data/{{env}}/gran.pub.json" \
      --bootnodes-file "data/{{env}}/bootnodes.json" \
      --name "Torus {{env}} $node_version" \
      > "tmp/spec/{{env}}.json"
  
  @echo "Spec file generated at: tmp/spec/{{env}}.json"

# Benchmarks

run-benchmarks:
  cargo build -r --features runtime-benchmarks
  ./target/release/torus-node benchmark pallet --pallet pallet_torus0 --chain dev --extrinsic "*" --steps 50 --repeat 20 --output pallets/torus0/src/weights.rs --template=./.maintain/frame-weight-template.hbs
  ./target/release/torus-node benchmark pallet --pallet pallet_governance --chain dev --extrinsic "*" --steps 50 --repeat 20 --output pallets/governance/src/weights.rs --template=./.maintain/frame-weight-template.hbs
  ./target/release/torus-node benchmark pallet --pallet pallet_emission0 --chain dev --extrinsic "*" --steps 50 --repeat 20 --output pallets/emission0/src/weights.rs --template=./.maintain/frame-weight-template.hbs
  ./target/release/torus-node benchmark pallet --pallet pallet_permission0 --chain dev --extrinsic "*" --steps 50 --repeat 20 --output pallets/permission0/src/weights.rs --template=./.maintain/frame-weight-template.hbs

# Runtime Update Testing

install-try-runtime:
  cargo install --git https://github.com/paritytech/try-runtime-cli --locked

try-runtime-upgrade-testnet:
    cargo build --release --features try-runtime,testnet
    RUST_BACKTRACE=1 RUST_LOG=info try-runtime --runtime target/release/wbuild/torus-runtime/torus_runtime.compact.compressed.wasm on-runtime-upgrade --blocktime 8000 live --uri wss://api.testnet.torus.network

try-runtime-upgrade-mainnet:
    cargo build --release --features try-runtime
    RUST_BACKTRACE=1 RUST_LOG=info try-runtime --runtime target/release/wbuild/torus-runtime/torus_runtime.compact.compressed.wasm on-runtime-upgrade --blocktime 8000 live --uri wss://api.torus.network

# Github Actions

run-workflows:
  act --secret-file .env \
    -P 'ubuntu-24.04-8core-bakunin=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-24.04-16core-friedrich=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-22.04-32core-karl=ghcr.io/catthehacker/ubuntu:act-22.04'

# Mcp

build-devnet-mcp:
  cargo clean -p torus-client
  cargo clean -p torus-mcp
  cargo b -p torus-mcp -r --no-default-features --features devnet

build-testnet-mcp:
  cargo clean -p torus-client
  cargo clean -p torus-mcp
  cargo b -p torus-mcp -r

install-claude-mcp: build-devnet-mcp
  claude mcp add torus-mcp target-mcp/release/torus-mcp
