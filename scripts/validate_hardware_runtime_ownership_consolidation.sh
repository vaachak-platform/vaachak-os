#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from pathlib import Path
import re
import sys

ROOT = Path('.')

required_files = [
    'target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_ownership.rs',
    'target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_ownership_smoke.rs',
    'docs/architecture/hardware-runtime-ownership.md',
    'target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs',
    'target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs',
    'target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs',
    'target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs',
    'target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs',
    'target-xteink-x4/src/vaachak_x4/physical/spi_bus_pulp_backend.rs',
    'target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_pulp_backend.rs',
    'target-xteink-x4/src/vaachak_x4/physical/sd_fat_readonly_pulp_backend.rs',
    'target-xteink-x4/src/vaachak_x4/physical/display_pulp_backend.rs',
    'target-xteink-x4/src/vaachak_x4/physical/input_pulp_backend.rs',
]

for file_name in required_files:
    if not (ROOT / file_name).exists():
        sys.exit(f"hardware_runtime_ownership_consolidation validation failed: missing required file: {file_name}")

physical_mod = (ROOT / 'target-xteink-x4/src/vaachak_x4/physical/mod.rs').read_text()
contracts_mod = (ROOT / 'target-xteink-x4/src/vaachak_x4/contracts/mod.rs').read_text()
if 'pub mod hardware_runtime_ownership;' not in physical_mod:
    sys.exit("hardware_runtime_ownership_consolidation validation failed: physical module export missing")
if 'pub mod hardware_runtime_ownership_smoke;' not in contracts_mod:
    sys.exit("hardware_runtime_ownership_consolidation validation failed: contracts module export missing")

hw_path = ROOT / 'target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_ownership.rs'
smoke_path = ROOT / 'target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_ownership_smoke.rs'
doc_path = ROOT / 'docs/architecture/hardware-runtime-ownership.md'
hw = hw_path.read_text()
smoke = smoke_path.read_text()
doc = doc_path.read_text()

required_hw_patterns = [
    r'pub struct VaachakHardwareRuntimeOwnership',
    r'hardware_runtime_ownership_consolidation=ok',
    r'VaachakSpiBusRuntimeOwner',
    r'VaachakStorageProbeMountRuntimeOwner',
    r'VaachakSdFatRuntimeReadonlyOwner',
    r'VaachakDisplayRuntimeOwner',
    r'VaachakInputRuntimeOwner',
    r'pub const OWNER_COUNT:\s*usize\s*=\s*5;',
    r'pub const fn entries\(',
    r'pub const fn consolidation_ok\(',
    r'PulpCompatibility',
    r'vendor/pulp-os imported runtime',
]
for pattern in required_hw_patterns:
    if not re.search(pattern, hw, re.S):
        sys.exit(f"hardware_runtime_ownership_consolidation validation failed: missing pattern {pattern!r} in {hw_path}")

required_smoke_patterns = [
    r'pub struct VaachakHardwareRuntimeOwnershipSmoke',
    r'VaachakHardwareRuntimeOwnership::entries\(\)',
    r'VaachakHardwareRuntimeOwnership::consolidation_ok\(\)',
]
for pattern in required_smoke_patterns:
    if not re.search(pattern, smoke, re.S):
        sys.exit(f"hardware_runtime_ownership_consolidation validation failed: missing pattern {pattern!r} in {smoke_path}")

required_doc_terms = [
    'VaachakSpiBusRuntimeOwner',
    'VaachakStorageProbeMountRuntimeOwner',
    'VaachakSdFatRuntimeReadonlyOwner',
    'VaachakDisplayRuntimeOwner',
    'VaachakInputRuntimeOwner',
    'PulpCompatibility',
    'vendor/pulp-os imported runtime',
    'hardware_runtime_ownership_consolidation=ok',
]
for term in required_doc_terms:
    if term not in doc:
        sys.exit(f"hardware_runtime_ownership_consolidation validation failed: missing doc term {term!r}")

def read(path: str) -> str:
    return (ROOT / path).read_text()

def require_const(path: str, name: str, value: str) -> None:
    text = read(path)
    pattern = rf'pub\s+const\s+{re.escape(name)}\s*:[^=]+?=\s*{re.escape(value)}\s*;'
    if not re.search(pattern, text, re.S):
        sys.exit(
            f"hardware_runtime_ownership_consolidation validation failed: expected {name} = {value} in {path}"
        )

require_const('target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs', 'SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK', 'true')
require_const('target-xteink-x4/src/vaachak_x4/physical/spi_bus_runtime_owner.rs', 'ARBITRATION_POLICY_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs', 'STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK', 'true')
require_const('target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs', 'SD_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_owner.rs', 'FAT_BEHAVIOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs', 'SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK', 'true')
require_const('target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs', 'FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/sd_fat_runtime_readonly_owner.rs', 'FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs', 'DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK', 'true')
require_const('target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs', 'SSD1677_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs', 'DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs', 'DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/display_runtime_owner.rs', 'DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs', 'INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK', 'true')
require_const('target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs', 'ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs', 'BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs', 'DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK', 'false')
require_const('target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs', 'NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK', 'false')

# The consolidation module must stay declarative. It may reference lifecycle
# names, but it must not expose new executor functions.
forbidden_fn_pattern = re.compile(
    r'pub\s+(?:const\s+)?fn\s+(write|append|delete|rename|mkdir|format|mount|probe|draw|refresh|poll|sample|debounce|dispatch)\b',
    re.S,
)
for path, text in [(hw_path, hw), (smoke_path, smoke)]:
    match = forbidden_fn_pattern.search(text)
    if match:
        sys.exit(
            f"hardware_runtime_ownership_consolidation validation failed: forbidden executor-style function {match.group(0)!r} in {path}"
        )

# No consolidation file should import app/runtime UI code or vendor hardware code directly.
for forbidden in ['vendor::', 'pulp_os::', 'apps::reader', 'apps::home']:
    if forbidden in hw or forbidden in smoke:
        sys.exit(
            f"hardware_runtime_ownership_consolidation validation failed: forbidden direct dependency {forbidden!r}"
        )

print('hardware_runtime_ownership_consolidation=ok')
PY
