#!/usr/bin/env bash
set -euo pipefail
REPO="$(pwd)"
TARGET_FILE="$REPO/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_binding.rs"
RUNTIME_MOD="$REPO/target-xteink-x4/src/vaachak_x4/runtime.rs"
DIR_CACHE="$REPO/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"

[ -f "$TARGET_FILE" ] || { echo "missing $TARGET_FILE" >&2; exit 1; }
grep -q 'PHASE_38K_GUARDED_WRITE_BACKEND_BINDING_MARKER' "$TARGET_FILE"
grep -q '^pub mod state_io_guarded_write_backend_binding;' "$RUNTIME_MOD"

if grep -nE 'filesystem|SD|FAT|SPI|display|input|power' "$TARGET_FILE"; then
  echo "forbidden live behavior term found in Phase 38K scaffold" >&2
  exit 1
fi

if [ -f "$DIR_CACHE" ]; then
  if grep -n 'if phase38i_is_epub_or_epu_name(name)' "$DIR_CACHE"; then
    echo "recursive phase38i helper still present in dir_cache.rs" >&2
    exit 1
  fi
fi

echo "phase38k-check=ok"
