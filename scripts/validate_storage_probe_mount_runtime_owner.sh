#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "storage_probe_mount_runtime_owner validation failed: $*" >&2
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

OWNER="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_pulp_backend.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_probe_mount_runtime_ownership_smoke.rs"
SPI_OWNER="target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs"
SPI_BACKEND="target-xteink-x4/src/vaachak_x4/physical/spi_bus_pulp_backend.rs"
PHYSICAL_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/storage-probe-mount-runtime-ownership.md"
SPI_DOC="docs/architecture/spi-bus-runtime-ownership.md"

for path in "$OWNER" "$BACKEND" "$SMOKE" "$SPI_OWNER" "$SPI_BACKEND" "$PHYSICAL_MOD" "$CONTRACTS_MOD" "$DOC" "$SPI_DOC"; do
  require_file "$path"
done

require_rg '^pub mod storage_probe_mount_pulp_backend;' "$PHYSICAL_MOD"
require_rg '^pub mod storage_probe_mount_runtime_owner;' "$PHYSICAL_MOD"
require_rg '^pub mod storage_probe_mount_runtime_ownership_smoke;' "$CONTRACTS_MOD"

require_rg 'struct VaachakStorageProbeMountRuntimeOwner' "$OWNER"
require_rg 'STORAGE_PROBE_MOUNT_RUNTIME_OWNERSHIP_MARKER' "$OWNER"
require_rg 'x4-storage-probe-mount-runtime-owner-ok' "$OWNER"
require_rg 'STORAGE_PROBE_MOUNT_IDENTITY' "$OWNER"
require_rg 'xteink-x4-sd-probe-mount-runtime' "$OWNER"
require_rg 'STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY' "$OWNER"
require_rg 'target-xteink-x4 Vaachak layer' "$OWNER"
require_rg 'STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$OWNER"
require_rg 'PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'VaachakStorageProbeMountRuntimeBackend::PulpCompatibility' "$OWNER"
require_rg 'ACTIVE_BACKEND' "$OWNER"
require_rg 'Self::PULP_COMPATIBILITY_BACKEND' "$OWNER"
require_rg 'ACTIVE_BACKEND_NAME' "$OWNER"
require_rg 'VaachakStorageProbeMountPulpBackend::BACKEND_NAME' "$OWNER"
require_rg 'ACTIVE_EXECUTOR_OWNER' "$OWNER"
require_rg 'vendor/pulp-os imported runtime' "$OWNER"
require_rg 'STORAGE_SD_CS_GPIO: u8 = 12' "$OWNER"
require_rg 'SD_IDENTIFICATION_KHZ: u32 = 400' "$OWNER"
require_rg 'OPERATIONAL_SPI_MHZ: u32 = 20' "$OWNER"
require_rg 'VaachakSpiBusRuntimeOwner' "$OWNER"
require_rg 'VaachakSpiRuntimeUser::Storage' "$OWNER"
require_rg 'VaachakSpiTransactionKind::StorageProbeMetadata' "$OWNER"
require_rg 'VaachakSpiTransactionKind::StorageMountMetadata' "$OWNER"
require_rg 'shared_spi_owner_available' "$OWNER"
require_rg 'storage_user_registered_on_spi' "$OWNER"
require_rg 'storage_chip_select_ok' "$OWNER"
require_rg 'storage_spi_metadata_ok' "$OWNER"
require_rg 'lifecycle_authority_ok' "$OWNER"
require_rg 'ownership_ok' "$OWNER"
require_rg 'SD_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'FAT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'FAT_READ_WRITE_LIST_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$OWNER"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$OWNER"

require_rg 'struct VaachakStorageProbeMountPulpBackend' "$BACKEND"
require_rg 'BACKEND_NAME' "$BACKEND"
require_rg 'PulpCompatibility' "$BACKEND"
require_rg 'ACTIVE_HARDWARE_EXECUTOR: bool = true' "$BACKEND"
require_rg 'ACTIVE_HARDWARE_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_CARD_DETECTION_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_SD_IDENTIFICATION_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_SD_MOUNT_EXECUTOR_OWNER' "$BACKEND"
require_rg 'ACTIVE_FAT_EXECUTOR_OWNER' "$BACKEND"
require_rg 'vendor/pulp-os imported runtime' "$BACKEND"
require_rg 'STORAGE_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'FAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'FAT_READ_WRITE_LIST_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'DISPLAY_RUNTIME_MOVED_TO_VAACHAK: bool = false' "$BACKEND"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BACKEND"
require_rg 'bridge_ok' "$BACKEND"

