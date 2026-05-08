#!/usr/bin/env bash
set -euo pipefail

fail() {
  printf '%s\n' "input_runtime_owner validation failed: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_rg() {
  local pattern="$1"
  local path="$2"
  rg -n "$pattern" "$path" >/dev/null || fail "missing pattern '$pattern' in $path"
}

OWNER="target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/input_pulp_backend.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/input_runtime_ownership_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
INPUT_BOUNDARY="target-xteink-x4/src/vaachak_x4/contracts/input.rs"
ACTIVE_MAPPER="target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs"
ADC_RUNTIME="target-xteink-x4/src/vaachak_x4/input/input_adc_runtime.rs"
DISPLAY_OWNER="target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs"
SD_FAT_OWNER="target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs"
DOC="docs/architecture/input-runtime-ownership.md"

for path in "$OWNER" "$BACKEND" "$SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$INPUT_BOUNDARY" "$ACTIVE_MAPPER" "$ADC_RUNTIME" "$DOC"; do
  require_file "$path"
done

require_rg '^pub mod input_pulp_backend;' "$PHYSICAL_MOD"
require_rg '^pub mod input_runtime_owner;' "$PHYSICAL_MOD"
require_rg '^pub mod input_runtime_ownership_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakInputRuntimeOwner' "$OWNER"
require_rg 'INPUT_RUNTIME_OWNERSHIP_MARKER' "$OWNER"
require_rg 'x4-input-runtime-owner-ok' "$OWNER"
require_rg 'INPUT_RUNTIME_IDENTITY' "$OWNER"
require_rg 'xteink-x4-button-adc-input-runtime' "$OWNER"
require_rg 'INPUT_RUNTIME_OWNERSHIP_AUTHORITY' "$OWNER"
require_rg 'target-xteink-x4 Vaachak layer' "$OWNER"
require_rg 'INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'VaachakInputRuntimeBackend::PulpCompatibility' "$OWNER"
require_rg 'ACTIVE_BACKEND' "$OWNER"
require_rg 'Self::PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'ACTIVE_BACKEND_NAME' "$OWNER"
require_rg 'VaachakInputPulpBackend::BACKEND_NAME' "$OWNER"
require_rg 'ACTIVE_EXECUTOR_OWNER' "$OWNER"
require_rg 'vendor/pulp-os imported runtime' "$OWNER"
require_rg 'CURRENT_INPUT_BOUNDARY_DEPENDENCY' "$OWNER"
require_rg 'VaachakInputBoundary' "$OWNER"
require_rg 'CURRENT_SEMANTIC_MAPPER_DEPENDENCY' "$OWNER"
require_rg 'VaachakActiveInputSemanticMapper' "$OWNER"
require_rg 'CURRENT_ADC_CLASSIFICATION_DEPENDENCY' "$OWNER"
require_rg 'VaachakInputAdcRuntimeBridge' "$OWNER"
require_rg 'CURRENT_SHELL_INPUT_BOUNDARY' "$OWNER"
require_rg 'Pulp AppManager input dispatch remains active' "$OWNER"
require_rg 'ROW1_ADC_GPIO: u8 = 1' "$OWNER"
require_rg 'ROW2_ADC_GPIO: u8 = 2' "$OWNER"
require_rg 'POWER_BUTTON_GPIO: u8 = 3' "$OWNER"
require_rg 'ROW1_RIGHT_CENTER_MV: u16 = 3' "$OWNER"
require_rg 'ROW1_LEFT_CENTER_MV: u16 = 1113' "$OWNER"
require_rg 'ROW1_CONFIRM_CENTER_MV: u16 = 1984' "$OWNER"
require_rg 'ROW1_BACK_CENTER_MV: u16 = 2556' "$OWNER"
require_rg 'ROW2_VOLDOWN_CENTER_MV: u16 = 3' "$OWNER"
require_rg 'ROW2_VOLUP_CENTER_MV: u16 = 1659' "$OWNER"
require_rg 'OVERSAMPLE_COUNT: u32 = 4' "$OWNER"
require_rg 'DEBOUNCE_WINDOW_MS: u64 = 15' "$OWNER"
require_rg 'LONG_PRESS_WINDOW_MS: u64 = 1000' "$OWNER"
require_rg 'REPEAT_INTERVAL_MS: u64 = 150' "$OWNER"
require_rg 'operation_metadata_is_safe' "$OWNER"
require_rg 'ownership_ok' "$OWNER"
require_rg 'ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$OWNER"

require_rg 'struct VaachakInputPulpBackend' "$BACKEND"
require_rg 'BACKEND_NAME' "$BACKEND"
require_rg 'PulpCompatibility' "$BACKEND"
require_rg 'ACTIVE_HARDWARE_EXECUTOR: bool = true' "$BACKEND"
require_rg 'ACTIVE_ADC_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_BUTTON_SCAN_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_DEBOUNCE_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_REPEAT_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_NAVIGATION_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_SHELL_INPUT_EXECUTOR_OWNER' "$BACKEND"
require_rg 'vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'bridge_ok' "$BACKEND"

