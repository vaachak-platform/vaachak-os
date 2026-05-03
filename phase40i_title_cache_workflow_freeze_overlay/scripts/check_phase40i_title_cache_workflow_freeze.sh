#!/usr/bin/env bash
set -euo pipefail

grep -q 'PHASE_40I_TITLE_CACHE_WORKFLOW_FREEZE_MARKER' \
  target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze.rs
grep -q 'PHASE_40I_TXT_BODY_TITLE_SCANNING_DISABLED: bool = true' \
  target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze.rs
grep -q 'PHASE_40I_TXT_TITLES_FROM_TITLES_BIN: bool = true' \
  target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze.rs
grep -q 'phase40i_acceptance_status' \
  target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze_acceptance.rs
grep -q 'pub mod state_io_title_cache_workflow_freeze;' \
  target-xteink-x4/src/vaachak_x4/runtime.rs

echo "phase40i-check=ok"
