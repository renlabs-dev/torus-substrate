# CLAUDE.md

This file provides guidance to Claude Code when working with this Substrate blockchain repository.

## References

- @README.md - Project overview, quick start, network setup
- @CONTRIBUTING.md - Development setup (Nix), guidelines, testing
- @docs/pallet-structure.md - Architecture and API design
- @docs/xtask-manual.md - Development tooling guide

## Core Pallets

- **`torus0`**: Agent registration, staking, burn mechanisms
- **`emission0`**: Token distribution with linear emission algorithm  
- **`governance`**: Proposals, voting, treasury, roles
- **`permission0`**: Permission and access control

## Essential Commands

```sh
nix develop          # Enter development environment
just                 # Run checks and tests (default)
just check           # Clippy linting only
just test            # Test suite only
cargo xtask coverage # Test coverage report
```

## Critical Safety Rules

- Never use `unwrap()`, `expect()`, or panicking operations in pallet code
- Use `checked_div()` instead of `/` for arithmetic operations  
- Panics in runtime code will halt the chain
- Use `FixedU128` for financial calculations
