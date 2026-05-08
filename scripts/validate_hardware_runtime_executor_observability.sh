#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  printf 'hardware_runtime_executor_observability validation failed: %s\n' "$1" >&2
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
OBS='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_observability.rs'
OBS_BACKEND='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_observability_pulp_backend.rs'
WIRING='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_wiring.rs'
WIRING_BACKEND='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_wiring_pulp_backend.rs'
EXECUTOR='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor.rs'
SMOKE='target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_observability_smoke.rs'
DOC='docs/architecture/hardware-runtime-executor-observability.md'
WIRING_DOC='docs/architecture/hardware-runtime-executor-wiring.md'

for f in "$OBS" "$OBS_BACKEND" "$WIRING" "$WIRING_BACKEND" "$EXECUTOR" "$SMOKE" "$DOC"; do
  require_file "$f"
done

require_pattern "$PHYSICAL_MOD" '^pub mod hardware_runtime_observability_pulp_backend;'
require_pattern "$PHYSICAL_MOD" '^pub mod hardware_runtime_executor_observability;'
require_pattern "$CONTRACTS_MOD" '^pub mod hardware_runtime_executor_observability_smoke;'

# Required prior executor and wiring layers must remain present.
require_pattern "$EXECUTOR" 'pub struct VaachakHardwareRuntimeExecutor;'
require_pattern "$EXECUTOR" 'hardware_runtime_executor_extraction=ok'
require_pattern "$WIRING" 'pub struct VaachakHardwareRuntimeExecutorWiring;'
require_pattern "$WIRING" 'hardware_runtime_executor_wiring=ok'
require_pattern "$WIRING" 'SELECTED_RUNTIME_PATH_COUNT: usize = 10;'
require_pattern "$WIRING_BACKEND" 'pub struct VaachakHardwareRuntimeWiringPulpBackend;'
require_pattern "$WIRING_BACKEND" 'LOW_LEVEL_EXECUTION_REMAINS_PULP_COMPATIBLE: bool = true;'
require_pattern "$WIRING_BACKEND" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'
require_pattern "$WIRING_BACKEND" 'APP_NAVIGATION_CHANGED: bool = false;'
require_pattern "$WIRING_BACKEND" 'FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'

require_pattern "$OBS_BACKEND" 'pub struct VaachakHardwareRuntimeObservabilityPulpBackend;'
require_pattern "$OBS_BACKEND" 'OBSERVABILITY_BACKEND_ACTIVE: bool = true;'
require_pattern "$OBS_BACKEND" 'OBSERVABILITY_ROUTES_THROUGH_WIRED_EXECUTOR: bool = true;'
require_pattern "$OBS_BACKEND" 'LOW_LEVEL_EXECUTION_REMAINS_PULP_COMPATIBLE: bool = true;'
require_pattern "$OBS_BACKEND" 'OBSERVABILITY_MUTATES_HARDWARE_BEHAVIOR: bool = false;'
require_pattern "$OBS_BACKEND" 'BOOT_MARKERS_ARE_METADATA_ONLY: bool = true;'
require_pattern "$OBS_BACKEND" 'DEBUG_MARKERS_ARE_METADATA_ONLY: bool = true;'
require_pattern "$OBS_BACKEND" 'DISPLAY_EXECUTION_CHANGED: bool = false;'
require_pattern "$OBS_BACKEND" 'STORAGE_EXECUTION_CHANGED: bool = false;'
require_pattern "$OBS_BACKEND" 'INPUT_EXECUTION_CHANGED: bool = false;'
require_pattern "$OBS_BACKEND" 'SPI_EXECUTION_CHANGED: bool = false;'
require_pattern "$OBS_BACKEND" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'
require_pattern "$OBS_BACKEND" 'APP_NAVIGATION_CHANGED: bool = false;'
require_pattern "$OBS_BACKEND" 'FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'
require_pattern "$OBS_BACKEND" 'VaachakHardwareRuntimeWiringPulpBackend::backend_ok\(\)'
require_pattern "$OBS_BACKEND" 'backend_ok'

