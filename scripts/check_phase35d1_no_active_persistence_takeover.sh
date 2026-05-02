#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35d1_no_takeover_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35d1_no_takeover_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35d1_no_takeover_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35D-1 no active persistence takeover check"
echo

contains "active wrapper still calls existing storage state runtime preflight" \
  'storage_state_runtime::VaachakStorageStateRuntimeBridge::active_runtime_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "active wrapper calls alloc preflight only after heap allocator setup" \
  'heap_allocator!\(#\[ram\(reclaimed\)\] size: 64_000\)|active_runtime_alloc_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "active wrapper still constructs imported Pulp AppManager directly" \
  'let mut app_mgr = AppManager::new' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "kernel run still receives direct app manager" \
  'kernel\.run\(&mut app_mgr\)\.await' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "button/input runtime path remains present" \
  'InputDriver::new|ButtonMapper::new|tasks::input_task\(input\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "active wrapper does not directly call reader state facade records" \
  'VaachakReadingProgressRecord|VaachakBookmarkRecord|VaachakBookmarkIndexRecord|VaachakReaderThemeRecord|VaachakBookMetaRecord|apps::reader_state' \
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
echo "Phase 35D-1 no active persistence takeover check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
