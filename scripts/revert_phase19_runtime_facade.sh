#!/usr/bin/env bash
set -euo pipefail

latest="$(ls -dt .phase_backups/phase19-runtime-facade-* 2>/dev/null | head -1 || true)"
if [[ -z "$latest" ]]; then
  echo "ERROR: no Phase 19 backup found under .phase_backups/" >&2
  exit 1
fi

cp "$latest/main.rs" target-xteink-x4/src/main.rs
cp "$latest/runtime.mod.rs" target-xteink-x4/src/runtime/mod.rs
cp "$latest/pulp_runtime.rs" target-xteink-x4/src/runtime/pulp_runtime.rs

if [[ -f "$latest/vaachak_runtime.rs" ]]; then
  cp "$latest/vaachak_runtime.rs" target-xteink-x4/src/runtime/vaachak_runtime.rs
else
  rm -f target-xteink-x4/src/runtime/vaachak_runtime.rs
fi

echo "Reverted Phase 19 runtime facade using backup: $latest"
