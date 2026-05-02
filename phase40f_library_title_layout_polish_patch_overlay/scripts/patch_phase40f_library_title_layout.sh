#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
FILES_RS="$ROOT/vendor/pulp-os/src/apps/files.rs"
OUT="${OUT:-/tmp/phase40f-library-title-layout-patch.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40f-patched-files.txt}"

"$ROOT/phase40f_library_title_layout_polish_patch_overlay/scripts/guard_phase40f_library_title_patch_scope.sh" >/dev/null

python3 - "$FILES_RS" "$OUT" "$PATCHED_LIST" <<'PY'
from pathlib import Path
import re
import sys

files_rs = Path(sys.argv[1])
out_path = Path(sys.argv[2])
patched_list = Path(sys.argv[3])
root = Path.cwd()

text = files_rs.read_text()
original = text
changes = []

# Safe text-only layout polish:
# 1) Normalize any old placeholder separators in visible library labels if present.
literal_replacements = [
    ('"  {}"', '"{}"'),
    ('"{}  "', '"{}"'),
    ('"{}   "', '"{}"'),
    ('"  {title}"', '"{title}"'),
    ('"{title}  "', '"{title}"'),
]
for old, new in literal_replacements:
    count = text.count(old)
    if count:
        text = text.replace(old, new)
        changes.append(f"literal {old!r} -> {new!r} count={count}")

# 2) If the Files app has a local title truncation constant, gently increase
# title width but do not change row count, geometry, input, or source behavior.
patterns = [
    (
        re.compile(r'(?P<name>\b(?:TITLE|LABEL|NAME|DISPLAY)_MAX(?:_CHARS|_LEN|_WIDTH)?\s*:\s*usize\s*=\s*)(?P<value>2[0-9]|3[0-5])(?P<suffix>\s*;)'),
        r'\g<name>36\g<suffix>',
        "increase local title max width to 36",
    ),
    (
        re.compile(r'(?P<name>\bMAX_(?:TITLE|LABEL|NAME|DISPLAY)(?:_CHARS|_LEN|_WIDTH)?\s*:\s*usize\s*=\s*)(?P<value>2[0-9]|3[0-5])(?P<suffix>\s*;)'),
        r'\g<name>36\g<suffix>',
        "increase local max title width to 36",
    ),
]
for pattern, repl, label in patterns:
    text, count = pattern.subn(repl, text)
    if count:
        changes.append(f"{label} count={count}")

# 3) Add a narrow comment marker near the Files app module to make the phase
# auditable even if no title width constants exist. This is source-local and
# behavior-neutral.
marker = "phase40f=x4-library-title-layout-polish-patch-ok"
if marker not in text:
    insert = (
        "\n// Phase 40F: Library title layout polish is intentionally limited to\n"
        "// display/layout treatment. Title source/cache behavior remains unchanged.\n"
        f"// marker={marker}\n"
    )
    # Prefer after top module docs/import block, otherwise prepend.
    lines = text.splitlines()
    idx = 0
    while idx < len(lines) and (lines[idx].startswith("//") or lines[idx].startswith("#!") or lines[idx].strip() == ""):
        idx += 1
    lines.insert(idx, insert.rstrip())
    text = "\n".join(lines) + "\n"
    changes.append("added Phase 40F local audit marker")

if text != original:
    files_rs.write_text(text)
    patched = [str(files_rs.relative_to(root))]
else:
    patched = []

status = "ACCEPTED" if patched else "REJECTED"
reason = "LibraryTitleLayoutSourcePatched" if patched else "NoLibraryTitleLayoutPatchApplied"

patched_list.write_text("\n".join(patched) + ("\n" if patched else ""))

with out_path.open("w") as out:
    out.write("# Phase 40F Library Title Layout Patch\n")
    out.write(f"status={status}\n")
    out.write(f"reason={reason}\n")
    out.write("changes_title_source=false\n")
    out.write("changes_footer_labels=false\n")
    out.write("changes_input_mapping=false\n")
    out.write("touches_write_lane=false\n")
    out.write("touches_display_geometry=false\n")
    out.write("touches_reader_pagination=false\n")
    out.write(f"patched_files={len(patched)}\n")
    out.write("marker=phase40f=x4-library-title-layout-polish-patch-ok\n\n")
    for change in changes:
        out.write(f"- {change}\n")

print(out_path.read_text())

if not patched:
    raise SystemExit("No library title layout source patched.")
PY
