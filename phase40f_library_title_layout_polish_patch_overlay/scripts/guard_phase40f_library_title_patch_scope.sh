#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40f-library-title-patch-scope-guard.txt}"
status="ACCEPTED"
reason="LibraryTitlePatchScopeAllowed"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_polish_candidate_plan.rs ] || fail "MissingPhase40EPlan"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_rendering_patch.rs ] || fail "MissingPhase40DFooterPatch"
[ -f vendor/pulp-os/src/apps/files.rs ] || fail "MissingFilesApp"
[ -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs ] || fail "MissingTypedStateWiring"

old_footer_order="$(rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true)"
direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true)"

if [ -n "$old_footer_order" ]; then
  status="REJECTED"
  reason="OldFooterOrderFound"
elif [ -n "$direct_reader_writes" ]; then
  status="REJECTED"
  reason="DirectReaderWritesRemain"
fi

{
  echo "# Phase 40F Library Title Patch Scope Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo "old_footer_order_present=$([ -n "$old_footer_order" ] && echo yes || echo no)"
  echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)"
  echo "allowed_change=library-title-layout-only"
  echo "forbidden=title-source,footer-labels,input-mapping,write-lane,display-geometry,reader-pagination"
  echo "marker=phase40f=x4-library-title-layout-polish-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
