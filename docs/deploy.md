# Deploy steps

1. Checkout to the commit/branch that should be deployed with `git checkout <ref>`.
2. Do some last code checks.
    - Make sure all tests pass by running `cargo test`
    - Make sure the runtime migrations work by running `just try-runtime-upgrade-{testnet,mainnet}`  
3. Create the release tag following the format `runtime/{test,main}net-{spec_version}`. Example: runtime/testnet-12 (the spec version can be found in `runtime/src/lib.rs` under the `runtime_macro!` call). This will trigger a Github Action that builds the runtime WASM binary.
4. Download said runtime binary and check it's checksum.
    - The .zip file can be found on the `Artifacts` section of the Action that built it. Search for your commit message that has `Build Torus runtime` written below it on [this page](https://github.com/renlabs-dev/torus-substrate/actions). The correct file name is `torus_runtime.compact.compressed`.
    - Check that the hash given by the command `sha256sum <path-to-zipped-file>` is the same as the one beside the download button on the action page, on the `Digest` column.
    - The WASM blob to be used on the next steps is inside the downloaded zipped file. Extract it somewhere easy to find later.
6. Connect the Torus Testnet Root account to [Polkadot Developer Interface](https://polkadot.js.org/apps/#/explorer) using the [PolkadotJS browser extension](https://polkadot.js.org/extension/).
    - Make sure the interface is configured to point to the right net by clicking on the left sidebar, scrolling all the way to the bottom and under the development tab it should be Custom with the address `wss://api.testnet.torus.network` (for testnet) or `wss://api.torus.network` (for mainnet).
7. Upload the code to the runtime using the [Polkadot Developer Interface](https://polkadot.js.org/apps/#/explorer).
    - Click on the `Sudo` button under the `Developer` tab.
    - Select the `system` pallet and the `setCode` extrinsic.
    - Select the .wasm file extracted on the 4th step.
    - Sign and Submit.

> As of april 9th 2025 the try-runtime cli is broken, install the fork by running `cargo install --git https://github.com/renlabs-dev/try-runtime-cli --locked`.
