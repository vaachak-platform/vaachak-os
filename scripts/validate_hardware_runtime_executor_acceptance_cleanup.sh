#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  printf 'hardware_runtime_executor_acceptance_cleanup validation failed: %s\n' "$1" >&2
  exit 1
}

require_file() {
  local file="$1"
  [[ -f "$file" ]] || fail "missing file $file"
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  require_file "$file"
  rg -n --pcre2 "$pattern" "$file" >/dev/null || fail "missing pattern '$pattern' in $file"
}

require_absent_pattern() {
  local file="$1"
  local pattern="$2"
  require_file "$file"
  if rg -n --pcre2 "$pattern" "$file" >/dev/null; then
    fail "forbidden pattern '$pattern' found in $file"
  fi
}

PHYSICAL_MOD='target-xteink-x4/src/vaachak_x4/physical/mod.rs'
CONTRACTS_MOD='target-xteink-x4/src/vaachak_x4/contracts/mod.rs'
ACCEPTANCE='target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_acceptance.rs'
SMOKE='target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_acceptance_smoke.rs'
DOC='docs/architecture/hardware-runtime-executor-acceptance.md'
CLEANUP='scripts/cleanup_hardware_runtime_executor_artifacts.sh'
VALIDATOR='scripts/validate_hardware_runtime_executor_acceptance_cleanup.sh'
BOOT='target-xteink-x4/src/vaachak_x4/boot.rs'
IMPORTED_RUNTIME='target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs'

required_stack_files=(
  'target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_ownership.rs'
  'target-xteink-x4/src/vaachak_x4/physical/spi_bus_arbitration_runtime_owner.rs'
  'target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_executor_bridge.rs'
  'target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor.rs'
  'target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_wiring.rs'
  'target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_observability.rs'
  'target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_boot_markers.rs'
  'target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_ownership_smoke.rs'
  'target-xteink-x4/src/vaachak_x4/contracts/spi_bus_arbitration_runtime_ownership_smoke.rs'
  'target-xteink-x4/src/vaachak_x4/contracts/storage_probe_mount_runtime_executor_bridge_smoke.rs'
  'target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_smoke.rs'
  'target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_wiring_smoke.rs'
  'target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_observability_smoke.rs'
  'target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_boot_markers_smoke.rs'
  'docs/architecture/hardware-runtime-ownership.md'
  'docs/architecture/spi-bus-arbitration-runtime-ownership.md'
  'docs/architecture/storage-probe-mount-runtime-executor-bridge.md'
  'docs/architecture/hardware-runtime-executor-extraction.md'
  'docs/architecture/hardware-runtime-executor-wiring.md'
  'docs/architecture/hardware-runtime-executor-observability.md'
  'docs/architecture/hardware-runtime-executor-boot-markers.md'
)

for f in "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$ACCEPTANCE" "$SMOKE" "$DOC" "$CLEANUP" "$VALIDATOR" "$BOOT" "$IMPORTED_RUNTIME"; do
  require_file "$f"
done
for f in "${required_stack_files[@]}"; do
  require_file "$f"
done

require_pattern "$PHYSICAL_MOD" '^pub mod hardware_runtime_executor_acceptance;'
require_pattern "$CONTRACTS_MOD" '^pub mod hardware_runtime_executor_acceptance_smoke;'

