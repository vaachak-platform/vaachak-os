#!/usr/bin/env bash
set -euo pipefail

grep -q 'phase40g-repair2=x4-text-title-cache-safety-ok' vendor/pulp-os/src/apps/files.rs
grep -q 'trimmed\[..title_prefix.len()\].eq_ignore_ascii_case(title_prefix)' vendor/pulp-os/src/apps/files.rs
grep -q 'pub mod state_io_text_title_cache_safety_repair;' target-xteink-x4/src/vaachak_x4/runtime.rs
grep -q 'PHASE_40G_REPAIR2_MARKER' target-xteink-x4/src/vaachak_x4/runtime/state_io_text_title_cache_safety_repair.rs

echo "phase40g-repair2-check=ok"
