#!/usr/bin/env bash
set -euo pipefail

if [[ ! -d .phase_backups/phase18 ]]; then
  echo "ERROR: no .phase_backups/phase18 directory found" >&2
  exit 1
fi

latest="$(ls -1t .phase_backups/phase18/main.rs.main-before-runtime-extract.* 2>/dev/null | head -1 || true)"
if [[ -z "$latest" ]]; then
  echo "ERROR: no Phase 18 main.rs backup found" >&2
  exit 1
fi

cp target-xteink-x4/src/main.rs "target-xteink-x4/src/main.rs.bak-phase18-revert-$(date +%Y%m%d-%H%M%S)"
cp "$latest" target-xteink-x4/src/main.rs

echo "Restored target-xteink-x4/src/main.rs from $latest"
echo "Runtime adapter files were left in place for inspection: target-xteink-x4/src/runtime/"
