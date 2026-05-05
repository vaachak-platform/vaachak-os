#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  scripts/prepare_daily_mantra_sd_assets.sh /Volumes/SDCARD [extra generator args]

Examples:
  scripts/prepare_daily_mantra_sd_assets.sh /Volumes/X4SD
  scripts/prepare_daily_mantra_sd_assets.sh /Volumes/X4SD --devanagari-font "$HOME/Fonts/NotoSansDevanagari-Regular.ttf"
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || $# -lt 1 ]]; then
  usage
  exit 0
fi

sd_root="$1"
shift || true

if [[ ! -d "$sd_root" ]]; then
  echo "error: SD root does not exist: $sd_root" >&2
  exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source_file="$repo_root/assets/text/daily_hindu_mantras.txt"
generator="$repo_root/tools/daily-mantra-sleep-images/generate_daily_sleep_images.py"
output_dir="$sd_root/sleep/daily"

if [[ ! -f "$source_file" ]]; then
  echo "error: missing $source_file" >&2
  exit 1
fi

if [[ ! -x "$generator" ]]; then
  echo "error: missing executable generator: $generator" >&2
  exit 1
fi

mkdir -p "$output_dir"
python3 "$generator" --source "$source_file" --output "$output_dir" "$@"

"$repo_root/scripts/verify_daily_mantra_sd_assets.sh" "$sd_root"
