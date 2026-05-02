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
  if rg -n -e "$pattern" "$@" >/tmp/phase35e0_input_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35e0_input_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35e0_input_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35E-0 input semantics runtime facade check"
echo

facade="target-xteink-x4/src/vaachak_x4/input/input_semantics_runtime.rs"

exists target-xteink-x4/src/vaachak_x4/input/mod.rs
exists "$facade"
exists docs/phase35e0/PHASE35E0_INPUT_SEMANTICS_RUNTIME_FACADE.md
exists docs/phase35e0/PHASE35E0_ACCEPTANCE.md
exists docs/phase35e0/PHASE35E0_NOTES.md

contains "Vaachak input module is exported" \
  'pub mod input' \
  target-xteink-x4/src/vaachak_x4/mod.rs

contains "input semantics runtime bridge exists" \
  'VaachakInputSemanticsRuntimeBridge|active_runtime_preflight|preflight_report' \
  "$facade"

contains "facade defines physical buttons" \
  'VaachakPhysicalButton|Right|Left|Confirm|Back|VolUp|VolDown|Power' \
  "$facade"

contains "facade defines runtime actions" \
  'VaachakRuntimeInputAction|Next|Previous|NextJump|PreviousJump|Select|Menu' \
  "$facade"

contains "facade validates default and swapped mappings" \
  'default_layout_ok|swapped_layout_ok|VaachakRuntimeButtonMapper::swapped' \
  "$facade"

contains "active runtime calls Vaachak input semantics runtime preflight" \
  'input_semantics_runtime::VaachakInputSemanticsRuntimeBridge::active_runtime_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "input semantics runtime facade does not own ADC/debounce/repeat hardware" \
  'Adc::new|read_oneshot|decode_ladder|ROW1_THRESHOLDS|ROW2_THRESHOLDS|DEBOUNCE_MS|LONG_PRESS_MS|REPEAT_MS|InputDriver|EventQueue|power_button_is_low' \
  "$facade"

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35E-0 input semantics runtime facade check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
