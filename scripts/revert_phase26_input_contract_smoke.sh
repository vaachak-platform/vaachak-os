#!/usr/bin/env bash
set -euo pipefail

if [[ ! -d .phase_backups/phase26 ]]; then
  echo "ERROR: .phase_backups/phase26 not found" >&2
  exit 1
fi

latest_mod="$(ls -1t .phase_backups/phase26/mod.rs.*.bak 2>/dev/null | head -1 || true)"
latest_facade="$(ls -1t .phase_backups/phase26/vaachak_runtime.rs.*.bak 2>/dev/null | head -1 || true)"
latest_contract="$(ls -1t .phase_backups/phase26/input_contract_smoke.rs.*.bak 2>/dev/null | head -1 || true)"

if [[ -n "$latest_mod" ]]; then
  cp "$latest_mod" target-xteink-x4/src/runtime/mod.rs
  echo "Restored target-xteink-x4/src/runtime/mod.rs from $latest_mod"
fi

if [[ -n "$latest_facade" ]]; then
  cp "$latest_facade" target-xteink-x4/src/runtime/vaachak_runtime.rs
  echo "Restored target-xteink-x4/src/runtime/vaachak_runtime.rs from $latest_facade"
fi

if [[ -n "$latest_contract" ]]; then
  cp "$latest_contract" target-xteink-x4/src/runtime/input_contract_smoke.rs
  echo "Restored target-xteink-x4/src/runtime/input_contract_smoke.rs from $latest_contract"
else
  rm -f target-xteink-x4/src/runtime/input_contract_smoke.rs
  echo "Removed target-xteink-x4/src/runtime/input_contract_smoke.rs"
fi
