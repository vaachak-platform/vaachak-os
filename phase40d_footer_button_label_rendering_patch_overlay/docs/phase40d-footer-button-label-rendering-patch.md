# Phase 40D — Footer/Button Label Rendering Patch

Phase 40D applies the footer label correction planned by Phase 40C.

Expected visible footer order:

```text
Back Select Open Stay
```

This patch must not change:

```text
hal-xteink-x4/src/input.rs
target-xteink-x4/src/vaachak_x4/input/*
target-xteink-x4/src/vaachak_x4/contracts/input*.rs
vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
display geometry / rotation code
```

Run:

```bash
./phase40d_footer_button_label_rendering_patch_overlay/scripts/guard_phase40d_footer_patch_scope.sh
./phase40d_footer_button_label_rendering_patch_overlay/scripts/patch_phase40d_footer_label_rendering.sh
./phase40d_footer_button_label_rendering_patch_overlay/scripts/inspect_phase40d_footer_label_rendering.sh
```

Then build, flash, and confirm on device that Files/Library and Reader show:

```text
Back Select Open Stay
```
