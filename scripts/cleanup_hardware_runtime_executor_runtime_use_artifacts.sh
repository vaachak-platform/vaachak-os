#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
INCLUDE_CURRENT="false"
if [[ "${1:-}" == "--include-current" ]]; then
  REPO_ROOT="$(pwd)"
  INCLUDE_CURRENT="true"
elif [[ "${2:-}" == "--include-current" ]]; then
  INCLUDE_CURRENT="true"
fi

cd "$REPO_ROOT"

artifacts=(
  hardware_runtime_executor_runtime_use
  hardware_runtime_executor_runtime_use_validator_fix
  hardware_runtime_executor_runtime_use_cleanup
)

removed_count=0
for artifact in "${artifacts[@]}"; do
  if [[ "$artifact" == "hardware_runtime_executor_runtime_use_cleanup" && "$INCLUDE_CURRENT" != "true" ]]; then
    continue
  fi
  for path in "$artifact" "$artifact.zip"; do
    if [[ -e "$path" ]]; then
      rm -rf -- "$path"
      removed_count=$((removed_count + 1))
    fi
  done
done

printf 'hardware_runtime_executor_runtime_use_cleanup_artifacts=ok removed=%s\n' "$removed_count"
