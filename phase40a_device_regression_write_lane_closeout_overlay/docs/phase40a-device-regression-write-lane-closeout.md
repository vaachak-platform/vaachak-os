# Phase 40A — Device Regression and Write-Lane Closeout

Phase 40A closes the write-lane cleanup after Phase 39P accepted the post-cleanup
runtime surface.

Accepted write path:

```text
vendor/pulp-os/src/apps/reader/mod.rs
  -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
  -> KernelHandle
  -> _X4/state
  -> restore
```

Device regression checklist:

```text
Home opens
Files/Library opens
EPUB titles still display correctly
Open EPUB
Scroll a few pages
Back returns to Library
Reopen same EPUB and confirm progress restores
Change theme and confirm it persists after reopen
Add/remove bookmark and confirm bookmark list/index updates
No crash/reboot
```

Acceptance requires:

```text
release build baseline accepted
accepted write path guard accepted
runtime export inventory clean
SD state persistence present
manual device regression confirmed
manual restore confirmation provided
```
