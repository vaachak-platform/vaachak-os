#!/usr/bin/env bash
set -euo pipefail

failures=0
ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

not_contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase36a_hw_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase36a_hw_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 36A no input hardware regression check"
echo

not_contains "Phase 36A active mapper does not own ADC/debounce/input polling" \
  'Adc::new|read_oneshot|decode_ladder|ROW1_THRESHOLDS|ROW2_THRESHOLDS|DEBOUNCE_MS|REPEAT|Instant::now|input_task|InputDriver::new|read_raw|poll\(' \
  target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs

not_contains "Phase 36A active mapper does not own SD/SPI/display behavior" \
  'SdCard::new|open_raw_volume|spi::master|RefCellDevice|epd\.init|speed_up_spi|refresh\(|write_cmd|write_data|draw_' \
  target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs

not_contains "Phase 36A imported runtime does not add physical input hardware setup" \
  'Vaachak.*Adc|Vaachak.*Debounce|Vaachak.*InputDriver|decode_ladder|ROW1_THRESHOLDS|ROW2_THRESHOLDS' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

echo
echo "Phase 36A no input hardware regression check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
