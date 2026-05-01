#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
from datetime import datetime
import re
import shutil
import sys

ROOT = Path.cwd()
MAIN = ROOT / "target-xteink-x4" / "src" / "main.rs"
RUNTIME_DIR = ROOT / "target-xteink-x4" / "src" / "runtime"
RUNTIME_MOD = RUNTIME_DIR / "mod.rs"
RUNTIME_FILE = RUNTIME_DIR / "pulp_runtime.rs"
BACKUP_DIR = ROOT / ".phase_backups" / "phase18"
PHASE18 = "phase18=x4-runtime-adapter-ok"
PHASE17 = "phase17=x4-reader-refactor-ok"
PHASE16 = "phase16=x4-reader-parity-ok"

CRATE_ROOT_TEMPLATE = """#![cfg_attr(target_arch = \"riscv32\", no_std)]
#![cfg_attr(target_arch = \"riscv32\", no_main)]

#[cfg(target_arch = \"riscv32\")]
mod runtime;

#[cfg(not(target_arch = \"riscv32\"))]
fn main() {
    println!(\"VaachakOS X4 host placeholder: phase18=x4-runtime-adapter-ok\");
}
"""

MOD_TEMPLATE = """//! Vaachak-owned runtime adapter boundary for the imported X4/Pulp runtime.
//!
//! Phase 18 intentionally keeps the working X4/Pulp reader runtime intact.
//! `pulp_runtime` tracks `vendor/pulp-os/src/bin/main.rs` with only expected
//! local changes: the Cargo crate alias and Vaachak phase markers.

pub mod pulp_runtime;
"""


def read(path: Path) -> str:
    return path.read_text()


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def backup(path: Path, label: str) -> Path:
    BACKUP_DIR.mkdir(parents=True, exist_ok=True)
    stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    dst = BACKUP_DIR / f"{path.name}.{label}.{stamp}"
    shutil.copy2(path, dst)
    return dst


def split_crate_attrs(src: str) -> tuple[str, str]:
    """Return (crate_attrs, rest). Keep only leading #![...] attrs at crate root."""
    lines = src.splitlines(keepends=True)
    attrs: list[str] = []
    i = 0
    while i < len(lines):
        stripped = lines[i].strip()
        if stripped == "":
            attrs.append(lines[i])
            i += 1
            continue
        if stripped.startswith("#!["):
            attrs.append(lines[i])
            i += 1
            continue
        break
    return "".join(attrs), "".join(lines[i:])


def remove_crate_attrs(src: str) -> str:
    _, rest = split_crate_attrs(src)
    return rest.lstrip("\n")


def insert_phase18_marker(src: str) -> str:
    if PHASE18 in src:
        return src

    lines = src.splitlines(keepends=True)
    out: list[str] = []
    inserted = False

    for line in lines:
        out.append(line)
        if not inserted and PHASE17 in line:
            out.append(line.replace(PHASE17, PHASE18))
            inserted = True

    if inserted:
        return "".join(out)

    # Fallback: insert after Phase 16 marker if Phase 17 is unexpectedly absent.
    out = []
    for line in lines:
        out.append(line)
        if not inserted and PHASE16 in line:
            out.append(line.replace(PHASE16, PHASE18))
            inserted = True

    if inserted:
        return "".join(out)

    raise SystemExit(
        "ERROR: could not find phase16/phase17 marker in active runtime; "
        "refusing to insert phase18 at an unsafe location"
    )


def is_already_extracted(src: str) -> bool:
    return "mod runtime;" in src and RUNTIME_FILE.exists()


def ensure_gitignore() -> None:
    p = ROOT / ".gitignore"
    existing = p.read_text() if p.exists() else ""
    additions = [
        "",
        "# Phase-generated backups",
        "*.bak-phase*",
        "*.phase15a-backup.*",
        ".phase_backups/",
    ]
    changed = False
    out = existing.rstrip("\n")
    for line in additions:
        if line and line not in existing:
            out += "\n" + line
            changed = True
        elif line == "" and changed:
            out += "\n"
    if changed:
        p.write_text(out.rstrip("\n") + "\n")


def main() -> int:
    if not MAIN.exists():
        raise SystemExit(f"ERROR: missing {MAIN}")

    src = read(MAIN)
    BACKUP_DIR.mkdir(parents=True, exist_ok=True)

    if is_already_extracted(src):
        print("Phase 18 runtime adapter already appears to be installed")
        if not RUNTIME_FILE.exists():
            raise SystemExit("ERROR: main.rs references runtime but runtime/pulp_runtime.rs is missing")
        backup(RUNTIME_FILE, "runtime-before-marker-refresh")
        runtime_src = insert_phase18_marker(read(RUNTIME_FILE))
        write(RUNTIME_FILE, runtime_src)
    else:
        main_backup = backup(MAIN, "main-before-runtime-extract")
        print(f"Backed up active main.rs to {main_backup}")

        runtime_src = remove_crate_attrs(src)
        runtime_src = insert_phase18_marker(runtime_src)
        write(RUNTIME_FILE, runtime_src)
        print(f"Wrote runtime adapter implementation: {RUNTIME_FILE}")

        write(RUNTIME_MOD, MOD_TEMPLATE)
        print(f"Wrote runtime module boundary: {RUNTIME_MOD}")

        write(MAIN, CRATE_ROOT_TEMPLATE)
        print(f"Replaced active main.rs with runtime adapter shell: {MAIN}")

    ensure_gitignore()
    print("Updated .gitignore with phase backup hygiene rules if needed")
    print("Phase 18 marker ensured: phase18=x4-runtime-adapter-ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
