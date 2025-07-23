# CLAUDE.md

This file provides strict guidance to Claude Code when working with this Substrate blockchain repository. These rules are non-negotiable and must be followed exactly.

## Project Overview

Torus is a stake-driven peer-to-peer network built on Substrate. The blockchain manages agents (validators and miners), token emissions, staking, and governance. Code quality is critical - runtime panics will halt the entire chain.

## References

- @README.md - Project overview, quick start, network setup
- @CONTRIBUTING.md - Development setup (Nix), guidelines, testing
- @docs/pallet-structure.md - Architecture and API design
- @docs/xtask-manual.md - Development tooling guide
- @docs/linear-emission.md - Token distribution algorithm

## Core Pallets

- **`torus0`**: Agent registration, staking, burn mechanisms, fee management
- **`emission0`**: Token distribution with linear emission algorithm, weight control
- **`governance`**: Proposals, voting, treasury, roles (allocators, curators)
- **`permission0`**: Permission and access control

### Permission0 Pallet Structure

The `permission0` pallet manages delegated permissions and access control within the Torus network. Key components:

**Core Permission Types** (`pallets/permission0/src/permission.rs`):
- `PermissionContract<T>` - Main permission structure with delegator, recipient, scope, duration, and enforcement
- `PermissionId` - Unique permission identifier (H256 hash)
- `PermissionScope<T>` - Defines what actions the permission covers
- `NamespaceScope<T>` - Defines namespace path permissions for delegation

**Permission Scopes** (`pallets/permission0/src/permission/`):
- `pallets/permission0/src/permission/curator.rs` - `CuratorPermissions` and `CuratorScope` types
- `pallets/permission0/src/permission/emission.rs` - `EmissionAllocation`, `DistributionControl`, and `EmissionScope` types

**Implementation Handlers** (`pallets/permission0/src/ext/`):
- `pallets/permission0/src/ext/curator_impl.rs` - Functions for curator permission enforcement
- `pallets/permission0/src/ext/emission_impl.rs` - Functions for emission permission enforcement  
- `pallets/permission0/src/ext/namespace_impl.rs` - Functions for namespace permission enforcement

## Architecture Principles

- **API-first design**: Each pallet has separate `api` crate to prevent circular dependencies
- **Domain separation**: Complex logic split into focused modules (agent.rs, stake.rs, etc.)
- **Storage efficiency**: Use container types to minimize state size
- **Zero-panic policy**: Runtime code must NEVER panic under any circumstances

## Project Structure

- All pallet tests are located within the /tests folder in each pallet's folder

## Essential Commands

```sh
# Development environment (REQUIRED - provides correct Rust version, dependencies)
nix develop

# Running tests and checks
just                 # Run all checks and tests (default)
just check           # Clippy linting only
just test            # Test suite only
cargo xtask coverage # Generate code coverage report

# Local development
cargo xtask run local --alice        # Run local node with Alice account
cargo xtask generate-spec gen-new    # Create new chain spec
cargo build --release                # Build the node
```

## STRICT RUST CODING RULES

### Error Handling - CRITICAL SAFETY

- **MUST NEVER** use `unwrap()`, `expect()`, `assert!()`, or any panicking operations in pallet code
- **MUST ALWAYS** use `ensure!` macro for validation, NEVER `assert!`
- **MUST ALWAYS** use the `?` operator for error propagation
- **MUST ALWAYS** use pattern matching with proper error handling:
  ```rust
  let Some(value) = some_option else {
      return Err(Error::<T>::SomeError.into());
  };
  ```

### Arithmetic Operations - NO EXCEPTIONS

- **MUST NEVER** use raw arithmetic operators (`+`, `-`, `*`, `/`) in runtime code
- **MUST ALWAYS** use `saturating_add()`, `saturating_sub()`, `saturating_mul()` for balance operations
- **MUST ALWAYS** use `checked_div()` for division - NEVER the `/` operator
- **MUST ALWAYS** use `FixedU128` for ALL percentage and ratio calculations
- **MUST ALWAYS** handle overflow explicitly with `checked_*` operations when needed

