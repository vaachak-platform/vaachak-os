#!/usr/bin/env bash
set -euo pipefail

echo "This revert helper is intentionally conservative."
echo "Use git restore or your pre-Phase-35 branch/tag to revert this large extraction."
echo

echo "Suggested commands if Phase 35 Full is uncommitted:"
echo "  git restore target-xteink-x4/src/main.rs target-xteink-x4/src/vaachak_x4"
echo "  git clean -fd target-xteink-x4/src/vaachak_x4/physical target-xteink-x4/src/vaachak_x4/apps docs/phase35_full scripts/check_phase35_full_*"
