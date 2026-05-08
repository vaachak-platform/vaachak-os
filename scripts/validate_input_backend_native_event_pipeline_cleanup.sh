#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "input_backend_native_event_pipeline_cleanup validation failed: $*" >&2
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

require_normalized_text() {
  local file="$1"
  local text="$2"
  python3 - "$file" "$text" <<'PY' || exit 1
from pathlib import Path
import re
import sys
path = Path(sys.argv[1])
needle = re.sub(r"\s+", " ", sys.argv[2]).strip()
haystack = re.sub(r"\s+", " ", path.read_text()).strip()
if needle not in haystack:
    print(f"missing normalized text '{sys.argv[2]}' in {path}", file=sys.stderr)
    sys.exit(1)
PY
}

PHYS="target-xteink-x4/src/vaachak_x4/physical"
CONTRACTS="target-xteink-x4/src/vaachak_x4/contracts"
PIPE="$PHYS/input_backend_native_event_pipeline.rs"
CLEANUP="$PHYS/input_backend_native_event_pipeline_cleanup.rs"
SMOKE="$CONTRACTS/input_backend_native_event_pipeline_cleanup_smoke.rs"
INPUT_EXEC="$PHYS/input_backend_native_executor.rs"
TAKEOVER="$PHYS/hardware_runtime_backend_takeover.rs"
PHYSICAL_MOD="$PHYS/mod.rs"
CONTRACTS_MOD="$CONTRACTS/mod.rs"
DOC="docs/architecture/input-backend-native-event-pipeline-cleanup.md"
PIPE_DOC="docs/architecture/input-backend-native-event-pipeline.md"

require_file "$PIPE"
require_file "$CLEANUP"
require_file "$SMOKE"
require_file "$DOC"
require_file "$PIPE_DOC"

require_text "$PHYSICAL_MOD" "pub mod input_backend_native_event_pipeline;"
require_text "$PHYSICAL_MOD" "pub mod input_backend_native_event_pipeline_cleanup;"
require_text "$CONTRACTS_MOD" "pub mod input_backend_native_event_pipeline_smoke;"
require_text "$CONTRACTS_MOD" "pub mod input_backend_native_event_pipeline_cleanup_smoke;"

require_text "$CLEANUP" "pub struct VaachakInputBackendNativeEventPipelineCleanup"
require_text "$CLEANUP" "INPUT_BACKEND_NATIVE_EVENT_PIPELINE_CLEANUP_MARKER"
require_text "$CLEANUP" "input_backend_native_event_pipeline_cleanup=ok"
require_text "$CLEANUP" "VaachakNativeEventPipelineWithPulpSampling"
require_text "$CLEANUP" "PulpCompatibility"
require_text "$CLEANUP" "VaachakInputBackendNativeEventPipeline::event_pipeline_ok()"
require_text "$CLEANUP" "VaachakInputBackendNativeExecutorCleanup::cleanup_ok()"
require_text "$CLEANUP" "VaachakDisplayBackendNativeRefreshShellCleanup::cleanup_ok()"
require_text "$CLEANUP" "RAW_SAMPLE_NORMALIZATION_OWNED_BY_VAACHAK"
require_text "$CLEANUP" "STABLE_STATE_TRACKING_OWNED_BY_VAACHAK"
require_text "$CLEANUP" "DEBOUNCE_WINDOW_METADATA_OWNED_BY_VAACHAK"
require_text "$CLEANUP" "PRESS_RELEASE_REPEAT_CLASSIFICATION_OWNED_BY_VAACHAK"
require_text "$CLEANUP" "NAVIGATION_INTENT_MAPPING_OWNED_BY_VAACHAK"
require_text "$CLEANUP" "PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK"
require_text "$CLEANUP" "FINAL_APP_NAVIGATION_DISPATCH_CHANGED"

require_text "$SMOKE" "VaachakInputBackendNativeEventPipelineCleanupSmoke"
require_text "$SMOKE" "VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()"
require_text "$SMOKE" "input_backend_native_event_pipeline_cleanup=ok"

require_text "$PIPE" "VaachakInputBackendNativeEventPipeline"
require_text "$PIPE" "classify_event"
require_text "$PIPE" "generate_event"
require_text "$PIPE" "map_button_to_navigation_intent"
require_text "$PIPE" "execute_scan_pipeline"
require_text "$PIPE" "execute_navigation_pipeline"

require_text "$INPUT_EXEC" "VaachakInputBackendNativeEventPipeline"
require_text "$INPUT_EXEC" "execute_scan_pipeline()"
require_text "$INPUT_EXEC" "execute_navigation_pipeline()"
require_text "$INPUT_EXEC" "VaachakInputBackendNativeEventPipeline::event_pipeline_ok()"

require_text "$TAKEOVER" "VaachakInputBackendNativeEventPipeline"
require_text "$TAKEOVER" "input_native_event_pipeline_ready"
require_text "$TAKEOVER" "execute_scan_pipeline()"
require_text "$TAKEOVER" "execute_navigation_pipeline()"

require_text "$DOC" "input_backend_native_event_pipeline_cleanup=ok"
require_text "$DOC" "VaachakNativeEventPipelineWithPulpSampling"
require_text "$DOC" "PulpCompatibility"
require_text "$DOC" "raw sampled button state normalization"
require_text "$DOC" "press event generation"
require_text "$DOC" "repeat event generation"
require_text "$PIPE_DOC" "input_backend_native_event_pipeline_cleanup=ok"

require_normalized_text "$CLEANUP" "physical_adc_gpio_sampling_moved_to_vaachak: VaachakInputBackendNativeEventPipeline::PHYSICAL_ADC_GPIO_SAMPLING_MOVED_TO_VAACHAK"
require_normalized_text "$CLEANUP" "final_app_navigation_dispatch_changed: VaachakInputBackendNativeEventPipeline::FINAL_APP_NAVIGATION_DISPATCH_CHANGED"

if grep -R --line-number -E "fn[[:space:]]+(sample_adc|poll_gpio|poll_buttons|read_adc|dispatch_navigation|draw_pixel|refresh_display|mount_sd|format|delete|rename|mkdir|write_file)|SSD1677|SdMmc|FatFs" \
  "$CLEANUP" "$SMOKE" >/tmp/input_backend_native_event_pipeline_cleanup_forbidden.txt; then
  cat /tmp/input_backend_native_event_pipeline_cleanup_forbidden.txt >&2
  fail "cleanup introduced forbidden hardware/display/storage/UI execution functions"
fi

if grep -R --line-number -E "input_backend_native_event_pipeline|VaachakInputBackendNativeEventPipeline" \
    target-xteink-x4/src/apps 2>/tmp/input_backend_native_event_pipeline_cleanup_app_rg.err; then
  fail "app UX path references native input event pipeline directly"
fi

require_file "scripts/validate_hardware_runtime_backend_takeover_bridge.sh"
require_file "scripts/validate_hardware_runtime_backend_takeover_cleanup.sh"
require_file "scripts/validate_input_backend_native_executor.sh"
require_file "scripts/validate_input_backend_native_executor_cleanup.sh"
require_file "scripts/validate_display_backend_native_refresh_shell.sh"
require_file "scripts/validate_display_backend_native_refresh_shell_cleanup.sh"
require_file "scripts/validate_input_backend_native_event_pipeline.sh"

echo "input_backend_native_event_pipeline_cleanup=ok"
