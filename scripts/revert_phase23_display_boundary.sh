#!/usr/bin/env bash
set -euo pipefail

latest="$(find .phase_backups/phase23-display-boundary -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort | tail -1 || true)"
if [[ -z "$latest" ]]; then
  echo "ERROR: no Phase 23 backup found under .phase_backups/phase23-display-boundary" >&2
  exit 1
fi

for f in \
  target-xteink-x4/src/runtime/display_boundary.rs \
  target-xteink-x4/src/runtime/mod.rs \
  target-xteink-x4/src/runtime/vaachak_runtime.rs; do
  if [[ -f "$latest/$f" ]]; then
    cp "$latest/$f" "$f"
    echo "restored $f"
  fi
done

echo "Reverted Phase 23 display boundary changes from $latest"
