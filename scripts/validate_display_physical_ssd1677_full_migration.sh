#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "display_physical_ssd1677_full_migration validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local text="$1"
  local file="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_absent_text() {
  local text="$1"
  local file="$2"
  if grep -Fq "$text" "$file"; then
    fail "forbidden text '$text' in $file"
  fi
}

SRC=target-xteink-x4/src/vaachak_x4/physical/display_physical_ssd1677_native_driver.rs
SMOKE=target-xteink-x4/src/vaachak_x4/contracts/display_physical_ssd1677_native_driver_smoke.rs
DOC=docs/architecture/display-physical-ssd1677-full-migration.md
SPI_SRC=target-xteink-x4/src/vaachak_x4/physical/spi_physical_native_driver.rs

require_file "$SRC"
require_file "$SMOKE"
require_file "$DOC"
require_file scripts/validate_display_physical_ssd1677_full_migration.sh
require_file "$SPI_SRC"

require_text "pub mod display_physical_ssd1677_native_driver;" target-xteink-x4/src/vaachak_x4/physical/mod.rs
require_text "pub mod display_physical_ssd1677_native_driver_smoke;" target-xteink-x4/src/vaachak_x4/contracts/mod.rs

require_text "VaachakDisplayPhysicalSsd1677NativeDriver" "$SRC"
require_text "DISPLAY_PHYSICAL_SSD1677_FULL_MIGRATION_MARKER" "$SRC"
require_text "display_physical_ssd1677_full_migration=ok" "$SRC"
require_text "ACTIVE_BACKEND_NAME: &'static str = \"VaachakNativeSsd1677PhysicalDriver\"" "$SRC"
require_text "TRANSPORT_BACKEND_NAME: &'static str = \"VaachakNativeSpiPhysicalDriver\"" "$SRC"
require_text "SSD1677_COMMAND_SEQUENCE_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SSD1677_REFRESH_LIFECYCLE_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SSD1677_BUSY_HANDLING_POLICY_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SSD1677_DISPLAY_STATE_TRACKING_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SSD1677_RAM_WINDOW_TRACKING_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SSD1677_RESET_POLICY_MOVED_TO_VAACHAK: bool = true" "$SRC"
require_text "SSD1677_USES_NATIVE_SPI_DRIVER: bool = true" "$SRC"
require_text "PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED: bool = false" "$SRC"
require_text "IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE: bool = false" "$SRC"
require_text "VaachakSpiPhysicalNativeDriver::display_request" "$SRC"
require_text "VaachakSpiPhysicalNativeDriver::execute_with_backend" "$SRC"
require_text "VaachakSpiPhysicalNativeDriver::full_migration_ok" "$SRC"
require_text "full_refresh_sequence" "$SRC"
require_text "partial_refresh_sequence" "$SRC"
require_text "clear_frame_sequence" "$SRC"
require_text "sleep_sequence" "$SRC"
require_text "busy_policy" "$SRC"
require_text "reset_policy" "$SRC"
require_text "initial_state" "$SRC"
require_text "migration_report" "$SRC"
require_text "full_migration_ok" "$SRC"

require_text "VaachakDisplayPhysicalSsd1677NativeDriverSmoke" "$SMOKE"
require_text "DISPLAY_PHYSICAL_SSD1677_NATIVE_DRIVER_SMOKE_OK" "$SMOKE"
require_text "VaachakNativeSsd1677PhysicalDriver" "$DOC"
require_text "VaachakNativeSpiPhysicalDriver" "$DOC"
require_text "Pulp display fallback: \`false\`" "$DOC"
require_text "SSD1677 command sequencing" "$DOC"
require_text "BUSY timeout/poll policy" "$DOC"

# Ensure the SPI dependency is the accepted native SPI boundary, not imported Pulp SPI.
require_text "SPI_FULLY_MIGRATED_TO_VAACHAK: bool = true" "$SPI_SRC"
require_text "PULP_SPI_TRANSFER_FALLBACK_ENABLED: bool = false" "$SPI_SRC"
require_text "IMPORTED_PULP_SPI_RUNTIME_ACTIVE: bool = false" "$SPI_SRC"

# Guard against accidentally selecting the old Pulp display executor in this new boundary.
require_absent_text "VaachakHardwareRuntimePulpCompatibilityBackend" "$SRC"
require_absent_text "VaachakDisplayRefreshCommandExecutorWithPulpExecutor" "$SRC"
require_absent_text "vendor/pulp-os imported runtime" "$SRC"

# Guard against unrelated UX/app behavior wiring.
if [ -d target-xteink-x4/src/apps ]; then
  if grep -R "display_physical_ssd1677_native_driver" target-xteink-x4/src/apps >/dev/null 2>&1; then
    fail "app UX files reference SSD1677 physical driver directly"
  fi
fi
if [ -d vendor/pulp-os ]; then
  if grep -R "display_physical_ssd1677_native_driver" vendor/pulp-os >/dev/null 2>&1; then
    fail "vendor/pulp-os was modified to reference Vaachak SSD1677 driver"
  fi
fi

echo "display_physical_ssd1677_full_migration=ok"
