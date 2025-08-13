# torus-mcp

A simple MCP that interacts with the torus network.  
The MCP is currently used internally for tests on top of a premade set of dev accounts.

## How to use

Build it

```
# testnet
just build-testnet-mcp
# devnet (must have a devnet running locally on the port 9944)
just build-devnet-mcp
```

Move the output so it doesn't get lost with the project (Optional)

Install it and be happy! (e.g `claude mcp add torus-mcp /local/to/the/executable`)
```
