# Phase 35D-2 Notes

The May 1, 2026 hardware flash failed because a reader-state preflight allocated before the heap allocator was installed.

Phase 35D-2 makes that class of bug visible in local checks. Future extraction phases should add any allocation-using runtime probes behind the post-heap alloc preflight, or make the probe truly allocation-free.

The next active extraction should still be small. A good candidate is a single reader-state filename/format substitution that does not wrap the app manager and does not touch input.
