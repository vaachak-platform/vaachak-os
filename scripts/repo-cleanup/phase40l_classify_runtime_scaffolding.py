#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path


DO_NOT_TOUCH_KEYWORDS = (
    "input",
    "display",
    "spi",
    "sd",
    "fat",
    "backend",
    "typed_record",
    "write_lane",
    "runtime_file",
    "reader",
    "footer_button_label_rendering_patch",
)

PRUNE_NAME_KEYWORDS = (
    "acceptance",
    "candidate_plan",
    "freeze",
    "closeout",
    "checklist",
    "handoff",
    "cleanup_plan",
    "report",
)

BEHAVIOR_KEYWORDS = (
    "render",
    "draw",
    "load_",
    "save_",
    "write_",
    "read_",
    "open_",
    "input",
    "button",
    "footer",
    "title",
    "TITLEMAP",
    "TITLES",
    "reader",
    "progress",
    "restore",
    "sd",
    "fat",
    "backend",
)


def classify_file(path: Path, runtime_mod_text: str) -> tuple[str, str]:
    name = path.name
    stem = path.stem
    text = path.read_text(encoding="utf-8", errors="ignore")

    exported = f"pub mod {stem};" in runtime_mod_text
    behavior_score = sum(1 for k in BEHAVIOR_KEYWORDS if k in text)
    do_not_touch_name = any(k in name for k in DO_NOT_TOUCH_KEYWORDS)
    prune_name = any(k in name for k in PRUNE_NAME_KEYWORDS)

    if do_not_touch_name and behavior_score > 1:
        return "DO-NOT-TOUCH", "behavior-surface-keywords"

    if "PLAN_ONLY: bool = true" in text:
        return "PRUNE-CANDIDATE", "plan-only-marker-module"

    if prune_name and "MARKER" in text and "Report" in text and behavior_score <= 2:
        return "PRUNE-CANDIDATE", "marker-report-scaffolding"

    if "ACCEPTANCE_MARKER" in text and behavior_score <= 2:
        return "PRUNE-CANDIDATE", "acceptance-marker-scaffolding"

    if exported and prune_name:
        return "REVIEW", "exported-scaffolding"

    if exported:
        return "REVIEW", "exported-runtime-module"

    if "phase" in name or "Phase" in text or "PHASE_" in text:
        return "REVIEW", "phase-named-module"

    return "KEEP", "not-phase-scaffolding"


def main() -> int:
    parser = argparse.ArgumentParser(description="Classify Vaachak X4 runtime phase scaffolding.")
    parser.add_argument("--runtime-dir", default="target-xteink-x4/src/vaachak_x4/runtime")
    parser.add_argument("--runtime-mod", default="target-xteink-x4/src/vaachak_x4/runtime.rs")
    parser.add_argument("--out", default="/tmp/phase40l-runtime-scaffolding-classification.tsv")
    args = parser.parse_args()

    runtime_dir = Path(args.runtime_dir)
    runtime_mod = Path(args.runtime_mod)
    out = Path(args.out)

    if not runtime_dir.is_dir():
        raise SystemExit(f"runtime dir missing: {runtime_dir}")

    runtime_mod_text = runtime_mod.read_text(encoding="utf-8", errors="ignore") if runtime_mod.exists() else ""

    files = sorted(
        p for p in runtime_dir.glob("*.rs")
        if "phase" in p.name
        or "repair" in p.name
        or "freeze" in p.name
        or "plan" in p.name
        or "acceptance" in p.name
        or "state_io_" in p.name
    )

    rows = ["classification\treason\tpath"]
    counts: dict[str, int] = {}

    for path in files:
        classification, reason = classify_file(path, runtime_mod_text)
        counts[classification] = counts.get(classification, 0) + 1
        rows.append(f"{classification}\t{reason}\t{path}")

    out.write_text("\n".join(rows) + "\n", encoding="utf-8")

    print("# Phase 40L Runtime Scaffolding Classification")
    print(f"out={out}")
    for key in sorted(counts):
        print(f"{key.lower().replace('-', '_')}={counts[key]}")
    print("marker=phase40l=x4-runtime-phase-scaffolding-prune-plan-ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
