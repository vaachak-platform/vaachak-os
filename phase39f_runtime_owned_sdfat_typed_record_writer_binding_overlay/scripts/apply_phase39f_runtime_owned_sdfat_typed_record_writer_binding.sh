#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39f_runtime_owned_sdfat_typed_record_writer_binding_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_typed_record_sdfat_adapter;' "$RUNTIME_MOD"; then
  echo "Phase 39E export missing; apply Phase 39E first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_runtime_owned_sdfat_writer.rs state_io_runtime_owned_sdfat_writer_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_runtime_owned_sdfat_writer;" \
  "pub mod state_io_runtime_owned_sdfat_writer_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39f_runtime_owned_sdfat_typed_record_writer_binding.sh"

echo "phase39f=x4-runtime-owned-sdfat-typed-record-writer-binding-ok"
echo "phase39f-acceptance=x4-runtime-owned-sdfat-writer-acceptance-ok"
echo "Phase 39F overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
