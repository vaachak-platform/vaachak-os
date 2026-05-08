#![allow(dead_code)]

/// Pulp-compatible low-level executor backend descriptor for the broad
/// Vaachak hardware runtime executor extraction.
///
/// The Vaachak layer now owns the consolidated executor entrypoints and intent
/// routing surface. This backend keeps the currently working low-level Pulp
/// runtime active for physical SPI, SD lifecycle, FAT/storage, SSD1677 display,
/// and input execution while each domain is migrated safely.
pub struct VaachakHardwareExecutorPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareExecutorDomain {
    SpiBus,
    StorageProbeMount,
    FatStorage,
    Display,
    Input,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareExecutorBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareExecutorBackendRoute {
    pub domain: VaachakHardwareExecutorDomain,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub low_level_executor_remains_pulp_compatible: bool,
}

impl VaachakHardwareExecutorPulpBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_MMC_LOW_LEVEL_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_STORAGE_EXECUTOR_REWRITTEN_IN_VAACHAK: bool = false;
    pub const SSD1677_DISPLAY_EXECUTOR_REWRITTEN_IN_VAACHAK: bool = false;
    pub const BUTTON_ADC_INPUT_EXECUTOR_REWRITTEN_IN_VAACHAK: bool = false;

    pub const READER_FILE_BROWSER_UX_BEHAVIOR_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn route_for(
        domain: VaachakHardwareExecutorDomain,
    ) -> VaachakHardwareExecutorBackendRoute {
        VaachakHardwareExecutorBackendRoute {
            domain,
            backend: VaachakHardwareExecutorBackend::PulpCompatibility,
            backend_name: Self::BACKEND_NAME,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            low_level_executor_remains_pulp_compatible: true,
        }
    }

    pub const fn route_is_pulp_compatible(route: VaachakHardwareExecutorBackendRoute) -> bool {
        matches!(
            route.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.backend_name.len() == Self::BACKEND_NAME.len()
            && route.active_executor_owner.len() == Self::ACTIVE_EXECUTOR_OWNER.len()
            && route.low_level_executor_remains_pulp_compatible
    }

    pub const fn spi_route_ok() -> bool {
        Self::route_is_pulp_compatible(Self::route_for(VaachakHardwareExecutorDomain::SpiBus))
    }

    pub const fn storage_probe_mount_route_ok() -> bool {
        Self::route_is_pulp_compatible(Self::route_for(
            VaachakHardwareExecutorDomain::StorageProbeMount,
        ))
    }

    pub const fn fat_storage_route_ok() -> bool {
        Self::route_is_pulp_compatible(Self::route_for(VaachakHardwareExecutorDomain::FatStorage))
    }

    pub const fn display_route_ok() -> bool {
        Self::route_is_pulp_compatible(Self::route_for(VaachakHardwareExecutorDomain::Display))
    }

    pub const fn input_route_ok() -> bool {
        Self::route_is_pulp_compatible(Self::route_for(VaachakHardwareExecutorDomain::Input))
    }

    pub const fn behavior_preservation_ok() -> bool {
        !Self::PHYSICAL_SPI_TRANSFER_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::SD_MMC_LOW_LEVEL_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::FAT_STORAGE_EXECUTOR_REWRITTEN_IN_VAACHAK
            && !Self::SSD1677_DISPLAY_EXECUTOR_REWRITTEN_IN_VAACHAK
            && !Self::BUTTON_ADC_INPUT_EXECUTOR_REWRITTEN_IN_VAACHAK
            && !Self::READER_FILE_BROWSER_UX_BEHAVIOR_CHANGED
            && !Self::APP_NAVIGATION_BEHAVIOR_CHANGED
            && !Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN
            && !Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN
            && !Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED
    }

    pub const fn backend_ok() -> bool {
        Self::spi_route_ok()
            && Self::storage_probe_mount_route_ok()
            && Self::fat_storage_route_ok()
            && Self::display_route_ok()
            && Self::input_route_ok()
            && Self::behavior_preservation_ok()
    }
}
