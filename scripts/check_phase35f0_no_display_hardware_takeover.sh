#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35f0_hardware_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35f0_hardware_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35f0_hardware_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35F-0 no display hardware takeover check"
echo

runtime="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"
facade="target-xteink-x4/src/vaachak_x4/display/display_geometry_runtime.rs"

contains "active runtime still initializes imported Pulp display" \
  'let mut epd = board\.display\.epd|epd\.init\(&mut delay\)' \
  "$runtime"

contains "active runtime still gives display and strip to imported Kernel" \
  'Kernel::new|STRIP\.take\(\)' \
  "$runtime"

contains "normal boot marker remains runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

not_contains "Vaachak display facade does not own physical display operations" \
  'DisplayDriver|epd\.init|write_cmd|write_data|send_data|busy|full_refresh|partial_refresh|write_full_frame|draw\(|StripBuffer|RefCellDevice|spi::master' \
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
echo "Phase 35F-0 no display hardware takeover check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
