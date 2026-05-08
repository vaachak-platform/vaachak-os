#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "sd_fat_runtime_readonly_owner validation failed: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing required file: $path"
}

require_rg() {
  local pattern="$1"
  local path="$2"
  rg -n "$pattern" "$path" >/dev/null || fail "missing pattern '$pattern' in $path"
}

OWNER="target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/sd_fat_readonly_pulp_backend.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/sd_fat_runtime_readonly_ownership_smoke.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
SPI_OWNER="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs"
PROBE_OWNER="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs"
READONLY_BOUNDARY="target-xteink-x4/src/vaachak_x4/io/storage_readonly_boundary.rs"
DOC="docs/architecture/sd-fat-runtime-readonly-ownership.md"
SPI_DOC="docs/architecture/spi-bus-runtime-ownership.md"
PROBE_DOC="docs/architecture/storage-probe-mount-runtime-ownership.md"
BOUNDARY_DOC="docs/architecture/storage-readonly-boundary.md"

for path in "$OWNER" "$BACKEND" "$SMOKE" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$SPI_OWNER" "$PROBE_OWNER" "$READONLY_BOUNDARY" "$DOC" "$SPI_DOC" "$PROBE_DOC" "$BOUNDARY_DOC"; do
  require_file "$path"
done

require_rg '^pub mod sd_fat_readonly_pulp_backend;' "$PHYSICAL_MOD"
require_rg '^pub mod sd_fat_runtime_readonly_owner;' "$PHYSICAL_MOD"
require_rg '^pub mod sd_fat_runtime_readonly_ownership_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakSdFatRuntimeReadonlyOwner' "$OWNER"
require_rg 'SD_FAT_RUNTIME_READONLY_OWNERSHIP_MARKER' "$OWNER"
require_rg 'x4-sd-fat-runtime-readonly-owner-ok' "$OWNER"
require_rg 'SD_FAT_RUNTIME_READONLY_IDENTITY' "$OWNER"
require_rg 'xteink-x4-sd-fat-runtime-readonly' "$OWNER"
require_rg 'SD_FAT_READONLY_OWNERSHIP_AUTHORITY' "$OWNER"
require_rg 'target-xteink-x4 Vaachak layer' "$OWNER"
require_rg 'SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'VaachakSdFatRuntimeReadonlyBackend::PulpCompatibility' "$OWNER"
require_rg 'ACTIVE_BACKEND' "$OWNER"
require_rg 'Self::PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'ACTIVE_BACKEND_NAME' "$OWNER"
require_rg 'VaachakSdFatReadonlyPulpBackend::BACKEND_NAME' "$OWNER"
require_rg 'ACTIVE_EXECUTOR_OWNER' "$OWNER"
require_rg 'vendor/pulp-os imported runtime' "$OWNER"
require_rg 'STORAGE_SD_CS_GPIO: u8 = 12' "$OWNER"
require_rg 'SD_IDENTIFICATION_KHZ: u32 = 400' "$OWNER"
require_rg 'OPERATIONAL_SPI_MHZ: u32 = 20' "$OWNER"
require_rg 'VaachakSpiBusRuntimeOwner' "$OWNER"
require_rg 'VaachakSpiRuntimeUser::Storage' "$OWNER"
require_rg 'VaachakSpiTransactionKind::StorageFatIoMetadata' "$OWNER"
require_rg 'VaachakStorageProbeMountRuntimeOwner::ownership_ok' "$OWNER"
require_rg 'VaachakStorageReadonlyBoundaryContract::active_runtime_preflight' "$OWNER"
require_rg 'FileExists' "$OWNER"
require_rg 'ReadFileStart' "$OWNER"
require_rg 'ReadChunk' "$OWNER"
require_rg 'ListDirectoryMetadata' "$OWNER"
require_rg 'ResolveCurrentStoragePaths' "$OWNER"
require_rg 'readonly_operations_registered' "$OWNER"
require_rg 'writable_operations_denied' "$OWNER"
require_rg 'ownership_ok' "$OWNER"
require_rg 'FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'SD_PROBE_MOUNT_BEHAVIOR_CHANGED: bool = false' "$OWNER"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$OWNER"

