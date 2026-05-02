#!/usr/bin/env bash
set -euo pipefail

repo="${1:-}"
if [[ -z "$repo" ]]; then
  echo "Usage: $0 /path/to/vaachak-os"
  exit 1
fi

mkdir -p "$repo/docs/phase35_full" "$repo/scripts"
cp codex_prompt_phase35_full_physical_extraction.md "$repo/"
cp AGENTS_phase35_full_addendum.md "$repo/"
cp plans_phase35_full_physical_extraction.md "$repo/"
cp docs/phase35_full/*.md "$repo/docs/phase35_full/"
cp scripts/check_phase35_full_*.sh "$repo/scripts/"
cp scripts/revert_phase35_full_physical_extraction.sh "$repo/scripts/"
chmod +x "$repo"/scripts/check_phase35_full_*.sh "$repo/scripts/revert_phase35_full_physical_extraction.sh"

echo "Installed Phase 35 Full guidance files into $repo"
