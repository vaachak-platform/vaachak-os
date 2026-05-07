#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-.}"
OVERLAY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
for f in \
  vendor/pulp-os/src/apps/reader/mod.rs \
  vendor/pulp-os/src/apps/reader/prepared_txt.rs \
  vendor/pulp-os/src/apps/upload.rs \
  vendor/pulp-os/kernel/src/drivers/storage.rs \
  vendor/pulp-os/assets/upload.html \
  vendor/pulp-os/src/apps/home.rs
 do
   mkdir -p ".vaachak_backups/$(dirname "$f")"
   if [ -f "$f" ]; then
     cp "$f" ".vaachak_backups/$f.$(date +%Y%m%d-%H%M%S).bak"
   fi
   mkdir -p "$(dirname "$f")"
   cp "$OVERLAY_DIR/files/$f" "$f"
 done
printf '%s\n' "Applied transfer + prepared reader diagnostic fix overlay."
