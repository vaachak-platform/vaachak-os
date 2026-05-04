#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39h_typed_state_runtime_callsite_wiring_bundle_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_runtime_file_api_integration_gate;' "$RUNTIME_MOD"; then
  echo "Phase 39G export missing; apply Phase 39G first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_typed_state_runtime_callsite_wiring.rs state_io_typed_state_runtime_callsite_wiring_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_typed_state_runtime_callsite_wiring;" \
  "pub mod state_io_typed_state_runtime_callsite_wiring_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39h_typed_state_runtime_callsite_wiring_bundle.sh"

echo "phase39h=x4-typed-state-runtime-callsite-wiring-bundle-ok"
echo "phase39h-acceptance=x4-typed-state-runtime-callsite-wiring-acceptance-ok"
echo "Phase 39H overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "Optional callsite locator:"
echo "  ./phase39h_typed_state_runtime_callsite_wiring_bundle_overlay/scripts/find_phase39h_reader_save_callsites.sh"
