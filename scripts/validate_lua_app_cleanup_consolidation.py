#!/usr/bin/env python3
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path.cwd().resolve()
APPS = ROOT / "examples" / "sd-card" / "VAACHAK" / "APPS"
SCRIPT_WHITELIST = {
    "validate_lua_app_cleanup_consolidation.py",
    "validate_lua_app_cleanup_consolidation.sh",
}
ARTIFACT_PREFIXES = (
    "lua_",
    "wifi_transfer_",
    "vaachak_warning_",
    "vaachak_final_warning_",
    "storage_readonly_adapter_facade",
    "epub_",
    "sleep_",
    "phase",
)
ARTIFACT_HINTS = (
    "_overlay",
    "_repair",
    "_cleanup",
    "_contract",
    "_model",
    "_bridge",
    "_runtime",
    "_feature_gate",
    "_validator",
)
GENERATED_SCRIPT_PREFIXES = (
    "apply_lua_",
    "patch_lua_",
    "validate_lua_",
    "apply_wifi_transfer_",
    "patch_wifi_transfer_",
    "validate_wifi_transfer_",
    "apply_vaachak_warning",
    "patch_vaachak_warning",
    "validate_vaachak_warning",
    "apply_storage_readonly",
    "patch_storage_readonly",
    "validate_storage_readonly",
    "apply_epub_",
    "patch_epub_",
    "validate_epub_",
    "apply_sleep_",
    "patch_sleep_",
    "validate_sleep_",
)

errors: list[str] = []


def err(msg: str) -> None:
    errors.append("ERROR: " + msg)


def prune_macos_metadata() -> None:
    for p in ROOT.rglob(".DS_Store"):
        if ".git" in p.parts:
            continue
        try:
            p.unlink()
        except FileNotFoundError:
            pass
    for p in list(ROOT.rglob("__MACOSX")):
        if ".git" in p.parts:
            continue
        if p.is_dir():
            import shutil
            shutil.rmtree(p, ignore_errors=True)


def is_83_upper(name: str) -> bool:
    if name in {".", ".."}:
        return False
    if name == "README.TXT":
        return True
    parts = name.split(".")
    if len(parts) > 2:
        return False
    stem = parts[0]
    ext = parts[1] if len(parts) == 2 else ""
    allowed = re.compile(r"^[A-Z0-9_]+$")
    if not stem or len(stem) > 8 or not allowed.match(stem):
        return False
    if ext and (len(ext) > 3 or not allowed.match(ext)):
        return False
    return True


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def validate_no_old_artifacts() -> None:
    active_overlay_names = {"lua_app_cleanup_ds_store_repair"}
    for child in ROOT.iterdir():
        lower = child.name.lower()
        if lower in {".git", "target", "vendor"} or child.name in active_overlay_names:
            continue
        if child.is_file() and child.suffix.lower() == ".zip" and any(lower.startswith(p) for p in ARTIFACT_PREFIXES):
            err(f"old deliverable zip still present in repo root: {child.name}")
        if child.is_dir() and any(lower.startswith(p) for p in ARTIFACT_PREFIXES) and any(h in lower for h in ARTIFACT_HINTS):
            err(f"old extracted deliverable folder still present in repo root: {child.name}")


def validate_scripts() -> None:
    scripts = ROOT / "scripts"
    if not scripts.exists():
        err("scripts directory missing")
        return
    for child in scripts.iterdir():
        if not child.is_file():
            continue
        if child.name in SCRIPT_WHITELIST:
            continue
        if any(child.name.startswith(prefix) for prefix in GENERATED_SCRIPT_PREFIXES):
            err(f"old generated deliverable script still present: scripts/{child.name}")
    for needed in SCRIPT_WHITELIST:
        if not (scripts / needed).exists():
            err(f"missing final cleanup validator: scripts/{needed}")


