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
  if rg -n -e "$pattern" "$@" >/tmp/phase35h0_spi_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35h0_spi_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35h0_spi_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35H-0 SPI bus arbitration facade check"
echo

facade="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime.rs"

exists target-xteink-x4/src/vaachak_x4/physical/mod.rs
exists "$facade"
exists docs/phase35h0/PHASE35H0_SPI_BUS_ARBITRATION_FACADE.md
exists docs/phase35h0/PHASE35H0_ACCEPTANCE.md
exists docs/phase35h0/PHASE35H0_NOTES.md

contains "Vaachak physical module is exported" \
  'pub mod physical' \
  target-xteink-x4/src/vaachak_x4/mod.rs

contains "SPI bus runtime bridge exists" \
  'VaachakSpiBusRuntimeBridge|active_runtime_preflight|preflight_report' \
  "$facade"

contains "facade defines SPI and chip-select pins" \
  'sclk_gpio: 8|mosi_gpio: 10|miso_gpio: 7|epd_cs_gpio: 21|sd_cs_gpio: 12' \
  "$facade"

contains "facade defines timing and DMA contract" \
  'sd_probe_khz: 400|operational_mhz: 20|dma_channel: 0|dma_tx_bytes: 4096|dma_rx_bytes: 4096' \
  "$facade"

contains "facade validates arbitration selection rules" \
  'selection_is_valid|display_selected|storage_selected|phase_allows_storage_io|phase_allows_display_io' \
  "$facade"

contains "active runtime calls Vaachak SPI bus preflight" \
  'spi_bus_runtime::VaachakSpiBusRuntimeBridge::active_runtime_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "SPI bus facade does not own physical SPI/SD/display construction" \
  'spi::master|Spi::new|with_sck|with_mosi|with_miso|with_dma|CriticalSectionDevice|RefCellDevice|SdCard::new|DisplayDriver::new|DmaRxBuf|DmaTxBuf|apply_config' \
  "$facade"

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

echo
echo "Phase 35H-0 SPI bus arbitration facade check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
