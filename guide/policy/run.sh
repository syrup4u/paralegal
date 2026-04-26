#!/bin/bash

choice=$1

# Experiment options
if [ "$choice" == "0" ]; then
    TARGET_DIR="../file-db-example"
    POLICY_FILE="policy.txt"
elif [ "$choice" == "1" ]; then
    TARGET_DIR="../exp-1"
    POLICY_FILE="policy.txt"
elif [ "$choice" == "2" ]; then
    TARGET_DIR="../exp-2"
    POLICY_FILE="policy.txt"
elif [ "$choice" == "3" ]; then
    TARGET_DIR="../exp-3"
    POLICY_FILE="policy_ACB.txt"
elif [ "$choice" == "4" ]; then
    TARGET_DIR="../exp-4"
    POLICY_FILE="policy_ACB.txt"
elif [ "$choice" == "5" ]; then
    TARGET_DIR="../exp-5"
    POLICY_FILE="policy_ACB.txt"
else
    echo "Invalid choice."
    exit 1
fi

# Run
sed -i 's/let p = Path::new(".*");/let p = Path::new("'$POLICY_FILE'");/' build.rs
cargo run -- --external-annotations external-annotations.toml $TARGET_DIR
