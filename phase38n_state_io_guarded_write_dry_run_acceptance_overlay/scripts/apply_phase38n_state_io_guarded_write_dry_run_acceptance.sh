#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase38n_state_io_guarded_write_dry_run_acceptance_overlay"
SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_dry_run_acceptance.rs"
DST="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_dry_run_acceptance.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$SRC" ]; then
  echo "missing overlay source: $SRC" >&2
  exit 1
fi

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_guarded_write_backend_dry_run_executor;' "$RUNTIME_MOD"; then
  echo "Phase 38M export missing; apply Phase 38M first." >&2
  exit 1
fi

mkdir -p "$(dirname "$DST")"
cp -v "$SRC" "$DST"

EXPORT="pub mod state_io_guarded_write_dry_run_acceptance;"
if ! grep -Fxq "$EXPORT" "$RUNTIME_MOD"; then
  printf '\n%s\n' "$EXPORT" >> "$RUNTIME_MOD"
  echo "added state_io_guarded_write_dry_run_acceptance export to $RUNTIME_MOD"
else
  echo "state_io_guarded_write_dry_run_acceptance export already present in $RUNTIME_MOD"
fi

"$OVERLAY/scripts/check_phase38n_state_io_guarded_write_dry_run_acceptance.sh"

echo "phase38n=x4-state-io-guarded-write-dry-run-acceptance-ok"
echo "Phase 38N overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