require_pattern "$ACCEPTANCE" 'pub struct VaachakHardwareRuntimeExecutorAcceptance;'
require_pattern "$ACCEPTANCE" 'hardware_runtime_executor_acceptance_cleanup=ok'
require_pattern "$ACCEPTANCE" 'target-xteink-x4 Vaachak layer'
require_pattern "$ACCEPTANCE" 'REQUIRED_ACCEPTED_LAYER_COUNT: usize = 7;'
require_pattern "$ACCEPTANCE" 'REQUIRED_BOOT_MARKER_COUNT: usize = 8;'
require_pattern "$ACCEPTANCE" 'PULP_COMPATIBLE_BACKEND_ACTIVE: bool = true;'
require_pattern "$ACCEPTANCE" 'READER_FILE_BROWSER_UX_CHANGED: bool = false;'
require_pattern "$ACCEPTANCE" 'APP_NAVIGATION_CHANGED: bool = false;'
require_pattern "$ACCEPTANCE" 'DISPLAY_ALGORITHM_REWRITTEN: bool = false;'
require_pattern "$ACCEPTANCE" 'INPUT_ALGORITHM_REWRITTEN: bool = false;'
require_pattern "$ACCEPTANCE" 'STORAGE_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;'
require_pattern "$ACCEPTANCE" 'VaachakHardwareRuntimeOwnership::consolidation_ok\(\)'
require_pattern "$ACCEPTANCE" 'VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok\(\)'
require_pattern "$ACCEPTANCE" 'VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok\(\)'
require_pattern "$ACCEPTANCE" 'VaachakHardwareRuntimeExecutor::extraction_ok\(\)'
require_pattern "$ACCEPTANCE" 'VaachakHardwareRuntimeExecutorWiring::wiring_ok\(\)'
require_pattern "$ACCEPTANCE" 'VaachakHardwareRuntimeExecutorObservability::observability_ok\(\)'
require_pattern "$ACCEPTANCE" 'VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok\(\)'
require_pattern "$ACCEPTANCE" 'VaachakHardwareRuntimeExecutorObservability::BOOT_MARKER_COUNT'
require_pattern "$ACCEPTANCE" 'VaachakHardwareRuntimeExecutorBootMarkers::BOOT_MARKER_COUNT'
require_pattern "$ACCEPTANCE" 'accepted_layer_markers'
require_pattern "$ACCEPTANCE" 'acceptance_ok'

require_pattern "$SMOKE" 'pub struct VaachakHardwareRuntimeExecutorAcceptanceSmoke;'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorAcceptance::accepted_layer_markers\(\)\.len\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorAcceptance::boot_marker_count_ok\(\)'
require_pattern "$SMOKE" 'VaachakHardwareRuntimeExecutorAcceptance::behavior_preserved\(\)'

for marker in \
  'hardware_runtime_ownership_consolidation=ok' \
  'spi_bus_arbitration_runtime_owner=ok' \
  'storage_probe_mount_runtime_executor_bridge=ok' \
  'hardware_runtime_executor_extraction=ok' \
  'hardware_runtime_executor_wiring=ok' \
  'hardware_runtime_executor_observability=ok' \
  'hardware_runtime_executor_boot_markers=ok' \
  'hardware_runtime_executor_acceptance_cleanup=ok'; do
  require_pattern "$DOC" "$marker"
done

require_pattern "$DOC" 'cleanup_hardware_runtime_executor_artifacts\.sh --include-current'
require_pattern "$DOC" 'cargo fmt --all'
require_pattern "$DOC" 'cargo build'
require_pattern "$CLEANUP" 'hardware_runtime_executor_cleanup_artifacts=ok'
require_pattern "$CLEANUP" 'storage_readonly_adapter_facade'
require_pattern "$CLEANUP" 'hardware_runtime_executor_boot_markers'
require_pattern "$CLEANUP" 'hardware_runtime_executor_acceptance_cleanup'
require_pattern "$CLEANUP" 'rm -rf -- "\$path"'

require_pattern "$BOOT" 'emit_hardware_runtime_executor_boot_markers'
require_pattern "$IMPORTED_RUNTIME" 'VaachakBoot::emit_hardware_runtime_executor_boot_markers\(\);'

for file in "$ACCEPTANCE" "$SMOKE"; do
  require_absent_pattern "$file" 'pub\s+(const\s+)?fn\s+(write|append|delete|rename|mkdir|format|erase|draw_pixels|draw_bitmap|refresh_full|refresh_partial|scan_adc|debounce_event|toggle_chip_select|spi_transfer|mount_sd|probe_sd)\b'
  require_absent_pattern "$file" '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|x4_kernel::drivers::storage|x4_kernel::drivers::display|x4_kernel::drivers::input|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|draw_packed_pixels|paint_stack\(|set_pixels\(|wait_until_idle\()'
done

if [[ -d hardware_runtime_executor_acceptance_cleanup/src || -d hardware_runtime_executor_acceptance_cleanup/vendor ]]; then
  fail 'overlay unexpectedly contains src/ or vendor/ runtime source'
fi
if [[ -d hardware_runtime_executor_acceptance_cleanup/target-xteink-x4/src/apps ]]; then
  fail 'overlay unexpectedly contains app source changes'
fi

printf '%s\n' 'hardware_runtime_executor_acceptance_cleanup=ok'
