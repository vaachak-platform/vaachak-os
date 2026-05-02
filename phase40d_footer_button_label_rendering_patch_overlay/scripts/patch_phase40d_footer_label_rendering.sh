#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OUT="${OUT:-/tmp/phase40d-footer-label-rendering-patch.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40d-patched-files.txt}"

"$ROOT/phase40d_footer_button_label_rendering_patch_overlay/scripts/guard_phase40d_footer_patch_scope.sh" >/dev/null

python3 - "$OUT" "$PATCHED_LIST" <<'PY'
from pathlib import Path
import re
import sys

out_path = Path(sys.argv[1])
patched_list_path = Path(sys.argv[2])

root = Path.cwd()

allowed_roots = [
    root / "vendor/pulp-os/src/apps",
    root / "vendor/pulp-os/src/ui",
    root / "vendor/pulp-os/src/widgets",
    root / "hal-xteink-x4/src/display_smoke.rs",
    root / "target-xteink-x4/src/vaachak_x4/display",
    root / "target-xteink-x4/src/vaachak_x4/ui",
]

protected_prefixes = [
    root / "hal-xteink-x4/src/input.rs",
    root / "target-xteink-x4/src/vaachak_x4/input",
    root / "target-xteink-x4/src/vaachak_x4/contracts/input.rs",
    root / "target-xteink-x4/src/vaachak_x4/contracts/input_semantics.rs",
    root / "target-xteink-x4/src/vaachak_x4/contracts/input_contract_smoke.rs",
    root / "vendor/pulp-os/src/apps/reader/typed_state_wiring.rs",
]

def is_protected(path: Path) -> bool:
    resolved = path.resolve()
    for protected in protected_prefixes:
        try:
            p = protected.resolve()
        except FileNotFoundError:
            p = protected
        if resolved == p or str(resolved).startswith(str(p) + "/"):
            return True
    return False

def candidate_files():
    seen = set()
    for entry in allowed_roots:
        if entry.is_file():
            files = [entry]
        elif entry.is_dir():
            files = list(entry.rglob("*.rs"))
        else:
            files = []
        for f in files:
            if f in seen or is_protected(f):
                continue
            seen.add(f)
            yield f

replacements = [
    ('"Select open Back Stay"', '"Back Select Open Stay"'),
    ('"Select Open Back Stay"', '"Back Select Open Stay"'),
    ('"Select  Open  Back  Stay"', '"Back  Select  Open  Stay"'),
    ('"Select   Open   Back   Stay"', '"Back   Select   Open   Stay"'),
    ('"Select\\0Open\\0Back\\0Stay"', '"Back\\0Select\\0Open\\0Stay"'),
    ('b"Select open Back Stay"', 'b"Back Select Open Stay"'),
    ('b"Select Open Back Stay"', 'b"Back Select Open Stay"'),
    ('b"Select  Open  Back  Stay"', 'b"Back  Select  Open  Stay"'),
    ('b"Select\\0Open\\0Back\\0Stay"', 'b"Back\\0Select\\0Open\\0Stay"'),
    ('["Select", "open", "Back", "Stay"]', '["Back", "Select", "Open", "Stay"]'),
    ('["Select", "Open", "Back", "Stay"]', '["Back", "Select", "Open", "Stay"]'),
    ('[b"Select", b"open", b"Back", b"Stay"]', '[b"Back", b"Select", b"Open", b"Stay"]'),
    ('[b"Select", b"Open", b"Back", b"Stay"]', '[b"Back", b"Select", b"Open", b"Stay"]'),
    ('&["Select", "open", "Back", "Stay"]', '&["Back", "Select", "Open", "Stay"]'),
    ('&["Select", "Open", "Back", "Stay"]', '&["Back", "Select", "Open", "Stay"]'),
    ('&[b"Select", b"open", b"Back", b"Stay"]', '&[b"Back", b"Select", b"Open", b"Stay"]'),
    ('&[b"Select", b"Open", b"Back", b"Stay"]', '&[b"Back", b"Select", b"Open", b"Stay"]'),
    ('("Select", "open", "Back", "Stay")', '("Back", "Select", "Open", "Stay")'),
    ('("Select", "Open", "Back", "Stay")', '("Back", "Select", "Open", "Stay")'),
]

