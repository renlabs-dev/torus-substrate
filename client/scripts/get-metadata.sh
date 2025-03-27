#!/bin/bash
set -e

NETWORK=$1
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIENT_DIR="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$CLIENT_DIR/data/metadata"

case $NETWORK in
  "mainnet")
    URL="wss://api.torus.network"
    ;;
  "testnet")
    URL="wss://api.testnet.torus.network"
    ;;
  "dev")
    URL="ws://localhost:9944"
    ;;
  *)
    echo "Usage: $0 {mainnet|testnet|dev}" >&2
    exit 1
    ;;
esac

mkdir -p "$OUTPUT_DIR"
subxt metadata --url "$URL" -f bytes > "$OUTPUT_DIR/$NETWORK.scale"
echo "Fetched $NETWORK metadata to $OUTPUT_DIR/$NETWORK.scale" >&2