def validate_docs() -> None:
    required_docs = [
        ROOT / "docs" / "lua" / "lua-app-deployment.md",
        ROOT / "docs" / "architecture" / "lua-apps.md",
        ROOT / "docs" / "network" / "wifi-transfer-nested-dirs.md",
        ROOT / "docs" / "development" / "repo-hygiene.md",
    ]
    for doc in required_docs:
        text = read(doc)
        if not text:
            err(f"missing required doc: {doc.relative_to(ROOT)}")
            continue
        for marker in ("/VAACHAK/APPS", "MANTRA", "CALENDAR", "PANCHANG", "daily_mantra"):
            if marker not in text:
                err(f"{doc.relative_to(ROOT)} missing Lua deployment marker: {marker}")
    deploy = read(ROOT / "docs" / "lua" / "lua-app-deployment.md")
    for path in (
        "/VAACHAK/APPS/MANTRA/APP.TOM",
        "/VAACHAK/APPS/MANTRA/MAIN.LUA",
        "/VAACHAK/APPS/MANTRA/MANTRAS.TXT",
        "/VAACHAK/APPS/CALENDAR/APP.TOM",
        "/VAACHAK/APPS/CALENDAR/MAIN.LUA",
        "/VAACHAK/APPS/CALENDAR/EVENTS.TXT",
        "/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT",
    ):
        if path not in deploy:
            err(f"lua-app-deployment.md missing final path: {path}")


def parse_manifest(text: str) -> dict[str, str]:
    out: dict[str, str] = {}
    for raw in text.splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip()
        if value.startswith('"') and value.endswith('"'):
            value = value[1:-1]
        out[key] = value
    return out


def validate_samples() -> None:
    if not APPS.exists():
        err("examples/sd-card/VAACHAK/APPS missing")
        return
    expected = {
        "MANTRA": ("daily_mantra", ["APP.TOM", "MAIN.LUA", "MANTRAS.TXT"]),
        "CALENDAR": ("calendar", ["APP.TOM", "MAIN.LUA", "EVENTS.TXT"]),
        "PANCHANG": ("panchang", ["APP.TOM", "MAIN.LUA", "DATA/Y2026.TXT"]),
    }
    for child in APPS.iterdir():
        if child.name == "README.TXT":
            continue
        if not is_83_upper(child.name):
            err(f"sample APPS child is not uppercase 8.3-safe: {child.name}")
        if child.is_dir() and child.name not in expected:
            err(f"unexpected sample app folder under APPS: {child.name}")
    for folder, (app_id, files) in expected.items():
        app_dir = APPS / folder
        if not app_dir.is_dir():
            err(f"missing sample app folder: {folder}")
            continue
        for rel in files:
            target = app_dir / rel
            if not target.exists():
                err(f"missing sample file: {target.relative_to(ROOT)}")
        for path in app_dir.rglob("*"):
            if path.is_file() or path.is_dir():
                for part in path.relative_to(app_dir).parts:
                    if not is_83_upper(part):
                        err(f"sample path component is not uppercase 8.3-safe: {path.relative_to(ROOT)}")
        manifest = parse_manifest(read(app_dir / "APP.TOM"))
        actual_id = manifest.get("id")
        if actual_id != app_id:
            err(f"{folder}/APP.TOM id mismatch: expected {app_id}, got {actual_id}")
        if actual_id and not re.match(r"^[a-z][a-z0-9_]*$", actual_id):
            err(f"{folder}/APP.TOM id is not logical snake_case: {actual_id}")
        entry = manifest.get("entry")
        if entry != "MAIN.LUA":
            err(f"{folder}/APP.TOM entry should be MAIN.LUA, got {entry}")


def main() -> int:
    prune_macos_metadata()
    validate_no_old_artifacts()
    validate_scripts()
    validate_docs()
    validate_samples()
    if errors:
        for e in errors:
            print(e)
        return 1
    print("lua app cleanup consolidation static validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
