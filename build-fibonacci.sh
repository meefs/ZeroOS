#!/bin/bash
# Build and run fibonacci using cargo-spike

set -e

PROJECT_ROOT="$(cargo metadata --format-version=1 --no-deps | jq -r '.workspace_root')"

echo "Building cargo-bolt..."
cargo build -p zeroos-build --bin cargo-bolt --release --quiet

echo "Building cargo-spike..."
cargo build -p spike-build --bin cargo-spike --release --quiet

export PATH="${PROJECT_ROOT}/target/release:$HOME/.local/riscv/bin/:$PATH"

echo "Building fibonacci in no-std mode..."
# no-std mode
cargo spike build -p fibonacci --features=debug --quiet
cargo spike run target/riscv64imac-unknown-none-elf/debug/fibonacci --isa RV64IMAC --instructions 10000000

echo "Building fibonacci in std mode..."
# std mode - enable vfs, memory, and thread features for syscall support
cargo spike build -p fibonacci --mode std --features=std,debug --quiet
RUST_LOG=debug cargo spike run target/riscv64imac-zero-linux-musl/debug/fibonacci --isa RV64IMAC --instructions 100000000 -l --log fib-std.log
