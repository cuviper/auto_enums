#!/bin/bash

# A script to run a simplified version of the checks done by CI.
#
# Usage:
#     bash scripts/ci.sh
#
# Note: This script requires nightly Rust, rustfmt, and clippy

set -euo pipefail

if [[ "${1:-none}" == "+"* ]]; then
    toolchain="${1}"
else
    cargo +nightly -V >/dev/null || exit 1
    toolchain="+nightly"
fi

if [[ "${toolchain:-+nightly}" != "+nightly"* ]] || ! rustfmt -V &>/dev/null || ! cargo clippy -V &>/dev/null; then
    echo "error: ci.sh requires nightly Rust, rustfmt, and clippy"
    exit 1
fi

echo "Running 'cargo ${toolchain} fmt --all'"
cargo "${toolchain}" fmt --all

echo "Running 'cargo ${toolchain} clippy --all --all-features --all-targets'"
cargo "${toolchain}" clippy --all --all-features --all-targets -Zunstable-options

echo "Running 'cargo ${toolchain} test --all --all-features'"
cargo "${toolchain}" test --all --all-features

echo "Running 'cargo ${toolchain} doc --no-deps --all --all-features'"
cargo "${toolchain}" doc --no-deps --all --all-features
