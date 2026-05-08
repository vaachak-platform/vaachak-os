#!/bin/sh
set -eu
removed=0
for path in \
  display_backend_native_refresh_shell \
  display_backend_native_refresh_shell.zip \
  display_backend_native_refresh_shell_validator_fix \
  display_backend_native_refresh_shell_validator_fix.zip \
  display_backend_native_refresh_shell_validator_fix2 \
  display_backend_native_refresh_shell_validator_fix2.zip \
  display_backend_native_refresh_shell_cleanup_validator_fix \
  display_backend_native_refresh_shell_cleanup_validator_fix.zip \
  display_backend_native_refresh_shell_cleanup_validator_fix2 \
  display_backend_native_refresh_shell_cleanup_validator_fix2.zip
 do
  if [ -e "$path" ]; then
    rm -rf "$path"
    removed=$((removed + 1))
  fi
done
echo "display_backend_native_refresh_shell_cleanup_artifacts=ok removed=$removed"
