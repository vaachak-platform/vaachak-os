#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 /path/to/vaachak-os" >&2
  exit 2
fi

repo="$1"
if [[ ! -f "$repo/Cargo.toml" ]]; then
  echo "ERROR: $repo does not look like vaachak-os repo root" >&2
  exit 1
fi

bundle_root="$(cd "$(dirname "$0")/.." && pwd)"

mkdir -p "$repo/docs/phase35" "$repo/scripts"
cp "$bundle_root/codex_prompt_phase35_physical_extraction.md" "$repo/"
cp "$bundle_root/AGENTS_phase35_addendum.md" "$repo/"
cp "$bundle_root/plans_phase35_physical_extraction.md" "$repo/"
cp "$bundle_root/docs/phase35/"*.md "$repo/docs/phase35/"
cp "$bundle_root/scripts/"*.sh "$repo/scripts/"
chmod +x "$repo/scripts/"*.sh

echo "Installed Phase 35 guidance files into $repo"
