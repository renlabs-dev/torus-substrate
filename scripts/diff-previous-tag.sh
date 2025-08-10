#!/usr/bin/env bash
set -euo pipefail

# Path to runtime lib
RUNTIME_LIB="runtime/src/lib.rs"

spec_version=$(grep -Eo "spec_version:[[:space:]]+[0-9]+" "$RUNTIME_LIB" | cut -f2 -d' ')
prev=$((spec_version - 1))

echo "Current spec_version: $spec_version"
echo "Looking for previous version: $prev"

tags=("runtime/mainnet-$prev" "runtime/testnet-$prev")

found=""
for tag in "${tags[@]}"; do
  if git rev-parse --verify --quiet "$tag" > /dev/null; then
    found=$tag
    break
  fi
done

if [[ -n "$found" ]]; then
  echo "Found previous tag: $found"
else
  echo "Error: no previous runtime tag found for version $prev" >&2
  exit 1
fi

git --no-pager diff -U5 "$found"..HEAD -- ':!*.json' ':!*.lock' ':!**/tests/*.rs' ':!project-selector' ':!**/weights.rs'
