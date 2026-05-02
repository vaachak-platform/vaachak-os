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
  if rg -n -e "$pattern" "$@" >/tmp/phase35c0_reader_state_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35c0_reader_state_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35c0_reader_state_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35C-0 reader state facade check"
echo

exists target-xteink-x4/src/vaachak_x4/apps/mod.rs
exists target-xteink-x4/src/vaachak_x4/apps/reader_state.rs
exists docs/phase35c0/PHASE35C0_READER_STATE_FACADE.md
exists docs/phase35c0/PHASE35C0_ACCEPTANCE.md
exists docs/phase35c0/PHASE35C0_NOTES.md

contains "Vaachak app module is exported" \
  'pub mod apps' \
  target-xteink-x4/src/vaachak_x4/mod.rs

contains "reader state facade defines book identity" \
  'VaachakBookId|VaachakBookIdentity' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "reader state facade defines metadata records" \
  'VaachakBookMetaRecord|meta_record_file_for|MTA|Metadata' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "reader state facade defines theme records" \
  'VaachakReaderThemePreset|VaachakReaderThemeRecord|theme_record_file_for|THM|Theme' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "reader state facade uses storage path helpers" \
  'VaachakStoragePathHelpers|VaachakStateFileKind' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

contains "reader state facade preserves encode/decode helpers" \
  'encode_line|decode_line|push_field|split_fields|percent_decode' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

not_contains "reader state facade does not own physical SD/SPI/FAT IO" \
  'SdCard::new|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice|AsyncVolumeManager|VolumeManager|FileMode' \
  target-xteink-x4/src/vaachak_x4/apps/reader_state.rs

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
echo "Phase 35C-0 reader state facade check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
