#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39c_progress_write_lane_integration_bundle_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_progress_write_backend_binding;' "$RUNTIME_MOD"; then
  echo "Phase 39A export missing; apply Phase 39A first." >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_progress_write_callback_backend;' "$RUNTIME_MOD"; then
  echo "Phase 39B export missing; apply Phase 39B first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_progress_write_lane.rs state_io_progress_write_lane_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_progress_write_lane;" \
  "pub mod state_io_progress_write_lane_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39c_progress_write_lane_integration_bundle.sh"

echo "phase39c=x4-progress-write-lane-integration-bundle-ok"
echo "phase39c-acceptance=x4-progress-write-lane-acceptance-ok"
echo "Phase 39C overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
