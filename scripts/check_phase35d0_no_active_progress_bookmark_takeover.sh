#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35d0_no_takeover_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35d0_no_takeover_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35d0_no_takeover_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35D-0 no active progress/bookmark takeover check"
echo

contains "runtime still constructs imported Pulp AppManager directly" \
  'let mut app_mgr = AppManager::new' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "kernel boot still receives direct app manager" \
  'kernel\.boot\(&mut app_mgr\)\.await' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "kernel run still receives direct app manager" \
  'kernel\.run\(&mut app_mgr\)\.await' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "button/input runtime path remains present" \
  'InputDriver::new|ButtonMapper::new|tasks::input_task\(input\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "normal boot marker remains current accepted marker" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs target-xteink-x4/src/vaachak_x4

not_contains "runtime wrapper does not call Vaachak progress/bookmark facade" \
  'VaachakReadingProgressRecord|VaachakBookmarkRecord|VaachakBookmarkIndexRecord|ACTIVE_PROGRESS_BOOKMARK_IO_MOVED_IN_PHASE35D0' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

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
echo "Phase 35D-0 no active progress/bookmark takeover check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
