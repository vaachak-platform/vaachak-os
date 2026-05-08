#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "display_backend_native_refresh_command_executor validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local text="$1"
  local file="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_absent_regex() {
  local regex="$1"
  local file="$2"
  if grep -Eq "$regex" "$file"; then
    fail "forbidden pattern '$regex' in $file"
  fi
}

require_file target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_command_executor.rs
require_file target-xteink-x4/src/vaachak_x4/contracts/display_backend_native_refresh_command_executor_smoke.rs
require_file docs/architecture/display-backend-native-refresh-command-executor.md
require_file scripts/validate_display_backend_native_refresh_command_executor.sh

require_text "pub mod display_backend_native_refresh_command_executor;" target-xteink-x4/src/vaachak_x4/physical/mod.rs
require_text "pub mod display_backend_native_refresh_command_executor_smoke;" target-xteink-x4/src/vaachak_x4/contracts/mod.rs

SRC=target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_command_executor.rs
SHELL_SRC=target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_shell.rs
TAKEOVER_SRC=target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover.rs
DOC=docs/architecture/display-backend-native-refresh-command-executor.md

require_text "VaachakDisplayBackendNativeRefreshCommandExecutor" "$SRC"
require_text "DISPLAY_BACKEND_NATIVE_REFRESH_COMMAND_EXECUTOR_MARKER" "$SRC"
require_text "display_backend_native_refresh_command_executor=ok" "$SRC"
require_text "COMMAND_SELECTION_OWNED_BY_VAACHAK: bool = true" "$SRC"
require_text "PARTIAL_REFRESH_ESCALATION_OWNED_BY_VAACHAK: bool = true" "$SRC"
require_text "DISPLAY_REQUEST_CONSTRUCTION_OWNED_BY_VAACHAK: bool = true" "$SRC"
require_text "PULP_EXECUTOR_AVAILABLE: bool = true" "$SRC"
require_text "SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK: bool = false" "$SRC"
require_text "WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK: bool = false" "$SRC"
require_text "SPI_TRANSFER_OR_CHIP_SELECT_CHANGED: bool = false" "$SRC"
require_text "VaachakDisplayRefreshCommandReason::PartialRefreshUnsafeEscalatedToFull" "$SRC"
require_text "execute_partial_refresh_or_escalate_command" "$SRC"
require_text "VaachakHardwareRuntimePulpCompatibilityBackend" "$SRC"
require_text "PulpCompatibility" "$SRC"
require_text "command_executor_ok" "$SRC"

require_text "VaachakDisplayBackendNativeRefreshCommandExecutor" "$SHELL_SRC"
require_text "display_native_refresh_command_executor_ready" "$SHELL_SRC"
require_text "execute_full_refresh_command" "$SHELL_SRC"
require_text "execute_partial_refresh_command" "$SHELL_SRC"

require_text "VaachakDisplayBackendNativeRefreshCommandExecutor" "$TAKEOVER_SRC"
require_text "display_native_refresh_command_executor_ready" "$TAKEOVER_SRC"
require_text "command_executor_ok" "$TAKEOVER_SRC"

require_text "PulpCompatibility" "$DOC"
require_text "refresh command selection" "$DOC"
require_text "partial-to-full escalation" "$DOC"
require_text "SSD1677 draw buffer algorithm" "$DOC"

# Guard against accidental low-level display implementation migration in the new Vaachak command executor.
require_absent_regex "fn[[:space:]]+(ssd1677|draw|busy_wait|wait_busy|spi_transfer|toggle_cs|set_cs|flush_framebuffer|write_command|write_data)" "$SRC"
require_absent_regex "struct[[:space:]]+(Ssd1677|SSD1677|FrameBuffer|Waveform)" "$SRC"
require_absent_regex "impl[[:space:]]+(Ssd1677|SSD1677|FrameBuffer|Waveform)" "$SRC"

# Guard against unrelated behavior changes in known app/reader surfaces.
if [ -d target-xteink-x4/src/apps ]; then
  if grep -R "display_backend_native_refresh_command_executor" target-xteink-x4/src/apps >/dev/null 2>&1; then
    fail "app UX files reference display command executor directly"
  fi
fi
if [ -d vendor/pulp-os ]; then
  if grep -R "display_backend_native_refresh_command_executor" vendor/pulp-os >/dev/null 2>&1; then
    fail "vendor/pulp-os was modified to reference display command executor"
  fi
fi

echo "display_backend_native_refresh_command_executor=ok"
