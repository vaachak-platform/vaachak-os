#!/bin/sh
set -eu
removed=0
for path in \
  hardware_runtime_executor_live_path_handoff \
  hardware_runtime_executor_live_path_handoff.zip \
  hardware_runtime_executor_live_path_handoff_validator_fix \
  hardware_runtime_executor_live_path_handoff_validator_fix.zip \
  hardware_runtime_executor_live_handoff_validator_fix \
  hardware_runtime_executor_live_handoff_validator_fix.zip \
  hardware_runtime_executor_live_handoff_cleanup_validator_fix \
  hardware_runtime_executor_live_handoff_cleanup_validator_fix.zip
 do
  if [ -e "$path" ]; then
    rm -rf "$path"
    removed=$((removed + 1))
  fi
done
echo "hardware_runtime_executor_live_handoff_cleanup_artifacts=ok removed=$removed"
