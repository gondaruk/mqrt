#!/usr/bin/env bash
# Taken from https://github.com/vectordotdev/vector/blob/master/scripts/version.sh

set -euo pipefail

VERSION="${VERSION:-"$(awk -F ' = ' '$1 ~ /version/ { gsub(/["]/, "", $2); printf("%s",$2) }' Cargo.toml)"}"
echo "$VERSION"
