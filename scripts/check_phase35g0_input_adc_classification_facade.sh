#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

exists() {
  if [[ -e "$1" ]]; then ok "exists: $1"; else fail "missing: $1"; fi
}

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35g0_adc_rg.txt 2>/dev/null; then
    ok "$desc"
  else
    fail "$desc"
    printf '      pattern: %s\n' "$pattern"
    printf '      path:    %s\n' "$*"
  fi
}

not_contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35g0_adc_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35g0_adc_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35G-0 input ADC classification facade check"
echo

facade="target-xteink-x4/src/vaachak_x4/input/input_adc_runtime.rs"

exists "$facade"
exists docs/phase35g0/PHASE35G0_INPUT_ADC_CLASSIFICATION_FACADE.md
exists docs/phase35g0/PHASE35G0_ACCEPTANCE.md
exists docs/phase35g0/PHASE35G0_NOTES.md

contains "Vaachak input module exports ADC facade" \
  'pub mod input_adc_runtime' \
  target-xteink-x4/src/vaachak_x4/input/mod.rs

contains "ADC classification runtime bridge exists" \
  'VaachakInputAdcRuntimeBridge|active_runtime_preflight|preflight_report' \
  "$facade"

contains "facade defines GPIO row contracts" \
  'ROW1_GPIO: u8 = 1|ROW2_GPIO: u8 = 2|POWER_GPIO: u8 = 3' \
  "$facade"

contains "facade defines known ladder centers" \
  'center_mv: 3|center_mv: 1113|center_mv: 1984|center_mv: 2556|center_mv: 1659' \
  "$facade"

contains "facade records timing policy" \
  'oversample_count: 4|debounce_window_ms: 15|long_press_window_ms: 1000|repeat_interval_ms: 150' \
  "$facade"

contains "active runtime calls Vaachak ADC classification preflight" \
  'input_adc_runtime::VaachakInputAdcRuntimeBridge::active_runtime_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "ADC facade does not own physical ADC reads or debounce loop" \
  'Adc::new|read_oneshot|nb::block|Instant::now|Duration::from_millis|EventQueue|InputDriver|power_button_is_low' \
  "$facade"

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35G-0 input ADC classification facade check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
