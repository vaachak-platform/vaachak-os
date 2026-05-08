#![allow(dead_code)]

use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};

/// Pulp-compatible backend descriptor for the hardware executor wiring layer.
///
/// The wiring layer routes selected Vaachak runtime intents through the
/// consolidated `VaachakHardwareRuntimeExecutor` entrypoint. This backend keeps
/// the currently working Pulp-compatible low-level executors active underneath.
pub struct VaachakHardwareRuntimeWiringPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeWiringBackendRoute {
    pub domain: VaachakHardwareExecutorDomain,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub low_level_executor_owner: &'static str,
    pub wired_through_vaachak_executor: bool,
    pub low_level_backend_still_pulp_compatible: bool,
}

impl VaachakHardwareRuntimeWiringPulpBackend {
    pub const BACKEND_NAME: &'static str = VaachakHardwareExecutorPulpBackend::BACKEND_NAME;
    pub const LOW_LEVEL_EXECUTOR_OWNER: &'static str =
        VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER;

    pub const WIRING_BACKEND_ACTIVE: bool = true;
    pub const ROUTES_THROUGH_CONSOLIDATED_VAACHAK_EXECUTOR: bool = true;
    pub const LOW_LEVEL_EXECUTION_REMAINS_PULP_COMPATIBLE: bool = true;

    pub const PHYSICAL_SPI_TRANSFER_REWRITTEN: bool = false;
    pub const SD_MMC_LOW_LEVEL_REWRITTEN: bool = false;
    pub const FAT_STORAGE_REWRITTEN: bool = false;
    pub const SSD1677_DISPLAY_REWRITTEN: bool = false;
    pub const BUTTON_ADC_INPUT_REWRITTEN: bool = false;

    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn route_for(
        domain: VaachakHardwareExecutorDomain,
    ) -> VaachakHardwareRuntimeWiringBackendRoute {
        let route = VaachakHardwareExecutorPulpBackend::route_for(domain);
        VaachakHardwareRuntimeWiringBackendRoute {
            domain,
            backend: route.backend,
            backend_name: route.backend_name,
            low_level_executor_owner: route.active_executor_owner,
            wired_through_vaachak_executor: Self::ROUTES_THROUGH_CONSOLIDATED_VAACHAK_EXECUTOR,
            low_level_backend_still_pulp_compatible: route
                .low_level_executor_remains_pulp_compatible,
        }
    }

    pub const fn route_ok(route: VaachakHardwareRuntimeWiringBackendRoute) -> bool {
        matches!(
            route.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.backend_name.len() == Self::BACKEND_NAME.len()
            && route.low_level_executor_owner.len() == Self::LOW_LEVEL_EXECUTOR_OWNER.len()
            && route.wired_through_vaachak_executor
            && route.low_level_backend_still_pulp_compatible
    }

    pub const fn behavior_preserved() -> bool {
        !Self::PHYSICAL_SPI_TRANSFER_REWRITTEN
            && !Self::SD_MMC_LOW_LEVEL_REWRITTEN
            && !Self::FAT_STORAGE_REWRITTEN
            && !Self::SSD1677_DISPLAY_REWRITTEN
            && !Self::BUTTON_ADC_INPUT_REWRITTEN
            && !Self::READER_FILE_BROWSER_UX_CHANGED
            && !Self::APP_NAVIGATION_CHANGED
            && !Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN
            && !Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN
            && !Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED
    }

    pub const fn backend_ok() -> bool {
        Self::WIRING_BACKEND_ACTIVE
            && VaachakHardwareExecutorPulpBackend::backend_ok()
            && Self::route_ok(Self::route_for(VaachakHardwareExecutorDomain::SpiBus))
            && Self::route_ok(Self::route_for(
                VaachakHardwareExecutorDomain::StorageProbeMount,
            ))
            && Self::route_ok(Self::route_for(VaachakHardwareExecutorDomain::FatStorage))
            && Self::route_ok(Self::route_for(VaachakHardwareExecutorDomain::Display))
            && Self::route_ok(Self::route_for(VaachakHardwareExecutorDomain::Input))
            && Self::behavior_preserved()
    }
}
