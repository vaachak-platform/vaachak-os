#!/usr/bin/env bash
set -euo pipefail

cat <<'MSG'
Phase 35E-0 revert helper

This phase added a pure Vaachak input semantic runtime facade and a silent
runtime preflight call. Review with:

  git diff -- target-xteink-x4/src/vaachak_x4/mod.rs target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs target-xteink-x4/src/vaachak_x4/input docs/phase35e0 scripts/check_phase35e0_input_semantics_runtime_facade.sh scripts/check_phase35e0_no_input_hardware_takeover.sh

No automatic destructive action is performed by this helper.
MSG
