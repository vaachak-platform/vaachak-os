#!/usr/bin/env bash
set -euo pipefail

fail() {
  printf 'hardware_runtime_executor_wiring validation failed: %s\n' "$1" >&2
  exit 1
}

require_file() {
  local file="$1"
  [[ -f "$file" ]] || fail "missing file $file"
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  require_file "$file"
  rg -n --pcre2 "$pattern" "$file" >/dev/null || fail "missing pattern '$pattern' in $file"
}

require_absent_pattern() {
  local file="$1"
  local pattern="$2"
  require_file "$file"
  if rg -n --pcre2 "$pattern" "$file" >/dev/null; then
    fail "forbidden pattern '$pattern' found in $file"
  fi
}

PHYSICAL_MOD='target-xteink-x4/src/vaachak_x4/physical/mod.rs'
CONTRACTS_MOD='target-xteink-x4/src/vaachak_x4/contracts/mod.rs'
WIRING='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_wiring.rs'
WIRING_BACKEND='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_wiring_pulp_backend.rs'
EXECUTOR='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor.rs'
EXECUTOR_BACKEND='target-xteink-x4/src/vaachak_x4/physical/hardware_executor_pulp_backend.rs'
SPI='target-xteink-x4/src/vaachak_x4/physical/spi_executor_bridge.rs'
STORAGE='target-xteink-x4/src/vaachak_x4/physical/storage_executor_bridge.rs'
DISPLAY='target-xteink-x4/src/vaachak_x4/physical/display_executor_bridge.rs'
INPUT='target-xteink-x4/src/vaachak_x4/physical/input_executor_bridge.rs'
SMOKE='target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_wiring_smoke.rs'
DOC='docs/architecture/hardware-runtime-executor-wiring.md'
EXTRACTION_DOC='docs/architecture/hardware-runtime-executor-extraction.md'

for f in "$WIRING" "$WIRING_BACKEND" "$EXECUTOR" "$EXECUTOR_BACKEND" "$SPI" "$STORAGE" "$DISPLAY" "$INPUT" "$SMOKE" "$DOC" "$EXTRACTION_DOC"; do
  require_file "$f"
done

for module in \
  hardware_runtime_wiring_pulp_backend \
  hardware_runtime_executor_wiring; do
  require_pattern "$PHYSICAL_MOD" "^pub mod ${module};"
done
require_pattern "$CONTRACTS_MOD" '^pub mod hardware_runtime_executor_wiring_smoke;'

# Prior broad executor extraction must be present and still referenced.
require_pattern "$EXECUTOR" 'pub struct VaachakHardwareRuntimeExecutor;'
require_pattern "$EXECUTOR" 'hardware_runtime_executor_extraction=ok'
require_pattern "$EXECUTOR" 'VaachakSpiExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR" 'VaachakStorageExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR" 'VaachakDisplayExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR" 'VaachakInputExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR_BACKEND" 'pub struct VaachakHardwareExecutorPulpBackend;'
require_pattern "$EXECUTOR_BACKEND" 'PulpCompatibility'
require_pattern "$EXECUTOR_BACKEND" 'vendor/pulp-os imported runtime'

require_pattern "$WIRING_BACKEND" 'pub struct VaachakHardwareRuntimeWiringPulpBackend;'
require_pattern "$WIRING_BACKEND" 'WIRING_BACKEND_ACTIVE: bool = true;'
require_pattern "$WIRING_BACKEND" 'ROUTES_THROUGH_CONSOLIDATED_VAACHAK_EXECUTOR: bool = true;'
require_pattern "$WIRING_BACKEND" 'LOW_LEVEL_EXECUTION_REMAINS_PULP_COMPATIBLE: bool = true;'
require_pattern "$WIRING_BACKEND" 'VaachakHardwareExecutorPulpBackend::backend_ok\(\)'
require_pattern "$WIRING_BACKEND" 'PHYSICAL_SPI_TRANSFER_REWRITTEN: bool = false;'
require_pattern "$WIRING_BACKEND" 'SD_MMC_LOW_LEVEL_REWRITTEN: bool = false;'
require_pattern "$WIRING_BACKEND" 'FAT_STORAGE_REWRITTEN: bool = false;'
require_pattern "$WIRING_BACKEND" 'SSD1677_DISPLAY_REWRITTEN: bool = false;'
require_pattern "$WIRING_BACKEND" 'BUTTON_ADC_INPUT_REWRITTEN: bool = false;'
require_pattern "$WIRING_BACKEND" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'
require_pattern "$WIRING_BACKEND" 'APP_NAVIGATION_CHANGED: bool = false;'
require_pattern "$WIRING_BACKEND" 'FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'

