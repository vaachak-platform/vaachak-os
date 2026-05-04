#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39g_runtime_file_api_integration_gate_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_runtime_owned_sdfat_writer;' "$RUNTIME_MOD"; then
  echo "Phase 39F export missing; apply Phase 39F first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_runtime_file_api_integration_gate.rs state_io_runtime_file_api_integration_gate_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_runtime_file_api_integration_gate;" \
  "pub mod state_io_runtime_file_api_integration_gate_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39g_runtime_file_api_integration_gate.sh"

echo "phase39g=x4-runtime-file-api-integration-gate-ok"
echo "phase39g-acceptance=x4-runtime-file-api-integration-gate-acceptance-ok"
echo "Phase 39G overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "Optional locator:"
echo "  ./phase39g_runtime_file_api_integration_gate_overlay/scripts/find_phase39g_runtime_file_api_candidates.sh"
