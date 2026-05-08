#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "spi_bus_runtime_contract_consolidation validation failed: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing required file: $path"
}

require_rg() {
  local pattern="$1"
  local path="$2"
  rg -n "$pattern" "$path" >/dev/null || fail "missing pattern '$pattern' in $path"
}

SPI_RUNTIME="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime.rs"
SPI_CONTRACT="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_contract.rs"
SPI_SMOKE="target-xteink-x4/src/vaachak_x4/contracts/spi_bus_runtime_contract_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/spi-bus-runtime-contract.md"

for path in "$SPI_RUNTIME" "$SPI_CONTRACT" "$SPI_SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$DOC"; do
  require_file "$path"
done

require_rg '^pub mod spi_bus_runtime;' "$PHYSICAL_MOD"
require_rg '^pub mod spi_bus_runtime_contract;' "$PHYSICAL_MOD"
require_rg '^pub mod spi_bus_runtime_contract_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakSpiBusRuntimeContract' "$SPI_CONTRACT"
require_rg 'enum VaachakSpiSharedDevice' "$SPI_CONTRACT"
require_rg 'Display' "$SPI_CONTRACT"
require_rg 'Storage' "$SPI_CONTRACT"
require_rg 'struct VaachakSpiSharedUser' "$SPI_CONTRACT"
require_rg 'struct VaachakSpiPinOwnership' "$SPI_CONTRACT"
require_rg 'struct VaachakSpiTimingOwnership' "$SPI_CONTRACT"
require_rg 'struct VaachakSpiRuntimeContractReport' "$SPI_CONTRACT"
require_rg 'VaachakSpiBusRuntimeBridge' "$SPI_CONTRACT"
require_rg 'SPI_BUS_RUNTIME_CONTRACT_MARKER' "$SPI_CONTRACT"
require_rg 'x4-spi-bus-runtime-contract-ok' "$SPI_CONTRACT"

require_rg 'ACTIVE_ARBITRATION_OWNER:.*vendor/pulp-os imported runtime' "$SPI_CONTRACT"
require_rg 'ACTIVE_SD_RUNTIME_OWNER:.*vendor/pulp-os imported runtime' "$SPI_CONTRACT"
require_rg 'ACTIVE_DISPLAY_RUNTIME_OWNER:.*vendor/pulp-os imported runtime' "$SPI_CONTRACT"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$SPI_CONTRACT"
require_rg 'SD_DRIVER_MOVED_TO_VAACHAK: bool = false' "$SPI_CONTRACT"
require_rg 'SD_MOUNT_OR_PROBE_MOVED_TO_VAACHAK: bool = false' "$SPI_CONTRACT"
require_rg 'FAT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$SPI_CONTRACT"
require_rg 'DISPLAY_DRIVER_MOVED_TO_VAACHAK: bool = false' "$SPI_CONTRACT"
require_rg 'DISPLAY_REFRESH_MOVED_TO_VAACHAK: bool = false' "$SPI_CONTRACT"
require_rg 'selection_rule_allows' "$SPI_CONTRACT"
require_rg 'matches_existing_runtime_facade' "$SPI_CONTRACT"
require_rg 'no_runtime_behavior_moved' "$SPI_CONTRACT"

require_rg 'SPI_SCLK_GPIO: u8 = 8' "$SPI_CONTRACT"
require_rg 'SPI_MOSI_GPIO: u8 = 10' "$SPI_CONTRACT"
require_rg 'SPI_MISO_GPIO: u8 = 7' "$SPI_CONTRACT"
require_rg 'DISPLAY_CS_GPIO: u8 = 21' "$SPI_CONTRACT"
require_rg 'STORAGE_SD_CS_GPIO: u8 = 12' "$SPI_CONTRACT"
require_rg 'SD_PROBE_KHZ: u32 = 400' "$SPI_CONTRACT"
require_rg 'OPERATIONAL_MHZ: u32 = 20' "$SPI_CONTRACT"
require_rg 'DMA_CHANNEL: u8 = 0' "$SPI_CONTRACT"
require_rg 'DMA_TX_BYTES: usize = 4096' "$SPI_CONTRACT"
require_rg 'DMA_RX_BYTES: usize = 4096' "$SPI_CONTRACT"

