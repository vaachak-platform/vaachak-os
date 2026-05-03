#!/usr/bin/env bash
set -euo pipefail

grep -q 'phase40h=x4-host-title-map-txt-display-names-ok' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'PHASE40H_TITLE_MAP_FILE' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'phase40h_load_host_title_map' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'self.phase40h_load_host_title_map(sd);' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'PHASE_40H_HOST_TITLE_MAP_MARKER' target-xteink-x4/src/vaachak_x4/runtime/state_io_host_title_map_txt_display_names.rs
grep -q 'pub mod state_io_host_title_map_txt_display_names;' target-xteink-x4/src/vaachak_x4/runtime.rs

echo "phase40h-check=ok"