require_pattern "$WIRING" 'pub struct VaachakHardwareRuntimeExecutorWiring;'
require_pattern "$WIRING" 'hardware_runtime_executor_wiring=ok'
require_pattern "$WIRING" 'WIRING_ENTRYPOINT_ACTIVE: bool = true;'
require_pattern "$WIRING" 'SELECTED_RUNTIME_PATH_COUNT: usize = 10;'
require_pattern "$WIRING" 'BootStorageAvailability'
require_pattern "$WIRING" 'LibraryDirectoryListing'
require_pattern "$WIRING" 'ReaderFileOpenIntent'
require_pattern "$WIRING" 'ReaderFileChunkIntent'
require_pattern "$WIRING" 'DisplayFullRefreshHandoff'
require_pattern "$WIRING" 'DisplayPartialRefreshHandoff'
require_pattern "$WIRING" 'InputButtonScanHandoff'
require_pattern "$WIRING" 'InputNavigationHandoff'
require_pattern "$WIRING" 'SharedSpiDisplayHandoff'
require_pattern "$WIRING" 'SharedSpiStorageHandoff'
require_pattern "$WIRING" 'VaachakHardwareRuntimeExecutor::entry_for'
require_pattern "$WIRING" 'VaachakHardwareRuntimeExecutor::entry_is_safe'
require_pattern "$WIRING" 'VaachakHardwareRuntimeExecutor::extraction_ok\(\)'
require_pattern "$WIRING" 'VaachakStorageExecutorBridge::route_intent'
require_pattern "$WIRING" 'VaachakDisplayExecutorBridge::route_intent'
require_pattern "$WIRING" 'VaachakInputExecutorBridge::route_intent'
require_pattern "$WIRING" 'VaachakSpiExecutorBridge::route_transaction_intent'
require_pattern "$WIRING" 'RoutedThroughVaachakHardwareRuntimeExecutor'
require_pattern "$WIRING" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'
require_pattern "$WIRING" 'APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;'
require_pattern "$WIRING" 'DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;'
require_pattern "$WIRING" 'INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;'
require_pattern "$WIRING" 'FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'
require_pattern "$WIRING" 'storage_paths_wired'
require_pattern "$WIRING" 'display_paths_wired'
require_pattern "$WIRING" 'input_paths_wired'
require_pattern "$WIRING" 'spi_paths_wired'
require_pattern "$WIRING" 'wiring_ok'

require_pattern "$SMOKE" 'pub struct VaachakHardwareRuntimeExecutorWiringSmoke;'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorWiring::wiring_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutor::extraction_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeWiringPulpBackend::backend_ok\(\)'
require_pattern "$SMOKE" 'SELECTED_RUNTIME_PATH_COUNT == 10'

require_pattern "$DOC" 'hardware_runtime_executor_wiring=ok'
require_pattern "$DOC" 'BootStorageAvailability'
require_pattern "$DOC" 'LibraryDirectoryListing'
require_pattern "$DOC" 'ReaderFileOpenIntent'
require_pattern "$DOC" 'DisplayFullRefreshHandoff'
require_pattern "$DOC" 'InputNavigationHandoff'
require_pattern "$DOC" 'SharedSpiStorageHandoff'
require_pattern "$DOC" 'PulpCompatibility'
require_pattern "$EXTRACTION_DOC" 'hardware-runtime-executor-wiring.md'

for file in "$WIRING" "$WIRING_BACKEND" "$SMOKE"; do
  require_absent_pattern "$file" 'pub\s+(const\s+)?fn\s+(write|append|delete|rename|mkdir|format|erase|draw_pixels|draw_bitmap|refresh_full|refresh_partial|scan_adc|debounce_event|toggle_chip_select|spi_transfer|mount_sd|probe_sd)\b'
  require_absent_pattern "$file" '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|esp_hal::|x4_kernel::drivers::storage|x4_kernel::drivers::display|x4_kernel::drivers::input|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|draw_packed_pixels|paint_stack\(|set_pixels\(|wait_until_idle\()'
done

if [[ -d hardware_runtime_executor_wiring/src || -d hardware_runtime_executor_wiring/vendor ]]; then
  fail 'overlay unexpectedly contains src/ or vendor/ runtime source'
fi
if [[ -d hardware_runtime_executor_wiring/target-xteink-x4/src/apps ]]; then
  fail 'overlay unexpectedly contains app source changes'
fi

printf '%s\n' 'hardware_runtime_executor_wiring=ok'
