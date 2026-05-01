#!/usr/bin/env bash
set -euo pipefail

if [[ ! -d ".phase_backups/phase24" ]]; then
  echo "No .phase_backups/phase24 directory found."
  exit 1
fi

if [[ -f ".phase_backups/phase24/mod.rs.bak" ]]; then
  cp ".phase_backups/phase24/mod.rs.bak" "target-xteink-x4/src/runtime/mod.rs"
fi

if [[ -f ".phase_backups/phase24/vaachak_runtime.rs.bak" ]]; then
  cp ".phase_backups/phase24/vaachak_runtime.rs.bak" "target-xteink-x4/src/runtime/vaachak_runtime.rs"
fi

if [[ -f ".phase_backups/phase24/boundary_contract.rs.bak" ]]; then
  cp ".phase_backups/phase24/boundary_contract.rs.bak" "target-xteink-x4/src/runtime/boundary_contract.rs"
else
  rm -f "target-xteink-x4/src/runtime/boundary_contract.rs"
fi

echo "Phase 24 boundary contract overlay reverted from .phase_backups/phase24."
