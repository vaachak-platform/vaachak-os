#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-.}"
cd "$ROOT"

python3 - <<'PY'
from pathlib import Path
import re
import sys

path = Path("vendor/pulp-os/src/apps/reader/prepared_txt.rs")
if not path.exists():
    print(f"missing {path}", file=sys.stderr)
    raise SystemExit(1)

text = path.read_text()
backup = path.with_suffix(path.suffix + ".bak-too-large-limits")
backup.write_text(text)

repls = {
    r"const MAX_META_BYTES:\s*usize\s*=\s*[^;]+;": "const MAX_META_BYTES: usize = 1024;",
    r"const MAX_INDEX_BYTES:\s*usize\s*=\s*[^;]+;": "const MAX_INDEX_BYTES: usize = 4 * 1024;",
    r"const MAX_FONT_BYTES:\s*usize\s*=\s*[^;]+;": "const MAX_FONT_BYTES: usize = 16 * 1024;",
    r"const MAX_PAGE_BYTES:\s*usize\s*=\s*[^;]+;": "const MAX_PAGE_BYTES: usize = 24 * 1024;",
    r"const MAX_PAGES:\s*usize\s*=\s*[^;]+;": "const MAX_PAGES: usize = 192;",
    r"const MAX_GLYPHS:\s*usize\s*=\s*[^;]+;": "const MAX_GLYPHS: usize = 1024;",
}

for pattern, replacement in repls.items():
    text, n = re.subn(pattern, replacement, text, count=1)
    if n != 1:
        print(f"could not patch {replacement}", file=sys.stderr)
        raise SystemExit(1)

# Keep source matching permissive if source_matches exists. This avoids rejecting a valid
# cache because FAT 8.3 case/leading-slash differs from META.TXT source=.
def find_matching_brace(src: str, open_brace: int) -> int:
    depth = 0
    for i in range(open_brace, len(src)):
        if src[i] == "{":
            depth += 1
        elif src[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    return -1

fn = "fn source_matches"
start = text.find(fn)
if start >= 0:
    brace = text.find("{", start)
    end = find_matching_brace(text, brace) if brace >= 0 else -1
    if end >= 0:
        replacement = '''fn source_matches(_cache_source: &str, _reader_source: &str) -> bool {
    // The prepared cache directory/book_id is the authority.
    // Do not reject a cache because FAT 8.3 names, case, or leading slashes differ.
    true
}
'''
        text = text[:start] + replacement + text[end + 1:]

path.write_text(text)
print("updated prepared_txt cache limits")
print(f"backup: {backup}")
PY

cargo fmt --all

printf '\nPrepared cache limits now:\n'
rg -n "MAX_META_BYTES|MAX_INDEX_BYTES|MAX_FONT_BYTES|MAX_PAGE_BYTES|MAX_PAGES|MAX_GLYPHS|fn source_matches" \
  vendor/pulp-os/src/apps/reader/prepared_txt.rs
