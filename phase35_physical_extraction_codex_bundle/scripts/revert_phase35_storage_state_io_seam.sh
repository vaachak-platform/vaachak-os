#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
cd "$repo_root"

rm -rf target-xteink-x4/src/vaachak_x4/io
rm -rf docs/phase35
rm -f scripts/check_phase35_physical_extraction_plan.sh \
      scripts/check_phase35_storage_state_io_seam.sh \
      scripts/check_phase35_no_hardware_regression.sh \
      scripts/check_imported_reader_runtime_sync_phase35.sh \
      scripts/revert_phase35_storage_state_io_seam.sh

# Leave source edits to imported runtime/manual work for git restore by operator.
echo "Removed Phase 35 scaffold docs/scripts/io directory. Use git restore for any source edits if needed."
