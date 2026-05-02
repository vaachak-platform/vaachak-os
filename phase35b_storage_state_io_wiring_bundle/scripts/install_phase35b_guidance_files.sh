#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 /path/to/vaachak-os"
  exit 1
fi

repo="$1"

if [[ ! -f "$repo/Cargo.toml" ]]; then
  echo "ERROR: $repo does not look like vaachak-os repo root"
  exit 1
fi

mkdir -p "$repo/docs/phase35b" "$repo/scripts"

cp codex_prompt_phase35b_storage_state_io_wiring.md "$repo/"
cp AGENTS_phase35b_addendum.md "$repo/"
cp plans_phase35b_storage_state_io_wiring.md "$repo/"

cp docs/phase35b/*.md "$repo/docs/phase35b/"
cp scripts/check_imported_reader_runtime_sync_phase35b.sh "$repo/scripts/"
cp scripts/check_phase35b_storage_state_io_wiring.sh "$repo/scripts/"
cp scripts/check_phase35b_no_vendor_or_hardware_regression.sh "$repo/scripts/"
cp scripts/revert_phase35b_storage_state_io_wiring.sh "$repo/scripts/"

chmod +x "$repo/scripts/check_imported_reader_runtime_sync_phase35b.sh" \
         "$repo/scripts/check_phase35b_storage_state_io_wiring.sh" \
         "$repo/scripts/check_phase35b_no_vendor_or_hardware_regression.sh" \
         "$repo/scripts/revert_phase35b_storage_state_io_wiring.sh"

echo "Installed Phase 35B guidance files into $repo"
