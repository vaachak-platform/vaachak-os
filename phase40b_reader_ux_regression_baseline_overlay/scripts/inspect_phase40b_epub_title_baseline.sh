#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT="${OUT:-/tmp/phase40b-epub-title-baseline.txt}"

if [ ! -d "$SD" ]; then
  echo "SD mount not found: $SD" >&2
  exit 2
fi

{
  echo "# Phase 40B EPUB Title Baseline"
  echo "sd=$SD"
  echo
  echo "## EPUB files on SD root"
  find "$SD" -maxdepth 1 -type f \( -iname '*.epub' -o -iname '*.epu' \) -printf '%f\t%s bytes\n' | sort || true
  echo
  echo "## _X4 title/recent cache strings"
  for file in "$SD/_X4/TITLES.BIN" "$SD/_X4/RECENT" "$SD/_X4/state/RECENT.TXT" "$SD/_PULP/TITLES.BIN" "$SD/_PULP/RECENT"; do
    if [ -f "$file" ]; then
      echo "--- $file ---"
      strings -a "$file" 2>/dev/null | head -120 || true
    fi
  done
  echo
  echo "## _X4 state files"
  if [ -d "$SD/_X4/state" ]; then
    find "$SD/_X4/state" -maxdepth 1 -type f -printf '%f\t%s bytes\n' | sort || true
  elif [ -d "$SD/_X4/STATE" ]; then
    find "$SD/_X4/STATE" -maxdepth 1 -type f -printf '%f\t%s bytes\n' | sort || true
  else
    echo "no _X4 state directory found"
  fi
  echo
  echo "marker=phase40b=x4-reader-ux-regression-baseline-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
