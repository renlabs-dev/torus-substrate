# Contributing to Torus

Thanks for your interest in contributing to Torus! Follow this document to learn how we write our code and contribute. The guidelines are _not_ written in stone and each case has its own needs, so feel free to reach out to the team in our Discord server (we don't do DMs).

## Table of contents

- [Development environment setup](#development-environment-setup)
- [Project structure](#project-structure)
- [Code style and guidelines](#code-style-and-guidelines)
- [Storage modifications and migrations](#storage-modifications-and-migrations)
- [Development workflow](#development-workflow)
- [Testing guidelines](#testing-guidelines)
- [Pull request process](#pull-request-process)

## Development environment setup

The project uses Nix flakes to manage dependencies and ensure a consistent development environment across all contributors. It's worth your time setting up! Believe me, you don't want RocksDB building from scratch.

1. Install Nix package manager by following the instructions at [nixos.org](https://nixos.org/download.html)
2. Enable flakes by adding the following to your `~/.config/nix/nix.conf` or `/etc/nix/nix.conf`:
   ```
   experimental-features = nix-command flakes
   ```
3. Enter the development environment:

   ```bash
   nix develop
   ```

   This will set up the correct Rust version, git, and precompiled binaries for dependencies like RocksDB and OpenSSL.

4. Build the project:
   ```bash
   cargo build --release
   ```

## Project structure

Our project is organized into 5 main crates:

- **`node/`**: Contains the actual node implementation including network setup, RPC call definitions, synchronization behavior, database configurations, and more. Mostly boilerplate from substrate.

- **`pallets/`**: Contains the runtime logic modules separated by responsibility:

  - `emission0`: handles all token emission logic;
  - `governance`: manages proposals, allocators, votes, and new agent applications;
  - `torus0`: manages agent registrations and interactions with the network.

- **`runtime/`**: Defines the configuration used by the node and lists all the used pallets.

## Code style and guidelines

We follow standard Rust coding conventions with some additional requirements specific to blockchain development.

### General Rust guidelines

Be idiomatic. Use `Iterator`s, `Result`s, etc. PLEASE. Take a look at [Rust's API guidelines](https://rust-lang.github.io/api-guidelines/). Our CI will reject your changes if they don't pass formatting (it's 202X, enable autoformatting FFS) or clippy check. Code must be reasonably documented. Not everything, of course, but things like storage values, extrinsics, errors, must be documented as this information is extracted to other places like JS SDKs.

### Pallet-specific guidelines

- **Always use the `polkadot_sdk` umbrella dependency** instead of individual substrate dependencies to maintain version consistency. Last thing you want is a massive Cargo file with 150 substrate dependencies written out manually. Don't ask me how the experience of bumping them feels like. Some parts of the ecosystem do not have umbrellas, that's the case of Frontier, but we do what we can.
- **Avoid panicking in runtime code:**
  - Never use `unwrap()`, `expect()`, or other panic-causing operations in pallet code;
  - the WASM runtime that Substrate runs on cannot unwind, so panics in functions like `on_initialize` will halt the chain (been there - more than once);
  - use `Option` and `Result` types properly for error handling. NEVER allow `unused_must_use`. NEVER;
  - always prefer panic-free functions and treat all potential errors (`checked_div` in favor of `/`, etc.).

## Storage modifications and migrations

Modifying blockchain storage requires special care to maintain chain compatibility. When you modify a storage (creating/deleting storages, changing names, switching types, altering structs, reordering enum variants, ...) make sure to:

1. Increment the storage version for the affected pallet if it hasn't been updated since the last runtime upgrade

   - Check the git history to determine if the version has already been incremented
   - Always use `VersionedMigration` from `frame_support` to handle version increments

2. Create a storage migration when you alter existing storages.

### Implementing migrations

Migrations must be carefully implemented to ensure data consistency:

1. Create a new migration module that maps old values to the new storage format
2. ALWAYS use `VersionedMigration`
3. Add the migration to the runtime's list of migrations
4. Test the migration thoroughly with realistic data
5. Document the migration process in your PR

For detailed guidance on creating migrations, refer to the [Substrate Storage Migrations documentation](https://docs.polkadot.com/develop/parachains/maintenance/storage-migrations/).

## Development workflow

1. Fork the repo and create a new branch for your feature or bugfix:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes, following our code guidelines and testing requirements

3. Run tests and checks to ensure your changes don't break existing functionality:

   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```

4. Check code coverage to ensure sufficient test coverage:

   ```bash
   cargo xtask coverage
   ```

5. Submit a pull request with a clear description of your changes.
   We are very strict about title/descriptions. Your title SHOULD follow the conventional commits format (`feat(pallets/emission0): emitting 10x to myself`), but MUST be well described. Commits are squashed, so your PR's description will be the body of the merged commit, be generous!

## Testing guidelines

Strong test coverage is a core value for our project. All pallets must maintain high test coverage. Trust me, you want this. Of course, coverage alone doesn't mean it is _well_ tested. But it's a good enough metric. We will review your tests.

### Test coverage

> Code coverage is done via [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov). It comes with Nix.

Run the following command to generate a `Cobertura` xml file on `target/cov.xml` that can be used with the [Coverage Gutters](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters) VSCode plugin to display which functions and branches are not covered by tests yet.

```bash
cargo xtask coverage
```

If the `--html` attribute is passed to the command, an HTML website will be generated instead. It serves the same purpose as the plugin mentioned and can be accessed on `target/llvm-cov/html/index.html`

When contributing, please:

- Ensure your changes don't decrease overall test coverage
- Add tests for new functionality
- Update existing tests when modifying behavior
- Write both unit tests and integration tests where appropriate

While the `node/` and `runtime/` crates currently have simpler testing requirements, any complex logic added to these crates should also include appropriate tests.

## Pull request process

1. Ensure your code passes all tests and meets our quality standards
2. Update documentation if necessary
3. Include a clear description of the changes and the problem they solve
4. Link to any relevant issues
5. Request review from appropriate team members
6. Address any review feedback

Thank you for contributing to our project!
