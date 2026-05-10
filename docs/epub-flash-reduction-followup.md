# EPUB flash reduction follow-up

This patch keeps physical SSD1677 refresh execution in the current imported runtime and adds a Vaachak-owned redraw policy wrapper around the app layer.

## What this suppresses

- EPUB `Loading` percentage redraws before the first readable page is ready.
- `Indexing` and `Loading page` redraws that were still able to reach e-paper after the earlier fix.
- Background `Caching` progress redraws over an already-readable EPUB page.
- Loading-only partial redraws caused by clearing the cache/loading overlay.

## What remains expected

- The Reader page-turn waveform. A real page turn changes most of the 800x480 e-paper surface, so the SSD1677 still needs a visible update pulse.
- Boot/full initialization refresh.
- Transition refreshes between apps.
- Ghost-clearing full refreshes after the configured partial refresh count.
- Sleep/sleep-image refresh.

## Ownership

Vaachak owns the redraw decision wrapper in `target-xteink-x4/src/vaachak_x4/display/redraw_policy_runtime.rs`.

This patch does not replace:

- `vendor/pulp-os/kernel/src/kernel/scheduler.rs`
- `vendor/pulp-os/src/apps/manager.rs`
- `vendor/pulp-os/src/apps/reader/mod.rs`
