#!/usr/bin/env bash
set -euo pipefail

# version.sh
#
# SUMMARY
#
#   Responsible for computing the release version of Mqrt.
#   This is based on version in Cargo.toml.

VERSION="${VERSION:-"$(awk -F ' = ' '$1 ~ /version/ { gsub(/["]/, "", $2); printf("%s",$2) }' Cargo.toml)"}"
echo "$VERSION"
