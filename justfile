default: check test

# Development

check:
  cargo clippy

test:
  cargo test

run-localnode profile="--alice":
  cargo xtask run local {{profile}}

# Specs

base_spec_path := "node/specs/base.json"

gen-base-spec:
  cargo run -p torus-node --release -- build-spec --chain dev > "{{base_spec_path}}"

gen-spec-file env: gen-base-spec
  mkdir -p tmp/spec

  node_version=$(cargo run -p torus-node -- --version) \
  && scripts/adjust-spec-file.py "{{env}}" "{{base_spec_path}}" \
      --balances-file data/torus-genesis-balances.json \
      --merge-balances \
      --name "Torus {{env}} $node_version" \
      > "tmp/spec/{{env}}.json"
  
  @echo "Spec file generated at: tmp/spec/{{env}}.json"

run-benchmarks:
  cargo build -r --features runtime-benchmarks
  . /target/release/node-subspace build-spec --disable-default-bootnode --chain local > specs/benchmarks.json
  ./target/release/node-subspace benchmark pallet --chain specs/local.json --pallet pallet_torus0  --extrinsic "*" --steps 50 --repeat 20 --output pallets/torus0/src/weights.rs --template=./.maintain/frame-weight-template.hbs
  ./target/release/node-subspace benchmark pallet --chain specs/local.json --pallet pallet_governance  --extrinsic "*" --steps 50 --repeat 20 --output pallets/governance/src/weights.rs --template=./.maintain/frame-weight-template.hbs
  ./target/release/node-subspace benchmark pallet --chain specs/local.json --pallet pallet_emission0  --extrinsic "*" --steps 50 --repeat 20 --output pallets/emission0/src/weights.rs --template=./.maintain/frame-weight-template.hbs


# Github Actions

run-workflows:
  act --secret-file .env \
    -P 'ubuntu-24.04-8core-bakunin=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-24.04-16core-friedrich=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-22.04-32core-karl=ghcr.io/catthehacker/ubuntu:act-22.04'

