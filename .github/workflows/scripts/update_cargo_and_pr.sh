#!/bin/bash

# Check if new version file exists
if [ ! -f new_polkavm_version.txt ]; then
  echo "No new version to update."
  exit 0
fi

NEW_VERSION=$(cat new_polkavm_version.txt)

# Update Cargo.toml with the new version
sed -i "s/polkavm = \".*\"/polkavm = \"$NEW_VERSION\"/" Cargo.toml

# Git configuration
git config --global user.name 'GitHub Actions'
git config --global user.email 'actions@github.com'

# Commit and push changes
git checkout -b update-polkavm-$NEW_VERSION
git add Cargo.toml
git commit -m "Update polkavm to version $NEW_VERSION"
git push origin update-polkavm-$NEW_VERSION

# Create a pull request using GitHub CLI
gh pr create --title "Update polkavm to version $NEW_VERSION" --body "Automated PR to update polkavm dependency."
