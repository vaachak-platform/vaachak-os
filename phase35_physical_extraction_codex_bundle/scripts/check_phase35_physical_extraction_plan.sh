#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }
warn() { printf 'WARN %s\n' "$*"; warnings=$((warnings + 1)); }

exists() {
  if [[ -e "$1" ]]; then ok "exists: $1"; else fail "missing: $1"; fi
}

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35_plan_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35_plan_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35_plan_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35 physical extraction plan check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/vaachak_x4/mod.rs
exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

exists docs/phase35/PHASE35_PHYSICAL_EXTRACTION_PLAN.md
exists docs/phase35/PHASE35_STORAGE_STATE_IO_SEAM.md
exists docs/phase35/PHASE35_ACCEPTANCE.md
exists docs/phase35/PHASE35_RISK_REGISTER.md
exists docs/phase35/PHASE35_NEXT_PHASES.md

exists scripts/check_phase35_physical_extraction_plan.sh
exists scripts/check_phase35_storage_state_io_seam.sh
exists scripts/check_phase35_no_hardware_regression.sh
exists scripts/check_imported_reader_runtime_sync_phase35.sh

contains "normal boot marker remains Vaachak runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "plan lists storage state IO" 'Storage state IO|storage state IO' docs/phase35
contains "plan lists input semantic mapping" 'Input semantic mapping|input semantic mapping' docs/phase35
contains "plan lists display geometry helper usage" 'Display geometry helper usage|display geometry helper usage' docs/phase35
contains "plan lists input ADC/debounce" 'Input ADC/debounce|ADC/debounce' docs/phase35
contains "plan lists SD/SPI arbitration" 'SD/SPI arbitration|SPI arbitration' docs/phase35
contains "plan lists SSD1677 refresh/strip rendering" 'SSD1677 refresh|strip rendering' docs/phase35
contains "plan lists reader app internals" 'Reader app internals|reader app internals' docs/phase35

contains "Phase 35 limited to storage state IO seam" 'Storage State IO|storage state IO seam|storage state IO boundary' docs/phase35

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

echo
echo "Phase 35 physical extraction plan check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
