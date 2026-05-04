# Phase 38K Overlay Manifest

Files:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_binding.rs
- scripts/apply_phase38k_state_io_guarded_write_backend_binding.sh
- scripts/check_phase38k_state_io_guarded_write_backend_binding.sh

Repair notes:
- Removes guard-triggering prose from the scaffold.
- Repairs recursive Phase 38I helper in `vendor/pulp-os/kernel/src/kernel/dir_cache.rs` when present.
