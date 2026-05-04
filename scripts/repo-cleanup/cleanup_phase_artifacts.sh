#!/usr/bin/env bash
set -euo pipefail

MODE="${MODE:-dry-run}" # dry-run | archive | delete
ARCHIVE_DIR="${ARCHIVE_DIR:-_archive/phase-artifacts}"
OUT="${OUT:-/tmp/phase40k-cleanup-phase-artifacts.txt}"

items_file="$(mktemp)"
find . -maxdepth 1 \( \
  -type d -name 'phase*_overlay' -o \
  -type d -name 'phase*_repair*_overlay' -o \
  -type f -name 'phase*.zip' \
\) -print | sort > "$items_file"

count="$(wc -l < "$items_file" | tr -d ' ')"

{
  echo "# Phase Artifact Cleanup"
  echo "mode=$MODE"
  echo "count=$count"
  echo "archive_dir=$ARCHIVE_DIR"
  echo
  cat "$items_file"
} | tee "$OUT"

if [ "$MODE" = "dry-run" ]; then
  echo "Dry run only. Use MODE=archive or MODE=delete."
  rm -f "$items_file"
  exit 0
fi

if [ "$MODE" = "archive" ]; then
  mkdir -p "$ARCHIVE_DIR"
  while IFS= read -r item; do
    [ -e "$item" ] || continue
    mv -v "$item" "$ARCHIVE_DIR/"
  done < "$items_file"
elif [ "$MODE" = "delete" ]; then
  while IFS= read -r item; do
    [ -e "$item" ] || continue
    rm -rfv "$item"
  done < "$items_file"
else
  echo "Unknown MODE=$MODE" >&2
  rm -f "$items_file"
  exit 2
fi

rm -f "$items_file"
echo "phase40k=x4-repository-cleanup-new-device-deploy-baseline-ok"
