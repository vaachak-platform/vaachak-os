#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase38s_state_io_write_lane_handoff_consolidation_overlay"
SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_handoff_consolidation.rs"
DST="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_handoff_consolidation.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$SRC" ]; then
  echo "missing overlay source: $SRC" >&2
  exit 1
fi

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

for required in \
  'pub mod state_io_write_design_consolidation;' \
  'pub mod state_io_guarded_write_backend_dry_run_executor;' \
  'pub mod state_io_guarded_write_dry_run_acceptance;' \
  'pub mod state_io_guarded_write_backend_adapter_acceptance;' \
  'pub mod state_io_guarded_persistent_backend_stub;' \
  'pub mod state_io_guarded_read_before_write_stub;'
do
  if ! grep -q "$required" "$RUNTIME_MOD"; then
    echo "Missing prerequisite export in runtime.rs: $required" >&2
    exit 1
  fi
done

mkdir -p "$(dirname "$DST")"
cp -v "$SRC" "$DST"

EXPORT="pub mod state_io_write_lane_handoff_consolidation;"
if ! grep -Fxq "$EXPORT" "$RUNTIME_MOD"; then
  printf '\n%s\n' "$EXPORT" >> "$RUNTIME_MOD"
  echo "added state_io_write_lane_handoff_consolidation export to $RUNTIME_MOD"
else
  echo "state_io_write_lane_handoff_consolidation export already present in $RUNTIME_MOD"
fi

"$OVERLAY/scripts/check_phase38s_state_io_write_lane_handoff_consolidation.sh"

echo "phase38s=x4-state-io-write-lane-handoff-consolidation-ok"
echo "Phase 38S overlay applied. This is the final Phase 38."
echo "Next phase should be:"
echo "  Phase 39A — Guarded Progress State Write Backend Binding"
echo "Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
