#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "display_backend_native_refresh_command_executor_cleanup validation failed: $*" >&2
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

require_regex() {
  local regex="$1"
  local file="$2"
  grep -Eq "$regex" "$file" || fail "missing pattern '$regex' in $file"
}

require_absent_regex() {
  local regex="$1"
  local file="$2"
  if grep -Eq "$regex" "$file"; then
    fail "forbidden pattern '$regex' in $file"
  fi
}

SRC=target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_command_executor_cleanup.rs
COMMAND_SRC=target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_command_executor.rs
SHELL_SRC=target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_shell.rs
DOC=docs/architecture/display-backend-native-refresh-command-executor-cleanup.md
COMMAND_DOC=docs/architecture/display-backend-native-refresh-command-executor.md

require_file "$SRC"
require_file target-xteink-x4/src/vaachak_x4/contracts/display_backend_native_refresh_command_executor_cleanup_smoke.rs
require_file "$DOC"
require_file "$COMMAND_DOC"
require_file scripts/validate_display_backend_native_refresh_command_executor_cleanup.sh

require_text "pub mod display_backend_native_refresh_command_executor_cleanup;" target-xteink-x4/src/vaachak_x4/physical/mod.rs
require_text "pub mod display_backend_native_refresh_command_executor_cleanup_smoke;" target-xteink-x4/src/vaachak_x4/contracts/mod.rs

require_text "VaachakDisplayBackendNativeRefreshCommandExecutorCleanup" "$SRC"
require_text "DISPLAY_BACKEND_NATIVE_REFRESH_COMMAND_EXECUTOR_CLEANUP_MARKER" "$SRC"
require_text "display_backend_native_refresh_command_executor_cleanup=ok" "$SRC"
require_text "VaachakDisplayBackendNativeRefreshCommandExecutor::command_executor_ok()" "$SRC"
require_text "VaachakDisplayBackendNativeRefreshShellCleanup::cleanup_ok()" "$SRC"
require_text "VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()" "$SRC"
require_text "VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()" "$SRC"
require_text "RUSTFMT_INNER_ATTRIBUTE_REPAIR_FOLDED: bool = true" "$SRC"
require_text "ACTIVE_NATIVE_BACKEND_NAME" "$SRC"
require_text "VaachakDisplayRefreshCommandExecutorWithPulpExecutor" "$SRC"
require_text "PulpCompatibility" "$SRC"
require_text "SSD1677_DRAW_ALGORITHM_MOVED_TO_VAACHAK: bool = false" "$SRC"
require_text "WAVEFORM_OR_BUSY_WAIT_MOVED_TO_VAACHAK: bool = false" "$SRC"
require_text "SPI_TRANSFER_OR_CHIP_SELECT_CHANGED: bool = false" "$SRC"
require_text "STORAGE_BEHAVIOR_CHANGED: bool = false" "$SRC"
require_text "INPUT_BEHAVIOR_CHANGED: bool = false" "$SRC"
require_text "READER_FILE_BROWSER_UX_CHANGED: bool = false" "$SRC"
require_text "APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false" "$SRC"
require_text "cleanup_ok" "$SRC"

require_text "display_backend_native_refresh_command_executor_cleanup=ok" "$DOC"
require_text "PulpCompatibility" "$DOC"
require_text "rustfmt repair" "$DOC"
require_text "SSD1677 draw buffer logic" "$DOC"
require_text "display-backend-native-refresh-command-executor-cleanup.md" "$COMMAND_DOC"

# Folded fmt fix: the shell module must not retain the rustfmt-blocking inner attribute.
if grep -Fxq '#![allow(dead_code)]' "$SHELL_SRC"; then
  fail "display refresh shell still contains rustfmt-blocking inner dead_code attribute"
fi

require_text "VaachakDisplayBackendNativeRefreshCommandExecutor" "$COMMAND_SRC"
require_text "command_executor_ok" "$COMMAND_SRC"
require_text "PulpCompatibility" "$COMMAND_SRC"

# Guard against accidental low-level display implementation migration in the cleanup layer.
require_absent_regex "fn[[:space:]]+(ssd1677|draw|busy_wait|wait_busy|spi_transfer|toggle_cs|set_cs|flush_framebuffer|write_command|write_data)" "$SRC"
require_absent_regex "struct[[:space:]]+(Ssd1677|SSD1677|FrameBuffer|Waveform)" "$SRC"
require_absent_regex "impl[[:space:]]+(Ssd1677|SSD1677|FrameBuffer|Waveform)" "$SRC"

# Guard against unrelated behavior changes in known app/reader/vendor surfaces.
if [ -d target-xteink-x4/src/apps ]; then
  if grep -R "display_backend_native_refresh_command_executor_cleanup" target-xteink-x4/src/apps >/dev/null 2>&1; then
    fail "app UX files reference display command executor cleanup directly"
  fi
fi
if [ -d vendor/pulp-os ]; then
  if grep -R "display_backend_native_refresh_command_executor_cleanup" vendor/pulp-os >/dev/null 2>&1; then
    fail "vendor/pulp-os references display command executor cleanup"
  fi
fi

# Temporary fmt-fix validator should be folded away by the cleanup.
if [ -f scripts/validate_display_backend_native_refresh_command_executor_fmt_fix.sh ]; then
  fail "temporary display command executor fmt-fix validator still present"
fi

echo "display_backend_native_refresh_command_executor_cleanup=ok"
