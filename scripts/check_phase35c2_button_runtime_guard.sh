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
  if rg -n -e "$pattern" "$@" >/tmp/phase35c2_guard_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35c2_guard_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35c2_guard_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35C-2 button runtime guard check"
echo

exists docs/phase35c2/PHASE35C2_BUTTON_RUNTIME_GUARD.md
exists docs/phase35c2/PHASE35C2_ACCEPTANCE.md
exists docs/phase35c2/PHASE35C2_NOTES.md
exists scripts/check_phase35c2_button_runtime_guard.sh
exists scripts/check_phase35c2_direct_app_manager_runtime.sh

contains "active runtime constructs Pulp InputDriver" \
  'InputDriver::new\(board\.input\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "active runtime spawns Pulp input task" \
  'tasks::input_task\(input\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "active runtime uses ButtonMapper in AppManager construction" \
  'ButtonMapper::new\(\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "active runtime uses direct mutable app manager" \
  'let mut app_mgr = AppManager::new' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "active source has no Vaachak AppLayer wrapper" \
  'VaachakReaderThemeMetadataAppLayer|reader_theme_metadata|impl AppLayer for Vaachak' \
  target-xteink-x4/src

contains "normal boot marker remains runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

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
echo "Phase 35C-2 button runtime guard check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
