//! Phase 36A — Vaachak X4 boot/runtime manifest.
//!
//! This module is intentionally a pure metadata/facade layer.
//! It does not move, wrap, or call SD/FAT/SPI/display/input/runtime behavior.
//!
//! Design note:
//! Phase 35G/35H overlays were intentionally facade-only and may not expose
//! stable Rust symbol names yet. This manifest therefore records the accepted
//! phase contracts as local metadata instead of importing guessed names from
//! those modules.

#![allow(dead_code)]

/// Acceptance marker for this overlay.
pub const PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER: &str = "phase36a=x4-boot-runtime-manifest-ok";

/// Accepted Phase 35G contract marker.
pub const PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER: &str = "phase35g=x4-state-registry-adapter-ok";

/// Accepted Phase 35H contract marker.
pub const PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER: &str =
    "phase35h=x4-spi-bus-arbitration-facade-ok";

/// Current ownership model for the Phase 36A manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct X4BootRuntimeManifest {
    pub phase_marker: &'static str,
    pub state_registry_marker: &'static str,
    pub spi_bus_facade_marker: &'static str,
    pub state_registry_owner: &'static str,
    pub spi_contract_owner: &'static str,
    pub physical_behavior_owner: &'static str,
    pub storage_behavior_owner: &'static str,
    pub display_behavior_owner: &'static str,
    pub input_behavior_owner: &'static str,
    pub moved_behavior: bool,
}

/// Boot/runtime metadata for the current X4 extraction state.
pub const X4_BOOT_RUNTIME_MANIFEST: X4BootRuntimeManifest = X4BootRuntimeManifest {
    phase_marker: PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER,
    state_registry_marker: PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER,
    spi_bus_facade_marker: PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER,
    state_registry_owner: "Vaachak state registry facade",
    spi_contract_owner: "Vaachak SPI bus contract facade",
    physical_behavior_owner: "imported Pulp runtime",
    storage_behavior_owner: "imported Pulp runtime",
    display_behavior_owner: "imported Pulp runtime",
    input_behavior_owner: "imported Pulp runtime",
    moved_behavior: false,
};

/// Human-readable boot manifest labels in stable order.
pub const X4_BOOT_RUNTIME_MANIFEST_LINES: [&str; 9] = [
    PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER,
    PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER,
    PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER,
    "state-registry-owner=vaachak-facade",
    "spi-contract-owner=vaachak-facade",
    "physical-behavior-owner=imported-pulp-runtime",
    "storage-behavior-owner=imported-pulp-runtime",
    "display-behavior-owner=imported-pulp-runtime",
    "input-behavior-owner=imported-pulp-runtime",
];

#[inline]
pub const fn boot_runtime_manifest() -> &'static X4BootRuntimeManifest {
    &X4_BOOT_RUNTIME_MANIFEST
}

#[inline]
pub const fn boot_runtime_manifest_lines() -> &'static [&'static str; 9] {
    &X4_BOOT_RUNTIME_MANIFEST_LINES
}

#[inline]
pub const fn phase36a_marker() -> &'static str {
    PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER
}
