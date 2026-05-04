#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OUT="/tmp/phase39g-runtime-file-api-candidates.txt"

{
  echo "# Phase 39G Runtime File API Candidate Search"
  echo "# root: $ROOT"
  echo
  echo "## likely file/write APIs"
  rg -n --hidden --glob '!target/**' --glob '!*.zip' \
    'write_all|write_record|write_file|open_file|create_file|remove_file|rename|flush|FileMode|VolumeManager|embedded_sdmmc|SdCard|Controller|Dir|File|STATE|BKMK|RECENT|TITLES|PRG|THM|MTA|BKM|BMIDX' \
    vendor/pulp-os target-xteink-x4 hal-xteink-x4 core 2>/dev/null || true
  echo
  echo "## likely runtime/kernel structs"
  rg -n --hidden --glob '!target/**' --glob '!*.zip' \
    'struct .*Kernel|struct .*Handle|impl .*Kernel|impl .*File|pub fn .*file|fn .*file|DirCache|State|Storage|Sd|Fat' \
    vendor/pulp-os target-xteink-x4 hal-xteink-x4 core 2>/dev/null || true
} | tee "$OUT"

echo
echo "Wrote: $OUT"
