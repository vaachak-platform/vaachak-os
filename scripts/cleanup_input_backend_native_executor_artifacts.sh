#!/bin/sh
set -eu
removed=0
for path in \
  input_backend_native_executor \
  input_backend_native_executor.zip \
  input_backend_native_executor_validator_fix \
  input_backend_native_executor_validator_fix.zip \
  input_backend_native_executor_validator_fix2 \
  input_backend_native_executor_validator_fix2.zip \
  input_backend_native_executor_cleanup_validator_fix \
  input_backend_native_executor_cleanup_validator_fix.zip \
  input_backend_native_executor_cleanup_validator_fix2 \
  input_backend_native_executor_cleanup_validator_fix2.zip
 do
  if [ -e "$path" ]; then
    rm -rf "$path"
    removed=$((removed + 1))
  fi
done
echo "input_backend_native_executor_cleanup_artifacts=ok removed=$removed"
