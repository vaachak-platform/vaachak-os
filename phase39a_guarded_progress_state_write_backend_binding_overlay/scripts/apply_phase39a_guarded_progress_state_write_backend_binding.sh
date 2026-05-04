#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39a_guarded_progress_state_write_backend_binding_overlay"
SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_progress_write_backend_binding.rs"
DST="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_progress_write_backend_binding.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$SRC" ]; then
  echo "missing overlay source: $SRC" >&2
  exit 1
fi

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_write_lane_handoff_consolidation;' "$RUNTIME_MOD"; then
  echo "Phase 38S handoff export missing; apply/repair Phase 38S first." >&2
  exit 1
fi

mkdir -p "$(dirname "$DST")"
cp -v "$SRC" "$DST"

EXPORT="pub mod state_io_progress_write_backend_binding;"
if ! grep -Fxq "$EXPORT" "$RUNTIME_MOD"; then
  printf '\n%s\n' "$EXPORT" >> "$RUNTIME_MOD"
  echo "added state_io_progress_write_backend_binding export to $RUNTIME_MOD"
else
  echo "state_io_progress_write_backend_binding export already present in $RUNTIME_MOD"
fi

"$OVERLAY/scripts/check_phase39a_guarded_progress_state_write_backend_binding.sh"

echo "phase39a=x4-guarded-progress-state-write-backend-binding-ok"
echo "Phase 39A overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
