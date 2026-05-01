#!/usr/bin/env bash
set -euo pipefail

runtime_dir="target-xteink-x4/src/runtime"

for f in \
  "$runtime_dir/mod.rs" \
  "$runtime_dir/vaachak_runtime.rs" \
  "$runtime_dir/display_contract_smoke.rs"
do
  b="$f.bak-phase27"
  if [[ -f "$b" ]]; then
    cp "$b" "$f"
    echo "restored $f from $b"
  fi
done

rm -f scripts/check_reader_runtime_sync_phase27.sh \
      scripts/check_phase27_display_contract_smoke.sh \
      scripts/revert_phase27_display_contract_smoke.sh

echo "Reverted Phase 27 overlay where backups were available"