require_pattern "$OBS" 'pub struct VaachakHardwareRuntimeExecutorObservability;'
require_pattern "$OBS" 'hardware_runtime_executor_observability=ok'
require_pattern "$OBS" 'OBSERVABILITY_ENTRYPOINT_ACTIVE: bool = true;'
require_pattern "$OBS" 'BOOT_MARKER_COUNT: usize = 8;'
require_pattern "$OBS" 'WIRED_RUNTIME_PATH_COUNT: usize'
require_pattern "$OBS" 'VaachakHardwareRuntimeExecutorWiring::SELECTED_RUNTIME_PATH_COUNT'
require_pattern "$OBS" 'MARKERS_ARE_METADATA_ONLY: bool = true;'
require_pattern "$OBS" 'EMITS_TEXT_TO_DISPLAY: bool = false;'
require_pattern "$OBS" 'TOUCHES_SPI_TRANSFER: bool = false;'
require_pattern "$OBS" 'TOUCHES_SD_MMC: bool = false;'
require_pattern "$OBS" 'TOUCHES_FAT_IMPLEMENTATION: bool = false;'
require_pattern "$OBS" 'TOUCHES_SSD1677_RENDERING: bool = false;'
require_pattern "$OBS" 'TOUCHES_INPUT_ADC: bool = false;'
require_pattern "$OBS" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'
require_pattern "$OBS" 'APP_NAVIGATION_CHANGED: bool = false;'
require_pattern "$OBS" 'FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'
require_pattern "$OBS" 'hardware\.executor\.layer\.selected'
require_pattern "$OBS" 'hardware\.executor\.wiring\.selected'
require_pattern "$OBS" 'hardware\.executor\.backend\.pulp_compatible'
require_pattern "$OBS" 'hardware\.executor\.spi\.paths\.selected'
require_pattern "$OBS" 'hardware\.executor\.storage\.paths\.selected'
require_pattern "$OBS" 'hardware\.executor\.display\.paths\.selected'
require_pattern "$OBS" 'hardware\.executor\.input\.paths\.selected'
require_pattern "$OBS" 'hardware\.executor\.behavior\.preserved'
require_pattern "$OBS" 'hardware\.executor\.path\.selected'
require_pattern "$OBS" 'VaachakHardwareRuntimeExecutor::extraction_ok\(\)'
require_pattern "$OBS" 'VaachakHardwareRuntimeExecutorWiring::wiring_ok\(\)'
require_pattern "$OBS" 'VaachakHardwareRuntimeExecutorWiring::selected_paths\(\)'
require_pattern "$OBS" 'VaachakHardwareRuntimeObservabilityPulpBackend::backend_ok\(\)'
require_pattern "$OBS" 'boot_markers_selected'
require_pattern "$OBS" 'all_wired_paths_observed'
require_pattern "$OBS" 'observability_ok'

require_pattern "$SMOKE" 'pub struct VaachakHardwareRuntimeExecutorObservabilitySmoke;'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorObservability::observability_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutor::extraction_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorWiring::wiring_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeObservabilityPulpBackend::backend_ok\(\)'
require_pattern "$SMOKE" 'BOOT_MARKER_COUNT\s*==\s*8'

require_pattern "$DOC" 'hardware_runtime_executor_observability=ok'
require_pattern "$DOC" 'hardware\.executor\.layer\.selected'
require_pattern "$DOC" 'hardware\.executor\.wiring\.selected'
require_pattern "$DOC" 'hardware\.executor\.backend\.pulp_compatible'
require_pattern "$DOC" 'BootStorageAvailability'
require_pattern "$DOC" 'ReaderFileChunkIntent'
require_pattern "$DOC" 'DisplayPartialRefreshHandoff'
require_pattern "$DOC" 'InputNavigationHandoff'
require_pattern "$DOC" 'SharedSpiStorageHandoff'
require_pattern "$DOC" 'PulpCompatibility'
require_pattern "$DOC" 'metadata only'
if [[ -f "$WIRING_DOC" ]]; then
  require_pattern "$WIRING_DOC" 'hardware-runtime-executor-observability.md'
fi

for file in "$OBS" "$OBS_BACKEND" "$SMOKE"; do
  require_absent_pattern "$file" 'pub\s+(const\s+)?fn\s+(write|append|delete|rename|mkdir|format|erase|draw_pixels|draw_bitmap|refresh_full|refresh_partial|scan_adc|debounce_event|toggle_chip_select|spi_transfer|mount_sd|probe_sd|print|println|log_to_display)\b'
  require_absent_pattern "$file" '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|esp_hal::|x4_kernel::drivers::storage|x4_kernel::drivers::display|x4_kernel::drivers::input|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|draw_packed_pixels|paint_stack\(|set_pixels\(|wait_until_idle\(|println!|format!)'
done

if [[ -d hardware_runtime_executor_observability/src || -d hardware_runtime_executor_observability/vendor ]]; then
  fail 'overlay unexpectedly contains src/ or vendor/ runtime source'
fi
if [[ -d hardware_runtime_executor_observability/target-xteink-x4/src/apps ]]; then
  fail 'overlay unexpectedly contains app source changes'
fi

printf '%s\n' 'hardware_runtime_executor_observability=ok'
