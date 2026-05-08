#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

OWNER="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_executor_bridge.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_executor_pulp_backend.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_probe_mount_runtime_executor_bridge_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/storage-probe-mount-runtime-executor-bridge.md"
STORAGE_OWNER="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs"
SPI_ARBITER="target-xteink-x4/src/vaachak_x4/physical/spi_bus_arbitration_runtime_owner.rs"
SD_FAT_OWNER="target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs"
DISPLAY_OWNER="target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs"
INPUT_OWNER="target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs"

fail() {
  printf 'storage_probe_mount_runtime_executor_bridge validation failed: %s\n' "$*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing $path"
}

require_rg() {
  local pattern="$1"
  local path="$2"
  rg -n "$pattern" "$path" >/dev/null || fail "missing pattern '$pattern' in $path"
}

reject_rg() {
  local pattern="$1"
  shift
  if rg -n "$pattern" "$@" >/tmp/storage_probe_mount_executor_reject.txt; then
    cat /tmp/storage_probe_mount_executor_reject.txt >&2
    fail "forbidden pattern '$pattern' found"
  fi
}

for path in \
  "$OWNER" "$BACKEND" "$SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$DOC" \
  "$STORAGE_OWNER" "$SPI_ARBITER" "$SD_FAT_OWNER" "$DISPLAY_OWNER" "$INPUT_OWNER"; do
  require_file "$path"
done

require_rg '^pub mod storage_probe_mount_executor_pulp_backend;' "$PHYSICAL_MOD"
require_rg '^pub mod storage_probe_mount_runtime_executor_bridge;' "$PHYSICAL_MOD"
require_rg '^pub mod storage_probe_mount_runtime_executor_bridge_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakStorageProbeMountRuntimeExecutorBridge' "$OWNER"
require_rg 'STORAGE_PROBE_MOUNT_RUNTIME_EXECUTOR_BRIDGE_MARKER' "$OWNER"
require_rg 'storage_probe_mount_runtime_executor_bridge=ok' "$OWNER"
require_rg 'LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'execute_lifecycle_intent' "$OWNER"
require_rg 'RoutedToPulpCompatibilityExecutor' "$OWNER"
require_rg 'VaachakStorageProbeMountRuntimeOwner::ownership_ok' "$OWNER"
require_rg 'VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok' "$OWNER"
require_rg 'VaachakStorageProbeMountExecutorPulpBackend::backend_ok' "$OWNER"
require_rg 'DetectCard' "$OWNER"
require_rg 'IdentifyCardAtSafeSpeed' "$OWNER"
require_rg 'ObserveCardAvailability' "$OWNER"
require_rg 'ObserveFatVolumeAvailability' "$OWNER"
require_rg 'FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'executor_bridge_ok' "$OWNER"

require_rg 'struct VaachakStorageProbeMountExecutorPulpBackend' "$BACKEND"
require_rg 'BACKEND_NAME:.*PulpCompatibility|BACKEND_NAME' "$BACKEND"
require_rg 'ACTIVE_EXECUTOR_OWNER:.*vendor/pulp-os imported runtime|ACTIVE_EXECUTOR_OWNER' "$BACKEND"
require_rg 'LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK: bool = true' "$BACKEND"
require_rg 'LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'executor_owner_for' "$BACKEND"
require_rg 'backend_ok' "$BACKEND"

require_rg 'VaachakStorageProbeMountRuntimeExecutorBridgeSmoke' "$SMOKE"
require_rg 'storage_probe_mount_runtime_executor_bridge_smoke=ok' "$SMOKE"
require_rg 'all_lifecycle_intents_route_to_pulp' "$SMOKE"
require_rg 'no_behavior_regression_flags' "$SMOKE"
require_rg 'LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'DISPLAY_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$SMOKE"

require_rg 'storage_probe_mount_runtime_executor_bridge=ok' "$DOC"
require_rg 'lifecycle execution entrypoint' "$DOC"
require_rg 'Pulp-compatible executor' "$DOC"
require_rg 'VaachakStorageProbeMountRuntimeExecutorBridge::execute_lifecycle_intent' "$DOC"
require_rg 'VaachakStorageProbeMountRuntimeOwner::ownership_ok' "$DOC"
require_rg 'VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok' "$DOC"
require_rg 'FAT read/write/list behavior did not move' "$DOC"

require_rg 'STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$STORAGE_OWNER"
require_rg 'SPI_BUS_ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = true' "$SPI_ARBITER"
require_rg 'SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SD_FAT_OWNER"
require_rg 'DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$DISPLAY_OWNER"
require_rg 'INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$INPUT_OWNER"

# The new executor bridge must not add low-level hardware/storage/display driver calls.
reject_rg '(x4_kernel::drivers::storage|x4_kernel::drivers::display|embedded_hal::|embedded_hal_bus|esp_hal::|fatfs::|sdmmc|SdCard|VolumeManager|DisplayInterface|FrameBuffer|Board::init|init_spi|speed_up_spi|draw_packed_pixels|set_pixels|wait_until_idle|partial_update|full_update)' \
  "$OWNER" "$BACKEND" "$SMOKE"

# No behavior APIs should be implemented in the new bridge; it is an intent router only.
reject_rg '\bfn +(read|write|append|delete|remove|rename|truncate|mkdir|create|open|close|format|init_card|init_spi|speed_up_spi|toggle|select|deselect|draw|refresh|partial_refresh|full_refresh|flush|paint|set_pixels|wait_until_idle)\b' \
  "$OWNER" "$BACKEND" "$SMOKE"

# The overlay must not touch vendor, app, UI, IO, reader, file-browser, or display implementation paths.
if [[ -d storage_probe_mount_runtime_executor_bridge ]]; then
  if find storage_probe_mount_runtime_executor_bridge -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/ui|target-xteink-x4/src/vaachak_x4/io|target-xteink-x4/src/vaachak_x4/display)/' >/tmp/storage_probe_mount_executor_overlay_paths.txt; then
    cat /tmp/storage_probe_mount_executor_overlay_paths.txt >&2
    fail 'overlay modifies forbidden runtime/app paths'
  fi
fi

printf '%s\n' 'storage_probe_mount_runtime_executor_bridge=ok'
