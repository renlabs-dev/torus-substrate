# torus-cli

Torus blockchain cli using the torus client.

## Available commands

> `--testnet` can be added in order to run the command on the testnet.  
  
| Command | Description |
| ------- | ----------- |
| `torurs agent info <account>` | Shows information about the given agent. |
| `torurs agent register <key> <name> <description> <url> [-y]` | Registers key as an agent. |
| `torurs agent unregister <key> [-y]` | Unregisters key as an agent. |
| `torurs balance <account>` | Shows the account balance. |
| `torurs balance check <account>` | Does the same as the command above. |
| `torurs balance transfer <key> <target_account> <amount> [-y]` | Transfers **amount** from **key** to **address**. |
| `torurs key list` | Lists all saved keys. |
| `torurs key info <name>` | Shows information about a key prompting for the password if it is encrypted. |
| `torurs key create <name> [-p]` | Generates a new sr25519 key and prompts for a encryption password if `-p` is given. |
| `torurs key delete <name> [-y]` | Deletes a saved key. |
| `torurs namespace info <account>` | Shows all namespaces related to the given account. |
| `torurs namespace register <key> <path>` | Registers a namespace on the given account. |
| `torurs namespace unregister <key>` | Unregisters a namespace from the given account. |
| `torurs stake <account>` | Prints all keys **account** is staking with the amount. |
| `torurs stake given <account>` | Does the same as the command above. |
| `torurs stake received <account>` | Prints all keys **account** is staked by with the amount. |
| `torurs stake add <key> <target_account> <amount> [-y]` | Stakes **target_account** with **amount** from **key**. |
| `torurs stake remove <key> <target_account> <amount> [-y]` | Removes **amount** staked to **target_account** from **key**. |
| `torurs stake transfer <key> <source_account> <target_account> <amount> [-y]` | Transfers **amount** **key** has staked on **source_account** to **target_account**. |
  
> **address**: a ss58 address.  
> **key**: The name of a saved key.  
> **account**: either a ss58 address or the name of a saved key.  
> [-y]: The confirmation prompt is auto accepted if it is provided. 