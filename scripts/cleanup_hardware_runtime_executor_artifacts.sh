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
  storage_readonly_adapter_facade
  storage_readonly_pulp_bridge
  storage_readonly_boundary_consolidation
  spi_bus_runtime_contract_consolidation
  storage_probe_mount_contract
  spi_bus_runtime_ownership_bridge
  storage_probe_mount_runtime_owner
  storage_probe_mount_runtime_owner_validator_fix
  sd_fat_runtime_readonly_owner
  display_runtime_owner
  input_runtime_owner
  hardware_runtime_ownership_consolidation
  spi_bus_arbitration_runtime_owner
  storage_probe_mount_runtime_executor_bridge
  hardware_runtime_executor_extraction
  hardware_runtime_executor_wiring
  hardware_runtime_executor_observability
  hardware_runtime_executor_observability_validator_fix
  hardware_runtime_executor_observability_validator_fix2
  hardware_runtime_executor_boot_markers
  hardware_runtime_executor_acceptance_cleanup
)

removed_count=0
for artifact in "${artifacts[@]}"; do
  if [[ "$artifact" == "hardware_runtime_executor_acceptance_cleanup" && "$INCLUDE_CURRENT" != "true" ]]; then
    continue
  fi
  for path in "$artifact" "$artifact.zip"; do
    if [[ -e "$path" ]]; then
      rm -rf -- "$path"
      removed_count=$((removed_count + 1))
    fi
  done
done

printf 'hardware_runtime_executor_cleanup_artifacts=ok removed=%s\n' "$removed_count"
