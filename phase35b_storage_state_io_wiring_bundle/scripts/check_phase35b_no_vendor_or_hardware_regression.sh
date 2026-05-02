#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

not_contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35b_no_regression_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35b_no_regression_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35B no vendor/hardware regression check"
echo

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

not_contains "Phase 35B bridge does not own input ADC/debounce" \
  'Adc::new|read_oneshot|debounce|repeat|ButtonLadder|Input::new' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs \
  target-xteink-x4/src/vaachak_x4/io/storage_state.rs \
  target-xteink-x4/src/vaachak_x4/io/storage_state_adapter.rs

not_contains "Phase 35B bridge does not own SD/SPI/FAT physical IO" \
  'SdCard::new|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice|AsyncVolumeManager|VolumeManager|FileMode' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

not_contains "Phase 35B bridge does not own SSD1677 refresh/rendering" \
  'SSD1677|ssd1677|draw_|refresh\(|write_cmd|write_data|set_ram|delay_ms|strip' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs \
  target-xteink-x4/src/vaachak_x4/io/storage_state.rs \
  target-xteink-x4/src/vaachak_x4/io/storage_state_adapter.rs

not_contains "Phase 35B does not touch reader app internals in active target namespace" \
  'ReaderApp::new|AppManager::new|FilesApp::new|SettingsApp::new|EpubMeta|EpubSpine|html_strip|ZipIndex' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

not_contains "imported runtime wrapper does not add storage-state physical IO" \
  'open_file_in_dir|open_dir|read\(|write\(|close_file|AsyncVolumeManager|VolumeManager|FileMode' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

echo
echo "Phase 35B no vendor/hardware regression check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
