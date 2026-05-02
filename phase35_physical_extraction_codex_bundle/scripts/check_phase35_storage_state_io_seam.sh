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
  if rg -n -e "$pattern" "$@" >/tmp/phase35_storage_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35_storage_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35_storage_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35 storage state IO seam check"
echo

exists target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
exists target-xteink-x4/src/vaachak_x4/contracts/storage_state_contract.rs

if [[ -e target-xteink-x4/src/vaachak_x4/io/storage_state.rs ]]; then
  ok "exists: target-xteink-x4/src/vaachak_x4/io/storage_state.rs"
  contains "storage state IO seam defines state kinds" \
    'Progress|Bookmark|Theme|Metadata|VaachakStateIoKind' \
    target-xteink-x4/src/vaachak_x4/io/storage_state.rs
  contains "storage state IO seam defines trait or interface" \
    'trait VaachakStorageStateIo|struct VaachakStorageState' \
    target-xteink-x4/src/vaachak_x4/io/storage_state.rs
  contains "storage state IO seam references path helpers" \
    'storage_path_helpers|VaachakStoragePathHelpers' \
    target-xteink-x4/src/vaachak_x4/io/storage_state.rs target-xteink-x4/src/vaachak_x4/io/storage_state_adapter.rs 2>/dev/null || true
  not_contains "storage state IO seam does not own physical SD/SPI/FAT IO" \
    'SdCard::new|AsyncVolumeManager|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice' \
    target-xteink-x4/src/vaachak_x4/io
else
  warn "storage state IO seam implementation file not present; Phase 35 may be docs/checks-only"
fi

contains "storage path helpers remain present" \
  'progress_path|bookmark_path|theme_path|metadata_path|bookmark_index|state_file' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

not_contains "Vaachak contracts do not own EPUB cache IO" \
  'epub_cache|EpubCache|chapter_cache|cache_chapter|smol_epub::cache' \
  target-xteink-x4/src/vaachak_x4

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35 storage state IO seam check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
