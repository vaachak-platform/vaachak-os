#!/usr/bin/env bash
set -euo pipefail

current_dir="hardware_physical_full_migration_cleanup"
removed=0

# Remove extracted overlay folders from previous deliverables. We detect generated
# overlays by their repo-root MANIFEST.txt + README-APPLY.md pair and keep the
# currently running cleanup overlay so the user can inspect it if desired.
for entry in ./*; do
  [ -d "$entry" ] || continue
  name="${entry#./}"
  [ "$name" != "$current_dir" ] || continue
  if [ -f "$entry/MANIFEST.txt" ] && [ -f "$entry/README-APPLY.md" ]; then
    rm -rf "$entry"
    removed=$((removed + 1))
  fi
done

# Remove generated overlay zip files. We only remove zip archives that contain
# the generated overlay MANIFEST.txt + README-APPLY.md pair, so unrelated zip
# archives are preserved.
for zipfile in ./*.zip; do
  [ -f "$zipfile" ] || continue
  if unzip -Z -1 "$zipfile" >/tmp/vaachak-overlay-zip-list.$$ 2>/dev/null; then
    if grep -Eq '(^|/)MANIFEST\.txt$' /tmp/vaachak-overlay-zip-list.$$ \
      && grep -Eq '(^|/)README-APPLY\.md$' /tmp/vaachak-overlay-zip-list.$$; then
      rm -f "$zipfile"
      removed=$((removed + 1))
    fi
  fi
  rm -f /tmp/vaachak-overlay-zip-list.$$
done

# Remove accidental Python cache folders created under generated overlays if any
# remain outside the current cleanup folder.
find . -maxdepth 3 -type d -name __pycache__ \
  ! -path "./target/*" \
  ! -path "./vendor/*" \
  ! -path "./$current_dir/*" \
  -prune -exec rm -rf {} + 2>/dev/null || true

printf 'hardware_physical_full_migration_cleanup_artifacts=ok removed=%s\n' "$removed"
