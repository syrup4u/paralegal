#!/bin/bash

# Test 1
if [ "$1" == "test1" ]; then
    TARGET_DIR="../bug-db-example"
    POLICY_FILE="policy.txt"
elif [ "$1" == "test2" ]; then
    TARGET_DIR="../exp-jwt"
    POLICY_FILE="policy_jwt2.txt"
else
    echo "Usage: $0 [test1|test2]"
    exit 1
fi

# Run
sed -i 's/let p = Path::new(".*");/let p = Path::new("'$POLICY_FILE'");/' build.rs
cargo run -- --external-annotations external-annotations.toml $TARGET_DIR
