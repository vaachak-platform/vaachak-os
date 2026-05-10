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
