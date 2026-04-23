#!/bin/bash

choice=$1

if [ "$choice" == "0" ]; then
    TARGET_DIR="../file-db-example"
    POLICY_FILE="policy.txt"
elif [ "$choice" == "1" ]; then
    TARGET_DIR="../exp-1"
    POLICY_FILE="policy_db3.txt"
elif [ "$choice" == "2" ]; then
    TARGET_DIR="../exp-2"
    POLICY_FILE="policy.txt"
elif [ "$choice" == "3" ]; then
    TARGET_DIR="../exp-3"
    POLICY_FILE="policy_ACB.txt"
elif [ "$choice" == "4" ]; then
    TARGET_DIR="../exp-4"
    POLICY_FILE="policy_ACB.txt"
else
    echo "Invalid choice."
    exit 1
fi

# Toy Case
# TARGET_DIR="../file-db-example"
# POLICY_FILE="policy.txt"

# Test 1-1
# TARGET_DIR="../bug-db-example"
# POLICY_FILE="policy_db.txt"

# Test 1-2
# TARGET_DIR="../bug-db-example"
# POLICY_FILE="policy_db3.txt"

# Test 1-3
# TARGET_DIR="../bug-db-example2"
# POLICY_FILE="policy_db.txt"

# Test 2
# TARGET_DIR="../exp-jwt"
# POLICY_FILE="policy_jwt2.txt"

# Test 3
# TARGET_DIR="../exp-jwt-2"
# POLICY_FILE="policy_jwt3.txt"

# Test 4
# TARGET_DIR="../exp-jwt-3"
# POLICY_FILE="policy_jwt4.txt"

# Run
sed -i 's/let p = Path::new(".*");/let p = Path::new("'$POLICY_FILE'");/' build.rs
cargo run -- --external-annotations external-annotations.toml $TARGET_DIR
