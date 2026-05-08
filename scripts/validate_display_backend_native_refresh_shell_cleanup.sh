#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "display_backend_native_refresh_shell_cleanup validation failed: $*" >&2
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
  local regex="$2"
  python3 - "$file" "$regex" <<'PY'
from pathlib import Path
import re
import sys
path = Path(sys.argv[1])
regex = sys.argv[2]
text = path.read_text()
if not re.search(regex, text, re.S):
    raise SystemExit(1)
PY
}

PHYSICAL="target-xteink-x4/src/vaachak_x4/physical"
CONTRACTS="target-xteink-x4/src/vaachak_x4/contracts"
DOCS="docs/architecture"

SHELL="$PHYSICAL/display_backend_native_refresh_shell.rs"
CLEANUP="$PHYSICAL/display_backend_native_refresh_shell_cleanup.rs"
SMOKE="$CONTRACTS/display_backend_native_refresh_shell_cleanup_smoke.rs"
DOC="$DOCS/display-backend-native-refresh-shell-cleanup.md"
BASE_DOC="$DOCS/display-backend-native-refresh-shell.md"

require_file "$SHELL"
require_file "$CLEANUP"
require_file "$SMOKE"
require_file "$DOC"
require_file "$BASE_DOC"

require_text "$PHYSICAL/mod.rs" "pub mod display_backend_native_refresh_shell;"
require_text "$PHYSICAL/mod.rs" "pub mod display_backend_native_refresh_shell_cleanup;"
require_text "$CONTRACTS/mod.rs" "pub mod display_backend_native_refresh_shell_smoke;"
require_text "$CONTRACTS/mod.rs" "pub mod display_backend_native_refresh_shell_cleanup_smoke;"

require_text "$CLEANUP" "VaachakDisplayBackendNativeRefreshShellCleanup"
require_text "$CLEANUP" "DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_CLEANUP_MARKER"
require_text "$CLEANUP" "display_backend_native_refresh_shell_cleanup=ok"
require_text "$CLEANUP" "VaachakDisplayBackendNativeRefreshShell::native_refresh_shell_ok()"
require_text "$CLEANUP" "VaachakHardwareRuntimeBackendTakeover::takeover_ok()"
require_text "$CLEANUP" "VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()"
require_text "$CLEANUP" "VaachakInputBackendNativeExecutorCleanup::cleanup_ok()"
require_text "$CLEANUP" "VaachakDisplayNativeRefreshShellWithPulpExecutor"
require_text "$CLEANUP" "PulpCompatibility"
require_text "$CLEANUP" "REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK"
require_text "$CLEANUP" "REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK"
require_text "$CLEANUP" "emit_display_backend_native_refresh_shell_cleanup_marker"

require_regex "$CLEANUP" "refresh_command_shell_owned_by_vaachak.*VaachakDisplayBackendNativeRefreshShell::REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK"
require_regex "$CLEANUP" "refresh_intent_mapping_owned_by_vaachak.*VaachakDisplayBackendNativeRefreshShell::REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK"
require_regex "$CLEANUP" "selected_backend_is_vaachak_display_native_refresh_shell_with_pulp_executor.*VaachakDisplayBackendNativeRefreshShell::ACTIVE_NATIVE_BACKEND_NAME"
require_regex "$CLEANUP" "pulp_compatibility_executor_active.*VaachakDisplayBackendNativeRefreshShell::REFRESH_EXECUTOR_FALLBACK_NAME"

python3 - <<'PY'
from pathlib import Path
src = Path('target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_shell_cleanup.rs').read_text()
required_false = [
    'SSD1677_EXECUTOR_MOVED_TO_VAACHAK',
    'DRAW_BUFFER_ALGORITHM_REWRITTEN',
    'FULL_REFRESH_ALGORITHM_REWRITTEN',
    'PARTIAL_REFRESH_ALGORITHM_REWRITTEN',
    'BUSY_WAIT_ALGORITHM_REWRITTEN',
    'SPI_TRANSFER_OR_CHIP_SELECT_CHANGED',
    'STORAGE_BEHAVIOR_CHANGED',
    'INPUT_BEHAVIOR_CHANGED',
    'READER_FILE_BROWSER_UX_CHANGED',
    'APP_NAVIGATION_BEHAVIOR_CHANGED',
]
for name in required_false:
    needle = f'pub const {name}: bool = false;'
    if needle not in src:
        raise SystemExit(f"display_backend_native_refresh_shell_cleanup validation failed: missing stable false guard {needle}")
