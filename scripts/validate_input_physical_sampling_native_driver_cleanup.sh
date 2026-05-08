#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "input_physical_sampling_native_driver_cleanup validation failed: $*" >&2
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

PHYS="target-xteink-x4/src/vaachak_x4/physical"
CONTRACTS="target-xteink-x4/src/vaachak_x4/contracts"
DRIVER="$PHYS/input_physical_sampling_native_driver.rs"
CLEANUP="$PHYS/input_physical_sampling_native_driver_cleanup.rs"
SMOKE="$CONTRACTS/input_physical_sampling_native_driver_cleanup_smoke.rs"
PHYSICAL_MOD="$PHYS/mod.rs"
CONTRACTS_MOD="$CONTRACTS/mod.rs"
DOC="docs/architecture/input-physical-sampling-native-driver-cleanup.md"
DRIVER_DOC="docs/architecture/input-physical-sampling-native-driver.md"

require_file "$DRIVER"
require_file "$CLEANUP"
require_file "$SMOKE"
require_file "$DOC"
require_file "$DRIVER_DOC"
require_file "$PHYSICAL_MOD"
require_file "$CONTRACTS_MOD"

require_text "$PHYSICAL_MOD" "pub mod input_physical_sampling_native_driver;"
require_text "$PHYSICAL_MOD" "pub mod input_physical_sampling_native_driver_cleanup;"
require_text "$CONTRACTS_MOD" "pub mod input_physical_sampling_native_driver_smoke;"
require_text "$CONTRACTS_MOD" "pub mod input_physical_sampling_native_driver_cleanup_smoke;"

require_text "$CLEANUP" "pub struct VaachakInputPhysicalSamplingNativeDriverCleanup"
require_text "$CLEANUP" "INPUT_PHYSICAL_SAMPLING_NATIVE_DRIVER_CLEANUP_MARKER"
require_text "$CLEANUP" "input_physical_sampling_native_driver_cleanup=ok"
require_text "$CLEANUP" "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback"
require_text "$CLEANUP" "PulpCompatibility"
require_text "$CLEANUP" "VaachakInputPhysicalSamplingNativeDriver::native_physical_sampling_ok()"
require_text "$CLEANUP" "VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()"
require_text "$CLEANUP" "VaachakPhysicalDriverMigrationPlan::migration_plan_ok()"
require_text "$CLEANUP" "raw_adc_ladder_sample_interpretation_moved_to_vaachak"
require_text "$CLEANUP" "oversample_reduction_moved_to_vaachak"
require_text "$CLEANUP" "power_gpio_level_interpretation_moved_to_vaachak"
require_text "$CLEANUP" "adc_peripheral_read_executor_moved_to_vaachak"
require_text "$CLEANUP" "gpio_poll_executor_moved_to_vaachak"
require_text "$CLEANUP" "FINAL_APP_NAVIGATION_DISPATCH_CHANGED"

require_text "$SMOKE" "VaachakInputPhysicalSamplingNativeDriverCleanupSmoke"
require_text "$SMOKE" "VaachakInputPhysicalSamplingNativeDriverCleanup::cleanup_ok()"
require_text "$SMOKE" "input_physical_sampling_native_driver_cleanup=ok"

require_text "$DRIVER" "VaachakInputPhysicalSamplingNativeDriver"
require_text "$DRIVER" "RAW_ADC_LADDER_SAMPLE_INTERPRETATION_MOVED_TO_VAACHAK"
require_text "$DRIVER" "OVERSAMPLE_REDUCTION_MOVED_TO_VAACHAK"
require_text "$DRIVER" "POWER_GPIO_LEVEL_INTERPRETATION_MOVED_TO_VAACHAK"
require_text "$DRIVER" "ADC_PERIPHERAL_READ_EXECUTOR_MOVED_TO_VAACHAK: bool = false"
require_text "$DRIVER" "GPIO_POLL_EXECUTOR_MOVED_TO_VAACHAK: bool = false"
require_text "$DRIVER" "handoff_to_native_event_pipeline"

require_text "$DOC" "input_physical_sampling_native_driver_cleanup=ok"
require_text "$DOC" "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback"
require_text "$DOC" "PulpCompatibility"
require_text "$DOC" "X4 ADC ladder sample interpretation"
require_text "$DOC" "4-sample oversample reduction"
require_text "$DRIVER_DOC" "input_physical_sampling_native_driver_cleanup=ok"

# Guard against accidental scope expansion in the cleanup checkpoint.
if grep -R --line-number -E "fn[[:space:]]+(sample_adc|poll_gpio|poll_buttons|read_adc|dispatch_navigation|draw_pixel|refresh_display|mount_sd|format|delete|rename|mkdir|write_file)|SSD1677|SdMmc|FatFs" \
  "$CLEANUP" "$SMOKE" >/tmp/input_physical_sampling_native_driver_cleanup_forbidden.txt; then
  cat /tmp/input_physical_sampling_native_driver_cleanup_forbidden.txt >&2
  fail "cleanup introduced forbidden hardware/display/storage/UI execution functions"
fi

if grep -R --line-number -E "input_physical_sampling_native_driver|VaachakInputPhysicalSamplingNativeDriver" \
    target-xteink-x4/src/apps 2>/tmp/input_physical_sampling_native_driver_cleanup_app_rg.err; then
  fail "app UX path references physical input sampling driver directly"
fi

require_file "scripts/validate_input_physical_sampling_native_driver.sh"
require_file "scripts/validate_physical_driver_migration_plan.sh"
require_file "scripts/cleanup_input_physical_sampling_native_driver_artifacts.sh"

echo "input_physical_sampling_native_driver_cleanup=ok"
