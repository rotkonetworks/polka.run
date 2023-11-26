#!/bin/bash

# # Fetch the latest tag from polkavm repository
# LATEST_TAG=$(curl -s https://api.github.com/repos/koute/polkavm/tags | jq -r '.[0].name')

# Fetch the latest version from crates.io
LATEST_VERSION=$(curl -s https://crates.io/api/v1/crates/polkavm | jq -r '.crate.newest_version')

# Extract the current polkavm version from Cargo.toml
CURRENT_VERSION=$(grep 'polkavm =' Cargo.toml | cut -d '"' -f 2)

# Compare versions
if [ "$LATEST_VERSION" != "$CURRENT_VERSION" ]; then
  echo $LATEST_VERSION > new_polkavm_version.txt
  echo "New version found: $LATEST_VERSION"
else
  echo "Current version is up to date."
fi
