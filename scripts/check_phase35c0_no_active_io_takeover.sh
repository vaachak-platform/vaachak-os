#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35c0_no_active_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35c0_no_active_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35c0_no_active_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35C-0 no active IO takeover check"
echo

contains "normal boot marker remains runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "facade explicitly says active reader state IO has not moved" \
  'ACTIVE_READER_STATE_IO_MOVED_IN_PHASE35C0: bool = false' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

not_contains "imported runtime does not call Phase 35C-0 reader state facade as active IO" \
  'apps::reader_state|VaachakBookMetaRecord|VaachakReaderThemeRecord|ACTIVE_READER_STATE_IO_MOVED_IN_PHASE35C0' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "Phase 35C-0 does not claim physical runtime ownership marker" \
  'vaachak=x4-physical-runtime-owned' \
  target-xteink-x4/src docs/phase35c0

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

echo
echo "Phase 35C-0 no active IO takeover check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
