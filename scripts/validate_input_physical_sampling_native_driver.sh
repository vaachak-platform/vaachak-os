#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "input_physical_sampling_native_driver validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local needle="$1"
  local file="$2"
  grep -Fq "$needle" "$file" || fail "missing text '$needle' in $file"
}

PHYSICAL="target-xteink-x4/src/vaachak_x4/physical/input_physical_sampling_native_driver.rs"
CONTRACT="target-xteink-x4/src/vaachak_x4/contracts/input_physical_sampling_native_driver_smoke.rs"
DOC="docs/architecture/input-physical-sampling-native-driver.md"

require_file "$PHYSICAL"
require_file "$CONTRACT"
require_file "$DOC"
require_file "target-xteink-x4/src/vaachak_x4/physical/mod.rs"
require_file "target-xteink-x4/src/vaachak_x4/contracts/mod.rs"

require_text "pub mod input_physical_sampling_native_driver;" "target-xteink-x4/src/vaachak_x4/physical/mod.rs"
require_text "pub mod input_physical_sampling_native_driver_smoke;" "target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
require_text "input_physical_sampling_native_driver=ok" "$PHYSICAL"
require_text "VaachakInputPhysicalSamplingNativeDriver" "$PHYSICAL"
require_text "VaachakPhysicalSamplingWithPulpAdcGpioReadFallback" "$PHYSICAL"
require_text "RAW_ADC_LADDER_SAMPLE_INTERPRETATION_MOVED_TO_VAACHAK" "$PHYSICAL"
require_text "OVERSAMPLE_REDUCTION_MOVED_TO_VAACHAK" "$PHYSICAL"
require_text "POWER_GPIO_LEVEL_INTERPRETATION_MOVED_TO_VAACHAK" "$PHYSICAL"
require_text "ADC_PERIPHERAL_READ_EXECUTOR_MOVED_TO_VAACHAK: bool = false" "$PHYSICAL"
require_text "GPIO_POLL_EXECUTOR_MOVED_TO_VAACHAK: bool = false" "$PHYSICAL"
require_text "PulpCompatibility" "$PHYSICAL"
require_text "classify_adc_ladder_mv" "$PHYSICAL"
require_text "reduce_oversample_window" "$PHYSICAL"
require_text "handoff_to_native_event_pipeline" "$PHYSICAL"
require_text "native_physical_sampling_driver_ready" "$PHYSICAL"
require_text "VaachakInputPhysicalSamplingNativeDriverSmoke" "$CONTRACT"
require_text "Input Physical Sampling Native Driver" "$DOC"

# Guard against accidental scope expansion in this deliverable.
if grep -Eq "SdMmc|Fat|SSD1677|draw_buffer|chip_select|spi_transfer" "$PHYSICAL"; then
  fail "input physical sampling driver must not introduce storage/display/SPI driver behavior"
fi

# Reader/file-browser/app behavior is represented only by immutable safety fields in
# this metadata module. The apply script does not patch src/apps or reader UI files.

echo "input_physical_sampling_native_driver=ok"
