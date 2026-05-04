# Phase 39E Typed Record SD/FAT Adapter Compile Repair

This repair fixes the Phase 39E compile/clippy errors without changing behavior.

Rust does not allow derived `PartialEq` enum comparison inside this `const fn` on the
current toolchain, so `wrote_once` is changed to a normal runtime function.

The unused imports are removed so `-D warnings` clippy passes.
