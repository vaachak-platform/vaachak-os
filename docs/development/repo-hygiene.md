# Repository Hygiene

The repository should not commit generated deliverable artifacts.

Do not commit:

- overlay zip files
- extracted overlay folders
- one-off `*_fix` folders
- migration slice folders
- temporary apply scripts
- temporary patch scripts
- slice-specific validators
- smoke-only contract modules

The `scripts/` directory should contain production helper scripts only. Historical deliverable scripts should be deleted once their behavior has been folded into production code or documentation.

Run:

```bash
./scripts/check_repo_hygiene.sh
```

before committing cleanup work.

<!-- VAACHAK:LUA_DEPLOYMENT_CONTRACT:START -->
## Lua app cleanup and commit preparation

Before committing Lua app work, remove old overlay zip files, extracted overlay folders, and generated patch/apply/validator scripts from previous deliverables. Keep canonical docs and examples only.

Current canonical Lua sample app path root:

```text
examples/sd-card/VAACHAK/APPS
```

Current SD deployment root:

```text
/VAACHAK/APPS
```

Final sample app folder map:

```text
MANTRA   -> daily_mantra
CALENDAR -> calendar
PANCHANG -> panchang
```
<!-- VAACHAK:LUA_DEPLOYMENT_CONTRACT:END -->
