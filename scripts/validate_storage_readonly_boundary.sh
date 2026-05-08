#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "storage_readonly_boundary validation failed: $*" >&2
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

FACADE="target-xteink-x4/src/vaachak_x4/io/storage_readonly_adapter.rs"
BRIDGE="target-xteink-x4/src/vaachak_x4/io/storage_readonly_pulp_bridge.rs"
BOUNDARY="target-xteink-x4/src/vaachak_x4/io/storage_readonly_boundary.rs"
FACADE_SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_adapter_facade_smoke.rs"
BRIDGE_SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_pulp_bridge_smoke.rs"
BOUNDARY_SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_boundary_smoke.rs"
IO_MOD="target-xteink-x4/src/vaachak_x4/io/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/storage-readonly-boundary.md"
FACADE_DOC="docs/architecture/storage-readonly-adapter-facade.md"
BRIDGE_DOC="docs/architecture/storage-readonly-pulp-bridge.md"
RUNTIME_DOC="docs/architecture/runtime-adapter-contracts.md"
FACADE_VALIDATOR="scripts/validate_storage_readonly_adapter_facade.sh"
BRIDGE_VALIDATOR="scripts/validate_storage_readonly_pulp_bridge.sh"

for path in "$FACADE" "$BRIDGE" "$BOUNDARY" "$FACADE_SMOKE" "$BRIDGE_SMOKE" "$BOUNDARY_SMOKE" "$IO_MOD" "$CONTRACTS_MOD" "$DOC" "$FACADE_DOC" "$BRIDGE_DOC" "$RUNTIME_DOC" "$FACADE_VALIDATOR" "$BRIDGE_VALIDATOR"; do
  require_file "$path"
done

require_rg '^pub mod storage_readonly_adapter;' "$IO_MOD"
require_rg '^pub mod storage_readonly_boundary;' "$IO_MOD"
require_rg '^pub mod storage_readonly_pulp_bridge;' "$IO_MOD"
require_rg '^pub mod storage_readonly_adapter_facade_smoke;' "$CONTRACTS_MOD"
require_rg '^pub mod storage_readonly_boundary_smoke;' "$CONTRACTS_MOD"
require_rg '^pub mod storage_readonly_pulp_bridge_smoke;' "$CONTRACTS_MOD"

require_rg 'trait VaachakReadonlyStorage' "$FACADE"
require_rg 'struct VaachakReadonlyStorageFacade' "$FACADE"
require_rg 'PULP_BACKED_ACTIVE_PATHS' "$FACADE"
require_rg 'struct PulpReadonlyStorageBridge' "$BRIDGE"
require_rg 'trait PulpReadonlyStorageBackend' "$BRIDGE"
require_rg 'struct X4PulpReadonlyStorageBackend' "$BRIDGE"
require_rg 'impl<B> VaachakReadonlyStorage for PulpReadonlyStorageBridge<B>' "$BRIDGE"
require_rg 'struct VaachakStorageReadonlyBoundary' "$BOUNDARY"
require_rg 'VaachakReadonlyStorageFacade<PulpReadonlyStorageBridge<B>>' "$BOUNDARY"
require_rg 'new_pulp_backed' "$BOUNDARY"
require_rg 'impl<B> VaachakReadonlyStorage for VaachakStorageReadonlyBoundary<B>' "$BOUNDARY"
require_rg 'STORAGE_READONLY_BOUNDARY_MARKER' "$BOUNDARY"
require_rg 'x4-storage-readonly-boundary-ok' "$BOUNDARY"
require_rg 'STORAGE_READONLY_BOUNDARY_ACTIVE_BACKEND_OWNER:.*vendor/pulp-os imported runtime' "$BOUNDARY"

require_rg 'fn file_exists\(' "$BOUNDARY"
require_rg 'fn read_file_start\(' "$BOUNDARY"
require_rg 'fn read_chunk\(' "$BOUNDARY"
require_rg 'fn list_directory_metadata' "$BOUNDARY"
require_rg 'fn resolve_current_storage_paths\(' "$BOUNDARY"

