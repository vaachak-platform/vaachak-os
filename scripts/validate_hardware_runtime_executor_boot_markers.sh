#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  printf 'hardware_runtime_executor_boot_markers validation failed: %s\n' "$1" >&2
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
BOOT='target-xteink-x4/src/vaachak_x4/boot.rs'
IMPORTED_RUNTIME='target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs'
BOOT_MARKERS='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_boot_markers.rs'
OBS='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_observability.rs'
WIRING='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_wiring.rs'
SMOKE='target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_boot_markers_smoke.rs'
DOC='docs/architecture/hardware-runtime-executor-boot-markers.md'
OBS_DOC='docs/architecture/hardware-runtime-executor-observability.md'
VALIDATOR='scripts/validate_hardware_runtime_executor_boot_markers.sh'

for f in "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$BOOT" "$IMPORTED_RUNTIME" "$BOOT_MARKERS" "$OBS" "$WIRING" "$SMOKE" "$DOC" "$VALIDATOR"; do
  require_file "$f"
done

require_pattern "$PHYSICAL_MOD" '^pub mod hardware_runtime_executor_boot_markers;'
require_pattern "$CONTRACTS_MOD" '^pub mod hardware_runtime_executor_boot_markers_smoke;'

require_pattern "$OBS" 'hardware_runtime_executor_observability=ok'
require_pattern "$OBS" 'BOOT_MARKER_COUNT: usize = 8;'
require_pattern "$OBS" 'hardware\.executor\.behavior\.preserved'
require_pattern "$WIRING" 'hardware_runtime_executor_wiring=ok'
require_pattern "$WIRING" 'SELECTED_RUNTIME_PATH_COUNT: usize = 10;'

require_pattern "$BOOT_MARKERS" 'pub struct VaachakHardwareRuntimeExecutorBootMarkers;'
require_pattern "$BOOT_MARKERS" 'hardware_runtime_executor_boot_markers=ok'
require_pattern "$BOOT_MARKERS" 'HARDWARE_RUNTIME_EXECUTOR_BOOT_MARKERS_OWNER'
require_pattern "$BOOT_MARKERS" 'target-xteink-x4 Vaachak layer'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKER_ENTRYPOINT_ACTIVE: bool = true;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKER_COUNT: usize'
require_pattern "$BOOT_MARKERS" 'VaachakHardwareRuntimeExecutorObservability::BOOT_MARKER_COUNT'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKERS_ROUTE_THROUGH_OBSERVABILITY: bool = true;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKERS_EMIT_TO_DEBUG_STREAM: bool = true;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKERS_WRITE_TO_DISPLAY: bool = false;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKERS_TOUCH_SPI_TRANSFER: bool = false;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKERS_TOUCH_STORAGE_EXECUTION: bool = false;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKERS_TOUCH_DISPLAY_RENDERING: bool = false;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKERS_TOUCH_INPUT_EXECUTION: bool = false;'
require_pattern "$BOOT_MARKERS" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'
require_pattern "$BOOT_MARKERS" 'APP_NAVIGATION_CHANGED: bool = false;'
require_pattern "$BOOT_MARKERS" 'BOOT_MARKER_COUNT == 8'
require_pattern "$BOOT_MARKERS" 'boot_marker_set_ready'
require_pattern "$BOOT_MARKERS" 'boot_markers_ok'
require_pattern "$BOOT_MARKERS" 'emit_boot_markers'
require_pattern "$BOOT_MARKERS" 'HARDWARE_RUNTIME_EXECUTOR_OBSERVABILITY_MARKER'
require_pattern "$BOOT_MARKERS" 'hardware_runtime_executor_boot_markers=failed'
require_pattern "$BOOT_MARKERS" 'hardware\.executor\.marker\.failed'
require_pattern "$BOOT_MARKERS" 'VaachakHardwareRuntimeExecutorObservability::boot_markers\(\)'
require_pattern "$BOOT_MARKERS" 'VaachakHardwareRuntimeExecutorObservability::all_wired_paths_observed\(\)'

require_pattern "$BOOT" 'emit_hardware_runtime_executor_boot_markers'
require_pattern "$BOOT" 'VaachakHardwareRuntimeExecutorBootMarkers::emit_boot_markers\(\)'
require_pattern "$IMPORTED_RUNTIME" 'VaachakBoot::emit_runtime_ready_marker\(\);'
require_pattern "$IMPORTED_RUNTIME" 'VaachakBoot::emit_hardware_runtime_executor_boot_markers\(\);'

require_pattern "$SMOKE" 'pub struct VaachakHardwareRuntimeExecutorBootMarkersSmoke;'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorObservability::observability_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorWiring::wiring_ok\(\)'
require_pattern "$SMOKE" 'BOOT_MARKER_COUNT == 8'
require_pattern "$SMOKE" 'BOOT_MARKERS_EMIT_TO_DEBUG_STREAM'
require_pattern "$SMOKE" 'BOOT_MARKERS_WRITE_TO_DISPLAY'

require_pattern "$DOC" 'hardware_runtime_executor_boot_markers=ok'
require_pattern "$DOC" 'hardware_runtime_executor_observability=ok'
require_pattern "$DOC" 'hardware\.executor\.layer\.selected'
require_pattern "$DOC" 'hardware\.executor\.wiring\.selected'
require_pattern "$DOC" 'hardware\.executor\.backend\.pulp_compatible'
require_pattern "$DOC" 'hardware\.executor\.spi\.paths\.selected'
require_pattern "$DOC" 'hardware\.executor\.storage\.paths\.selected'
require_pattern "$DOC" 'hardware\.executor\.display\.paths\.selected'
require_pattern "$DOC" 'hardware\.executor\.input\.paths\.selected'
require_pattern "$DOC" 'hardware\.executor\.behavior\.preserved'
require_pattern "$DOC" 'serial/debug boot markers only'
require_pattern "$DOC" 'not rendered on the e-paper display'
if [[ -f "$OBS_DOC" ]]; then
  require_pattern "$OBS_DOC" 'hardware-runtime-executor-boot-markers.md'
fi

for file in "$BOOT_MARKERS" "$SMOKE"; do
  require_absent_pattern "$file" 'pub\s+(const\s+)?fn\s+(write|append|delete|rename|mkdir|format|erase|draw_pixels|draw_bitmap|refresh_full|refresh_partial|scan_adc|debounce_event|toggle_chip_select|spi_transfer|mount_sd|probe_sd)\b'
  require_absent_pattern "$file" '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|x4_kernel::drivers::storage|x4_kernel::drivers::display|x4_kernel::drivers::input|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|draw_packed_pixels|paint_stack\(|set_pixels\(|wait_until_idle\()'
done

if [[ -d hardware_runtime_executor_boot_markers/src || -d hardware_runtime_executor_boot_markers/vendor ]]; then
  fail 'overlay unexpectedly contains src/ or vendor/ runtime source'
fi
if [[ -d hardware_runtime_executor_boot_markers/target-xteink-x4/src/apps ]]; then
  fail 'overlay unexpectedly contains app source changes'
fi
if [[ -d hardware_runtime_executor_boot_markers/vendor ]]; then
  fail 'overlay unexpectedly contains vendor changes'
fi

printf '%s\n' 'hardware_runtime_executor_boot_markers=ok'
