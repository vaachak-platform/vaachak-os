#!/usr/bin/env bash
set -euo pipefail

cat <<'MSG'
Phase 35F-0 revert helper

This phase added a pure Vaachak display geometry runtime facade and a silent
runtime preflight call. Review with:

  git diff -- target-xteink-x4/src/vaachak_x4/mod.rs target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs target-xteink-x4/src/vaachak_x4/display docs/phase35f0 scripts/check_phase35f0_display_geometry_runtime_facade.sh scripts/check_phase35f0_no_display_hardware_takeover.sh

No automatic destructive action is performed by this helper.
MSG
