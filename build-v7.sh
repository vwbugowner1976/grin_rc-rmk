#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")"

TARGET="thumbv7em-none-eabihf"
BIN_NAME="grin-rc-rmk"
OUT_DIR="$PWD/target/$TARGET/release"
ELF="$OUT_DIR/$BIN_NAME"
BIN="$OUT_DIR/$BIN_NAME.bin"

# Windows firmware output
WINDOWS_FIRMWARE_DIR="/mnt/d/rmk-firmware/grin_rc-rmk"
WINDOWS_BIN="$WINDOWS_FIRMWARE_DIR/$BIN_NAME.bin"

echo "=== GRIN RC RMK build-v7 ==="
echo "  target : $TARGET"
echo "  package: $BIN_NAME"
echo "  output : $WINDOWS_BIN"
echo

command -v cargo >/dev/null || {
    echo "ERROR: cargo not found" >&2
    exit 1
}

rustup target list --installed 2>/dev/null | grep -Fxq "$TARGET" || {
    echo "ERROR: Rust target $TARGET is not installed." >&2
    echo "Run: rustup target add $TARGET" >&2
    exit 1
}

echo "[1/4] cargo build --release"
cargo build --release --target "$TARGET"

test -f "$ELF" || {
    echo "ERROR: expected ELF not found: $ELF" >&2
    exit 1
}

echo "[2/4] generate raw binary"

OBJCOPY=""

for tool in rust-objcopy arm-none-eabi-objcopy llvm-objcopy; do
    if command -v "$tool" >/dev/null 2>&1; then
        OBJCOPY="$tool"
        break
    fi
done

if [ -n "$OBJCOPY" ]; then
    echo "  objcopy: $OBJCOPY"
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

echo "[3/4] firmware hashes"

sha256sum "$ELF"
sha256sum "$BIN"

echo "[4/4] copy BIN to Windows"

mkdir -p "$WINDOWS_FIRMWARE_DIR"
cp -f "$BIN" "$WINDOWS_BIN"

test -f "$WINDOWS_BIN" || {
    echo "ERROR: firmware copy failed: $WINDOWS_BIN" >&2
    exit 1
}

echo
echo "========================================"
echo "BUILD SUCCESS"
echo "========================================"
echo
echo "ELF:"
echo "  $ELF"
echo
echo "BIN:"
echo "  $BIN"
echo
echo "Windows firmware:"
echo "  $WINDOWS_BIN"
echo
echo "Windows path:"
echo "  D:\\rmk-firmware\\grin_rc-rmk\\$BIN_NAME.bin"
echo
echo "BIN SHA256:"
sha256sum "$WINDOWS_BIN"
echo
echo "Flash/debug target:"
echo "  STM32F411CEUx"
echo "  probe-rs run --chip STM32F411CEUx $ELF"
