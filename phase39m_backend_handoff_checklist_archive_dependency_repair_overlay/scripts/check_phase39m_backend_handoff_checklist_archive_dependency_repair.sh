#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/runtime/state_io_backend_handoff_checklist.rs"
ARCHIVE="docs/archive/phase38-39-scaffolding/runtime/state_io_shadow_write_acceptance.rs"

test -f "$FILE"
test -f "$ARCHIVE"

if rg -n 'state_io_shadow_write_acceptance' "$FILE"; then
  echo "backend handoff checklist still references archived state_io_shadow_write_acceptance module" >&2
  exit 1
fi

rg -n 'Phase39mArchivedShadowWriteAcceptanceReport' "$FILE" >/dev/null
rg -n 'STATE_IO_SHADOW_WRITE_ACCEPTANCE_REPORT' "$FILE" >/dev/null
rg -n 'phase36o=x4-state-io-shadow-write-acceptance-ok' "$FILE" >/dev/null

# Accepted active path must still be intact.
test -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
rg -n 'typed_state_wiring::write_app_subdir' vendor/pulp-os/src/apps/reader/mod.rs >/dev/null
rg -n 'typed_state_wiring::ensure_state_dir' vendor/pulp-os/src/apps/reader/mod.rs >/dev/null

echo "phase39m-repair-check=ok"