### Function Structure - MANDATORY PATTERNS

- **MUST ALWAYS** write functions as generic `pub fn name<T: Config>()` rather than `impl<T: Config> Pallet<T>`
- **MUST ALWAYS** use type aliases: `AccountIdOf<T>`, `BalanceOf<T>`, etc.
- **MUST ALWAYS** validate ALL inputs in private functions before storage operations
- **MUST NEVER** expose unsafe operations through public functions

### Storage Operations - STRICT REQUIREMENTS

- **MUST ALWAYS** use `try_mutate` when the operation can fail
- **MUST ALWAYS** check existence with `contains_key` before accessing storage
- **MUST ALWAYS** use `BoundedVec` for ALL storage collections
- **MUST ALWAYS** validate data BEFORE storage mutations
- **MUST NEVER** perform multiple storage writes when one would suffice

### Extrinsic Requirements - NO FLEXIBILITY

- **MUST ALWAYS** use manual call indexing: `#[pallet::call_index(n)]`
- **MUST ALWAYS** specify weight info for ALL extrinsics
- **MUST ALWAYS** emit appropriate events after state changes
- **MUST ALWAYS** use `ensure_signed(origin)?` at the start of signed extrinsics

### Documentation - MANDATORY

- **MUST ALWAYS** document ALL storage items, extrinsics, errors, and public functions
- **MUST ALWAYS** use `///` doc comments for items exported to client SDKs
- **MUST NEVER** leave TODOs or incomplete implementations in production code

### Import and Dependency Rules

- **MUST ALWAYS** use `polkadot_sdk` umbrella crate - NEVER individual substrate dependencies
- **MUST ALWAYS** use `use crate::*` for intra-pallet imports
- **MUST NEVER** use wildcard imports except for preludes

### Type Safety - ZERO TOLERANCE

- **MUST ALWAYS** use `BoundedVec<T, ConstU32<MAX>>` for storage, NEVER `Vec<T>`
- **MUST ALWAYS** convert with error handling: `BoundedVec::try_from(vec)?`
- **MUST ALWAYS** use proper type conversions with `.try_into()` and handle errors
- **MUST NEVER** use `as` for lossy numeric conversions

### Common Code Patterns - REQUIRED

- **MUST ALWAYS** emit events after successful state changes:
  ```rust
  Pallet::<T>::deposit_event(Event::<T>::SomethingHappened(who, what));
  ```
- **MUST ALWAYS** validate string data is UTF-8:
  ```rust
  ensure!(core::str::from_utf8(bytes).is_ok(), Error::<T>::InvalidUtf8);
  ```
- **MUST ALWAYS** check bounds before operations:
  ```rust
  ensure!(value <= T::MaxValue::get(), Error::<T>::ValueTooLarge);
  ```

### Testing Requirements - NON-NEGOTIABLE

- **MUST ALWAYS** write comprehensive tests for ALL extrinsics
- **MUST ALWAYS** test error conditions and edge cases
- **MUST ALWAYS** benchmark ALL extrinsics for weight calculation
- **MUST NEVER** merge code without adequate test coverage

### Migration Safety - CRITICAL

- **MUST ALWAYS** use `VersionedMigration` for storage migrations
- **MUST ALWAYS** increment storage version when changing storage
- **MUST NEVER** modify storage structure without a migration
- **MUST ALWAYS** test migrations with realistic data

### Code Style - ENFORCED BY CI

- **MUST ALWAYS** run `cargo fmt` before committing
- **MUST ALWAYS** fix ALL clippy warnings
- **MUST ALWAYS** use descriptive variable names, no single letters
- **MUST NEVER** use repetitive and redundant comments within code
- **MUST NEVER** ignore compiler or clippy warnings with `#[allow(...)]`

### Before committing:

1. **MUST** run `cargo fmt`
2. **MUST** run `just check` and fix all warnings
3. **MUST** run `just test` and ensure all pass
4. **MUST** run `cargo xtask coverage` to verify coverage
5. **MUST** test runtime upgrades if storage changed