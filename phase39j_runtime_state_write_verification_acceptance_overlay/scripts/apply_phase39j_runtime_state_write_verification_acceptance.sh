#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39j_runtime_state_write_verification_acceptance_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_active_reader_save_callsite_wiring;' "$RUNTIME_MOD"; then
  echo "Phase 39I export missing; apply Phase 39I first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_runtime_state_write_verification.rs state_io_runtime_state_write_verification_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_runtime_state_write_verification;" \
  "pub mod state_io_runtime_state_write_verification_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39j_runtime_state_write_verification_acceptance.sh"

echo "phase39j=x4-runtime-state-write-verification-acceptance-ok"
echo "phase39j-acceptance=x4-runtime-state-write-verification-acceptance-report-ok"
echo "Phase 39J overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "SD verification:"
echo "  SD=/media/mindseye73/C0D2-109E ./phase39j_runtime_state_write_verification_acceptance_overlay/scripts/inspect_phase39j_sd_state.sh"
echo "  SD=/media/mindseye73/C0D2-109E RESTORE_VERIFIED=1 ./phase39j_runtime_state_write_verification_acceptance_overlay/scripts/accept_phase39j_sd_persistence.sh"
