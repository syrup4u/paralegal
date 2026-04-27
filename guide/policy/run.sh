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
elif [ "$choice" == "6" ]; then
    TARGET_DIR="../exp-6"
    POLICY_FILE="policy_AnotB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "a" ]; then
    TARGET_DIR="../exp-a"
    POLICY_FILE="policy_ACB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "b" ]; then
    TARGET_DIR="../exp-b"
    POLICY_FILE="policy_AnotB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "c" ]; then
    TARGET_DIR="../exp-c"
    POLICY_FILE="policy_ACB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "d" ]; then
    TARGET_DIR="../exp-d"
    POLICY_FILE="policy_ACB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "e" ]; then
    TARGET_DIR="../exp-e"
    POLICY_FILE="policy_AnotB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "f" ]; then
    TARGET_DIR="../exp-f"
    POLICY_FILE="policy_exists_process_sink.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "g" ]; then
    TARGET_DIR="../exp-g"
    POLICY_FILE="policy_ACB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "h" ]; then
    TARGET_DIR="../exp-h"
    POLICY_FILE="policy_ACB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "i" ]; then
    TARGET_DIR="../exp-i"
    POLICY_FILE="policy_AnotB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
elif [ "$choice" == "j" ]; then
    TARGET_DIR="../exp-j"
    POLICY_FILE="policy_ACB.txt"
    ANNOTATIONS="$TARGET_DIR/external-annotations.toml"
else
    echo "Invalid choice."
    exit 1
fi

ANNOTATIONS="${ANNOTATIONS:-external-annotations.toml}"

# Run
sed -i 's/let p = Path::new(".*");/let p = Path::new("'$POLICY_FILE'");/' build.rs
cargo run -- --external-annotations $ANNOTATIONS $TARGET_DIR
