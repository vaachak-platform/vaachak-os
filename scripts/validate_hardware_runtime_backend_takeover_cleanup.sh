#!/bin/sh
set -eu

fail() {
  echo "hardware_runtime_backend_takeover_cleanup validation failed: $1" >&2
  exit 1
}

require_file() {
  [ -f "$1" ] || fail "missing file $1"
}

require_text() {
  file="$1"
  text="$2"
  if ! grep -Fq "$text" "$file"; then
    fail "missing text '$text' in $file"
  fi
}

require_regex() {
  file="$1"
  regex="$2"
  perl -0ne "BEGIN { \$found = 0 } if (/$regex/s) { \$found = 1 } END { exit(\$found ? 0 : 1) }" "$file" || fail "missing pattern '$regex' in $file"
}

require_absent_regex() {
  file="$1"
  regex="$2"
  if grep -Eq "$regex" "$file"; then
    fail "unexpected pattern '$regex' in $file"
  fi
}

PHYS="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover_cleanup.rs"
SMOKE="target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_backend_takeover_cleanup_smoke.rs"
DOC="docs/architecture/hardware-runtime-backend-takeover-cleanup.md"
TAKEOVER_DOC="docs/architecture/hardware-runtime-backend-takeover.md"
PHYS_MOD="target-xteink-x4/src/vaachak_x4/physical/mod.rs"
CONTRACTS_MOD="target-xteink-x4/src/vaachak_x4/contracts/mod.rs"
BACKEND="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend.rs"
PULP="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_pulp.rs"
TAKEOVER="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_backend_takeover.rs"
LIVE="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff.rs"
LIVE_CLEANUP="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff_cleanup.rs"
RUNTIME_USE="target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use.rs"

require_file "$PHYS"
require_file "$SMOKE"
require_file "$DOC"
require_file "$TAKEOVER_DOC"
require_file "$BACKEND"
require_file "$PULP"
require_file "$TAKEOVER"
require_file "$LIVE"
require_file "$LIVE_CLEANUP"
require_file "$RUNTIME_USE"
require_file "scripts/cleanup_hardware_runtime_backend_takeover_artifacts.sh"
require_file "scripts/validate_hardware_runtime_backend_takeover_bridge.sh"

require_text "$PHYS_MOD" "pub mod hardware_runtime_backend_takeover_cleanup;"
require_text "$CONTRACTS_MOD" "pub mod hardware_runtime_backend_takeover_cleanup_smoke;"

require_text "$PHYS" "HARDWARE_RUNTIME_BACKEND_TAKEOVER_CLEANUP_MARKER"
require_text "$PHYS" "hardware_runtime_backend_takeover_cleanup=ok"
require_text "$PHYS" "VaachakHardwareRuntimeBackendTakeover::takeover_ok()"
require_text "$PHYS" "VaachakHardwareRuntimeBackendTakeover::backend_interface_calls_ok()"
require_text "$PHYS" "VaachakHardwareRuntimeBackendInterface::interface_ok()"
require_text "$PHYS" "VaachakHardwareRuntimePulpCompatibilityBackend::backend_ok()"
require_text "$PHYS" "VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok()"
require_text "$PHYS" "VaachakHardwareRuntimeExecutorLiveHandoffCleanup::live_handoff_cleanup_ok()"
require_text "$PHYS" "VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()"
require_regex "$PHYS" "VaachakHardwareRuntimeExecutorAcceptance::[[:space:]]*acceptance_ok\(\)"
require_text "$PHYS" "ACTIVE_BACKEND_NAME"
require_text "$PHYS" "PulpCompatibility"
require_text "$PHYS" "LOW_LEVEL_EXECUTOR_OWNER"
require_text "$PHYS" "vendor/pulp-os imported runtime"
require_text "$PHYS" "PHYSICAL_SPI_TRANSFER_CHANGED: bool = false"
require_text "$PHYS" "CHIP_SELECT_TOGGLING_CHANGED: bool = false"
require_text "$PHYS" "SD_MMC_FAT_ALGORITHM_CHANGED: bool = false"
require_text "$PHYS" "DISPLAY_DRAW_ALGORITHM_CHANGED: bool = false"
require_text "$PHYS" "INPUT_DEBOUNCE_NAVIGATION_CHANGED: bool = false"
require_text "$PHYS" "READER_FILE_BROWSER_UX_CHANGED: bool = false"
require_text "$PHYS" "APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false"
require_text "$PHYS" "DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false"

require_text "$SMOKE" "backend_takeover_cleanup_ok()"
require_text "$SMOKE" "hardware_runtime_backend_takeover_cleanup=ok"

require_text "$DOC" "hardware_runtime_backend_takeover_cleanup=ok"
require_text "$DOC" "hardware_runtime_backend_takeover_bridge"
require_text "$DOC" "PulpCompatibility"
require_text "$DOC" "backend owner = target-xteink-x4 Vaachak layer"
require_text "$DOC" "low-level executor = vendor/pulp-os imported runtime"
require_text "$TAKEOVER_DOC" "hardware_runtime_backend_takeover_bridge=ok"
require_text "$TAKEOVER_DOC" "hardware-runtime-backend-takeover-cleanup.md"
require_text "$TAKEOVER_DOC" "hardware_runtime_backend_takeover_cleanup=ok"
require_text "$TAKEOVER_DOC" "PulpCompatibility"

require_text "$BACKEND" "pub trait VaachakHardwareRuntimeBackend"
require_text "$PULP" "impl VaachakSpiTransactionExecutor for VaachakHardwareRuntimePulpCompatibilityBackend"
require_text "$PULP" "impl VaachakInputExecutor for VaachakHardwareRuntimePulpCompatibilityBackend"
require_text "$PULP" "PulpCompatibility"
require_text "$TAKEOVER" "hardware_runtime_backend_takeover_bridge=ok"
require_text "$TAKEOVER" "VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()"
require_text "$LIVE" "VaachakHardwareRuntimeBackendTakeover"
require_text "$LIVE" "backend_takeover_preflight_ok"

require_absent_regex "$PHYS" "fn[[:space:]]+(write|append|delete|rename|mkdir|format|draw|refresh|mount|probe|read|debounce_button)"

if grep -RInE "fn +(write|append|delete|rename|mkdir|format|draw_pixel|set_pixel|debounce_button)" \
  "$PHYS" "$SMOKE"; then
  fail "cleanup introduced native/destructive hardware or storage behavior"
fi

[ -x scripts/validate_hardware_runtime_backend_takeover_bridge.sh ] || fail "missing backend takeover bridge validator"
[ -x scripts/validate_hardware_runtime_executor_live_path_handoff.sh ] || fail "missing live handoff validator"
[ -x scripts/validate_hardware_runtime_executor_runtime_use.sh ] || fail "missing runtime-use validator"

echo "hardware_runtime_backend_takeover_cleanup=ok"