require_rg 'SD_MOUNT_OR_PROBE_MOVED_TO_BOUNDARY: bool = false' "$BOUNDARY"
require_rg 'SD_DRIVER_MOVED_TO_BOUNDARY: bool = false' "$BOUNDARY"
require_rg 'FAT_BEHAVIOR_MOVED_TO_BOUNDARY: bool = false' "$BOUNDARY"
require_rg 'SPI_ARBITRATION_MOVED_TO_BOUNDARY: bool = false' "$BOUNDARY"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_BOUNDARY: bool = false' "$BOUNDARY"
require_rg 'READER_OR_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BOUNDARY"
require_rg 'WRITABLE_STORAGE_BEHAVIOR_ADDED: bool = false' "$BOUNDARY"

require_rg 'VaachakStorageReadonlyBoundarySmoke' "$BOUNDARY_SMOKE"
require_rg 'x4-storage-readonly-boundary-smoke-ok' "$BOUNDARY_SMOKE"
require_rg 'VaachakStorageReadonlyAdapterFacadeSmoke' "$BOUNDARY_SMOKE"
require_rg 'VaachakStorageReadonlyPulpBridgeSmoke' "$BOUNDARY_SMOKE"
require_rg 'physical_behavior_moved' "$BOUNDARY_SMOKE"

require_rg 'Storage Read-Only Boundary' "$DOC"
require_rg 'canonical architecture document' "$DOC"
require_rg 'Vaachak owns' "$DOC"
require_rg 'Pulp-backed bridge remains the active implementation path' "$DOC"
require_rg 'SD card mount/probe behavior' "$DOC"
require_rg 'SPI arbitration' "$DOC"
require_rg 'Display behavior' "$DOC"
require_rg 'Reader behavior' "$DOC"
require_rg 'File browser behavior' "$DOC"
require_rg 'Write, append, delete, create, rename, truncate, mkdir, mount, unmount, or format operations' "$DOC"
require_rg 'final read-only storage boundary slice' "$DOC"
require_rg 'storage_readonly_boundary=ok' "$DOC"
require_rg 'docs/architecture/storage-readonly-boundary.md' "$FACADE_DOC"
require_rg 'docs/architecture/storage-readonly-boundary.md' "$BRIDGE_DOC"
require_rg 'Storage read-only boundary consolidation' "$RUNTIME_DOC"

bash "$FACADE_VALIDATOR" "$REPO_ROOT"
bash "$BRIDGE_VALIDATOR" "$REPO_ROOT"

if rg -n '(SdStorage::mount|SdStorage::init_card|probe_ok\(|Board::init|speed_up_spi\(|init_spi|display\.epd|epd\.|InputDriver::new|paint_stack\(|esp_hal::init|crate::apps|src/apps|vendor/pulp-os/src/apps|reader::|file_browser)' "$BOUNDARY" "$BOUNDARY_SMOKE"; then
  fail "consolidated boundary must not move SD mount/probe, SPI, display, input, reader, or file-browser behavior"
fi

if rg -n '\bfn +(write|delete|remove|mkdir|create|rename|truncate|append|mount|unmount|format|probe)[A-Za-z0-9_]*' "$FACADE" "$BRIDGE" "$BOUNDARY" "$FACADE_SMOKE" "$BRIDGE_SMOKE" "$BOUNDARY_SMOKE"; then
  fail "read-only storage boundary must not expose mutating or mount/probe APIs"
fi

if rg -n 'x4_kernel::drivers::storage::|x4_kernel::drivers::sdcard::|embedded_sdmmc|embedded_hal|embedded_hal_bus|esp_hal' "$BOUNDARY" "$BOUNDARY_SMOKE"; then
  fail "boundary entrypoint must not call Pulp, SD/FAT, SPI, or display APIs directly"
fi

if rg -n 'x4_kernel::drivers::storage::(write|append|delete|ensure|write_at|save_title|mkdir|rename|truncate|format)' "$BRIDGE"; then
  fail "Pulp bridge must remain read/list/size only"
fi

if [[ -d storage_readonly_boundary_consolidation ]]; then
  if find storage_readonly_boundary_consolidation -type f | rg -n '(^|/)(src/apps|vendor/pulp-os|vendor/smol-epub)/' >/dev/null; then
    fail "overlay folder includes app or vendor files; consolidation must remain boundary-only"
  fi
fi

if [[ -d .git ]]; then
  if git status --short -- vendor/pulp-os vendor/smol-epub | rg -n '.' >/dev/null; then
    git status --short -- vendor/pulp-os vendor/smol-epub >&2
    fail "vendored Pulp/smol-epub files changed; active SD/FAT behavior must remain imported"
  fi
fi

if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all -- --check
fi

echo "storage_readonly_boundary=ok"
