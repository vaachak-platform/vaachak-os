#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "spi_physical_native_driver_full_migration validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_absent_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    fail "unexpected text '$text' in $file"
  fi
}

SPI="target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/spi_physical_native_driver_smoke.rs"
DOC="docs/architecture/spi-physical-native-driver-full-migration.md"
PHYS_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACT_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
TAKEOVER="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover.rs"

require_file "$SPI"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PHYS_MOD"
require_file "$CONTRACT_MOD"

require_text "$PHYS_MOD" "pub mod spi_physical_native_driver;"
require_text "$CONTRACT_MOD" "pub mod spi_physical_native_driver_smoke;"

require_text "$SPI" "pub struct VaachakSpiPhysicalNativeDriver"
require_text "$SPI" "VaachakNativeSpiPhysicalDriver"
require_text "$SPI" "SPI_FULLY_MIGRATED_TO_VAACHAK: bool = true"
require_text "$SPI" "SPI_TRANSACTION_LIFECYCLE_MOVED_TO_VAACHAK: bool = true"
require_text "$SPI" "SPI_CHIP_SELECT_POLICY_MOVED_TO_VAACHAK: bool = true"
require_text "$SPI" "SPI_DISPLAY_STORAGE_ROUTING_MOVED_TO_VAACHAK: bool = true"
require_text "$SPI" "SPI_TRANSFER_REQUEST_CONSTRUCTION_MOVED_TO_VAACHAK: bool = true"
require_text "$SPI" "PULP_SPI_TRANSFER_FALLBACK_ENABLED: bool = false"
require_text "$SPI" "IMPORTED_PULP_SPI_RUNTIME_ACTIVE: bool = false"
require_text "$SPI" "LOW_LEVEL_HAL_PERIPHERAL_CALLS_REMAIN_TARGET_HAL_BOUNDARY: bool = true"
require_text "$SPI" "SCLK_GPIO: u8 = 8"
require_text "$SPI" "MOSI_GPIO: u8 = 10"
require_text "$SPI" "MISO_GPIO: u8 = 7"
require_text "$SPI" "DISPLAY_CS_GPIO: u8 = 21"
require_text "$SPI" "STORAGE_CS_GPIO: u8 = 12"
require_text "$SPI" "STORAGE_PROBE_HZ: u32 = 400_000"
require_text "$SPI" "OPERATIONAL_HZ: u32 = 20_000_000"
require_text "$SPI" "pub trait VaachakSpiNativePeripheralBackend"
require_text "$SPI" "fn configure_bus"
require_text "$SPI" "fn assert_chip_select"
require_text "$SPI" "fn transfer"
require_text "$SPI" "fn deassert_chip_select"
require_text "$SPI" "execute_with_backend"
require_text "$SPI" "display_request"
require_text "$SPI" "storage_request"
require_text "$SPI" "full_migration_ok"

require_text "$SMOKE" "VaachakSpiPhysicalNativeDriverSmoke"
require_text "$SMOKE" "spi_physical_native_driver_full_migration=ok"
require_text "$SMOKE" "VaachakSpiNativeBackend::VaachakNativeSpiPhysicalDriver"
require_text "$SMOKE" "VaachakSpiPhysicalNativeDriver::full_migration_ok()"

require_text "$DOC" "VaachakNativeSpiPhysicalDriver"
require_text "$DOC" "imported Pulp SPI runtime is no longer the active SPI owner"
require_text "$DOC" "target HAL boundary"

if [ -f "$TAKEOVER" ]; then
  require_text "$TAKEOVER" "vaachak_spi_physical_native_driver_full_migration_selected"
  require_text "$TAKEOVER" "VaachakSpiPhysicalNativeDriver::full_migration_ok()"
fi

# The native SPI migration module must not select PulpCompatibility as a backend.
require_absent_text "$SPI" "PulpCompatibility"
require_absent_text "$SPI" "pulp_reader_runtime"
require_absent_text "$SPI" "x4_kernel::drivers::storage"

echo "spi_physical_native_driver_full_migration=ok"
