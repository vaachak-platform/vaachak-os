#!/usr/bin/env bash
set -euo pipefail

fail() {
  printf 'hardware_runtime_executor_extraction validation failed: %s\n' "$1" >&2
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
EXECUTOR='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor.rs'
BACKEND='target-xteink-x4/src/vaachak_x4/physical/hardware_executor_pulp_backend.rs'
SPI='target-xteink-x4/src/vaachak_x4/physical/spi_executor_bridge.rs'
STORAGE='target-xteink-x4/src/vaachak_x4/physical/storage_executor_bridge.rs'
DISPLAY='target-xteink-x4/src/vaachak_x4/physical/display_executor_bridge.rs'
INPUT='target-xteink-x4/src/vaachak_x4/physical/input_executor_bridge.rs'
SMOKE='target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_smoke.rs'
DOC='docs/architecture/hardware-runtime-executor-extraction.md'

for f in "$EXECUTOR" "$BACKEND" "$SPI" "$STORAGE" "$DISPLAY" "$INPUT" "$SMOKE" "$DOC"; do
  require_file "$f"
done

for module in \
  hardware_executor_pulp_backend \
  spi_executor_bridge \
  storage_executor_bridge \
  display_executor_bridge \
  input_executor_bridge \
  hardware_runtime_executor; do
  require_pattern "$PHYSICAL_MOD" "^pub mod ${module};"
done
require_pattern "$CONTRACTS_MOD" '^pub mod hardware_runtime_executor_smoke;'

# Required prior ownership / executor stack. These checks fail clearly if this
# broad extraction is applied before the accepted hardware ownership layers.
for prior in \
  target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_ownership.rs \
  target-xteink-x4/src/vaachak_x4/physical/spi_bus_arbitration_runtime_owner.rs \
  target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_executor_bridge.rs \
  target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs \
  target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs \
  target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs; do
  require_file "$prior"
done

require_pattern "$EXECUTOR" 'pub struct VaachakHardwareRuntimeExecutor;'
require_pattern "$EXECUTOR" 'HARDWARE_RUNTIME_EXECUTOR_EXTRACTION_MARKER'
require_pattern "$EXECUTOR" 'hardware_runtime_executor_extraction=ok'
require_pattern "$EXECUTOR" 'CONSOLIDATED_EXECUTOR_ENTRYPOINT_ACTIVE: bool = true;'
require_pattern "$EXECUTOR" 'VaachakHardwareRuntimeOwnership::consolidation_ok\(\)'
require_pattern "$EXECUTOR" 'VaachakSpiExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR" 'VaachakStorageExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR" 'VaachakDisplayExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR" 'VaachakInputExecutorBridge::bridge_ok\(\)'
require_pattern "$EXECUTOR" 'READER_FILE_BROWSER_UX_BEHAVIOR_CHANGED: bool = false;'
require_pattern "$EXECUTOR" 'APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;'
require_pattern "$EXECUTOR" 'DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;'
require_pattern "$EXECUTOR" 'INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;'
require_pattern "$EXECUTOR" 'FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'

require_pattern "$BACKEND" 'pub struct VaachakHardwareExecutorPulpBackend;'
require_pattern "$BACKEND" 'BACKEND_NAME'
require_pattern "$BACKEND" 'PulpCompatibility'
require_pattern "$BACKEND" 'ACTIVE_EXECUTOR_OWNER'
require_pattern "$BACKEND" 'vendor/pulp-os imported runtime'
require_pattern "$BACKEND" 'PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false;'
require_pattern "$BACKEND" 'SD_MMC_LOW_LEVEL_EXECUTOR_MOVED_TO_VAACHAK: bool = false;'
require_pattern "$BACKEND" 'FAT_STORAGE_EXECUTOR_REWRITTEN_IN_VAACHAK: bool = false;'
require_pattern "$BACKEND" 'SSD1677_DISPLAY_EXECUTOR_REWRITTEN_IN_VAACHAK: bool = false;'
require_pattern "$BACKEND" 'BUTTON_ADC_INPUT_EXECUTOR_REWRITTEN_IN_VAACHAK: bool = false;'

require_pattern "$SPI" 'pub struct VaachakSpiExecutorBridge;'
require_pattern "$SPI" 'DisplayTransaction'
require_pattern "$SPI" 'StorageTransaction'
require_pattern "$SPI" 'SafeArbitrationHandoff'
require_pattern "$SPI" 'VaachakSpiBusArbitrationRuntimeOwner::grant_for'
require_pattern "$SPI" 'PHYSICAL_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false;'
require_pattern "$SPI" 'CHIP_SELECT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;'

require_pattern "$STORAGE" 'pub struct VaachakStorageExecutorBridge;'
require_pattern "$STORAGE" 'CardPresent'
require_pattern "$STORAGE" 'ProbeCard'
require_pattern "$STORAGE" 'MountStorage'
require_pattern "$STORAGE" 'StorageAvailableState'
require_pattern "$STORAGE" 'LibraryFileMetadataAccess'
require_pattern "$STORAGE" 'FileOpenReadIntent'
require_pattern "$STORAGE" 'FileReadChunkIntent'
require_pattern "$STORAGE" 'DirectoryListingIntent'
require_pattern "$STORAGE" 'StateCachePathResolution'
require_pattern "$STORAGE" 'VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok\(\)'
require_pattern "$STORAGE" 'VaachakSdFatRuntimeReadonlyOwner::ownership_ok\(\)'
require_pattern "$STORAGE" 'DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'
require_pattern "$STORAGE" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'

require_pattern "$DISPLAY" 'pub struct VaachakDisplayExecutorBridge;'
require_pattern "$DISPLAY" 'FullRefresh'
require_pattern "$DISPLAY" 'PartialRefresh'
require_pattern "$DISPLAY" 'ClearFrame'
require_pattern "$DISPLAY" 'SleepFrame'
require_pattern "$DISPLAY" 'RenderFrameMetadata'
require_pattern "$DISPLAY" 'VaachakDisplayRuntimeOwner::ownership_ok\(\)'
require_pattern "$DISPLAY" 'VaachakSpiExecutorBridge::route_transaction_intent'
require_pattern "$DISPLAY" 'DRAW_ALGORITHM_REWRITTEN: bool = false;'
require_pattern "$DISPLAY" 'FULL_REFRESH_REWRITTEN: bool = false;'
require_pattern "$DISPLAY" 'PARTIAL_REFRESH_REWRITTEN: bool = false;'

require_pattern "$INPUT" 'pub struct VaachakInputExecutorBridge;'
require_pattern "$INPUT" 'ButtonScan'
require_pattern "$INPUT" 'AdcLadderSample'
require_pattern "$INPUT" 'DebounceRepeatHandoff'
require_pattern "$INPUT" 'NavigationHandoff'
require_pattern "$INPUT" 'VaachakInputRuntimeOwner::ownership_ok\(\)'
require_pattern "$INPUT" 'ADC_SAMPLING_REWRITTEN: bool = false;'
require_pattern "$INPUT" 'BUTTON_SCAN_REWRITTEN: bool = false;'
require_pattern "$INPUT" 'DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;'
require_pattern "$INPUT" 'APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;'

require_pattern "$SMOKE" 'pub struct VaachakHardwareRuntimeExecutorSmoke;'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutor::extraction_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareExecutorPulpBackend::backend_ok\(\)'
require_pattern "$DOC" 'hardware_runtime_executor_extraction=ok'
require_pattern "$DOC" 'SPI bus runtime'
require_pattern "$DOC" 'SD probe/mount lifecycle'
require_pattern "$DOC" 'FAT/storage runtime boundary'
require_pattern "$DOC" 'Display runtime boundary'
require_pattern "$DOC" 'Input runtime boundary'

# Guard against accidentally adding active destructive/storage/display/input
# executor implementations in the new bridge files. Intent metadata is allowed;
# low-level implementations are not.
for file in "$SPI" "$STORAGE" "$DISPLAY" "$INPUT" "$EXECUTOR" "$BACKEND"; do
  require_absent_pattern "$file" 'pub\s+(const\s+)?fn\s+(write|append|delete|rename|mkdir|format|erase|draw_pixels|draw_bitmap|refresh_full|refresh_partial|scan_adc|debounce_event|toggle_chip_select|spi_transfer)\b'
done

# The broad executor overlay must not carry app, reader, or file-browser source.
if [[ -d hardware_runtime_executor_extraction/src || -d hardware_runtime_executor_extraction/vendor ]]; then
  fail 'overlay unexpectedly contains src/ or vendor/ runtime source'
fi
if [[ -d hardware_runtime_executor_extraction/target-xteink-x4/src/apps ]]; then
  fail 'overlay unexpectedly contains app source changes'
fi

printf '%s\n' 'hardware_runtime_executor_extraction=ok'
