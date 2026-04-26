#!/bin/bash

target_dir=$1
if [ -z "$target_dir" ]; then
    echo "Usage: $0 <target_dir>"
    exit 1
fi

find "$target_dir" -name "*.rs" | xargs wc -l