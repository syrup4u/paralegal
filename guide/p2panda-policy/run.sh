#!/bin/bash

choice=$1

TARGET_DIR="../exp-p2panda"
ANNOTATIONS="$TARGET_DIR/external-annotations.toml"

if [ "$choice" == "1" ]; then
    POLICY_FILE="policy_a.txt"
elif [ "$choice" == "2" ]; then
    POLICY_FILE="policy_b.txt"
elif [ "$choice" == "3" ]; then
    POLICY_FILE="policy_c.txt"
elif [ "$choice" == "4" ]; then
    POLICY_FILE="policy_d.txt"
elif [ "$choice" == "5" ]; then
    POLICY_FILE="policy_e.txt"
else
    echo "Invalid choice."
    exit 1
fi

# Run
sed -i 's/let p = Path::new(".*");/let p = Path::new("'$POLICY_FILE'");/' build.rs
cargo run -- --external-annotations $ANNOTATIONS $TARGET_DIR
