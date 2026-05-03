#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT_REL="${OUT_REL:-_X4/TITLEMAP.TSV}"
INCLUDE_MD="${INCLUDE_MD:-1}"

if [ ! -d "$SD" ]; then
  echo "SD mount not found: $SD" >&2
  exit 2
fi

mkdir -p "$SD/_X4"

python3 - "$SD" "$OUT_REL" "$INCLUDE_MD" <<'PY'
from pathlib import Path
import re
import sys
from collections import defaultdict

sd = Path(sys.argv[1])
out_rel = sys.argv[2]
include_md = sys.argv[3] == "1"
out_path = sd / out_rel

exts = {".txt"}
if include_md:
    exts.add(".md")

def sanitize_alias_part(text: str) -> str:
    out = []
    for ch in text:
        c = ch.upper()
        if "A" <= c <= "Z" or "0" <= c <= "9":
            out.append(c)
    return "".join(out) or "FILE"

def friendly_title(path: Path) -> str:
    stem = path.stem
    text = re.sub(r"[_\-.]+", " ", stem)
    text = re.sub(r"\s+", " ", text).strip()
    if not text:
        text = path.stem
    words = []
    small = {"a", "an", "and", "as", "at", "by", "for", "in", "of", "on", "or", "the", "to"}
    for i, word in enumerate(text.split(" ")):
        lower = word.lower()
        if i > 0 and lower in small:
            words.append(lower)
        else:
            words.append(lower[:1].upper() + lower[1:])
    return " ".join(words)

files = sorted(
    [p for p in sd.iterdir() if p.is_file() and p.suffix.lower() in exts],
    key=lambda p: p.name.lower(),
)

base_groups = defaultdict(list)
for p in files:
    ext = sanitize_alias_part(p.suffix[1:])[:3]
    base6 = sanitize_alias_part(p.stem)[:6]
    base_groups[(base6, ext)].append(p)

lines = []
seen = set()

for (base6, ext), group in sorted(base_groups.items()):
    for idx, p in enumerate(group, start=1):
        title = friendly_title(p)
        aliases = [
            p.name,
            p.name.upper(),
            f"{base6}~{idx}.{ext}",
            f"{base6.title()}~{idx}.{ext.lower()}",
        ]

        # Bounded fallback for volumes that allocated a different numeric tail.
        for n in range(1, 10):
            aliases.append(f"{base6}~{n}.{ext}")

        for alias in aliases:
            key = alias.upper()
            if key in seen:
                continue
            seen.add(key)
            lines.append(f"{alias}\t{title}\n")

out_path.write_text("".join(lines), encoding="utf-8")

print("# Phase 40H TXT Title Map Generated")
print("status=ACCEPTED")
print(f"sd={sd}")
print(f"out={out_path}")
print(f"files={len(files)}")
print(f"lines={len(lines)}")
print("marker=phase40h=x4-host-title-map-txt-display-names-ok")
PY

sync
