#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase38q_state_io_guarded_persistent_backend_stub_overlay"
SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_persistent_backend_stub.rs"
DST="$ROOT/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_persistent_backend_stub.rs"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$SRC" ]; then
  echo "missing overlay source: $SRC" >&2
  exit 1
fi

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_guarded_write_backend_adapter_acceptance;' "$RUNTIME_MOD"; then
  echo "Phase 38P export missing; apply Phase 38P first." >&2
  exit 1
fi

mkdir -p "$(dirname "$DST")"
cp -v "$SRC" "$DST"

EXPORT="pub mod state_io_guarded_persistent_backend_stub;"
if ! grep -Fxq "$EXPORT" "$RUNTIME_MOD"; then
  printf '\n%s\n' "$EXPORT" >> "$RUNTIME_MOD"
  echo "added state_io_guarded_persistent_backend_stub export to $RUNTIME_MOD"
else
  echo "state_io_guarded_persistent_backend_stub export already present in $RUNTIME_MOD"
fi

"$OVERLAY/scripts/check_phase38q_state_io_guarded_persistent_backend_stub.sh"

echo "phase38q=x4-state-io-guarded-persistent-backend-stub-ok"
echo "Phase 38Q overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
