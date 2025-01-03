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
  cargo run -p torus-node -- build-spec --chain dev > "{{base_spec_path}}"

gen-spec-file env: gen-base-spec
  mkdir -p tmp/spec
  scripts/adjust-spec-file.py "{{env}}" "{{base_spec_path}}" \
    --balances-file data/torus-genesis-balances.json \
    --merge-balances \
    > tmp/spec/{{env}}.json

# Github Actions

run-workflows:
  act --secret-file .env \
    -P 'ubuntu-24.04-8core-bakunin=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-24.04-16core-friedrich=ghcr.io/catthehacker/act-ubuntu:24.04' \
    -P 'ubuntu-22.04-32core-karl=ghcr.io/catthehacker/ubuntu:act-22.04'
