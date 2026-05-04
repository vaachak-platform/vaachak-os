#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase38m_state_io_guarded_write_backend_dry_run_executor_overlay"
SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_dry_run_executor.rs"
DST="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_dry_run_executor.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$SRC" ]; then
  echo "missing overlay source: $SRC" >&2
  exit 1
fi

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_guarded_write_backend_implementation_seam;' "$RUNTIME_MOD"; then
  echo "Phase 38L export missing; apply Phase 38L first." >&2
  exit 1
fi

mkdir -p "$(dirname "$DST")"
cp -v "$SRC" "$DST"

EXPORT="pub mod state_io_guarded_write_backend_dry_run_executor;"
if ! grep -Fxq "$EXPORT" "$RUNTIME_MOD"; then
  printf '\n%s\n' "$EXPORT" >> "$RUNTIME_MOD"
  echo "added state_io_guarded_write_backend_dry_run_executor export to $RUNTIME_MOD"
else
  echo "state_io_guarded_write_backend_dry_run_executor export already present in $RUNTIME_MOD"
fi

"$OVERLAY/scripts/check_phase38m_state_io_guarded_write_backend_dry_run_executor.sh"

echo "phase38m=x4-state-io-guarded-write-backend-dry-run-executor-ok"
echo "Phase 38M overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
