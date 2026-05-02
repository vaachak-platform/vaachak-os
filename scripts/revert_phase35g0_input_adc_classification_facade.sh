#!/usr/bin/env bash
set -euo pipefail

cat <<'MSG'
Phase 35G-0 revert helper

This phase added a pure Vaachak input ADC classification facade and a silent
runtime preflight call. Review with:

  git diff -- target-xteink-x4/src/vaachak_x4/input/mod.rs target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs target-xteink-x4/src/vaachak_x4/input/input_adc_runtime.rs docs/phase35g0 scripts/check_phase35g0_input_adc_classification_facade.sh scripts/check_phase35g0_no_input_hardware_takeover.sh

No automatic destructive action is performed by this helper.
MSG
