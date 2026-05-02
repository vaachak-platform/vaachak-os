#!/usr/bin/env bash
set -euo pipefail

BIN="${BIN:-target/riscv32imc-unknown-none-elf/release/target-xteink-x4}"
PORT="${PORT:-}"

cat <<EOF
# Phase 40A Flash Commands

# 1. Load ESP environment:
. "\$HOME/export-esp.sh"

# 2. Build release:
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

# 3A. Preferred if your Cargo runner is configured:
cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

# 3B. Direct espflash alternative:
espflash flash --chip esp32c3 --monitor ${PORT:+--port "$PORT" }"$BIN"

# Optional with explicit port:
PORT=/dev/ttyACM0 ./phase40a_device_regression_write_lane_closeout_overlay/scripts/print_phase40a_flash_commands.sh
EOF
