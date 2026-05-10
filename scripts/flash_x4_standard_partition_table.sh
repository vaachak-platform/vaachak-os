#!/usr/bin/env bash
set -euo pipefail

PORT="${1:-${ESPFLASH_PORT:-}}"
if [ -z "$PORT" ]; then
  cat >&2 <<'EOF'
usage: scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX

This performs the one-time migration from the old Vaachak single-factory
partition table to the standard Xteink X4 OTA-compatible table.
EOF
  exit 2
fi

./scripts/validate_x4_standard_partition_table_compatibility.sh
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

cat <<EOF

About to erase flash on $PORT and flash Vaachak OS using the standard X4
partition table from espflash.toml:

  partitions/xteink_x4_standard.bin

This erase is expected only for migration from the old Vaachak factory-only
layout. Keep the SD card contents backed up separately.
EOF

read -r -p "Type ERASE-AND-FLASH to continue: " answer
if [ "$answer" != "ERASE-AND-FLASH" ]; then
  echo "aborted"
  exit 1
fi

espflash erase-flash --chip esp32c3 --port "$PORT"
espflash flash \
  --monitor \
  --chip esp32c3 \
  --port "$PORT" \
  target/riscv32imc-unknown-none-elf/release/target-xteink-x4
