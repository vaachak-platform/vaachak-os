#!/usr/bin/env bash
set -euo pipefail
python3 scripts/validate_lua_app_cleanup_consolidation.py
cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
printf '%s\n' 'lua app cleanup consolidation validation passed'