require_rg 'struct VaachakSdFatReadonlyPulpBackend' "$BACKEND"
require_rg 'BACKEND_NAME' "$BACKEND"
require_rg 'PulpCompatibility' "$BACKEND"
require_rg 'ACTIVE_HARDWARE_EXECUTOR: bool = true' "$BACKEND"
require_rg 'ACTIVE_READONLY_FILE_EXISTS_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_READONLY_FILE_START_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_READONLY_CHUNK_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_DIRECTORY_METADATA_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_PATH_RESOLUTION_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_FAT_EXECUTOR_OWNER' "$BACKEND"
require_rg 'vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'READONLY_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DISPLAY_RUNTIME_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'bridge_ok' "$BACKEND"

require_rg 'struct VaachakSdFatRuntimeReadonlyOwnershipSmoke' "$SMOKE"
require_rg 'x4-sd-fat-runtime-readonly-ownership-smoke-ok' "$SMOKE"
require_rg 'VaachakSdFatRuntimeReadonlyOwner::ownership_ok' "$SMOKE"
require_rg 'VaachakSdFatReadonlyPulpBackend::bridge_ok' "$SMOKE"
require_rg 'SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SMOKE"
require_rg 'PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true' "$SMOKE"
require_rg 'FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'WRITE_APPEND_DELETE_RENAME_MKDIR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'SD_PROBE_MOUNT_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$SMOKE"
require_rg 'readonly_operations_registered' "$SMOKE"
require_rg 'writable_operations_denied' "$SMOKE"

require_rg 'SD/FAT Runtime Read-Only Ownership' "$DOC"
require_rg 'sd_fat_runtime_readonly_owner=ok' "$DOC"
require_rg 'SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true' "$DOC"
require_rg 'PulpCompatibility' "$DOC"
require_rg 'vendor/pulp-os imported runtime' "$DOC"
require_rg 'file exists' "$DOC"
require_rg 'read file start' "$DOC"
require_rg 'read chunk' "$DOC"
require_rg 'list directory metadata' "$DOC"
require_rg 'resolve current storage paths' "$DOC"
require_rg 'write/append/delete/rename/mkdir' "$DOC"
require_rg 'SD probe/mount' "$DOC"
require_rg 'SPI arbitration' "$DOC"
require_rg 'SSD1677 display rendering' "$DOC"
require_rg 'storage-probe-mount-runtime-ownership.md' "$DOC"
require_rg 'storage-readonly-boundary.md' "$DOC"

require_rg 'SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SPI_OWNER"
require_rg 'ownership_ok' "$PROBE_OWNER"
require_rg 'STORAGE_READONLY_BOUNDARY_MARKER' "$READONLY_BOUNDARY"

# New owner/backend must remain metadata-only and must not call/import concrete SD/FAT/SPI/display implementations.
if rg -n '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|esp_hal::|x4_kernel::drivers::storage|x4_kernel::drivers::sdcard|SdStorage::|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|NoDelay|Board::init|speed_up_spi|init_spi|display\.epd|paint_stack\(|draw_packed_pixels\(|set_pixels\(|flush\(|wait_until_idle\()' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "SD/FAT read-only owner must not import or call SD/FAT, SPI peripheral, or display behavior"
fi

# This slice may expose read-only operation metadata, but it must not expose direct filesystem/SPI/display executor functions.
if rg -n '\bfn +(write|append|delete|remove|rename|truncate|mkdir|create|open|close|format|init_card|init_spi|speed_up_spi|mount|unmount|probe|refresh|draw|transfer|toggle|select|deselect|lock|unlock)[A-Za-z0-9_]*' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "SD/FAT read-only owner exposes direct write, mount/probe, SPI, or display behavior"
fi

# Do not edit active runtime/app/vendor/display/io behavior in this overlay.
if [[ -d sd_fat_runtime_readonly_owner ]]; then
  if find sd_fat_runtime_readonly_owner -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/display|target-xteink-x4/src/vaachak_x4/io/storage_readonly_adapter\.rs|target-xteink-x4/src/vaachak_x4/io/storage_readonly_pulp_bridge\.rs|target-xteink-x4/src/vaachak_x4/ui)/' >/dev/null; then
    fail "overlay includes vendor/app/display/io behavior files; this slice must only add SD/FAT read-only runtime ownership files"
  fi
fi

printf '%s\n' 'sd_fat_runtime_readonly_owner=ok'
