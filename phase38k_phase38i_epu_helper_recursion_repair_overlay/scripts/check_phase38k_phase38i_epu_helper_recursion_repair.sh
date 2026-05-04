#!/usr/bin/env bash
set -euo pipefail

if rg -n 'if phase38i_is_epub_or_epu_name\(name\)' \
  vendor/pulp-os/kernel/src/kernel/dir_cache.rs \
  vendor/pulp-os/src/apps/files.rs; then
    echo "recursive helper call still present" >&2
    exit 1
fi

rg -n 'fn phase38i_is_epub_or_epu_name' \
  vendor/pulp-os/kernel/src/kernel/dir_cache.rs \
  vendor/pulp-os/src/apps/files.rs

echo "phase38k-phase38i-repair-check=ok"
