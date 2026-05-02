#!/usr/bin/env bash
set -euo pipefail

cat <<'MSG'
Phase 35D-1 revert helper

This phase added a format-only reader-state runtime bridge under:

  target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs

and connected it through the existing storage-state runtime preflight.

Use git to review and selectively revert:

  git diff -- target-xteink-x4/src/vaachak_x4/io/mod.rs target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs docs/phase35d1 scripts/check_phase35d1_reader_state_runtime_bridge.sh scripts/check_phase35d1_no_active_persistence_takeover.sh

No automatic destructive action is performed by this helper.
MSG
