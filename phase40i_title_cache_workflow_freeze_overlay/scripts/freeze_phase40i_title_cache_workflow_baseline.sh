#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/SD_CARD}"
OUT="${OUT:-/tmp/phase40i-title-cache-workflow-baseline.txt}"

./phase40i_title_cache_workflow_freeze_overlay/scripts/guard_phase40i_title_cache_workflow_source.sh >/dev/null
SD="$SD" ./phase40i_title_cache_workflow_freeze_overlay/scripts/inspect_phase40i_title_cache_workflow.sh >/dev/null

titlemap_lines=0
txt_title_lines=0
epub_title_lines=0
bad_phrase_count=0

if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
  titlemap_lines="$(wc -l < "$SD/_X4/TITLEMAP.TSV" | tr -d ' ')"
fi

if [ -f "$SD/_X4/TITLES.BIN" ]; then
  txt_title_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n '\.TXT|\.MD|POIROT~|THEMUR~|THESIG~|Poirot Investigates|Roger Ackroyd|Sign of the Four' || true) | wc -l | tr -d ' ')"
  epub_title_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n '\.EPU|\.EPUB|Alice|Dracula|Sherlock|Baskervilles' || true) | wc -l | tr -d ' ')"
  bad_phrase_count="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by|START OF THE PROJECT GUTENBERG' || true) | wc -l | tr -d ' ')"
fi

status="ACCEPTED"
reason="TitleCacheWorkflowBaselineAccepted"

if [ ! -f "$SD/_X4/TITLEMAP.TSV" ]; then
  status="REJECTED"; reason="TitleMapMissing"
elif [ ! -f "$SD/_X4/TITLES.BIN" ]; then
  status="REJECTED"; reason="TitlesBinMissing"
elif [ "$titlemap_lines" = "0" ]; then
  status="REJECTED"; reason="TitleMapEmpty"
elif [ "$txt_title_lines" = "0" ]; then
  status="REJECTED"; reason="TxtTitleLinesMissingFromTitlesBin"
elif [ "$bad_phrase_count" != "0" ]; then
  status="REJECTED"; reason="BadBodyTitlePhrasesCached"
fi

{
  echo "# Phase 40I Title Cache Workflow Baseline"
  echo "status=$status"
  echo "reason=$reason"
  echo "sd=$SD"
  echo "titlemap_lines=$titlemap_lines"
  echo "txt_title_lines=$txt_title_lines"
  echo "epub_title_lines=$epub_title_lines"
  echo "bad_phrase_count=$bad_phrase_count"
  echo "txt_body_title_scanning_disabled=true"
  echo "txt_titles_from_titles_bin=true"
  echo "epub_epu_metadata_enabled=true"
  echo "marker=phase40i=x4-title-cache-workflow-freeze-ok"
  echo "inspection=/tmp/phase40i-title-cache-workflow-inspection.txt"
  echo "runbook=docs/phase40i-title-cache-workflow-freeze-runbook.md"
} | tee "$OUT"

echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 4
fi
