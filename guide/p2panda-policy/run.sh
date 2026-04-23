#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="$SCRIPT_DIR/../exp-p2panda"
ANNOTATIONS="$TARGET_DIR/external-annotations.toml"

usage() {
    echo "Usage: $0 [policy-file]"
    echo
    echo "Examples:"
    echo "  $0"
    echo "  $0 policy2"
    echo "  $0 policy3.txt"
    echo
    echo "Available policies:"
    find "$SCRIPT_DIR" -maxdepth 1 -name 'policy*.txt' -printf '  %f\n' | sort
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
fi

POLICY_ARG="${1:-policy.txt}"
if [[ "$POLICY_ARG" == */* ]]; then
    POLICY_FILE="$POLICY_ARG"
elif [[ -f "$SCRIPT_DIR/$POLICY_ARG" ]]; then
    POLICY_FILE="$SCRIPT_DIR/$POLICY_ARG"
elif [[ "$POLICY_ARG" != *.txt && -f "$SCRIPT_DIR/$POLICY_ARG.txt" ]]; then
    POLICY_FILE="$SCRIPT_DIR/$POLICY_ARG.txt"
else
    echo "Policy file not found: $POLICY_ARG" >&2
    echo >&2
    usage >&2
    exit 1
fi

POLICY_DIR="$(cd "$(dirname "$POLICY_FILE")" && pwd)"
POLICY_FILE="$POLICY_DIR/$(basename "$POLICY_FILE")"

cd "$SCRIPT_DIR"
P2PANDA_POLICY_FILE="$POLICY_FILE" cargo run -- --external-annotations "$ANNOTATIONS" "$TARGET_DIR"
