#!/usr/bin/env bash
set -euo pipefail

removed=0
for path in \
  display_backend_native_refresh_command_executor \
  display_backend_native_refresh_command_executor.zip \
  display_backend_native_refresh_command_executor_fmt_fix \
  display_backend_native_refresh_command_executor_fmt_fix.zip; do
  if [ -e "$path" ]; then
    rm -rf "$path"
    removed=$((removed + 1))
  fi
done

if [ -f scripts/validate_display_backend_native_refresh_command_executor_fmt_fix.sh ]; then
  rm -f scripts/validate_display_backend_native_refresh_command_executor_fmt_fix.sh
  removed=$((removed + 1))
fi

# Remove accidental Python cache folders left by extracted overlays.
find . -path '*/__pycache__' -type d -prune -exec rm -rf {} + 2>/dev/null || true

echo "display_backend_native_refresh_command_executor_cleanup_artifacts=ok removed=${removed}"
