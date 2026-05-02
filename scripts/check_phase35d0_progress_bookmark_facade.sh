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
  if rg -n -e "$pattern" "$@" >/tmp/phase35d0_facade_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35d0_facade_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35d0_facade_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35D-0 progress/bookmark facade check"
echo

exists target-xteink-x4/src/vaachak_x4/apps/reader_state.rs
exists docs/phase35d0/PHASE35D0_PROGRESS_BOOKMARK_FACADE.md
exists docs/phase35d0/PHASE35D0_ACCEPTANCE.md
exists docs/phase35d0/PHASE35D0_NOTES.md

contains "progress record contract exists" \
  'VaachakReadingProgressRecord|progress_record_file_for|Progress|PRG' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "bookmark record contract exists" \
  'VaachakBookmarkRecord|bookmark_record_file_for|Bookmark|BKM' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "bookmark index contract exists" \
  'VaachakBookmarkIndexRecord|BOOKMARKS_INDEX_FILE|BMIDX\.TXT|BOOKMARK_JUMP_PREFIX' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "progress/bookmark payload helpers exist" \
  'encode_bookmarks|decode_bookmarks|encode_bookmarks_index|decode_bookmarks_index|decode_bookmark_jump' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "facade uses storage path helpers" \
  'VaachakStoragePathHelpers|VaachakStateFileKind' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

not_contains "reader state facade does not own physical SD/SPI/FAT IO" \
  'SdCard::new|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice|AsyncVolumeManager|VolumeManager|FileMode' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35D-0 progress/bookmark facade check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
