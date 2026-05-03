#!/usr/bin/env bash
set -euo pipefail

grep -q 'phase40g-repair3=x4-disable-txt-body-title-scanning-ok' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'TXT/MD body-title scanning is disabled' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'continue;' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'PHASE_40G_REPAIR3_MARKER' target-xteink-x4/src/vaachak_x4/runtime/state_io_disable_txt_body_title_scanning_repair.rs
grep -q 'pub mod state_io_disable_txt_body_title_scanning_repair;' target-xteink-x4/src/vaachak_x4/runtime.rs

echo "phase40g-repair3-check=ok"
