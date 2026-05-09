#!/usr/bin/env bash
set -euo pipefail

removed=0
remove_file() {
  local path="$1"
  if [ -e "$path" ]; then
    rm -f "$path"
    removed=$((removed + 1))
  fi
}

# Early transition/extraction notes that no longer describe the current architecture cleanly.
remove_file docs/architecture-overview.md
remove_file docs/extraction-note.md
remove_file docs/first-x4-slice.md
remove_file docs/hal-traits-refined-against-real-x4.md
remove_file docs/x4-hal-porting-backlog.md
remove_file docs/x4-hal-source-map.md
remove_file docs/next-steps.md

# Generated overlay artifacts at repository root.
find . -maxdepth 1 -type d \( \
  -name '*_overlay' -o \
  -name '*_fix' -o \
  -name '*_cleanup' -o \
  -name '*_migration' -o \
  -name 'vaachak_docs_roadmap_reset' \
\) -exec sh -c '
  for path do
    if [ -f "$path/MANIFEST.txt" ] || [ -f "$path/README-APPLY.md" ]; then
      rm -rf "$path"
      echo "$path"
    fi
  done
' sh {} + | while IFS= read -r _; do :; done

find . -maxdepth 1 -type f \( -name '*.zip' -o -name '*_fix*.zip' -o -name '*_cleanup*.zip' -o -name '*_migration*.zip' \) -delete
find . -type d -name '__pycache__' -prune -exec rm -rf {} +

printf 'legacy_documentation_artifact_cleanup=ok removed_docs=%s\n' "$removed"
