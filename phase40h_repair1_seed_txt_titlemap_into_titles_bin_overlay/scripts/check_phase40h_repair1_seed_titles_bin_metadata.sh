#!/usr/bin/env bash
set -euo pipefail

grep -q 'PHASE_40H_REPAIR1_MARKER' target-xteink-x4/src/vaachak_x4/runtime/state_io_seed_txt_titlemap_into_titles_bin_repair.rs
grep -q 'pub mod state_io_seed_txt_titlemap_into_titles_bin_repair;' target-xteink-x4/src/vaachak_x4/runtime.rs
grep -q 'phase40h_repair1_acceptance_status' target-xteink-x4/src/vaachak_x4/runtime/state_io_seed_txt_titlemap_into_titles_bin_repair_acceptance.rs

echo "phase40h-repair1-metadata-check=ok"
