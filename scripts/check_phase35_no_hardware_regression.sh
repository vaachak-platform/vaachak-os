#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

not_contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35_no_hw_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35_no_hw_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35 no hardware regression check"
echo

if [[ ! -d target-xteink-x4/src/vaachak_x4/io ]]; then
  ok "Phase 35 io seam directory is absent; no new Phase 35 hardware owner exists"
  echo
  echo "Phase 35 no hardware regression check complete: failures=${failures} warnings=${warnings}"
  exit 0
fi

not_contains "Phase 35 io seam does not own input ADC/debounce" \
  'Adc::new|read_oneshot|debounce|repeat|ButtonReader|ButtonLadder' \
  target-xteink-x4/src/vaachak_x4/io

not_contains "Phase 35 io seam does not own SD/SPI arbitration" \
  'spi::master|Spi::new|RefCellDevice|SdCard::new|AsyncVolumeManager|VolumeManager|open_raw_volume|BlockDevice' \
  target-xteink-x4/src/vaachak_x4/io

not_contains "Phase 35 io seam does not own SSD1677 refresh/rendering" \
  'Ssd1677|ssd1677|write_cmd|write_data|refresh\(|draw_|strip|set_ram|master_activate|DISPLAY_REFRESH|delay_ms' \
  target-xteink-x4/src/vaachak_x4/io

not_contains "Phase 35 io seam does not own reader app internals" \
  'ReaderApp::new|AppManager::new|FilesApp::new|smol_epub::epub|parse_opf|render_page|chapter_cache' \
  target-xteink-x4/src/vaachak_x4/io

not_contains "imported runtime wrapper does not add Phase 35 storage-state physical IO" \
  'VaachakStorageStateIoAdapter|VaachakStorageStatePathIo|read_state_path|write_state_path' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

echo
echo "Phase 35 no hardware regression check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
