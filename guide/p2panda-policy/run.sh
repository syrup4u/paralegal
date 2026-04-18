#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="$SCRIPT_DIR/../exp-p2panda"
ANNOTATIONS="$TARGET_DIR/external-annotations.toml"

cd "$SCRIPT_DIR"
cargo run -- --external-annotations "$ANNOTATIONS" "$TARGET_DIR"
