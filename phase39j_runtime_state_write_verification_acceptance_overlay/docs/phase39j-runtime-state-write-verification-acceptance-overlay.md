# Phase 39J — Runtime State Write Verification and SD Persistence Acceptance

Phase 39J is verification-focused.

It does not add another write abstraction. It adds target-runtime acceptance
metadata and SD inspection scripts to prove Phase 39I writes real state files.

Expected state files after device interaction:

```text
_X4/state/<BOOKID>.PRG
_X4/state/<BOOKID>.THM
_X4/state/<BOOKID>.MTA
_X4/state/<BOOKID>.BKM
_X4/state/BMIDX.TXT
```

Recommended device actions before running acceptance:

```text
Open EPUB
Scroll a few pages
Back to Library
Reopen same EPUB and confirm progress restores
Change theme and confirm it persists after reopen
Add/remove bookmark and confirm bookmark list/index updates
Power cycle/reflash only after sync/unmount safety
```

Run scripts:

```bash
SD=/media/mindseye73/C0D2-109E ./phase39j_runtime_state_write_verification_acceptance_overlay/scripts/inspect_phase39j_sd_state.sh
SD=/media/mindseye73/C0D2-109E RESTORE_VERIFIED=1 ./phase39j_runtime_state_write_verification_acceptance_overlay/scripts/accept_phase39j_sd_persistence.sh
```