require_rg 'struct VaachakStorageProbeMountRuntimeOwnershipSmoke' "$SMOKE"
require_rg 'x4-storage-probe-mount-runtime-ownership-smoke-ok' "$SMOKE"
require_rg 'VaachakStorageProbeMountRuntimeOwner::ownership_ok' "$SMOKE"
require_rg 'VaachakStorageProbeMountPulpBackend::bridge_ok' "$SMOKE"
require_rg 'STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SMOKE"
require_rg 'PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true' "$SMOKE"
require_rg 'FAT_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'SPI_ARBITRATION_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK: bool = false' "$SMOKE"
require_rg 'READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$SMOKE"

require_rg 'Storage Probe/Mount Runtime Ownership' "$DOC"
require_rg 'storage_probe_mount_runtime_owner=ok' "$DOC"
require_rg 'STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true' "$DOC"
require_rg 'PulpCompatibility' "$DOC"
require_rg '400 kHz' "$DOC"
require_rg 'GPIO12' "$DOC"
require_rg 'GPIO21' "$DOC"
require_rg 'spi-bus-runtime-ownership.md' "$DOC"
require_rg 'FAT read/write/list behavior' "$DOC"
require_rg 'SSD1677 display rendering' "$DOC"

require_rg 'SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true' "$SPI_OWNER"
require_rg 'ACTIVE_BACKEND' "$SPI_OWNER"
require_rg 'Self::PULP_COMPATIBILITY_BACKEND' "$SPI_OWNER"
require_rg 'SD_FAT_MOVED_TO_VAACHAK: bool = false' "$SPI_OWNER"
require_rg 'DISPLAY_RENDERING_MOVED_TO_VAACHAK: bool = false' "$SPI_OWNER"
require_rg 'BACKEND_NAME' "$SPI_BACKEND"
require_rg 'PulpCompatibility' "$SPI_BACKEND"

# New storage owner files must remain ownership/metadata only: no embedded SD/FAT/SPI/display implementation imports or calls.
if rg -n '(embedded_sdmmc|embedded_hal::|embedded_hal_bus|esp_hal::|x4_kernel::drivers::storage|x4_kernel::drivers::sdcard|SdStorage::|BlockDevice|VolumeManager|SpiDevice|ExclusiveDevice|NoDelay|Board::init|speed_up_spi|init_spi|display\.epd|paint_stack\(|draw_packed_pixels\(|set_pixels\(|flush\(|wait_until_idle\()' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "storage probe/mount runtime owner must not import or call SD/FAT, SPI peripheral, or display behavior"
fi

# Do not expose direct runtime operations in the new owner or backend. This slice moves authority, not executor implementation.
if rg -n '\bfn +(read|write|append|delete|remove|rename|truncate|mkdir|create|open|close|format|init_card|init_spi|speed_up_spi|refresh|draw|transfer|toggle|select|deselect|lock|unlock)[A-Za-z0-9_]*' "$OWNER" "$BACKEND" "$SMOKE"; then
  fail "storage probe/mount runtime owner exposes direct filesystem, SPI, or display behavior"
fi

# Keep the overlay narrow. It must not include vendor, app, display, io, or UI implementation files.
if [[ -d storage_probe_mount_runtime_owner ]]; then
  if find storage_probe_mount_runtime_owner -type f | rg -n '(^|/)(vendor/pulp-os|vendor/smol-epub|src/apps|target-xteink-x4/src/vaachak_x4/display|target-xteink-x4/src/vaachak_x4/io|target-xteink-x4/src/vaachak_x4/ui)/' >/dev/null; then
    fail "overlay includes vendor/app/display/io/ui files; this slice must only add storage probe/mount runtime ownership files"
  fi
fi

printf '%s\n' 'storage_probe_mount_runtime_owner=ok'
