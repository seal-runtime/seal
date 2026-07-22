#!/bin/sh
if command -v seal > /dev/null 2>&1; then
    SEAL=seal
elif [ -x "./target/release/seal" ]; then
    SEAL="./target/release/seal"
elif [ -x "./target/debug/seal" ]; then
    SEAL="./target/debug/seal"
else
    echo "error: seal not found in PATH or ./target/; can't generate constants"
    exit 1
fi

$SEAL ./src/scripts/signature_generation/generate.luau