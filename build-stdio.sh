#!/bin/bash
set -e

# Configuration
PROJECT_ROOT="$(cargo metadata --format-version=1 --no-deps | jq -r '.workspace_root')"
EXAMPLE_DIR="examples/stdio"

echo "🚀 Building stdio example..."

# 1. Build cargo-spike tool
cargo build -p zeroos-build --bin cargo-bolt --release --quiet

echo "Building cargo-spike..."
cargo build -p spike-build --bin cargo-spike --release --quiet

export PATH="${PROJECT_ROOT}/target/release:$HOME/.local/riscv/bin/:${PATH}"

# 2. Build Rust static library (stdio-ffi) using cargo spike build
echo "📦 Building stdio-ffi (Rust)..."
cd "${PROJECT_ROOT}"
cargo spike build \
	-p stdio-ffi \
	--mode std \
	--memory-origin 0x80000000 \
	--memory-size 128Mi \
	--heap-size 64Mi \
	--stack-size 2Mi \
	--release

# 3. Prepare ZeroOS output layout
echo "📋 Preparing output directories..."
TARGET_NAME="riscv64imac-zero-linux-musl"
OUTPUT_BASE="${PROJECT_ROOT}/target/${TARGET_NAME}/debug/zeroos/stdio-c"
mkdir -p "${OUTPUT_BASE}"
TARGET_SPEC="${OUTPUT_BASE}/${TARGET_NAME}.json"
LINKER_SCRIPT="${OUTPUT_BASE}/linker.ld"
OUTPUT_DIR="${OUTPUT_BASE}"

cargo spike generate target \
	--profile riscv64imac-zero-linux-musl \
	--output "${TARGET_SPEC}"

# 4. Generate linker script
echo "📋 Generating linker script..."
cargo spike generate linker \
	--ram-start 0x80000000 \
	--ram-size 128Mi \
	--heap-size 64Mi \
	--stack-size 2Mi \
	--entry-point _start \
	--output "${LINKER_SCRIPT}"

# 5. Build C application
echo "🔨 Building C application..."
cd "${PROJECT_ROOT}/${EXAMPLE_DIR}/c"
LIB_PATH="${PROJECT_ROOT}/target/${TARGET_NAME}/release/libstdio_ffi.a"
make clean OUTPUT_DIR="${OUTPUT_DIR}"
make CC="${HOME}/.bolt/musl/bin/riscv64-linux-musl-gcc" \
	LIB_FFI="${LIB_PATH}" \
	OUTPUT_DIR="${OUTPUT_DIR}" \
	LINKER="${LINKER_SCRIPT}" \
	LIB_DIR="${HOME}/.bolt/musl/riscv64-linux-musl/lib"

echo "✅ Build complete! Running..."
# 6. Run it using cargo-spike
cargo spike run "${OUTPUT_DIR}/stdio-c" --isa RV64IMAC --instructions 10000000
