#!/usr/bin/env bash
set -euo pipefail

echo "Phase 30 revert helper"
echo

backup_dir=".phase_backups/phase30/revert-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$backup_dir"

if [[ -f target-xteink-x4/src/main.rs ]]; then
  cp target-xteink-x4/src/main.rs "$backup_dir/main.rs.before-revert"
fi

if [[ -d target-xteink-x4/src/runtime ]]; then
  cp -a target-xteink-x4/src/runtime "$backup_dir/runtime.before-revert"
fi

if [[ -d target-xteink-x4/src/vaachak_x4 ]]; then
  cp -a target-xteink-x4/src/vaachak_x4 "$backup_dir/vaachak_x4.before-revert"
fi

echo "Backed up current target runtime files to $backup_dir"

if [[ -f target-xteink-x4/src/main.rs.bak-phase30 ]]; then
  cp target-xteink-x4/src/main.rs.bak-phase30 target-xteink-x4/src/main.rs
  echo "Restored target-xteink-x4/src/main.rs from .bak-phase30"
else
  echo "WARN missing target-xteink-x4/src/main.rs.bak-phase30"
fi

if [[ -d target-xteink-x4/src/runtime.bak-phase30 ]]; then
  rm -rf target-xteink-x4/src/runtime
  cp -a target-xteink-x4/src/runtime.bak-phase30 target-xteink-x4/src/runtime
  echo "Restored target-xteink-x4/src/runtime from runtime.bak-phase30"
else
  echo "WARN missing target-xteink-x4/src/runtime.bak-phase30"
fi

if [[ -d target-xteink-x4/src/vaachak_x4 ]]; then
  rm -rf target-xteink-x4/src/vaachak_x4
  echo "Removed target-xteink-x4/src/vaachak_x4"
fi

echo
echo "Revert helper complete. Run:"
echo "  cargo fmt --all"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
