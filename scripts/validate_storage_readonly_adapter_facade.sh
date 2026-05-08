#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${1:-$(pwd)}"
cd "$REPO_ROOT"

fail() {
  echo "storage_readonly_adapter_facade validation failed: $*" >&2
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
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/storage_readonly_adapter_facade_smoke.rs"
IO_MOD="target-xteink-x4/src/vaachak_x4/io/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
DOC="docs/architecture/storage-readonly-adapter-facade.md"
RUNTIME_DOC="docs/architecture/runtime-adapter-contracts.md"

require_file "$FACADE"
require_file "$SMOKE"
require_file "$IO_MOD"
require_file "$CONTRACTS_MOD"
require_file "$DOC"
require_file "$RUNTIME_DOC"

require_rg '^pub mod storage_readonly_adapter;' "$IO_MOD"
require_rg '^pub mod storage_readonly_adapter_facade_smoke;' "$CONTRACTS_MOD"

require_rg 'trait VaachakReadonlyStorage' "$FACADE"
require_rg 'fn file_exists\(' "$FACADE"
require_rg 'fn read_file_start\(' "$FACADE"
require_rg 'fn read_chunk\(' "$FACADE"
require_rg 'fn list_directory_metadata' "$FACADE"
require_rg 'fn resolve_current_storage_paths\(' "$FACADE"
require_rg 'struct VaachakReadonlyStorageFacade' "$FACADE"
require_rg 'struct VaachakResolvedStoragePaths' "$FACADE"
require_rg 'PULP_BACKED_ACTIVE_PATHS' "$FACADE"
require_rg 'ACTIVE_STORAGE_BACKEND_OWNER:.*vendor/pulp-os imported runtime' "$FACADE"
require_rg 'SD_MOUNT_OR_PROBE_MOVED_TO_FACADE: bool = false' "$FACADE"
require_rg 'SD_DRIVER_MOVED_TO_FACADE: bool = false' "$FACADE"
require_rg 'FAT_BEHAVIOR_MOVED_TO_FACADE: bool = false' "$FACADE"
require_rg 'SPI_ARBITRATION_MOVED_TO_FACADE: bool = false' "$FACADE"
require_rg 'DISPLAY_BEHAVIOR_MOVED_TO_FACADE: bool = false' "$FACADE"
require_rg 'WRITABLE_STORAGE_BEHAVIOR_ADDED: bool = false' "$FACADE"

require_rg 'VaachakStorageReadonlyAdapterFacadeSmoke' "$SMOKE"
require_rg 'x4-storage-readonly-adapter-facade-smoke-ok' "$SMOKE"
require_rg 'physical_behavior_moved' "$SMOKE"

require_rg 'Active implementation remains Pulp-backed' "$DOC"
require_rg 'SD card mount/probe behavior' "$DOC"
require_rg 'SPI arbitration' "$DOC"
require_rg 'Display behavior' "$DOC"
require_rg 'Any write, delete, create, rename, truncate, or append operation' "$DOC"
require_rg 'Storage read-only adapter facade' "$RUNTIME_DOC"

# The facade may name the current owner as vendor/pulp-os, but it must not import
# or call the active Pulp runtime, SD/FAT, SPI, or display implementation APIs.
if rg -n '(^use +pulp_os|^use +embedded_sdmmc|^use +embedded_hal|^use +embedded_hal_bus|^use +esp_hal|SdStorage::|Board::init|speed_up_spi\(|init_spi|mount\(|probe\(|display\.epd|epd\.)' "$FACADE" "$SMOKE"; then
  fail "new facade/smoke files must not import or call Pulp SD/FAT/SPI/display behavior"
fi

# Keep this facade read-only. Do not add mutating storage verbs to the Rust contract surface.
if rg -n '\b(write|delete|remove|mkdir|create|rename|truncate|append)(_storage|_file|_directory|_path|_state|\b)' "$FACADE" "$SMOKE"; then
  fail "read-only facade must not expose mutating storage operations"
fi

# The deliverable must not edit vendored active runtime code.
if [[ -d .git ]]; then
  if git status --short -- vendor/pulp-os vendor/smol-epub | rg -n '.' >/dev/null; then
    git status --short -- vendor/pulp-os vendor/smol-epub >&2
    fail "vendored Pulp/smol-epub files changed; this slice must keep active SD/FAT behavior in vendor code"
  fi
fi

# Optional formatting check when Rust tooling is installed. Static checks above are the source of truth
# for environments where cargo is unavailable.
if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all -- --check
fi

echo "storage_readonly_adapter_facade=ok"
