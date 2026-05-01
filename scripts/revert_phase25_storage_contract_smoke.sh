#!/usr/bin/env bash
set -euo pipefail

echo "Phase 25 revert helper"
echo "This removes the Phase 25 storage_state_contract module and marker call."

rm -f target-xteink-x4/src/runtime/storage_state_contract.rs

python3 - <<'PY'
from pathlib import Path

modrs = Path("target-xteink-x4/src/runtime/mod.rs")
if modrs.exists():
    lines = [line for line in modrs.read_text().splitlines() if line.strip() != "pub mod storage_state_contract;"]
    modrs.write_text("\n".join(lines) + "\n")

facade = Path("target-xteink-x4/src/runtime/vaachak_runtime.rs")
if facade.exists():
    s = facade.read_text()
    s = s.replace("        crate::runtime::storage_state_contract::VaachakStorageStateContract::emit_contract_marker();\n", "")
    facade.write_text(s)
PY

echo "Phase 25 overlay reverted. Re-run cargo fmt/check/clippy."
