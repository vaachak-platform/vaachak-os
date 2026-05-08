#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "input_backend_native_event_pipeline validation failed: $*" >&2
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
  grep -Eq "$pattern" "$file" || fail "missing pattern '$pattern' in $file"
}

PIPE="target-xteink-x4/src/vaachak_x4/physical/input_backend_native_event_pipeline.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/input_backend_native_event_pipeline_smoke.rs"
INPUT_EXEC="target-xteink-x4/src/vaachak_x4/physical/input_backend_native_executor.rs"
TAKEOVER="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/input-backend-native-event-pipeline.md"
INPUT_DOC="docs/architecture/input-backend-native-executor.md"

require_file "$PIPE"
require_file "$SMOKE"
require_file "$DOC"
require_file "$INPUT_DOC"

require_text "$PIPE" "pub struct VaachakInputBackendNativeEventPipeline"
require_text "$PIPE" "input_backend_native_event_pipeline=ok"
require_text "$PIPE" "VaachakNativeEventPipelineWithPulpSampling"
require_text "$PIPE" "PulpCompatibilityAdcGpioSampling"
require_text "$PIPE" "pub enum VaachakRawSampledButtonState"
require_text "$PIPE" "pub struct VaachakInputStableButtonState"
require_text "$PIPE" "pub struct VaachakInputDebounceWindow"
require_text "$PIPE" "pub enum VaachakInputEventKind"
require_text "$PIPE" "Press"
require_text "$PIPE" "Release"
require_text "$PIPE" "Repeat"
require_text "$PIPE" "pub enum VaachakInputNavigationIntent"
require_text "$PIPE" "map_button_to_navigation_intent"
require_text "$PIPE" "classify_event"
require_text "$PIPE" "generate_event"
require_text "$PIPE" "execute_scan_pipeline"
require_text "$PIPE" "execute_navigation_pipeline"
require_text "$PIPE" "RAW_SAMPLE_NORMALIZATION_OWNED_BY_VAACHAK: bool = true"
require_text "$PIPE" "STABLE_STATE_TRACKING_OWNED_BY_VAACHAK: bool = true"
require_text "$PIPE" "DEBOUNCE_WINDOW_METADATA_OWNED_BY_VAACHAK: bool = true"
require_text "$PIPE" "PRESS_RELEASE_REPEAT_CLASSIFICATION_OWNED_BY_VAACHAK: bool = true"
require_text "$PIPE" "NAVIGATION_INTENT_MAPPING_OWNED_BY_VAACHAK: bool = true"
require_text "$PIPE" "PHYSICAL_ADC_GPIO_SAMPLING_FALLBACK_ACTIVE: bool = true"
require_text "$PIPE" "PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK: bool = false"
require_text "$PIPE" "FINAL_APP_NAVIGATION_DISPATCH_CHANGED: bool = false"

require_text "$PHYSICAL_MOD" "pub mod input_backend_native_event_pipeline;"
require_text "$CONTRACTS_MOD" "pub mod input_backend_native_event_pipeline_smoke;"
require_text "$SMOKE" "VaachakInputBackendNativeEventPipelineSmoke"
require_text "$SMOKE" "VaachakInputBackendNativeEventPipeline::event_pipeline_ok()"

require_text "$INPUT_EXEC" "VaachakInputBackendNativeEventPipeline"
require_text "$INPUT_EXEC" "execute_scan_pipeline()"
require_text "$INPUT_EXEC" "execute_navigation_pipeline()"
require_text "$INPUT_EXEC" "VaachakInputBackendNativeEventPipeline::event_pipeline_ok()"

require_text "$TAKEOVER" "VaachakInputBackendNativeEventPipeline"
require_text "$TAKEOVER" "input_native_event_pipeline_ready"
require_text "$TAKEOVER" "execute_scan_pipeline()"
require_text "$TAKEOVER" "execute_navigation_pipeline()"

require_text "$DOC" "physical ADC ladder sampling"
require_text "$DOC" "PulpCompatibility"
require_text "$DOC" "press event generation"
require_text "$DOC" "release event generation"
require_text "$DOC" "repeat event generation"

# Guard against accidental migrations in the new behavior module.
if grep -Eq "ssd1677|SSD1677|draw_buffer|partial_refresh_algorithm|full_refresh_algorithm|SdMmc|format\(|delete|rename|mkdir|chip-select GPIO toggling" "$PIPE"; then
  fail "new input event pipeline contains display/storage/SPI behavior terms"
fi

# Guard against modifying app/reader/file-browser UX by referencing the new pipeline there.
if grep -R "input_backend_native_event_pipeline\|VaachakInputBackendNativeEventPipeline" \
    target-xteink-x4/src/apps 2>/dev/null; then
  fail "app UX path references native input event pipeline directly"
fi

# Existing validators should still exist when the accepted stack is present.
require_file "scripts/validate_hardware_runtime_backend_takeover_bridge.sh"
require_file "scripts/validate_hardware_runtime_backend_takeover_cleanup.sh"
require_file "scripts/validate_input_backend_native_executor.sh"
require_file "scripts/validate_input_backend_native_executor_cleanup.sh"
require_file "scripts/validate_display_backend_native_refresh_shell.sh"
require_file "scripts/validate_display_backend_native_refresh_shell_cleanup.sh"

echo "input_backend_native_event_pipeline=ok"
