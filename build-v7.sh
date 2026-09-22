#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")"

TARGET="thumbv7em-none-eabihf"
BIN_NAME="grin-rc-rmk"
OUT_DIR="$PWD/target/$TARGET/release"
ELF="$OUT_DIR/$BIN_NAME"
BIN="$OUT_DIR/$BIN_NAME.bin"

echo "=== GRIN RC RMK build-v7 ==="
echo "  target : $TARGET"
echo "  package: $BIN_NAME"
echo

command -v cargo >/dev/null || { echo "ERROR: cargo not found" >&2; exit 1; }
rustup target list --installed 2>/dev/null | grep -Fxq "$TARGET" || {
    echo "ERROR: Rust target $TARGET is not installed." >&2
    echo "Run: rustup target add $TARGET" >&2
    exit 1
}

echo "[1/3] cargo build --release"
cargo build --release --target "$TARGET"

test -f "$ELF" || {
    echo "ERROR: expected ELF not found: $ELF" >&2
    exit 1
}

echo "[2/3] generate raw binary"
OBJCOPY=""
for tool in rust-objcopy arm-none-eabi-objcopy llvm-objcopy; do
    if command -v "$tool" >/dev/null 2>&1; then
        OBJCOPY="$tool"
        break
    fi
done

if [ -n "$OBJCOPY" ]; then
    "$OBJCOPY" -O binary "$ELF" "$BIN"
else
    echo "ERROR: no objcopy found (rust-objcopy / arm-none-eabi-objcopy / llvm-objcopy)." >&2
    echo "The ELF build succeeded; install an ARM/LLVM objcopy to generate .bin." >&2
    exit 1
fi

test -f "$BIN" || {
    echo "ERROR: expected BIN not found: $BIN" >&2
    exit 1
}

echo "[3/3] firmware hashes"
sha256sum "$ELF"
sha256sum "$BIN"

echo
echo "DONE:"
echo "  ELF: $ELF"
echo "  BIN: $BIN"
echo
echo "Flash/debug target:"
echo "  STM32F411CEUx"
echo "  probe-rs run --chip STM32F411CEUx $ELF"
