# Pulp Runtime Closure Roadmap

## Current state

The active Vaachak X4 runtime lives under `target-xteink-x4/src/vaachak_x4/**`. `vendor/pulp-os` may remain as scoped compatibility/reference material, but new functionality should not be added there.

## Rules going forward

```text
- Do not add new functionality under vendor/pulp-os.
- Add new X4 runtime functionality under target-xteink-x4/src/vaachak_x4/**.
- Keep the X4/CrossPoint-compatible partition table unchanged.
- Keep optional Lua apps under /VAACHAK/APPS.
- Keep generated patch artifacts out of the repo.
```

## Current audit helper

```bash
./scripts/audit_remaining_pulp_runtime_dependencies.sh
```

The audit allows the retained `vendor/pulp-os` directory but rejects active package/Cargo dependencies on the old Pulp runtime.
