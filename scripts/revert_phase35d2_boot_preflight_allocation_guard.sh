#!/usr/bin/env bash
set -euo pipefail

cat <<'MSG'
Phase 35D-2 revert helper

This phase added boot-order guard docs/scripts and policy constants for the
pre-heap and post-heap runtime preflight split.

Review with:

  git diff -- target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs docs/phase35d2 scripts/check_phase35d2_boot_preflight_allocation_guard.sh scripts/check_phase35d2_no_behavior_takeover.sh

No automatic destructive action is performed by this helper.
MSG
