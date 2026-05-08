#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "hardware_native_behavior_consolidation validation failed: $1" >&2
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

require_regex() {
  local file="$1"
  local pattern="$2"
  python3 - "$file" "$pattern" <<'PYREGEX' || fail "missing pattern '$pattern' in $file"
import pathlib
import re
import sys
path = pathlib.Path(sys.argv[1])
pattern = sys.argv[2]
text = path.read_text()
raise SystemExit(0 if re.search(pattern, text, re.S) else 1)
PYREGEX
}

PHYS="target-xteink-x4/src/vaachak_x4/physical/hardware_native_behavior_consolidation.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/hardware_native_behavior_consolidation_smoke.rs"
DOC="docs/architecture/hardware-native-behavior-consolidation.md"

require_file "$PHYS"
require_file "$SMOKE"
require_file "$DOC"
require_file "target-xteink-x4/src/vaachak_x4/physical/mod.rs"
require_file "target-xteink-x4/src/vaachak_x4/contracts/mod.rs"

require_text "target-xteink-x4/src/vaachak_x4/physical/mod.rs" "pub mod hardware_native_behavior_consolidation;"
require_text "target-xteink-x4/src/vaachak_x4/contracts/mod.rs" "pub mod hardware_native_behavior_consolidation_smoke;"

require_text "$PHYS" "VaachakHardwareNativeBehaviorConsolidation"
require_text "$PHYS" "hardware_native_behavior_consolidation=ok"
require_text "$PHYS" "InputEventPipeline+DisplayRefreshCommandExecutor+StorageSdMmcFatExecutor"
require_text "$PHYS" "PulpCompatibility"
require_text "$PHYS" "VaachakInputBackendNativeEventPipeline::event_pipeline_ok()"
require_text "$PHYS" "VaachakDisplayBackendNativeRefreshCommandExecutor::command_executor_ok()"
require_text "$PHYS" "VaachakStorageBackendNativeSdMmcFatExecutor::native_sd_mmc_fat_executor_ok()"
require_text "$PHYS" "VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()"
require_text "$PHYS" "VaachakDisplayBackendNativeRefreshCommandExecutorCleanup::cleanup_ok()"
require_text "$PHYS" "VaachakStorageBackendNativeSdMmcFatExecutorCleanup::cleanup_ok()"

require_regex "$PHYS" "INPUT_EVENT_PIPELINE_BEHAVIOR_MOVED_TO_VAACHAK:\s*bool\s*=\s*true"
require_regex "$PHYS" "DISPLAY_REFRESH_COMMAND_BEHAVIOR_MOVED_TO_VAACHAK:\s*bool\s*=\s*true"
require_regex "$PHYS" "STORAGE_SD_MMC_FAT_COMMAND_BEHAVIOR_MOVED_TO_VAACHAK:\s*bool\s*=\s*true"
require_regex "$PHYS" "PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK:\s*bool\s*=\s*false"
require_regex "$PHYS" "SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK:\s*bool\s*=\s*false"
require_regex "$PHYS" "WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK:\s*bool\s*=\s*false"
require_regex "$PHYS" "LOW_LEVEL_SD_MMC_BLOCK_DRIVER_MOVED_TO_VAACHAK:\s*bool\s*=\s*false"
require_regex "$PHYS" "LOW_LEVEL_FAT_ALGORITHM_MOVED_TO_VAACHAK:\s*bool\s*=\s*false"
require_regex "$PHYS" "PHYSICAL_SPI_TRANSFER_CHANGED:\s*bool\s*=\s*false"
require_regex "$PHYS" "READER_FILE_BROWSER_UX_CHANGED:\s*bool\s*=\s*false"
require_regex "$PHYS" "APP_NAVIGATION_BEHAVIOR_CHANGED:\s*bool\s*=\s*false"

require_text "$SMOKE" "HardwareNativeBehaviorConsolidationSmoke"
require_text "$SMOKE" "VaachakHardwareNativeBehaviorConsolidation::report()"
require_text "$DOC" "Hardware Native Behavior Consolidation"
require_text "$DOC" "physical ADC/GPIO sampling"
require_text "$DOC" "SSD1677 draw buffer algorithm"
require_text "$DOC" "physical SD/MMC block driver"
require_text "$DOC" "low-level FAT algorithms"

# The consolidation must not touch app UX/navigation paths or vendor code.
if find hardware_native_behavior_consolidation -type f 2>/dev/null | grep -Eq '(^|/)vendor/|target-xteink-x4/src/apps/|target-xteink-x4/src/bin/'; then
  fail "overlay contains vendor/app/bin files"
fi

# Keep lower-level driver migration out of this checkpoint.
if grep -nE 'fn[[:space:]]+(read_block|write_block|spi_transfer|toggle_cs|busy_wait|draw_buffer|poll_adc|sample_adc)' "$PHYS"; then
  fail "consolidation module appears to add low-level physical driver functions"
fi

echo "hardware_native_behavior_consolidation=ok"
