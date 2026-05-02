#!/usr/bin/env bash
set -euo pipefail

failures=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase35h0_no_takeover_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35h0_no_takeover_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35h0_no_takeover_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35H-0 no SPI hardware takeover check"
echo

runtime="target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs"
facade="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime.rs"

contains "active runtime still initializes board through imported Pulp Board" \
  'let board = Board::init\(peripherals\)' \
  "$runtime"

contains "active runtime still speeds imported SPI bus through Pulp helper" \
  'speed_up_spi\(\)' \
  "$runtime"

contains "active runtime still mounts imported SD storage" \
  'SdStorage::mount\(card\)\.await|SdStorage::empty\(\)' \
  "$runtime"

contains "active runtime still initializes imported display" \
  'let mut epd = board\.display\.epd|epd\.init\(&mut delay\)' \
  "$runtime"

not_contains "Vaachak SPI facade does not own physical SPI/SD/display construction" \
  'spi::master|Spi::new|with_sck|with_mosi|with_miso|with_dma|CriticalSectionDevice|RefCellDevice|SdCard::new|DisplayDriver::new|DmaRxBuf|DmaTxBuf|apply_config' \
  "$facade"

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

echo
echo "Phase 35H-0 no SPI hardware takeover check complete: failures=${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