require_rg 'struct VaachakSpiBusRuntimeContractSmoke' "$SPI_SMOKE"
require_rg 'x4-spi-bus-runtime-contract-smoke-ok' "$SPI_SMOKE"
require_rg 'VaachakSpiBusRuntimeContract::report' "$SPI_SMOKE"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$SPI_SMOKE"
require_rg 'SD_RUNTIME_MOVED_TO_VAACHAK: bool = false' "$SPI_SMOKE"
require_rg 'DISPLAY_RUNTIME_MOVED_TO_VAACHAK: bool = false' "$SPI_SMOKE"

require_rg 'SPI Bus Runtime Contract Consolidation' "$DOC"
require_rg 'shared SPI users' "$DOC"
require_rg 'Display.*SSD1677' "$DOC"
require_rg 'Storage.*microSD' "$DOC"
require_rg 'GPIO21' "$DOC"
require_rg 'GPIO12' "$DOC"
require_rg 'SCLK GPIO8, MOSI GPIO10, MISO GPIO7' "$DOC"
require_rg 'SPI arbitration' "$DOC"
require_rg 'SD probe / mount' "$DOC"
require_rg 'Display refresh' "$DOC"
require_rg 'vendor/pulp-os' "$DOC"
require_rg 'spi_bus_runtime_contract_consolidation=ok' "$DOC"

# Existing runtime facade must remain metadata-only and Pulp-owned for active behavior.
require_rg 'PHYSICAL_SPI_OWNER:.*vendor/pulp-os imported runtime' "$SPI_RUNTIME"
require_rg 'PHYSICAL_SD_OWNER:.*vendor/pulp-os imported runtime' "$SPI_RUNTIME"
require_rg 'PHYSICAL_DISPLAY_OWNER:.*vendor/pulp-os imported runtime' "$SPI_RUNTIME"
require_rg 'PHYSICAL_SPI_OWNED_BY_BRIDGE: bool = false' "$SPI_RUNTIME"
require_rg 'PHYSICAL_SD_OWNED_BY_BRIDGE: bool = false' "$SPI_RUNTIME"
require_rg 'PHYSICAL_DISPLAY_OWNED_BY_BRIDGE: bool = false' "$SPI_RUNTIME"

# The new contract files must not call physical SD/FAT, SPI peripheral, or display behavior.
if rg -n '(SdStorage::mount|SdStorage::init_card|probe_ok\(|Board::init|speed_up_spi\(|init_spi|SpiDevice|ExclusiveDevice|NoDelay|embedded_sdmmc|embedded_hal::|embedded_hal_bus|esp_hal::|x4_kernel::drivers::storage::|x4_kernel::drivers::sdcard::|display\.epd|epd\.|paint_stack\(|refresh\(|draw_packed_pixels\()' "$SPI_CONTRACT" "$SPI_SMOKE"; then
  fail "SPI contract consolidation must not move SD/FAT, SPI arbitration, or display behavior"
fi

# This is an ownership contract only; no mutating/mount/probe/display runtime APIs should be added.
if rg -n '\bfn +(write|append|delete|remove|rename|truncate|mkdir|create|mount|unmount|format|probe|init_card|init_spi|speed_up_spi|refresh|draw|transfer|transaction)[A-Za-z0-9_]*' "$SPI_CONTRACT" "$SPI_SMOKE"; then
  fail "SPI contract consolidation exposes runtime behavior instead of metadata"
fi

if [[ -d spi_bus_runtime_contract_consolidation ]]; then
  if find spi_bus_runtime_contract_consolidation -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/display|target-xteink-x4/src/vaachak_x4/io)/' >/dev/null; then
    fail "overlay includes vendor/app/display/io files; this slice must remain SPI contract-only"
  fi
fi

if [[ -d .git ]]; then
  if git status --short -- vendor/pulp-os vendor/smol-epub | rg -n '.' >/dev/null; then
    git status --short -- vendor/pulp-os vendor/smol-epub >&2
    fail "vendored runtime files changed; Pulp must remain the active hardware behavior owner"
  fi
fi

if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all -- --check
fi

echo "spi_bus_runtime_contract_consolidation=ok"
