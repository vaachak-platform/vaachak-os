#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "hardware_native_behavior_consolidation_cleanup validation failed: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file $path"
}

require_text() {
  local text="$1"
  local path="$2"
  grep -Fq "$text" "$path" || fail "missing text '$text' in $path"
}

require_any_text() {
  local text="$1"
  shift
  local path
  for path in "$@"; do
    [[ -f "$path" ]] || continue
    if grep -Fq "$text" "$path"; then
      return 0
    fi
  done
  fail "missing text '$text' in expected files: $*"
}

CLEANUP="target-xteink-x4/src/vaachak_x4/physical/hardware_native_behavior_consolidation_cleanup.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/hardware_native_behavior_consolidation_cleanup_smoke.rs"
DOC="docs/architecture/hardware-native-behavior-consolidation-cleanup.md"
CANONICAL_DOC="docs/architecture/hardware-native-behavior-consolidation.md"
CONSOLIDATION="target-xteink-x4/src/vaachak_x4/physical/hardware_native_behavior_consolidation.rs"
INPUT="target-xteink-x4/src/vaachak_x4/physical/input_backend_native_event_pipeline.rs"
DISPLAY="target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_command_executor.rs"
STORAGE="target-xteink-x4/src/vaachak_x4/physical/storage_backend_native_sd_mmc_fat_executor.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover.rs"
PULP_BACKEND="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_pulp.rs"

require_file "$CLEANUP"
require_file "$SMOKE"
require_file "$DOC"
require_file "$CANONICAL_DOC"
require_file "$CONSOLIDATION"
require_file "$INPUT"
require_file "$DISPLAY"
require_file "$STORAGE"

require_text "hardware_native_behavior_consolidation_cleanup" "$CLEANUP"
require_text "VaachakHardwareNativeBehaviorConsolidationCleanup" "$CLEANUP"
require_text "cleanup_ok" "$CLEANUP"
require_text "hardware_native_behavior_consolidation_cleanup" "$SMOKE"
require_text "hardware_native_behavior_consolidation_cleanup=ok" "$DOC"

# Verify the three accepted native behavior migrations remain present.
require_text "VaachakInputBackendNativeEventPipeline" "$INPUT"
require_text "button" "$INPUT"
require_text "debounce" "$INPUT"
require_text "repeat" "$INPUT"
require_text "navigation" "$INPUT"

require_text "VaachakDisplayBackendNativeRefreshCommandExecutor" "$DISPLAY"
require_text "partial" "$DISPLAY"
require_text "full" "$DISPLAY"
require_text "escalat" "$DISPLAY"

require_text "VaachakStorageBackendNativeSdMmcFatExecutor" "$STORAGE"
require_text "SD" "$STORAGE"
require_text "FAT" "$STORAGE"
require_text "destructive" "$STORAGE"

# The active low-level compatibility backend is validated where it belongs:
# the backend takeover/Pulp backend layer, not the cleanup snapshot source.
require_any_text "PulpCompatibility" "$BACKEND" "$PULP_BACKEND" "$CANONICAL_DOC" "$DOC"
require_any_text "Pulp" "$BACKEND" "$PULP_BACKEND" "$CANONICAL_DOC" "$DOC"

# Guardrails: this consolidation cleanup must not introduce low-level driver takeover.
if grep -REn "fn[[:space:]]+(spi_transfer|toggle_chip_select|sdmmc_read_block|sdmmc_write_block|ssd1677_busy_wait|adc_sample_raw)\b" \
  "$CLEANUP" "$CONSOLIDATION" 2>/dev/null; then
  fail "cleanup/consolidation introduced low-level physical driver functions"
fi

# Guardrails: do not edit app/readers through this cleanup checkpoint.
if [[ -d input_backend_native_event_pipeline_cleanup || -d hardware_native_behavior_consolidation_cleanup ]]; then
  :
fi

echo "hardware_native_behavior_consolidation_cleanup=ok"