required_true = [
    'CLEANUP_ENTRYPOINT_ACTIVE',
    'OLD_OVERLAY_ARTIFACTS_SAFE_TO_REMOVE',
]
for name in required_true:
    needle = f'pub const {name}: bool = true;'
    if needle not in src:
        raise SystemExit(f"display_backend_native_refresh_shell_cleanup validation failed: missing stable true guard {needle}")
PY

require_text "$SMOKE" "VaachakDisplayBackendNativeRefreshShellCleanupSmoke"
require_text "$SMOKE" "VaachakDisplayBackendNativeRefreshShellCleanup::cleanup_ok()"
require_text "$SMOKE" "display_backend_native_refresh_shell_cleanup=ok"

require_text "$DOC" "display_backend_native_refresh_shell_cleanup=ok"
require_text "$DOC" "VaachakDisplayNativeRefreshShellWithPulpExecutor"
require_text "$DOC" "PulpCompatibility"
require_text "$DOC" "SSD1677 draw buffer logic"
require_text "$DOC" "physical SPI transfer"
require_text "$BASE_DOC" "display_backend_native_refresh_shell"
require_text "$BASE_DOC" "display_backend_native_refresh_shell_cleanup=ok"
require_text "$BASE_DOC" "PulpCompatibility"
require_text "$BASE_DOC" "SSD1677 draw buffer logic"
require_text "$BASE_DOC" "physical SPI transfer"

[ -x scripts/validate_hardware_runtime_backend_takeover_bridge.sh ] || fail "missing backend takeover bridge validator"
[ -x scripts/validate_hardware_runtime_backend_takeover_cleanup.sh ] || fail "missing backend takeover cleanup validator"
[ -x scripts/validate_input_backend_native_executor.sh ] || fail "missing input native executor validator"
[ -x scripts/validate_input_backend_native_executor_cleanup.sh ] || fail "missing input native executor cleanup validator"
[ -x scripts/validate_display_backend_native_refresh_shell.sh ] || fail "missing display native refresh shell validator"

if grep -R --line-number -E "fn[[:space:]]+(wait_busy|draw_pixel|draw_bitmap|write_ram|master_activation|toggle_cs|set_cs_low|set_cs_high|refresh_display|sample_adc|poll_buttons|debounce_button|dispatch_navigation|mount_sd|format|delete|rename|mkdir|write_file)" \
  "$CLEANUP" "$SMOKE" >/tmp/display_backend_native_refresh_shell_cleanup_forbidden.txt; then
  cat /tmp/display_backend_native_refresh_shell_cleanup_forbidden.txt >&2
  fail "cleanup introduced forbidden display/input/storage/SPI execution functions"
fi

if grep -R --line-number -E "display_draw_algorithm_rewrite_allowed:[[:space:]]*true|input_debounce_navigation_rewrite_allowed:[[:space:]]*true|destructive_operation_allowed:[[:space:]]*true" \
  "$SHELL" "$CLEANUP" >/tmp/display_backend_native_refresh_shell_cleanup_forbidden_flags.txt; then
  cat /tmp/display_backend_native_refresh_shell_cleanup_forbidden_flags.txt >&2
  fail "unsafe rewrite/destructive flags introduced"
fi

if git diff --name-only -- src/apps vendor/pulp-os 2>/dev/null | grep -q .; then
  fail "unexpected changes under src/apps or vendor/pulp-os"
fi

echo "display_backend_native_refresh_shell_cleanup=ok"
