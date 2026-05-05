#!/usr/bin/env bash
set -euo pipefail

repo="${1:-.}"
cd "$repo"

if [ ! -f Cargo.toml ]; then
  echo "error: run from the vaachak-os repository root or pass the repository path" >&2
  exit 1
fi

file="vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs"
if [ ! -f "$file" ]; then
  echo "error: missing $file" >&2
  exit 1
fi

stamp="$(date +%Y%m%d-%H%M%S)"
backup_dir=".vaachak_pre_github_backups/sleep_image_refresh_policy_resync_${stamp}"
mkdir -p "$backup_dir/$(dirname "$file")"
cp "$file" "$backup_dir/$file"

cp -f sleep_image_refresh_policy_resync_overlay/files/vendor/pulp-os/kernel/src/kernel/sleep_bitmap.rs "$file"

cargo fmt --all

echo "Resynced sleep bitmap refresh policy helpers. Backup: $backup_dir"
