#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39k_write_lane_cleanup_acceptance_freeze_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_runtime_state_write_verification;' "$RUNTIME_MOD"; then
  echo "Phase 39J export missing; apply Phase 39J first." >&2
  exit 1
fi

if [ ! -f "$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs" ]; then
  echo "Phase 39I active typed_state_wiring helper missing." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_write_lane_cleanup_acceptance_freeze.rs state_io_write_lane_cleanup_acceptance_freeze_report.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_write_lane_cleanup_acceptance_freeze;" \
  "pub mod state_io_write_lane_cleanup_acceptance_freeze_report;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39k_write_lane_cleanup_acceptance_freeze.sh"

echo "phase39k=x4-write-lane-cleanup-acceptance-freeze-ok"
echo "phase39k-acceptance=x4-write-lane-cleanup-freeze-report-ok"
echo "Phase 39K overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "Freeze acceptance:"
echo "  ./phase39k_write_lane_cleanup_acceptance_freeze_overlay/scripts/accept_phase39k_write_lane_freeze.sh"
echo ""
echo "Scaffolding inventory:"
echo "  ./phase39k_write_lane_cleanup_acceptance_freeze_overlay/scripts/inventory_phase39k_write_lane_scaffolding.sh"
