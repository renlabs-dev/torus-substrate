# Deploy steps

1. Checkout to the commit/branch that should be deployed with `git checkout <ref>`.
2. Do some last code checks.
    - Make sure all tests pass by running `cargo test`
    - Make sure the runtime migrations work by running `just try-runtime-upgrade-{testnet,mainnet}`  
3. Create the release tag following the format `runtime/{test,main}net-{spec_version}`. Example: runtime/testnet-12 (the spec version can be found in `runtime/src/lib.rs` under the `runtime_macro!` call). This will trigger a Github Action that will build the runtime wasm binary.
4. Download said runtime binary and check it's checksum.
5. Connect the Torus Testnet Root account to [Polkadot Developer Interface](https://polkadot.js.org/apps/#/explorer) using the [PolkadotJS browser extension](https://polkadot.js.org/extension/).
    - Make sure the interface is configured to point to the right net by clicking on the left sidebar, scrolling all the way to the bottom and under the development tab it should be Custom with the address `wss://api.testnet.torus.network` or `wss://api.torus.network`.
6. Upload the code to the runtime using the [Polkadot Developer Interface](https://polkadot.js.org/apps/#/explorer).
    - Click on the `Sudo` button under the `Developer` tab.
    - Select the `system` pallet and the `setCode` extrinsic.
    - Select the .wasm file that comes with the .zip downloaded from GitHub.
    - Sign and Submit.

> As of april 9th 2025 the try-runtime cli is broken, install the fork by running `cargo install --git https://github.com/renlabs-dev/try-runtime-cli --locked`.