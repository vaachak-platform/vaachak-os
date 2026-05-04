# Phase 39I Overlay Manifest

Phase 39I — Active Reader Save Callsite Wiring Bundle Overlay

Files added:
- vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs

Files changed:
- vendor/pulp-os/src/apps/reader/mod.rs
- target-xteink-x4/src/vaachak_x4/runtime.rs

Expected marker:
- phase39i=x4-active-reader-save-callsite-wiring-bundle-ok

Active callsite scope:
- progress record
- theme record
- metadata record
- bookmark record
- bookmark index
- bookmark stub
- recent record

Notes:
- This patch keeps existing KernelHandle filesystem behavior.
- It routes active reader write callsites through a Pulp-local typed-state facade.
- It does not make vendor/pulp-os depend on target-xteink-x4.
