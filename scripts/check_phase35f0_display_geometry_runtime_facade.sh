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
  if rg -n -e "$pattern" "$@" >/tmp/phase35f0_display_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35f0_display_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35f0_display_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35F-0 display geometry runtime facade check"
echo

facade="target-xteink-x4/src/vaachak_x4/display/display_geometry_runtime.rs"

exists target-xteink-x4/src/vaachak_x4/display/mod.rs
exists "$facade"
exists docs/phase35f0/PHASE35F0_DISPLAY_GEOMETRY_RUNTIME_FACADE.md
exists docs/phase35f0/PHASE35F0_ACCEPTANCE.md
exists docs/phase35f0/PHASE35F0_NOTES.md

contains "Vaachak display module is exported" \
  'pub mod display' \
  target-xteink-x4/src/vaachak_x4/mod.rs

contains "display geometry runtime bridge exists" \
  'VaachakDisplayGeometryRuntimeBridge|active_runtime_preflight|preflight_report' \
  "$facade"

contains "facade validates native and logical geometry" \
  'NATIVE_BOUNDS|LOGICAL_BOUNDS|800|480|logical_to_native_rect' \
  "$facade"

contains "facade validates strip mapping" \
  'native_strip_rect|native_strip_count|STRIP_ROWS' \
  "$facade"

contains "facade validates reader bounds" \
  'reader_text_bounds|reader_page_bounds|READER_TEXT_H|READER_TEXT_W' \
  "$facade"

contains "active runtime calls Vaachak display geometry runtime preflight" \
  'display_geometry_runtime::VaachakDisplayGeometryRuntimeBridge::active_runtime_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "display geometry runtime facade does not own SSD1677/display IO" \
  'DisplayDriver|epd\.init|write_cmd|write_data|send_data|busy|full_refresh|partial_refresh|write_full_frame|draw\(|StripBuffer|RefCellDevice|spi::master' \
  "$facade"

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35F-0 display geometry runtime facade check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