regex_replacements = [
    (
        re.compile(r'(?P<prefix>\b(?:const|static)\s+[A-Z0-9_]*FOOTER[A-Z0-9_]*[^=]*=\s*&?\[)\s*"Select"\s*,\s*"open"\s*,\s*"Back"\s*,\s*"Stay"\s*(?P<suffix>\])'),
        r'\g<prefix>"Back", "Select", "Open", "Stay"\g<suffix>',
    ),
    (
        re.compile(r'(?P<prefix>\b(?:const|static)\s+[A-Z0-9_]*FOOTER[A-Z0-9_]*[^=]*=\s*&?\[)\s*"Select"\s*,\s*"Open"\s*,\s*"Back"\s*,\s*"Stay"\s*(?P<suffix>\])'),
        r'\g<prefix>"Back", "Select", "Open", "Stay"\g<suffix>',
    ),
    (
        re.compile(r'(?P<prefix>\b(?:const|static)\s+[A-Z0-9_]*FOOTER[A-Z0-9_]*[^=]*=\s*&?\[)\s*b"Select"\s*,\s*b"open"\s*,\s*b"Back"\s*,\s*b"Stay"\s*(?P<suffix>\])'),
        r'\g<prefix>b"Back", b"Select", b"Open", b"Stay"\g<suffix>',
    ),
    (
        re.compile(r'(?P<prefix>\b(?:const|static)\s+[A-Z0-9_]*FOOTER[A-Z0-9_]*[^=]*=\s*&?\[)\s*b"Select"\s*,\s*b"Open"\s*,\s*b"Back"\s*,\s*b"Stay"\s*(?P<suffix>\])'),
        r'\g<prefix>b"Back", b"Select", b"Open", b"Stay"\g<suffix>',
    ),
]

patched = []
details = []

for f in candidate_files():
    try:
        text = f.read_text()
    except UnicodeDecodeError:
        continue

    original = text
    file_details = []

    for old, new in replacements:
        count = text.count(old)
        if count:
            text = text.replace(old, new)
            file_details.append(f"literal {old!r} -> {new!r} count={count}")

    for pattern, repl in regex_replacements:
        text, count = pattern.subn(repl, text)
        if count:
            file_details.append(f"regex {pattern.pattern} count={count}")

    if text != original:
        if is_protected(f):
            raise SystemExit(f"refusing to patch protected file: {f}")
        f.write_text(text)
        patched.append(f)
        details.append((f, file_details))

status = "ACCEPTED" if patched else "REJECTED"
reason = "FooterLabelSourcesPatched" if patched else "NoFooterLabelSourceMatched"

with out_path.open("w") as out:
    out.write("# Phase 40D Footer Label Rendering Patch\n")
    out.write(f"status={status}\n")
    out.write(f"reason={reason}\n")
    out.write("expected_footer=Back Select Open Stay\n")
    out.write("changes_input_mapping=false\n")
    out.write("touches_write_lane=false\n")
    out.write("touches_display_geometry=false\n")
    out.write(f"patched_files={len(patched)}\n")
    out.write("marker=phase40d=x4-footer-button-label-rendering-patch-ok\n\n")
    for f, ds in details:
        out.write(f"## {f.relative_to(root)}\n")
        for d in ds:
            out.write(f"- {d}\n")
        out.write("\n")

patched_list_path.write_text("\n".join(str(p.relative_to(root)) for p in patched) + ("\n" if patched else ""))

print(out_path.read_text())

if not patched:
    raise SystemExit(
        "No footer label source matched known old orders. Inspect /tmp/phase40d-footer-label-rendering-patch.txt and "
        "run rg -n 'Select|Open|Back|Stay|footer|Footer' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs"
    )
PY
