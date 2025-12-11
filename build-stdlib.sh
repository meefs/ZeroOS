#!/bin/bash
# Build and run stdlib using cargo-spike

set -e

PROJECT_ROOT="$(cargo metadata --format-version=1 --no-deps | jq -r '.workspace_root')"

echo "Building cargo-bolt..."
cargo build -p zeroos-build --bin cargo-bolt --release --quiet

echo "Building cargo-spike..."
cargo build -p spike-build --bin cargo-spike --release --quiet

export PATH="${PROJECT_ROOT}/target/release:$HOME/.local/riscv/bin/:${PATH}"

# std mode
echo "Building stdlib in std mode..."
cargo spike build -p stdlib --mode std --features=std --quiet
cargo spike run target/riscv64imac-zero-linux-musl/debug/stdlib --isa RV64IMAC --instructions 10000000
