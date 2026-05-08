#!/usr/bin/env bash
set -euo pipefail

removed=0
for path in \
  input_physical_sampling_native_driver \
  input_physical_sampling_native_driver.zip \
  input_physical_sampling_native_driver_takeover_fix \
  input_physical_sampling_native_driver_takeover_fix.zip \
  input_physical_sampling_native_driver_fmt_fix \
  input_physical_sampling_native_driver_fmt_fix.zip \
  input_physical_sampling_native_driver_validator_fix \
  input_physical_sampling_native_driver_validator_fix.zip; do
  if [ -e "$path" ]; then
    rm -rf "$path"
    removed=$((removed + 1))
  fi
done

# Remove accidental Python bytecode emitted by overlay helper execution, if any.
find input_physical_sampling_native_driver_cleanup -type d -name __pycache__ -prune -exec rm -rf {} + 2>/dev/null || true

echo "input_physical_sampling_native_driver_cleanup_artifacts=ok removed=$removed"
