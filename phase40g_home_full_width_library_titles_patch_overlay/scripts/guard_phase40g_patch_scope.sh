#!/usr/bin/env bash
set -euo pipefail
OUT="${OUT:-/tmp/phase40g-patch-scope-guard.txt}"
status="ACCEPTED"; reason="PatchScopeAllowed"
fail(){ status="REJECTED"; reason="$1"; }
[ -f vendor/pulp-os/src/apps/home.rs ] || fail MissingHomeApp
[ -f vendor/pulp-os/src/apps/files.rs ] || fail MissingFilesApp
[ -f vendor/pulp-os/kernel/src/kernel/dir_cache.rs ] || fail MissingDirCache
[ -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs ] || fail MissingTypedStateWiring
old_footer_order="$(rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true)"
direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true)"
[ -z "$old_footer_order" ] || fail OldFooterOrderFound
[ -z "$direct_reader_writes" ] || fail DirectReaderWritesRemain
{
 echo "# Phase 40G Patch Scope Guard"; echo "status=$status"; echo "reason=$reason";
 echo "old_footer_order_present=$([ -n "$old_footer_order" ] && echo yes || echo no)";
 echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)";
 echo "marker=phase40g=x4-home-full-width-library-title-patch-ok";
} | tee "$OUT"
[ "$status" = ACCEPTED ] || exit 3
