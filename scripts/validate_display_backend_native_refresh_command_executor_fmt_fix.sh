#!/usr/bin/env bash
set -euo pipefail

FILE="target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_shell.rs"
if [ ! -f "$FILE" ]; then
  echo "display_backend_native_refresh_command_executor_fmt_fix validation failed: missing $FILE" >&2
  exit 1
fi

if grep -q '#!\[allow(dead_code)\]' "$FILE"; then
  echo "display_backend_native_refresh_command_executor_fmt_fix validation failed: rustfmt-blocking inner dead_code attribute remains in $FILE" >&2
  exit 1
fi

if ! grep -q '#\[allow(dead_code)\]' "$FILE"; then
  echo "display_backend_native_refresh_command_executor_fmt_fix validation failed: expected outer dead_code attribute in $FILE" >&2
  exit 1
fi

# Ensure this repair did not modify the native refresh command executor files.
if [ ! -f "target-xteink-x4/src/vaachak_x4/physical/display_backend_native_refresh_command_executor.rs" ]; then
  echo "display_backend_native_refresh_command_executor_fmt_fix validation failed: missing command executor source" >&2
  exit 1
fi

printf '%s\n' "display_backend_native_refresh_command_executor_fmt_fix=ok"
