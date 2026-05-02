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
  if rg -n -e "$pattern" "$@" >/tmp/phase35d1_bridge_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35d1_bridge_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35d1_bridge_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35D-1 reader state runtime bridge check"
echo

exists target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs
exists docs/phase35d1/PHASE35D1_READER_STATE_RUNTIME_BRIDGE.md
exists docs/phase35d1/PHASE35D1_ACCEPTANCE.md
exists docs/phase35d1/PHASE35D1_NOTES.md

contains "io module exports reader state runtime bridge" \
  'pub mod reader_state_runtime' \
  target-xteink-x4/src/vaachak_x4/io/mod.rs

contains "reader state runtime bridge type exists" \
  'VaachakReaderStateRuntimeBridge|active_runtime_preflight|preflight_report' \
  target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs

contains "bridge exercises all reader state record formats" \
  'VaachakReadingProgressRecord|VaachakBookmarkRecord|VaachakBookmarkIndexRecord|VaachakReaderThemeRecord|VaachakBookMetaRecord' \
  target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs

contains "bridge validates typed state layout files" \
  'progress_file|bookmark_file|theme_file|meta_file|bookmarks_index_file|BMIDX\.TXT|PRG|BKM|THM|MTA' \
  target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs

contains "storage state runtime calls reader state runtime bridge from alloc preflight" \
  'VaachakReaderStateRuntimeBridge|active_runtime_alloc_preflight' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

not_contains "reader state runtime bridge does not own physical SD/SPI/FAT IO" \
  'SdCard::new|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice|AsyncVolumeManager|VolumeManager|FileMode' \
  target-xteink-x4/src/vaachak_x4/io/reader_state_runtime.rs

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35D-1 reader state runtime bridge check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
