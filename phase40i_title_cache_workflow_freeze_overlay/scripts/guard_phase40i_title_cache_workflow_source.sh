#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40i-title-cache-workflow-source-guard.txt}"
status="ACCEPTED"
reason="TitleCacheWorkflowSourceGuardAccepted"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f vendor/pulp-os/kernel/src/kernel/dir_cache.rs ] || fail "MissingDirCache"
[ -f vendor/pulp-os/src/apps/files.rs ] || fail "MissingFilesApp"

# Phase 40G Repair 3 disabled TXT/MD candidate scanning by continuing instead of returning TEXT kind.
if ! grep -q 'phase40g-repair3=x4-disable-txt-body-title-scanning-ok' vendor/pulp-os/kernel/src/kernel/dir_cache.rs; then
  fail "MissingPhase40GRepair3Marker"
fi

if ! grep -q 'TXT/MD body-title scanning is disabled' vendor/pulp-os/kernel/src/kernel/dir_cache.rs; then
  fail "MissingTxtBodyScanningDisabledComment"
fi

# Reject if dir_cache can still return TXT/MD title candidates to the scanner.
if python3 - <<'PY'
from pathlib import Path
text = Path("vendor/pulp-os/kernel/src/kernel/dir_cache.rs").read_text()
start = text.find("pub fn next_untitled_reader_title")
if start < 0:
    raise SystemExit(1)
end = text.find("\n    }", start)
block = text[start:end if end > start else len(text)]
raise SystemExit(0 if "PHASE40G_REPAIR_TITLE_KIND_TEXT" in block and "return Some" in block.split("PHASE40G_REPAIR_TITLE_KIND_TEXT")[0][-120:] else 1)
PY
then
  fail "TxtBodyTitleCandidateStillReturned"
fi

old_footer_order="$(rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true)"
direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true)"

if [ -n "$old_footer_order" ]; then
  fail "OldFooterOrderFound"
elif [ -n "$direct_reader_writes" ]; then
  fail "DirectReaderWritesRemain"
fi

{
  echo "# Phase 40I Title Cache Workflow Source Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo "txt_body_scanning_disabled_marker=yes"
  echo "old_footer_order_present=$([ -n "$old_footer_order" ] && echo yes || echo no)"
  echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)"
  echo "marker=phase40i=x4-title-cache-workflow-freeze-ok"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
