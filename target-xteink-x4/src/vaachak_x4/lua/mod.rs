//! Optional Vaachak Lua runtime integration seams.
//!
//! This module is compiled only when the `lua-runtime-probe` feature is enabled.
//! It intentionally does not link a Lua VM yet and does not load SD-card apps.

pub mod calendar_script;
pub mod catalog_bridge;
pub mod daily_mantra_script;
pub mod runtime_probe;
pub mod sd_manifest_reader_bridge;

#[cfg(feature = "lua-vm")]
pub mod daily_mantra_vm_bridge;
pub mod panchang_script;
#[cfg(feature = "lua-vm")]
pub mod vm_feature_gate;
