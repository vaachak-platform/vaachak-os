# Phase 35D-1 Notes

This is a bridge phase, not a persistence takeover phase.

The earliest Phase 35B storage-state preflight must stay allocation-free because it runs before heap setup. The reader-state bridge uses `alloc::String`, so it runs from a separate storage-state alloc preflight immediately after the heap allocator is installed.

This still avoids a Vaachak app-layer object. Button/input behavior remains guarded by the direct imported Pulp `AppManager`, `InputDriver`, `ButtonMapper`, and `input_task` path.

Next extraction work should choose one active persistence call site at a time. The safest candidates are small pure substitutions inside reader-state formatting or filename selection. Full reader-app ownership should wait until storage and input have been isolated enough to debug on hardware.