require_rg 'struct VaachakInputRuntimeOwnershipSmoke' "$SMOKE"
require_rg 'x4-input-runtime-ownership-smoke-ok' "$SMOKE"
require_rg 'VaachakInputRuntimeOwner::ownership_ok' "$SMOKE"
require_rg 'VaachakInputPulpBackend::bridge_ok' "$SMOKE"
require_rg 'INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SMOKE"
require_rg 'PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true' "$SMOKE"
require_rg 'ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'adc_metadata_is_safe' "$SMOKE"
require_rg 'semantic_metadata_is_safe' "$SMOKE"

require_rg 'Input Runtime Ownership' "$DOC"
require_rg 'input_runtime_owner=ok' "$DOC"
require_rg 'INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true' "$DOC"
require_rg 'PulpCompatibility' "$DOC"
require_rg 'vendor/pulp-os imported runtime' "$DOC"
require_rg 'GPIO1' "$DOC"
require_rg 'GPIO2' "$DOC"
require_rg 'GPIO3' "$DOC"
require_rg 'VaachakInputBoundary' "$DOC"
require_rg 'VaachakActiveInputSemanticMapper' "$DOC"
require_rg 'VaachakInputAdcRuntimeBridge' "$DOC"
require_rg 'button scan' "$DOC"
require_rg 'debounce/repeat' "$DOC"
require_rg 'navigation dispatch' "$DOC"
require_rg 'display behavior' "$DOC"
require_rg 'storage behavior' "$DOC"
require_rg 'reader/file-browser behavior' "$DOC"

require_rg 'PHYSICAL_ADC_READS_MOVED_TO_BOUNDARY: bool = false' "$INPUT_BOUNDARY"
require_rg 'DEBOUNCE_REPEAT_HANDLING_MOVED_TO_BOUNDARY: bool = false' "$INPUT_BOUNDARY"
require_rg 'BUTTON_EVENT_ROUTING_MOVED_TO_BOUNDARY: bool = false' "$INPUT_BOUNDARY"
require_rg 'ADC_AND_DEBOUNCE_OWNER' "$ACTIVE_MAPPER"
require_rg 'vendor/pulp-os imported runtime' "$ACTIVE_MAPPER"
require_rg 'PHYSICAL_ADC_SAMPLING_OWNED_BY_BRIDGE: bool = false' "$ADC_RUNTIME"
require_rg 'DEBOUNCE_LOOP_OWNED_BY_BRIDGE: bool = false' "$ADC_RUNTIME"

if [[ -f "$DISPLAY_OWNER" ]]; then
  require_rg 'ownership_ok' "$DISPLAY_OWNER"
  require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$OWNER"
fi
if [[ -f "$SD_FAT_OWNER" ]]; then
  require_rg 'ownership_ok' "$SD_FAT_OWNER"
  require_rg 'STORAGE_BEHAVIOR_CHANGED: bool = false' "$OWNER"
fi

# The input owner/backend/smoke must remain metadata-only and must not call/import concrete input/display/storage behavior.
if rg -n '(embedded_hal::|esp_hal::|adc1|oneshot|gpio::Input|pulp_os::drivers::input|pulp_os::board::button|pulp_os::board::action|x4_kernel::drivers::input|x4_kernel::drivers::storage|x4_kernel::drivers::display|ButtonMapper::new\(|read_oneshot|read_raw|read_mv|sample_adc|poll_event|next_event)' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "input runtime owner must not import or call concrete input, display, storage, reader, or file-browser behavior"
fi

# This slice can name metadata operations, but it must not expose executor functions for input/display/storage behavior.
if rg -n '\bfn +(scan|sample|read_adc|read_button|debounce|repeat|dispatch|poll|next_event|map_event|draw|refresh|partial_refresh|full_refresh|flush|transfer|write|append|delete|remove|rename|truncate|mkdir|create|open|close|format|mount|unmount|probe)\b' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "input runtime owner exposes direct input, display, storage, reader, or file-browser behavior"
fi

# Do not edit active runtime/app/vendor/storage/display/input behavior in this overlay.
if [[ -d input_runtime_owner ]]; then
  if find input_runtime_owner -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/ui|target-xteink-x4/src/vaachak_x4/io|target-xteink-x4/src/vaachak_x4/display|target-xteink-x4/src/vaachak_x4/input/(active_semantic_mapper|input_adc_runtime|input_semantics_runtime)\.rs)/' >/dev/null; then
    fail "overlay includes vendor/app/ui/io/display/input runtime behavior files; this slice must only add input runtime ownership files"
  fi
fi

printf '%s\n' 'input_runtime_owner=ok'
