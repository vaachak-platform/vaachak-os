//! Phase 36B boot/runtime marker emitter adapter.
//!
//! This module is intentionally pure metadata. It gives the X4 target one
//! Vaachak-owned place to enumerate the accepted boot/runtime extraction
//! markers without touching SD/FAT, SPI, display, input, or imported Pulp
//! runtime behavior.

/// Phase 36B acceptance marker.
pub const PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER: &str =
    "phase36b=x4-boot-runtime-marker-emitter-ok";

/// Accepted dependency marker from Phase 36A.
pub const PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER: &str = "phase36a=x4-boot-runtime-manifest-ok";

/// Accepted dependency marker from Phase 35G.
pub const PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER: &str = "phase35g=x4-state-registry-adapter-ok";

/// Accepted dependency marker from Phase 35H.
pub const PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER: &str =
    "phase35h=x4-spi-bus-arbitration-facade-ok";

/// Ordered boot/runtime extraction markers known to the Phase 36B facade.
pub const X4_BOOT_RUNTIME_MARKERS: [&str; 4] = [
    PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER,
    PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER,
    PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER,
    PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER,
];

/// Pure metadata facade for boot/runtime marker access.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct X4BootRuntimeMarkerEmitter {
    marker_count: usize,
}

/// Singleton marker emitter used by future boot/reporting code.
pub const X4_BOOT_RUNTIME_MARKER_EMITTER: X4BootRuntimeMarkerEmitter = X4BootRuntimeMarkerEmitter {
    marker_count: X4_BOOT_RUNTIME_MARKERS.len(),
};

impl X4BootRuntimeMarkerEmitter {
    /// Returns the Phase 36B marker.
    pub const fn phase_marker(&self) -> &'static str {
        PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER
    }

    /// Returns the number of known markers.
    pub const fn marker_count(&self) -> usize {
        self.marker_count
    }

    /// Returns all accepted markers in deterministic order.
    pub const fn markers(&self) -> [&'static str; 4] {
        X4_BOOT_RUNTIME_MARKERS
    }

    /// Returns a marker by index without requiring allocation.
    pub const fn marker_at(&self, index: usize) -> Option<&'static str> {
        match index {
            0 => Some(PHASE_36B_BOOT_RUNTIME_MARKER_EMITTER_MARKER),
            1 => Some(PHASE_36A_BOOT_RUNTIME_MANIFEST_MARKER),
            2 => Some(PHASE_35G_STATE_REGISTRY_ADAPTER_MARKER),
            3 => Some(PHASE_35H_SPI_BUS_ARBITRATION_FACADE_MARKER),
            _ => None,
        }
    }

    /// Returns true when all dependency markers are represented.
    pub const fn dependencies_declared(&self) -> bool {
        self.marker_count == 4
    }
}

/// Convenience constructor for code that prefers function access.
pub const fn phase36b_boot_runtime_marker_emitter() -> X4BootRuntimeMarkerEmitter {
    X4_BOOT_RUNTIME_MARKER_EMITTER
}
