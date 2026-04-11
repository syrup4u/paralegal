#!/bin/bash

# Test 1
TARGET_DIR="../bug-db-example"
POLICY_FILE="policy.txt"

# Test 2
# TARGET_DIR="../exp-jwt"
# POLICY_FILE="policy_jwt2.txt"

# Run
sed -i 's/let p = Path::new(".*");/let p = Path::new("'$POLICY_FILE'");/' build.rs
cargo run -- --external-annotations external-annotations.toml $TARGET_DIR
