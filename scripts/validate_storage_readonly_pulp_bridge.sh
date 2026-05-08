#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "storage_readonly_pulp_bridge validation failed: $*" >&2
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
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_pulp_bridge_smoke.rs"
IO_MOD="target-xteink-x4/src/vaachak_x4/io/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/storage-readonly-pulp-bridge.md"
RUNTIME_DOC="docs/architecture/runtime-adapter-contracts.md"
FACADE_DOC="docs/architecture/storage-readonly-adapter-facade.md"

require_file "$FACADE"
require_file "$BRIDGE"
require_file "$SMOKE"
require_file "$IO_MOD"
require_file "$CONTRACTS_MOD"
require_file "$DOC"
require_file "$RUNTIME_DOC"
require_file "$FACADE_DOC"

require_rg '^pub mod storage_readonly_pulp_bridge;' "$IO_MOD"
require_rg '^pub mod storage_readonly_pulp_bridge_smoke;' "$CONTRACTS_MOD"

require_rg 'trait VaachakReadonlyStorage' "$FACADE"
require_rg 'struct PulpReadonlyStorageBridge' "$BRIDGE"
require_rg 'trait PulpReadonlyStorageBackend' "$BRIDGE"
require_rg 'impl<B> VaachakReadonlyStorage for PulpReadonlyStorageBridge<B>' "$BRIDGE"
require_rg 'struct X4PulpReadonlyStorageBackend' "$BRIDGE"
require_rg 'cfg\(target_arch = "riscv32"\)' "$BRIDGE"
require_rg 'STORAGE_READONLY_PULP_BRIDGE_MARKER' "$BRIDGE"
require_rg 'x4-storage-readonly-pulp-bridge-ok' "$BRIDGE"
require_rg 'PULP_READONLY_ACTIVE_BACKEND_OWNER:.*vendor/pulp-os imported runtime' "$BRIDGE"
require_rg 'SD_MOUNT_OR_PROBE_MOVED_TO_BRIDGE: bool = false' "$BRIDGE"
require_rg 'SD_DRIVER_MOVED_TO_BRIDGE: bool = false' "$BRIDGE"
require_rg 'FAT_BEHAVIOR_MOVED_TO_BRIDGE: bool = false' "$BRIDGE"
require_rg 'SPI_ARBITRATION_MOVED_TO_BRIDGE: bool = false' "$BRIDGE"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_BRIDGE: bool = false' "$BRIDGE"
require_rg 'READER_OR_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false' "$BRIDGE"
require_rg 'WRITABLE_STORAGE_BEHAVIOR_ADDED: bool = false' "$BRIDGE"

require_rg 'fn file_exists\(' "$BRIDGE"
require_rg 'fn read_file_start\(' "$BRIDGE"
require_rg 'fn read_chunk\(' "$BRIDGE"
require_rg 'fn list_directory_metadata' "$BRIDGE"
require_rg 'fn resolve_current_storage_paths\(' "$BRIDGE"
require_rg 'file_size_root' "$BRIDGE"
require_rg 'read_file_start_root' "$BRIDGE"
require_rg 'read_file_chunk_root' "$BRIDGE"
require_rg 'list_root_entries' "$BRIDGE"
require_rg 'file_size_in_dir' "$BRIDGE"
require_rg 'read_file_start_in_dir' "$BRIDGE"
require_rg 'read_file_chunk_in_dir' "$BRIDGE"
require_rg 'list_dir_entries' "$BRIDGE"
require_rg 'file_size_in_subdir' "$BRIDGE"
require_rg 'read_file_start_in_subdir' "$BRIDGE"
require_rg 'read_file_chunk_in_subdir' "$BRIDGE"
require_rg 'list_subdir_entries' "$BRIDGE"

require_rg 'VaachakStorageReadonlyPulpBridgeSmoke' "$SMOKE"
require_rg 'x4-storage-readonly-pulp-bridge-smoke-ok' "$SMOKE"
require_rg 'physical_behavior_moved' "$SMOKE"

require_rg 'Storage Read-Only Pulp Bridge' "$DOC"
require_rg 'Vaachak owns adapter contract' "$DOC" || true
require_rg 'Pulp-backed read-only implementation bridge' "$DOC"
require_rg 'vendor/pulp-os' "$DOC"
require_rg 'Bridge layout' "$DOC"
require_rg 'Adapter call mapping' "$DOC"
require_rg 'Write, append, delete, create, rename, truncate, mkdir, mount, unmount, or format operations' "$DOC"
require_rg 'Storage read-only Pulp bridge' "$RUNTIME_DOC"

# The bridge is allowed to call only existing Pulp read/list/size helpers through the embedded backend.
if rg -n 'x4_kernel::drivers::storage::(write|append|delete|ensure|read_chunk_in_x4|write_at|save_title)' "$BRIDGE"; then
  fail "Pulp bridge must not call mutating Pulp storage helpers"
fi

# The bridge must not own physical initialization or display/SPI/input behavior.
if rg -n '(SdStorage::mount|SdStorage::init_card|probe_ok\(|Board::init|speed_up_spi\(|init_spi|display\.epd|epd\.|InputDriver::new|paint_stack\(|esp_hal::init)' "$BRIDGE" "$SMOKE"; then
  fail "Pulp bridge must not move SD mount/probe, SPI, display, or input behavior"
fi

# Keep the bridge API surface read-only. Comments and docs may name forbidden behaviors,
# but Rust function/type/trait names in this slice must not expose them.
if rg -n '\bfn +(write|delete|remove|mkdir|create|rename|truncate|append|mount|unmount|format)[A-Za-z0-9_]*' "$BRIDGE" "$SMOKE"; then
  fail "read-only bridge must not expose mutating or mount/probe APIs"
fi

# Keep existing facade read-only too.
if rg -n '\bfn +(write|delete|remove|mkdir|create|rename|truncate|append|mount|unmount|format)[A-Za-z0-9_]*' "$FACADE"; then
  fail "facade must remain read-only"
fi

# Do not edit vendored active runtime code in this slice.
if [[ -d .git ]]; then
  if git status --short -- vendor/pulp-os vendor/smol-epub | rg -n '.' >/dev/null; then
    git status --short -- vendor/pulp-os vendor/smol-epub >&2
    fail "vendored Pulp/smol-epub files changed; active SD/FAT behavior must remain imported"
  fi
fi

# Optional formatting check when Rust tooling is installed. Static checks above are the source of truth
# for environments where cargo is unavailable.
if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all -- --check
fi

echo "storage_readonly_pulp_bridge=ok"
