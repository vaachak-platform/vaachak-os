#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "display_backend_native_refresh_shell validation failed: $*" >&2
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

require_absent_text() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    fail "unexpected text '$text' in $file"
  fi
}

PHYSICAL="target-xteink-x4/src/vaachak_x4/physical"
CONTRACTS="target-xteink-x4/src/vaachak_x4/contracts"
DOCS="docs/architecture"

SHELL="$PHYSICAL/display_backend_native_refresh_shell.rs"
TAKEOVER="$PHYSICAL/hardware_runtime_backend_takeover.rs"
DOC="$DOCS/display-backend-native-refresh-shell.md"
SMOKE="$CONTRACTS/display_backend_native_refresh_shell_smoke.rs"

require_file "$SHELL"
require_file "$SMOKE"
require_file "$DOC"
require_file "$TAKEOVER"

require_text "$PHYSICAL/mod.rs" "pub mod display_backend_native_refresh_shell;"
require_text "$CONTRACTS/mod.rs" "pub mod display_backend_native_refresh_shell_smoke;"

require_text "$SHELL" "VaachakDisplayBackendNativeRefreshShell"
require_text "$SHELL" "DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_MARKER"
require_text "$SHELL" "display_backend_native_refresh_shell=ok"
require_text "$SHELL" "VaachakDisplayNativeRefreshShellWithPulpExecutor"
require_text "$SHELL" "PulpCompatibility"
require_text "$SHELL" "REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK"
require_text "$SHELL" "REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK"
require_text "$SHELL" "execute_full_refresh_handoff"
require_text "$SHELL" "execute_partial_refresh_handoff"
require_text "$SHELL" "display_request_for"
require_text "$SHELL" "native_refresh_shell_ok"

python3 - <<'PY'
from pathlib import Path
src = Path('target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_shell.rs').read_text()
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
        raise SystemExit(f"display_backend_native_refresh_shell validation failed: missing stable false guard {needle}")
required_true = [
    'REFRESH_COMMAND_SHELL_OWNED_BY_VAACHAK',
    'REFRESH_INTENT_MAPPING_OWNED_BY_VAACHAK',
    'PULP_REFRESH_EXECUTOR_AVAILABLE',
]
for name in required_true:
    needle = f'pub const {name}: bool = true;'
    if needle not in src:
        raise SystemExit(f"display_backend_native_refresh_shell validation failed: missing stable true guard {needle}")
PY

require_text "$TAKEOVER" "VaachakDisplayBackendNativeRefreshShell"
require_text "$TAKEOVER" "native_refresh_shell_ok"
require_text "$TAKEOVER" "execute_display_full_refresh_handoff"
require_text "$TAKEOVER" "execute_display_partial_refresh_handoff"
require_text "$TAKEOVER" "PulpCompatibility"

require_text "$SMOKE" "VaachakDisplayBackendNativeRefreshShellSmoke"
require_text "$SMOKE" "VaachakDisplayBackendNativeRefreshShell::native_refresh_shell_ok()"
require_text "$SMOKE" "VaachakHardwareRuntimeBackendTakeover::takeover_ok()"

require_text "$DOC" "display_backend_native_refresh_shell"
require_text "$DOC" "PulpCompatibility"
require_text "$DOC" "SSD1677 draw buffer logic"
require_text "$DOC" "physical SPI transfer"

# Make sure this overlay did not introduce low-level display execution symbols into the native shell.
require_absent_text "$SHELL" "wait_busy("
require_absent_text "$SHELL" "draw_pixel"
require_absent_text "$SHELL" "draw_bitmap"
require_absent_text "$SHELL" "write_ram"
require_absent_text "$SHELL" "master_activation"
require_absent_text "$SHELL" "toggle_cs"
require_absent_text "$SHELL" "set_cs_low"
require_absent_text "$SHELL" "set_cs_high"

# App/UX paths must remain untouched by this deliverable.
if git diff --name-only -- src/apps vendor/pulp-os 2>/dev/null | grep -q .; then
  fail "unexpected changes under src/apps or vendor/pulp-os"
fi

echo "display_backend_native_refresh_shell=ok"
