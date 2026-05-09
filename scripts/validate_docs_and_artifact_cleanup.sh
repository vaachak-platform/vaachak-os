#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$(pwd)}"
cd "$ROOT"

require_file() {
  local path="$1"
  if [ ! -f "$path" ]; then
    echo "vaachak_docs_and_artifact_cleanup validation failed: missing file $path" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local text="$2"
  if ! grep -Fq "$text" "$path"; then
    echo "vaachak_docs_and_artifact_cleanup validation failed: missing text '$text' in $path" >&2
    exit 1
  fi
}

require_file README.md
require_file docs/architecture/vaachak-os-hardware-runtime-architecture.md
require_file docs/architecture/pulp-os-post-hardware-migration-scope.md
require_file docs/operations/final-hardware-validation.md
require_file docs/operations/github-upload-checklist.md
require_file scripts/cleanup_legacy_deliverable_artifacts.sh

require_text README.md 'vaachak_hardware_runtime_final_acceptance=ok'
require_text README.md 'VaachakNativeSpiPhysicalDriver'
require_text README.md 'VaachakNativeSsd1677PhysicalDriver'
require_text README.md 'VaachakNativeSdMmcPhysicalDriver'
require_text README.md 'VaachakNativeFatAlgorithmDriver'
require_text README.md 'cleanup_legacy_deliverable_artifacts.sh'

require_text docs/architecture/vaachak-os-hardware-runtime-architecture.md 'SPI physical driver'
require_text docs/architecture/vaachak-os-hardware-runtime-architecture.md 'SSD1677 display driver'
require_text docs/architecture/vaachak-os-hardware-runtime-architecture.md 'SD/MMC physical driver'
require_text docs/architecture/vaachak-os-hardware-runtime-architecture.md 'FAT algorithm driver'
require_text docs/architecture/vaachak-os-hardware-runtime-architecture.md 'Input physical sampling'
require_text docs/architecture/pulp-os-post-hardware-migration-scope.md 'non-hardware compatibility/import surfaces'
require_text docs/operations/final-hardware-validation.md 'Device smoke checklist'
require_text docs/operations/github-upload-checklist.md 'Clean generated artifacts'

if [ ! -x scripts/cleanup_legacy_deliverable_artifacts.sh ]; then
  echo 'vaachak_docs_and_artifact_cleanup validation failed: cleanup script is not executable' >&2
  exit 1
fi

# Warn rather than fail if the current overlay is still extracted, because the
# user may be validating immediately after applying this deliverable.
leftover_count=0
for dir in ./*; do
  [ -d "$dir" ] || continue
  base="$(basename "$dir")"
  [ "$base" = 'vaachak_final_docs_cleanup' ] && continue
  if [ -f "$dir/MANIFEST.txt" ] && [ -f "$dir/README-APPLY.md" ]; then
    leftover_count=$((leftover_count + 1))
  fi
done

printf 'vaachak_docs_and_artifact_cleanup=ok leftover_overlay_dirs=%s\n' "$leftover_count"
