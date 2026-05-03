#!/usr/bin/env bash
set -euo pipefail

grep -q 'PHASE40G_REPAIR_HOME_RECENT_W' vendor/pulp-os/src/apps/home.rs
grep -q 'BitmapDynLabel::<96>::new(self.recent_preview_region(), self.ui_fonts.body)' vendor/pulp-os/src/apps/home.rs
grep -q 'FULL_CONTENT_W' vendor/pulp-os/src/apps/home.rs

grep -q 'next_untitled_reader_title' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'PHASE40G_REPAIR_TITLE_KIND_EPUB' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'PHASE40G_REPAIR_TITLE_KIND_TEXT' vendor/pulp-os/kernel/src/kernel/dir_cache.rs
grep -q 'phase40g_repair_is_text_title_name' vendor/pulp-os/kernel/src/kernel/dir_cache.rs

grep -q 'scan_one_reader_title' vendor/pulp-os/src/apps/files.rs
grep -q 'scan_one_text_title' vendor/pulp-os/src/apps/files.rs
grep -q 'phase40g_repair_extract_text_title' vendor/pulp-os/src/apps/files.rs

grep -q 'pub mod state_io_home_library_title_repair;' target-xteink-x4/src/vaachak_x4/runtime.rs
grep -q 'PHASE_40G_REPAIR_MARKER' target-xteink-x4/src/vaachak_x4/runtime/state_io_home_library_title_repair.rs

echo "phase40g-repair-check=ok"
