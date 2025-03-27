# Generated Network Interfaces

This directory contains generated Rust code based on blockchain metadata from different networks.

## How to Generate

Use the following commands to generate the interfaces:

```sh
# Generate interfaces for mainnet (default)
just gen_interfaces

# Generate interfaces for testnet
just gen_interfaces testnet

# Generate interfaces for local development node
just gen_interfaces dev

# Generate interfaces for all live networks (mainnet and testnet)
just gen_for_live

# Generate interfaces for all networks
just all_networks
```

## File Structure

- `mainnet.rs` - Generated code for Torus mainnet
- `testnet.rs` - Generated code for Torus testnet
- `dev.rs` - Generated code for local development node

## Notes

- These files are generated using the `subxt codegen` command from metadata fetched from running nodes
- The files are not checked into version control - they should be generated as needed
- Generation requires the `subxt` CLI tool, which is included in the Nix development environment
