#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "input_backend_native_executor_cleanup validation failed: $*" >&2
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
CLEANUP="$PHYS/input_backend_native_executor_cleanup.rs"
SMOKE="$CONTRACTS/input_backend_native_executor_cleanup_smoke.rs"

require_file "$PHYS/input_backend_native_executor.rs"
require_file "$CLEANUP"
require_file "$CONTRACTS/input_backend_native_executor_smoke.rs"
require_file "$SMOKE"
require_file "$DOCS/input-backend-native-executor.md"
require_file "$DOCS/input-backend-native-executor-cleanup.md"
require_file "scripts/validate_input_backend_native_executor.sh"
require_file "scripts/validate_input_backend_native_executor_cleanup.sh"
require_file "scripts/cleanup_input_backend_native_executor_artifacts.sh"

require_text "$PHYS/mod.rs" "pub mod input_backend_native_executor;"
require_text "$PHYS/mod.rs" "pub mod input_backend_native_executor_cleanup;"
require_text "$CONTRACTS/mod.rs" "pub mod input_backend_native_executor_smoke;"
require_text "$CONTRACTS/mod.rs" "pub mod input_backend_native_executor_cleanup_smoke;"

require_text "$CLEANUP" "input_backend_native_executor_cleanup=ok"
require_text "$CLEANUP" "VaachakInputBackendNativeExecutor::native_executor_ok()"
require_text "$CLEANUP" "VaachakHardwareRuntimeBackendTakeover::takeover_ok()"
require_text "$CLEANUP" "VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()"
require_text "$CLEANUP" "VaachakInputNativeWithPulpSampling"
require_text "$CLEANUP" "PulpCompatibility"
require_text "$CLEANUP" "PHYSICAL_ADC_SAMPLING_CHANGED: bool = false"
require_text "$CLEANUP" "GPIO_POLLING_CHANGED: bool = false"
require_text "$CLEANUP" "DEBOUNCE_REPEAT_EXECUTION_CHANGED: bool = false"
require_text "$CLEANUP" "NAVIGATION_DISPATCH_CHANGED: bool = false"
require_text "$CLEANUP" "DISPLAY_BEHAVIOR_CHANGED: bool = false"
require_text "$CLEANUP" "STORAGE_BEHAVIOR_CHANGED: bool = false"
require_text "$CLEANUP" "SPI_BEHAVIOR_CHANGED: bool = false"
require_text "$CLEANUP" "READER_FILE_BROWSER_UX_CHANGED: bool = false"
require_text "$CLEANUP" "APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false"

require_text "$SMOKE" "VaachakInputBackendNativeExecutorCleanup::cleanup_ok()"
require_text "$DOCS/input-backend-native-executor-cleanup.md" "input_backend_native_executor_cleanup=ok"
require_text "$DOCS/input-backend-native-executor-cleanup.md" "VaachakInputNativeWithPulpSampling"
require_text "$DOCS/input-backend-native-executor-cleanup.md" "PulpCompatibility"
require_text "$DOCS/input-backend-native-executor.md" "input-backend-native-executor-cleanup.md"

require_regex "$CLEANUP" "event_normalization_owned_by_vaachak.*VaachakInputBackendNativeExecutor::EVENT_NORMALIZATION_OWNED_BY_VAACHAK"
require_regex "$CLEANUP" "intent_mapping_owned_by_vaachak.*VaachakInputBackendNativeExecutor::INTENT_MAPPING_OWNED_BY_VAACHAK"

if grep -R --line-number -E "fn[[:space:]]+(sample_adc|poll_buttons|debounce_button|dispatch_navigation|draw_pixel|refresh_display|mount_sd|format|delete|rename|mkdir|write_file)" \
  "$CLEANUP" "$SMOKE" >/tmp/input_backend_native_executor_cleanup_forbidden.txt; then
  cat /tmp/input_backend_native_executor_cleanup_forbidden.txt >&2
  fail "cleanup introduced forbidden hardware/UI/storage execution functions"
fi

if grep -R --line-number -E "display_draw_algorithm_rewrite_allowed:[[:space:]]*true|input_debounce_navigation_rewrite_allowed:[[:space:]]*true|destructive_operation_allowed:[[:space:]]*true" \
  "$PHYS/input_backend_native_executor.rs" "$CLEANUP" >/tmp/input_backend_native_executor_cleanup_forbidden_flags.txt; then
  cat /tmp/input_backend_native_executor_cleanup_forbidden_flags.txt >&2
  fail "unsafe rewrite/destructive flags introduced"
fi

[ -x scripts/validate_hardware_runtime_backend_takeover_bridge.sh ] || fail "missing backend takeover bridge validator"
[ -x scripts/validate_hardware_runtime_backend_takeover_cleanup.sh ] || fail "missing backend takeover cleanup validator"
[ -x scripts/validate_input_backend_native_executor.sh ] || fail "missing input native executor validator"

echo "input_backend_native_executor_cleanup=ok"
