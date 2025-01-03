check:
  cargo clippy

run-localnode profile="--alice":
  cargo xtask run local {{profile}}

run-workflows:
  act --secret-file .env \
    -P 'ubuntu-24.04-8core-bakunin=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-24.04-16core-friedrich=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-22.04-32core-karl=ghcr.io/catthehacker/ubuntu:act-22.04'

try-runtime-upgrade-testnet:
    cargo build --release --features try-runtime
    RUST_BACKTRACE=1 RUST_LOG=info try-runtime --runtime target/release/wbuild/torus-runtime/torus_runtime.compact.compressed.wasm on-runtime-upgrade --blocktime 8000 live --uri wss://api.testnet.torus.network

try-runtime-upgrade-mainnet:
    cargo build --release --features try-runtime
    RUST_BACKTRACE=1 RUST_LOG=info try-runtime --runtime target/release/wbuild/torus-runtime/torus_runtime.compact.compressed.wasm on-runtime-upgrade --blocktime 8000 live --uri wss://api.torus.network
