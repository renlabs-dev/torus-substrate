#!/usr/bin/env python3

import argparse
import json

ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

parser = argparse.ArgumentParser(
    description='Adjust chain spec file with node configuration')
parser.add_argument(
    'node_env', help='Node environment (e.g. mainnet, testnet)', type=str)
parser.add_argument(
    'spec_file', help='Path to the input chain spec file', type=str)
parser.add_argument(
    "--aura-list-file", help="Path to the aura list file (JSON)", type=str)
parser.add_argument(
    "--gran-list-file", help="Path to the gran list file (JSON)", type=str)
parser.add_argument(
    "--balances-file", help="Path to the balances file (JSON)", type=str)
parser.add_argument(
    "--merge-balances", help="Merge external balances with the spec file balances", action="store_true")
parser.add_argument(
    "--sudo-key", help="Sudo key to use", type=str)

args = parser.parse_args()

node_env = args.node_env
base_spec_file = args.spec_file
aura_list_path = args.aura_list_file
gran_list_path = args.gran_list_file


def load_json(path: str):
    with open(path, 'r') as f:
        return json.load(f)


spec_data = load_json(base_spec_file)

if node_env == "mainnet":
    spec_data['chainType'] = "Live"
    spec_data['id'] = "torus"
else:
    spec_data['chainType'] = "Development"
    spec_data['id'] = f"torus-{node_env}"

patch_obj = spec_data['genesis']['runtimeGenesis']['patch']

if aura_list_path:
    # Inject AURA authority pub key list
    aura_list = load_json(aura_list_path)
    patch_obj['aura']['authorities'] = aura_list

if gran_list_path:
    # Inject GRANDPA authority pub key list
    gran_list = load_json(gran_list_path)
    patch_obj['grandpa']['authorities'] = gran_list

# Check sudo key is correct
spec_sudo_key = patch_obj['sudo']['key']
if args.sudo_key:
    patch_obj['sudo']['key'] = args.sudo_key
elif node_env != "mainnet":
    assert spec_sudo_key == ALICE

# # Inject EVM chain id
# patch_obj['evmChainId']['chainId'] = 69420

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
