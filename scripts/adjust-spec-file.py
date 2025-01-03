#!/usr/bin/env python3

import argparse
import json

# Utils

ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

def load_json(path: str):
    with open(path, 'r') as f:
        return json.load(f)


# CLI args

parser = argparse.ArgumentParser(
    description='Adjust chain spec file with node configuration')
arg = parser.add_argument

arg('node_env', help='Node environment (e.g. mainnet, testnet)', type=str)
arg('spec_file', help='Path to the input chain spec file', type=str)

arg("--bootnodes-file", help="Path to the bootnodes file (JSON)", type=str)
arg("--aura-list-file", help="Path to the aura list file (JSON)", type=str)
arg("--gran-list-file", help="Path to the gran list file (JSON)", type=str)
arg("--balances-file", help="Path to the balances file (JSON)", type=str)

arg("--merge-balances",
    help="Merge external balances with the spec file balances", action="store_true")

arg("--sudo-key", help="Sudo key to use", type=str)
arg("--name", help="Node name", type=str, default="Torus")

args = parser.parse_args()

node_env = args.node_env
base_spec_file = args.spec_file
bootnodes_path = args.bootnodes_file
aura_list_path = args.aura_list_file
gran_list_path = args.gran_list_file


spec_data = load_json(base_spec_file)

# == Node config / metadata ==

if node_env == "mainnet":
    spec_data['chainType'] = "Live"
    spec_data['id'] = "torus"
else:
    spec_data['chainType'] = "Development"
    spec_data['id'] = f"torus-{node_env}"

if args.name:
    spec_data['name'] = args.name

if bootnodes_path:
    # Inject bootnodes
    bootnodes = load_json(bootnodes_path)
    spec_data['bootNodes'] = bootnodes

# == Runtime values patch ==

patch_obj = spec_data['genesis']['runtimeGenesis']['patch']

if aura_list_path:
    # Inject AURA authority pub key list
    aura_list = load_json(aura_list_path)
    patch_obj['aura']['authorities'] = aura_list

if gran_list_path:
    # Inject GRANDPA authority pub key list
    gran_list = load_json(gran_list_path)
    patch_obj['grandpa']['authorities'] = gran_list

if args.sudo_key:
    patch_obj['sudo']['key'] = args.sudo_key
elif node_env != "mainnet":
    # Check sudo key for mainnet is not Alice
    assert patch_obj['sudo']['key'] == ALICE

if args.balances_file:
    balances = load_json(args.balances_file)
    if args.merge_balances:
        if node_env == "mainnet":
            raise RuntimeError("Cannot merge balances on mainnet")
        patch_obj['balances']['balances'] += balances
    else:
        patch_obj['balances']['balances'] = balances


json_txt = json.dumps(spec_data, indent=2)
print(json_txt)
