#!/usr/bin/env bash
set -euo pipefail

cat <<'MSG'
Phase 35D-0 revert helper

This phase extended target-xteink-x4/src/vaachak_x4/apps/reader_state.rs and added
docs/scripts only. Use git to review and selectively revert those edits:

  git diff -- target-xteink-x4/src/vaachak_x4/apps/reader_state.rs docs/phase35d0 scripts/check_phase35d0_progress_bookmark_facade.sh scripts/check_phase35d0_no_active_progress_bookmark_takeover.sh

No automatic destructive action is performed by this helper.
MSG
