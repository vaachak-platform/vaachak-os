#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

echo "== direct sleep bitmap loader audit =="

grep -n "pub mod sleep_bitmap;" vendor/pulp-os/kernel/src/kernel/mod.rs
grep -n "read_file_chunk_in_subdir" vendor/pulp-os/kernel/src/drivers/storage.rs
grep -n "read_file_start_in_subdir" vendor/pulp-os/kernel/src/drivers/storage.rs
grep -n "render_daily_sleep_bitmap" vendor/pulp-os/kernel/src/kernel/scheduler.rs
grep -n "resolve_sleep_bitmap" vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs

echo "OK: direct sleep bitmap loader wiring is present"
