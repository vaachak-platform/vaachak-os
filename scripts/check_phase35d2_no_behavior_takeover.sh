#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35d2_behavior_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35d2_behavior_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35d2_behavior_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35D-2 no behavior takeover check"
echo

runtime="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"

contains "normal boot marker remains runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "active wrapper still constructs imported Pulp AppManager directly" \
  'let mut app_mgr = AppManager::new' \
  "$runtime"

contains "kernel boot and run still use direct app manager" \
  'kernel\.boot\(&mut app_mgr\)\.await|kernel\.run\(&mut app_mgr\)\.await' \
  "$runtime"

contains "button/input runtime path remains present" \
  'InputDriver::new|ButtonMapper::new|tasks::input_task\(input\)' \
  "$runtime"

not_contains "Phase 35C-1 wrapper artifacts remain absent" \
  'VaachakReaderThemeMetadataAppLayer|ACTIVE_THEME_METADATA_IO_MOVED_IN_PHASE35C1|reader_theme_metadata' \
  target-xteink-x4/src docs

not_contains "active imported runtime wrapper does not add physical storage IO" \
  'VaachakStorageStatePathIo|read_state_path|write_state_path|open_file_in_dir|open_dir|close_file|AsyncVolumeManager|VolumeManager|FileMode' \
  "$runtime"

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

echo
echo "Phase 35D-2 no behavior takeover check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
