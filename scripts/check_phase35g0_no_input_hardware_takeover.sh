#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35g0_hardware_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35g0_hardware_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35g0_hardware_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35G-0 no input hardware takeover check"
echo

runtime="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"
facade="target-xteink-x4/src/vaachak_x4/input/input_adc_runtime.rs"

contains "active runtime still constructs Pulp InputDriver" \
  'InputDriver::new\(board\.input\)' \
  "$runtime"

contains "active runtime still constructs Pulp ButtonMapper" \
  'ButtonMapper::new\(\)' \
  "$runtime"

contains "active runtime still spawns Pulp input task" \
  'tasks::input_task\(input\)' \
  "$runtime"

contains "normal boot marker remains runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

not_contains "Vaachak ADC facade does not own hardware sampling" \
  'Adc::new|read_oneshot|nb::block|Instant::now|Duration::from_millis|EventQueue|InputDriver|power_button_is_low' \
  "$facade"

not_contains "Phase 35C-1 wrapper artifacts remain absent" \
  'VaachakReaderThemeMetadataAppLayer|ACTIVE_THEME_METADATA_IO_MOVED_IN_PHASE35C1|reader_theme_metadata' \
  target-xteink-x4/src docs

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

echo
echo "Phase 35G-0 no input hardware takeover check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
