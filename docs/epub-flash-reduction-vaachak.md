# Vaachak EPUB Flash Reduction

This fix keeps physical e-paper refresh execution in the existing imported runtime and adds a Vaachak-owned app-layer redraw policy in `target-xteink-x4`.

## Avoided flashes

- EPUB open no longer needs to show each transient loading label as a visible e-paper refresh when the page can be hydrated before the transition full frame.
- EPUB background cache progress is allowed to continue, but cache-progress loading text is cleared before it becomes a loading-only partial refresh over the readable page.
- Loading-only partial redraws created by the cache-progress clear are dropped by the Vaachak wrapper.

## Still expected flashes

- Boot console and initial display init full refresh.
- Real app/page transitions.
- Ghost-clearing full refresh after the configured partial-refresh budget.
- Sleep screen refreshes.

## Ownership

- Vaachak owns `target-xteink-x4/src/vaachak_x4/display/redraw_policy_runtime.rs`.
- Vaachak wraps the imported app layer from `target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs`.
- This overlay does not replace `vendor/pulp-os/src/apps/manager.rs`.
- This overlay does not replace `vendor/pulp-os/src/apps/reader/mod.rs`.
- This overlay does not replace `vendor/pulp-os/kernel/src/kernel/scheduler.rs`.
- Physical SSD1677 refresh, BUSY wait, shared SPI arbitration, SD mount/probe behavior, and strip rendering remain in the existing imported runtime.

## Acceptance marker

```text
vaachak-epub-redraw-policy-ok
```
