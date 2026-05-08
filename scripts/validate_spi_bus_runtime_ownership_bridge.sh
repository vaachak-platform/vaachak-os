#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "spi_bus_runtime_ownership_bridge validation failed: $*" >&2
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

OWNER="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/spi_bus_pulp_backend.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/spi_bus_runtime_ownership_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/spi-bus-runtime-ownership.md"
OLD_DOC="docs/architecture/spi-bus-runtime-contract.md"

for path in "$OWNER" "$BACKEND" "$SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$DOC" "$OLD_DOC"; do
  require_file "$path"
done

require_rg '^pub mod spi_bus_pulp_backend;' "$PHYSICAL_MOD"
require_rg '^pub mod spi_bus_runtime_owner;' "$PHYSICAL_MOD"
require_rg '^pub mod spi_bus_runtime_ownership_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakSpiBusRuntimeOwner' "$OWNER"
require_rg 'SPI_BUS_RUNTIME_OWNERSHIP_MARKER' "$OWNER"
require_rg 'x4-spi-bus-runtime-ownership-ok' "$OWNER"
require_rg 'SPI_BUS_IDENTITY:.*xteink-x4-shared-spi-bus' "$OWNER"
require_rg 'SPI_BUS_OWNERSHIP_AUTHORITY:.*target-xteink-x4 Vaachak layer' "$OWNER"
require_rg 'SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'PULP_COMPATIBILITY_BACKEND: VaachakSpiRuntimeBackend' "$OWNER"
require_rg 'VaachakSpiRuntimeBackend::PulpCompatibility' "$OWNER"
require_rg 'ACTIVE_BACKEND: VaachakSpiRuntimeBackend = Self::PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'ACTIVE_EXECUTOR_OWNER:.*vendor/pulp-os imported runtime' "$OWNER"
require_rg 'SPI_SCLK_GPIO: u8 = 8' "$OWNER"
require_rg 'SPI_MOSI_GPIO: u8 = 10' "$OWNER"
require_rg 'SPI_MISO_GPIO: u8 = 7' "$OWNER"
require_rg 'DISPLAY_CS_GPIO: u8 = 21' "$OWNER"
require_rg 'STORAGE_SD_CS_GPIO: u8 = 12' "$OWNER"
require_rg 'DISPLAY_USER_NAME:.*SSD1677 display' "$OWNER"
require_rg 'STORAGE_USER_NAME:.*microSD storage' "$OWNER"
require_rg 'registered_user' "$OWNER"
require_rg 'display_user' "$OWNER"
require_rg 'storage_user' "$OWNER"
require_rg 'transaction_ownership' "$OWNER"
require_rg 'transaction_metadata_is_safe' "$OWNER"
require_rg 'ownership_bridge_ok' "$OWNER"
require_rg 'ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'SD_FAT_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_RENDERING_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$OWNER"

require_rg 'struct VaachakSpiPulpBackend' "$BACKEND"
require_rg 'BACKEND_NAME:.*PulpCompatibility' "$BACKEND"
require_rg 'ACTIVE_HARDWARE_EXECUTOR: bool = true' "$BACKEND"
require_rg 'ACTIVE_HARDWARE_EXECUTOR_OWNER:.*vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'PHYSICAL_SPI_EXECUTOR_OWNER:.*vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'PHYSICAL_SD_EXECUTOR_OWNER:.*vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'PHYSICAL_DISPLAY_EXECUTOR_OWNER:.*vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'SPI_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SD_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DISPLAY_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SD_FAT_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DISPLAY_RENDERING_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'bridge_ok' "$BACKEND"

require_rg 'struct VaachakSpiBusRuntimeOwnershipSmoke' "$SMOKE"
require_rg 'x4-spi-bus-runtime-ownership-smoke-ok' "$SMOKE"
require_rg 'VaachakSpiBusRuntimeOwner::ownership_bridge_ok' "$SMOKE"
require_rg 'VaachakSpiPulpBackend::bridge_ok' "$SMOKE"
require_rg 'SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SMOKE"
require_rg 'PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true' "$SMOKE"
require_rg 'ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'SD_FAT_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_RENDERING_MOVED_TO_VAACHAK: bool = false' "$SMOKE"

require_rg 'SPI Bus Runtime Ownership Bridge' "$DOC"
require_rg 'ownership authority' "$DOC"
require_rg 'PulpCompatibility' "$DOC"
require_rg 'GPIO21' "$DOC"
require_rg 'GPIO12' "$DOC"
require_rg 'SCLK GPIO8, MOSI GPIO10, MISO GPIO7' "$DOC"
require_rg 'SD probe / mount' "$DOC"
require_rg 'SD FAT read/write/list behavior' "$DOC"
require_rg 'SSD1677 display rendering' "$DOC"
require_rg 'spi_bus_runtime_ownership_bridge=ok' "$DOC"
require_rg 'superseded by the canonical SPI hardware ownership document' "$OLD_DOC"
require_rg 'spi-bus-runtime-ownership.md' "$OLD_DOC"

# New Vaachak SPI ownership files must remain ownership/metadata only.
if rg -n '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|esp_hal::|x4_kernel::drivers::storage|x4_kernel::drivers::sdcard|SdStorage::|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|NoDelay|Board::init|speed_up_spi|init_spi|display\.epd|paint_stack\(|draw_packed_pixels\(|set_pixels\(|flush\(|wait_until_idle\()' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "SPI ownership bridge must not import or call SD/FAT, SPI peripheral, or display behavior"
fi

# Do not add direct runtime behavior functions in the ownership bridge.
if rg -n '\bfn +(read|write|append|delete|remove|rename|truncate|mkdir|create|open|close|mount|unmount|format|probe|init_card|init_spi|speed_up_spi|refresh|draw|transfer|toggle|select|deselect|lock|unlock)[A-Za-z0-9_]*' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "SPI ownership bridge exposes direct runtime hardware or filesystem behavior"
fi

# The overlay itself must not include moved runtime/UI/vendor files.
if [[ -d spi_bus_runtime_ownership_bridge ]]; then
  if find spi_bus_runtime_ownership_bridge -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/display|target-xteink-x4/src/vaachak_x4/io|target-xteink-x4/src/vaachak_x4/ui)/' >/dev/null; then
    fail "overlay includes vendor/app/display/io/ui files; this slice must only add the SPI ownership bridge"
  fi
fi

# Existing Pulp-facing SPI facade, when present, must still point at Pulp for physical behavior.
SPI_RUNTIME="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime.rs"
if [[ -f "$SPI_RUNTIME" ]]; then
  require_rg 'PHYSICAL_SPI_OWNER:.*vendor/pulp-os imported runtime' "$SPI_RUNTIME"
  require_rg 'PHYSICAL_SD_OWNER:.*vendor/pulp-os imported runtime' "$SPI_RUNTIME"
  require_rg 'PHYSICAL_DISPLAY_OWNER:.*vendor/pulp-os imported runtime' "$SPI_RUNTIME"
fi

echo "spi_bus_runtime_ownership_bridge=ok"
