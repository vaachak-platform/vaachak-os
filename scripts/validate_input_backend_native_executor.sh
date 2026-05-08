#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "input_backend_native_executor validation failed: $*" >&2
  exit 1
}

require_file() {
  [[ -f "$1" ]] || fail "missing file $1"
}

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_regex() {
  local file="$1"
  local pattern="$2"
  python3 - "$file" "$pattern" <<'PY' || exit 1
from pathlib import Path
import re, sys
path = Path(sys.argv[1])
pattern = sys.argv[2]
text = path.read_text()
if not re.search(pattern, text, flags=re.S):
    print(f"missing pattern {pattern!r} in {path}", file=sys.stderr)
    sys.exit(1)
PY
}

PHYS="target-xteink-x4/src/vaachak_x4/physical"
CONTRACTS="target-xteink-x4/src/vaachak_x4/contracts"
DOCS="docs/architecture"

require_file "$PHYS/input_backend_native_executor.rs"
require_file "$CONTRACTS/input_backend_native_executor_smoke.rs"
require_file "$DOCS/input-backend-native-executor.md"
require_file "scripts/validate_input_backend_native_executor.sh"

require_text "$PHYS/mod.rs" "pub mod input_backend_native_executor;"
require_text "$CONTRACTS/mod.rs" "pub mod input_backend_native_executor_smoke;"

require_text "$PHYS/input_backend_native_executor.rs" "input_backend_native_executor=ok"
require_text "$PHYS/input_backend_native_executor.rs" "VaachakInputNativeWithPulpSampling"
require_text "$PHYS/input_backend_native_executor.rs" "PulpCompatibility"
require_text "$PHYS/input_backend_native_executor.rs" "EVENT_NORMALIZATION_OWNED_BY_VAACHAK: bool = true"
require_text "$PHYS/input_backend_native_executor.rs" "INTENT_MAPPING_OWNED_BY_VAACHAK: bool = true"
require_text "$PHYS/input_backend_native_executor.rs" "PULP_SAMPLING_FALLBACK_AVAILABLE: bool = true"
require_text "$PHYS/input_backend_native_executor.rs" "PHYSICAL_ADC_SAMPLING_MOVED_TO_VAACHAK: bool = false"
require_text "$PHYS/input_backend_native_executor.rs" "DEBOUNCE_NAVIGATION_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYS/input_backend_native_executor.rs" "APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYS/input_backend_native_executor.rs" "DISPLAY_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYS/input_backend_native_executor.rs" "STORAGE_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYS/input_backend_native_executor.rs" "SPI_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYS/input_backend_native_executor.rs" "READER_FILE_BROWSER_UX_CHANGED: bool = false"

require_regex "$PHYS/input_backend_native_executor.rs" "Right.*=>.*RightPressed"
require_regex "$PHYS/input_backend_native_executor.rs" "RightPressed.*=>.*MoveNext"
require_regex "$PHYS/input_backend_native_executor.rs" "LeftPressed.*=>.*MovePrevious"
require_regex "$PHYS/input_backend_native_executor.rs" "ConfirmPressed.*=>.*Select"
require_regex "$PHYS/input_backend_native_executor.rs" "BackPressed.*=>.*Back"
require_regex "$PHYS/input_backend_native_executor.rs" "VolDownPressed.*=>.*VolumeDown"
require_regex "$PHYS/input_backend_native_executor.rs" "VolUpPressed.*=>.*VolumeUp"
require_regex "$PHYS/input_backend_native_executor.rs" "PowerPressed.*=>.*PowerHandoff"

require_text "$PHYS/hardware_runtime_backend_takeover.rs" "VaachakInputBackendNativeExecutor"
require_text "$PHYS/hardware_runtime_backend_takeover.rs" "VaachakInputBackendNativeExecutor::execute_scan_handoff()"
require_text "$PHYS/hardware_runtime_backend_takeover.rs" "VaachakInputBackendNativeExecutor::execute_navigation_handoff()"
require_text "$PHYS/hardware_runtime_backend_takeover.rs" "VaachakHardwareRuntimePulpCompatibilityBackend"

require_text "$CONTRACTS/input_backend_native_executor_smoke.rs" "VaachakInputBackendNativeExecutor::native_executor_ok()"
require_text "$DOCS/input-backend-native-executor.md" "VaachakInputNativeWithPulpSampling"
require_text "$DOCS/input-backend-native-executor.md" "PulpCompatibility"
require_text "$DOCS/input-backend-native-executor.md" "physical ADC ladder sampling"
require_text "$DOCS/hardware-runtime-backend-takeover.md" "VaachakInputBackendNativeExecutor"

if grep -R --line-number -E "fn[[:space:]]+(sample_adc|poll_buttons|debounce|dispatch_navigation|draw|refresh|mount|format|delete|rename|mkdir|write_file)" \
  "$PHYS/input_backend_native_executor.rs" >/tmp/input_backend_native_executor_forbidden.txt; then
  cat /tmp/input_backend_native_executor_forbidden.txt >&2
  fail "native input executor introduced forbidden hardware/UI/storage execution functions"
fi

if grep -R --line-number -E "display_draw_algorithm_rewrite_allowed:[[:space:]]*true|input_debounce_navigation_rewrite_allowed:[[:space:]]*true|destructive_operation_allowed:[[:space:]]*true" \
  "$PHYS/hardware_runtime_backend_takeover.rs" "$PHYS/input_backend_native_executor.rs" >/tmp/input_backend_native_executor_forbidden_flags.txt; then
  cat /tmp/input_backend_native_executor_forbidden_flags.txt >&2
  fail "unsafe rewrite/destructive flags introduced"
fi

echo "input_backend_native_executor=ok"
