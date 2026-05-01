#!/usr/bin/env bash
set -euo pipefail

rm -f target-xteink-x4/src/runtime/boundary_contract_smoke.rs

python3 - <<'PY'
from pathlib import Path

modrs = Path("target-xteink-x4/src/runtime/mod.rs")
if modrs.exists():
    lines = [line for line in modrs.read_text().splitlines() if line.strip() != "pub mod boundary_contract_smoke;"]
    modrs.write_text("\n".join(lines) + ("\n" if lines else ""))

facade = Path("target-xteink-x4/src/runtime/vaachak_runtime.rs")
if facade.exists():
    s = facade.read_text()
    s = s.replace(
        "        crate::runtime::boundary_contract_smoke::VaachakBoundaryContractSmoke::emit_boot_marker();\n",
        "",
    )
    facade.write_text(s)
PY

echo "Reverted Phase 28 boundary contract smoke files/calls. Backups, if any, remain in .phase_backups/phase28."
