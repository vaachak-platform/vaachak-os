#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase38o_state_io_guarded_write_backend_adapter_shape_overlay"
SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_adapter_shape.rs"
DST="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_adapter_shape.rs"
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

if ! grep -q 'pub mod state_io_guarded_write_dry_run_acceptance;' "$RUNTIME_MOD"; then
  echo "Phase 38N export missing; apply Phase 38N first." >&2
  exit 1
fi

mkdir -p "$(dirname "$DST")"
cp -v "$SRC" "$DST"

EXPORT="pub mod state_io_guarded_write_backend_adapter_shape;"
if ! grep -Fxq "$EXPORT" "$RUNTIME_MOD"; then
  printf '\n%s\n' "$EXPORT" >> "$RUNTIME_MOD"
  echo "added state_io_guarded_write_backend_adapter_shape export to $RUNTIME_MOD"
else
  echo "state_io_guarded_write_backend_adapter_shape export already present in $RUNTIME_MOD"
fi

"$OVERLAY/scripts/check_phase38o_state_io_guarded_write_backend_adapter_shape.sh"

echo "phase38o=x4-state-io-guarded-write-backend-adapter-shape-ok"
echo "Phase 38O overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
