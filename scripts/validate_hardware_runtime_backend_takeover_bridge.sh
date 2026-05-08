#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "hardware_runtime_backend_takeover_bridge validation failed: $*" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  local text="$1"
  local file="$2"
  grep -Fq "$text" "$file" || fail "missing text '$text' in $file"
}

require_regex() {
  local regex="$1"
  local file="$2"
  grep -Eq "$regex" "$file" || fail "missing pattern '$regex' in $file"
}

BACKEND="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend.rs"
PULP="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_pulp.rs"
TAKEOVER="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_backend_takeover_smoke.rs"
LIVE="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff.rs"
DOC="docs/architecture/hardware-runtime-backend-takeover.md"
LIVE_DOC="docs/architecture/hardware-runtime-executor-live-handoff.md"

require_file "$BACKEND"
require_file "$PULP"
require_file "$TAKEOVER"
require_file "$SMOKE"
require_file "$DOC"
require_file "$LIVE"

# Vaachak-owned backend traits.
require_text "pub trait VaachakSpiTransactionExecutor" "$BACKEND"
require_text "pub trait VaachakStorageProbeMountExecutor" "$BACKEND"
require_text "pub trait VaachakStorageFatAccessExecutor" "$BACKEND"
require_text "pub trait VaachakDisplayExecutor" "$BACKEND"
require_text "pub trait VaachakInputExecutor" "$BACKEND"
require_text "pub trait VaachakHardwareRuntimeBackend" "$BACKEND"
require_text "BACKEND_TRAITS_OWNER" "$BACKEND"
require_text "target-xteink-x4 Vaachak layer" "$BACKEND"

# Request/result structs.
require_text "VaachakSpiDisplayTransactionRequest" "$BACKEND"
require_text "VaachakSpiStorageTransactionRequest" "$BACKEND"
require_text "VaachakStorageProbeMountRequest" "$BACKEND"
require_text "VaachakStorageAccessRequest" "$BACKEND"
require_text "VaachakDisplayRequest" "$BACKEND"
require_text "VaachakInputRequest" "$BACKEND"
require_text "VaachakHardwareBackendHandoffResult" "$BACKEND"

# PulpCompatibility implements the Vaachak backend interface.
require_text "pub struct VaachakHardwareRuntimePulpCompatibilityBackend" "$PULP"
require_text "impl VaachakSpiTransactionExecutor for VaachakHardwareRuntimePulpCompatibilityBackend" "$PULP"
require_text "impl VaachakStorageProbeMountExecutor for VaachakHardwareRuntimePulpCompatibilityBackend" "$PULP"
require_text "impl VaachakStorageFatAccessExecutor for VaachakHardwareRuntimePulpCompatibilityBackend" "$PULP"
require_text "impl VaachakDisplayExecutor for VaachakHardwareRuntimePulpCompatibilityBackend" "$PULP"
require_text "impl VaachakInputExecutor for VaachakHardwareRuntimePulpCompatibilityBackend" "$PULP"
require_text "PulpCompatibility" "$PULP"
require_text "vendor/pulp-os imported runtime" "$PULP"

# Backend selection and takeover.
require_text "pub struct VaachakHardwareRuntimeBackendTakeover" "$TAKEOVER"
require_text "hardware_runtime_backend_takeover_bridge=ok" "$TAKEOVER"
require_text "ACTIVE_BACKEND" "$TAKEOVER"
require_text "PulpCompatibility" "$TAKEOVER"
require_text "execute_spi_display_transaction_handoff" "$TAKEOVER"
require_text "execute_spi_storage_transaction_handoff" "$TAKEOVER"
require_text "execute_storage_probe_mount_handoff" "$TAKEOVER"
require_text "execute_storage_directory_listing_handoff" "$TAKEOVER"
require_text "execute_storage_file_open_handoff" "$TAKEOVER"
require_text "execute_storage_file_read_handoff" "$TAKEOVER"
require_text "execute_display_full_refresh_handoff" "$TAKEOVER"
require_text "execute_display_partial_refresh_handoff" "$TAKEOVER"
require_text "execute_input_scan_handoff" "$TAKEOVER"
require_text "execute_input_navigation_handoff" "$TAKEOVER"
require_text "VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()" "$TAKEOVER"
require_text "VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()" "$TAKEOVER"

# Live handoff must call the backend takeover layer.
require_text "hardware_runtime_backend_takeover" "target-xteink-x4/src/vaachak_x4/physical/mod.rs"
require_text "hardware_runtime_backend_pulp" "target-xteink-x4/src/vaachak_x4/physical/mod.rs"
require_text "hardware_runtime_backend_takeover_smoke" "target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
require_text "VaachakHardwareRuntimeBackendTakeover" "$LIVE"
require_text "backend_takeover_preflight_ok" "$LIVE"
require_text "execute_storage_file_open_handoff().ok()" "$LIVE"
require_text "execute_display_full_refresh_handoff().ok()" "$LIVE"
require_text "execute_input_scan_handoff().ok()" "$LIVE"

# Static behavior-preservation checks.
require_text "DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false" "$TAKEOVER"
require_text "SD_MMC_FAT_ALGORITHM_REWRITTEN: bool = false" "$TAKEOVER"
require_text "INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false" "$TAKEOVER"
require_text "READER_FILE_BROWSER_UX_CHANGED: bool = false" "$TAKEOVER"
require_text "APP_NAVIGATION_CHANGED: bool = false" "$TAKEOVER"
require_text "DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false" "$TAKEOVER"
require_text "destructive_operation_allowed: false" "$TAKEOVER"

# Avoid accidental destructive/storage/display/input algorithm functions in the new backend layer.
if grep -RInE 'fn +(write|append|delete|rename|mkdir|format|remove_file|create_dir|draw_pixel|set_pixel|debounce_button)' \
  "$BACKEND" "$PULP" "$TAKEOVER"; then
  fail "unexpected low-level/destructive algorithm function in backend takeover layer"
fi

# Docs must describe the active backend and ownership split.
require_text "hardware_runtime_backend_takeover_bridge=ok" "$DOC"
require_text "PulpCompatibility" "$DOC"
require_text "backend owner = target-xteink-x4 Vaachak layer" "$DOC"
require_text "low-level executor = vendor/pulp-os imported runtime" "$DOC"
require_text "VaachakHardwareRuntimeBackendTakeover" "$LIVE_DOC"
require_text "PulpCompatibility" "$LIVE_DOC"

# Previous live-handoff validators should remain runnable when present.
[ -x scripts/validate_hardware_runtime_executor_live_path_handoff.sh ] || fail "missing previous live handoff validator"
[ -x scripts/validate_hardware_runtime_executor_runtime_use.sh ] || fail "missing previous runtime-use validator"

echo "hardware_runtime_backend_takeover_bridge=ok"
