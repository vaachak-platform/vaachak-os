#![allow(dead_code)]

use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::hardware_runtime_wiring_pulp_backend::VaachakHardwareRuntimeWiringPulpBackend;

/// Pulp-compatible backend descriptor for hardware runtime observability.
///
/// This backend reports which Vaachak executor path is selected. It does not
/// print, allocate, access peripherals, toggle chip-select, sample input, draw,
/// refresh, mount SD, or access FAT. The active low-level executor remains the
/// currently working Pulp-compatible backend.
pub struct VaachakHardwareRuntimeObservabilityPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeObservationSink {
    BootMarker,
    DebugMarker,
    ContractSmoke,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeObservabilityBackendRoute {
    pub sink: VaachakHardwareRuntimeObservationSink,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub route_selected: bool,
    pub low_level_backend_still_pulp_compatible: bool,
    pub mutates_hardware_behavior: bool,
}

impl VaachakHardwareRuntimeObservabilityPulpBackend {
    pub const BACKEND_NAME: &'static str = VaachakHardwareRuntimeWiringPulpBackend::BACKEND_NAME;
    pub const ACTIVE_EXECUTOR_OWNER: &'static str =
        VaachakHardwareRuntimeWiringPulpBackend::LOW_LEVEL_EXECUTOR_OWNER;

    pub const OBSERVABILITY_BACKEND_ACTIVE: bool = true;
    pub const OBSERVABILITY_ROUTES_THROUGH_WIRED_EXECUTOR: bool = true;
    pub const LOW_LEVEL_EXECUTION_REMAINS_PULP_COMPATIBLE: bool = true;
    pub const OBSERVABILITY_MUTATES_HARDWARE_BEHAVIOR: bool = false;
    pub const BOOT_MARKERS_ARE_METADATA_ONLY: bool = true;
    pub const DEBUG_MARKERS_ARE_METADATA_ONLY: bool = true;

    pub const DISPLAY_EXECUTION_CHANGED: bool = false;
    pub const STORAGE_EXECUTION_CHANGED: bool = false;
    pub const INPUT_EXECUTION_CHANGED: bool = false;
    pub const SPI_EXECUTION_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn route_for(
        sink: VaachakHardwareRuntimeObservationSink,
    ) -> VaachakHardwareRuntimeObservabilityBackendRoute {
        VaachakHardwareRuntimeObservabilityBackendRoute {
            sink,
            backend: VaachakHardwareExecutorBackend::PulpCompatibility,
            backend_name: Self::BACKEND_NAME,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            route_selected: Self::OBSERVABILITY_ROUTES_THROUGH_WIRED_EXECUTOR,
            low_level_backend_still_pulp_compatible:
                Self::LOW_LEVEL_EXECUTION_REMAINS_PULP_COMPATIBLE,
            mutates_hardware_behavior: Self::OBSERVABILITY_MUTATES_HARDWARE_BEHAVIOR,
        }
    }

    pub const fn route_ok(route: VaachakHardwareRuntimeObservabilityBackendRoute) -> bool {
        matches!(
            route.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.backend_name.len() == Self::BACKEND_NAME.len()
            && route.active_executor_owner.len() == Self::ACTIVE_EXECUTOR_OWNER.len()
            && route.route_selected
            && route.low_level_backend_still_pulp_compatible
            && !route.mutates_hardware_behavior
    }

    pub const fn behavior_preserved() -> bool {
        !Self::DISPLAY_EXECUTION_CHANGED
            && !Self::STORAGE_EXECUTION_CHANGED
            && !Self::INPUT_EXECUTION_CHANGED
            && !Self::SPI_EXECUTION_CHANGED
            && !Self::READER_FILE_BROWSER_UX_CHANGED
            && !Self::APP_NAVIGATION_CHANGED
            && !Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED
            && Self::BOOT_MARKERS_ARE_METADATA_ONLY
            && Self::DEBUG_MARKERS_ARE_METADATA_ONLY
    }

    pub const fn backend_ok() -> bool {
        Self::OBSERVABILITY_BACKEND_ACTIVE
            && VaachakHardwareRuntimeWiringPulpBackend::backend_ok()
            && Self::route_ok(Self::route_for(
                VaachakHardwareRuntimeObservationSink::BootMarker,
            ))
            && Self::route_ok(Self::route_for(
                VaachakHardwareRuntimeObservationSink::DebugMarker,
            ))
            && Self::route_ok(Self::route_for(
                VaachakHardwareRuntimeObservationSink::ContractSmoke,
            ))
            && Self::behavior_preserved()
    }
}
