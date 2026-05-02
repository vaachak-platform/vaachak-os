#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35c2_direct_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35c2_direct_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35c2_direct_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35C-2 direct AppManager runtime check"
echo

contains "kernel boot receives direct app manager variable" \
  'kernel\.boot\(&mut app_mgr\)\.await' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "kernel run receives direct app manager variable" \
  'kernel\.run\(&mut app_mgr\)\.await' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "Phase 35C-0 facade remains present" \
  'ACTIVE_READER_STATE_IO_MOVED_IN_PHASE35C0: bool = false|VaachakBookMetaRecord|VaachakReaderThemeRecord' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

not_contains "Phase 35C-0 facade is not active IO in runtime wrapper" \
  'apps::reader_state|VaachakBookMetaRecord|VaachakReaderThemeRecord|ACTIVE_READER_STATE_IO_MOVED_IN_PHASE35C0' \
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
echo "Phase 35C-2 direct AppManager runtime check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
