#!/bin/sh
set -eu
removed=0
for path in \
  hardware_runtime_backend_takeover_bridge \
  hardware_runtime_backend_takeover_bridge.zip \
  hardware_runtime_backend_takeover_bridge_validator_fix \
  hardware_runtime_backend_takeover_bridge_validator_fix.zip \
  hardware_runtime_backend_takeover_validator_fix \
  hardware_runtime_backend_takeover_validator_fix.zip \
  hardware_runtime_backend_takeover_cleanup_validator_fix \
  hardware_runtime_backend_takeover_cleanup_validator_fix.zip \
  hardware_runtime_backend_takeover_cleanup_validator_fix2 \
  hardware_runtime_backend_takeover_cleanup_validator_fix2.zip
 do
  if [ -e "$path" ]; then
    rm -rf "$path"
    removed=$((removed + 1))
  fi
done
echo "hardware_runtime_backend_takeover_cleanup_artifacts=ok removed=$removed"
