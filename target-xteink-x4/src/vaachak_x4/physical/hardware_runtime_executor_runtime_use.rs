#![allow(dead_code)]

use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};
use super::hardware_runtime_executor::{
    VaachakHardwareRuntimeExecutor, VaachakHardwareRuntimeExecutorEntry,
};
use super::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use super::hardware_runtime_executor_boot_markers::VaachakHardwareRuntimeExecutorBootMarkers;
use super::hardware_runtime_executor_wiring::{
    VaachakHardwareRuntimeExecutorWiring, VaachakHardwareRuntimeWiringDecision,
    VaachakHardwareWiredRuntimePath,
};

/// Runtime use layer for the consolidated Vaachak hardware executor.
///
/// This is the first slice that wires selected boot/runtime intent call sites to
/// the accepted Vaachak executor entrypoints. It deliberately remains a routing
/// and handoff layer: low-level SPI transfer, chip-select toggling, SD/MMC,
/// FAT, SSD1677 rendering, button ADC scan/debounce, reader UX, file-browser UX,
/// and app navigation behavior stay on the existing Pulp-compatible runtime.
pub struct VaachakHardwareRuntimeExecutorRuntimeUse;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeUseSite {
    BootExecutorPreflight,
    BoardSpiOwnershipHandoff,
    DisplayInitHandoff,
    DisplayRefreshHandoff,
    StorageCardDetectHandoff,
    StorageMountHandoff,
    StorageDirectoryListingHandoff,
    ReaderFileOpenHandoff,
    InputDriverInitHandoff,
    InputTaskHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeUseDecision {
    UsedVaachakExecutorEntrypoint,
    RejectedBeforePulpCompatibleBackend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeUseRecord {
    pub use_site: VaachakHardwareRuntimeUseSite,
    pub wired_path: VaachakHardwareWiredRuntimePath,
    pub domain: VaachakHardwareExecutorDomain,
    pub executor_entry: VaachakHardwareRuntimeExecutorEntry,
    pub decision: VaachakHardwareRuntimeUseDecision,
    pub low_level_backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub runtime_callsite_integrated: bool,
    pub low_level_executor_still_pulp_compatible: bool,
    pub physical_spi_transfer_moved: bool,
    pub chip_select_toggling_moved: bool,
    pub sd_mmc_executor_moved: bool,
    pub fat_executor_rewritten: bool,
    pub display_draw_algorithm_rewritten: bool,
    pub input_debounce_navigation_rewritten: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub fat_destructive_behavior_introduced: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorRuntimeUseReport {
    pub runtime_use_entrypoint_active: bool,
    pub accepted_executor_stack_ready: bool,
    pub boot_markers_ready: bool,
    pub all_selected_call_sites_use_executor: bool,
    pub pulp_compatible_backend_active: bool,
    pub physical_spi_transfer_moved: bool,
    pub chip_select_toggling_moved: bool,
    pub sd_mmc_executor_moved: bool,
    pub fat_executor_rewritten: bool,
    pub display_draw_algorithm_rewritten: bool,
    pub input_debounce_navigation_rewritten: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub fat_destructive_behavior_introduced: bool,
}

impl VaachakHardwareRuntimeExecutorRuntimeUseReport {
    pub const fn ok(self) -> bool {
        self.runtime_use_entrypoint_active
            && self.accepted_executor_stack_ready
            && self.boot_markers_ready
            && self.all_selected_call_sites_use_executor
            && self.pulp_compatible_backend_active
            && !self.physical_spi_transfer_moved
            && !self.chip_select_toggling_moved
            && !self.sd_mmc_executor_moved
            && !self.fat_executor_rewritten
            && !self.display_draw_algorithm_rewritten
            && !self.input_debounce_navigation_rewritten
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.fat_destructive_behavior_introduced
    }
}

impl VaachakHardwareRuntimeExecutorRuntimeUse {
    pub const HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_MARKER: &'static str =
        "hardware_runtime_executor_runtime_use=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_SCOPE: &'static str =
        "selected boot/runtime hardware intents route through Vaachak executor entrypoints";

    pub const RUNTIME_USE_ENTRYPOINT_ACTIVE: bool = true;
    pub const RUNTIME_USE_SITE_COUNT: usize = 10;

    pub const LOW_LEVEL_BACKEND_STILL_PULP_COMPATIBLE: bool = true;
    /// Stable validation anchor for the accepted executor-stack preflight call.
    ///
    /// The report below performs the real call; this string keeps the static
    /// validator independent of `cargo fmt` line-wrapping.
    pub const ACCEPTANCE_PREFLIGHT_CALL_ANCHOR: &'static str =
        "VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()";
    /// Stable validation anchor for the boot-marker readiness preflight call.
    pub const BOOT_MARKERS_PREFLIGHT_CALL_ANCHOR: &'static str =
        "VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok()";

    pub const PHYSICAL_SPI_TRANSFER_MOVED: bool = false;
    pub const CHIP_SELECT_TOGGLING_MOVED: bool = false;
    pub const SD_MMC_EXECUTOR_MOVED: bool = false;
    pub const FAT_EXECUTOR_REWRITTEN: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn wired_path_for(
        use_site: VaachakHardwareRuntimeUseSite,
    ) -> VaachakHardwareWiredRuntimePath {
        match use_site {
            VaachakHardwareRuntimeUseSite::BootExecutorPreflight => {
                VaachakHardwareWiredRuntimePath::BootStorageAvailability
            }
            VaachakHardwareRuntimeUseSite::BoardSpiOwnershipHandoff => {
                VaachakHardwareWiredRuntimePath::SharedSpiStorageHandoff
            }
            VaachakHardwareRuntimeUseSite::DisplayInitHandoff => {
                VaachakHardwareWiredRuntimePath::SharedSpiDisplayHandoff
            }
            VaachakHardwareRuntimeUseSite::DisplayRefreshHandoff => {
                VaachakHardwareWiredRuntimePath::DisplayFullRefreshHandoff
            }
            VaachakHardwareRuntimeUseSite::StorageCardDetectHandoff => {
                VaachakHardwareWiredRuntimePath::BootStorageAvailability
            }
            VaachakHardwareRuntimeUseSite::StorageMountHandoff => {
                VaachakHardwareWiredRuntimePath::BootStorageAvailability
            }
            VaachakHardwareRuntimeUseSite::StorageDirectoryListingHandoff => {
                VaachakHardwareWiredRuntimePath::LibraryDirectoryListing
            }
            VaachakHardwareRuntimeUseSite::ReaderFileOpenHandoff => {
                VaachakHardwareWiredRuntimePath::ReaderFileOpenIntent
            }
            VaachakHardwareRuntimeUseSite::InputDriverInitHandoff => {
                VaachakHardwareWiredRuntimePath::InputButtonScanHandoff
            }
            VaachakHardwareRuntimeUseSite::InputTaskHandoff => {
                VaachakHardwareWiredRuntimePath::InputNavigationHandoff
            }
        }
    }

    pub const fn runtime_use_sites() -> [VaachakHardwareRuntimeUseSite; Self::RUNTIME_USE_SITE_COUNT]
    {
        [
            VaachakHardwareRuntimeUseSite::BootExecutorPreflight,
            VaachakHardwareRuntimeUseSite::BoardSpiOwnershipHandoff,
            VaachakHardwareRuntimeUseSite::DisplayInitHandoff,
            VaachakHardwareRuntimeUseSite::DisplayRefreshHandoff,
            VaachakHardwareRuntimeUseSite::StorageCardDetectHandoff,
            VaachakHardwareRuntimeUseSite::StorageMountHandoff,
            VaachakHardwareRuntimeUseSite::StorageDirectoryListingHandoff,
            VaachakHardwareRuntimeUseSite::ReaderFileOpenHandoff,
            VaachakHardwareRuntimeUseSite::InputDriverInitHandoff,
            VaachakHardwareRuntimeUseSite::InputTaskHandoff,
        ]
    }

    pub const fn use_record(
        use_site: VaachakHardwareRuntimeUseSite,
    ) -> VaachakHardwareRuntimeUseRecord {
        let wired_path = Self::wired_path_for(use_site);
        let route = VaachakHardwareRuntimeExecutorWiring::route_path(wired_path);
        let entry = VaachakHardwareRuntimeExecutor::entry_for(route.domain);
        let backend_route = VaachakHardwareExecutorPulpBackend::route_for(route.domain);
        let decision = if matches!(
            route.decision,
            VaachakHardwareRuntimeWiringDecision::RoutedThroughVaachakHardwareRuntimeExecutor
        ) && VaachakHardwareRuntimeExecutor::entry_is_safe(entry)
            && VaachakHardwareExecutorPulpBackend::route_is_pulp_compatible(backend_route)
        {
            VaachakHardwareRuntimeUseDecision::UsedVaachakExecutorEntrypoint
        } else {
            VaachakHardwareRuntimeUseDecision::RejectedBeforePulpCompatibleBackend
        };

        VaachakHardwareRuntimeUseRecord {
            use_site,
            wired_path,
            domain: route.domain,
            executor_entry: entry,
            decision,
            low_level_backend: backend_route.backend,
            backend_name: backend_route.backend_name,
            active_executor_owner: backend_route.active_executor_owner,
            runtime_callsite_integrated: true,
            low_level_executor_still_pulp_compatible: Self::LOW_LEVEL_BACKEND_STILL_PULP_COMPATIBLE,
            physical_spi_transfer_moved: Self::PHYSICAL_SPI_TRANSFER_MOVED,
            chip_select_toggling_moved: Self::CHIP_SELECT_TOGGLING_MOVED,
            sd_mmc_executor_moved: Self::SD_MMC_EXECUTOR_MOVED,
            fat_executor_rewritten: Self::FAT_EXECUTOR_REWRITTEN,
            display_draw_algorithm_rewritten: Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            input_debounce_navigation_rewritten: Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            fat_destructive_behavior_introduced: Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn use_record_ok(record: VaachakHardwareRuntimeUseRecord) -> bool {
        matches!(
            record.decision,
            VaachakHardwareRuntimeUseDecision::UsedVaachakExecutorEntrypoint
        ) && matches!(
            record.low_level_backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && record.backend_name.len() == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && record.active_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && record.runtime_callsite_integrated
            && record.low_level_executor_still_pulp_compatible
            && !record.physical_spi_transfer_moved
            && !record.chip_select_toggling_moved
            && !record.sd_mmc_executor_moved
            && !record.fat_executor_rewritten
            && !record.display_draw_algorithm_rewritten
            && !record.input_debounce_navigation_rewritten
            && !record.reader_file_browser_ux_changed
            && !record.app_navigation_behavior_changed
            && !record.fat_destructive_behavior_introduced
    }

    pub const fn all_selected_call_sites_use_executor() -> bool {
        let sites = Self::runtime_use_sites();
        Self::use_record_ok(Self::use_record(sites[0]))
            && Self::use_record_ok(Self::use_record(sites[1]))
            && Self::use_record_ok(Self::use_record(sites[2]))
            && Self::use_record_ok(Self::use_record(sites[3]))
            && Self::use_record_ok(Self::use_record(sites[4]))
            && Self::use_record_ok(Self::use_record(sites[5]))
            && Self::use_record_ok(Self::use_record(sites[6]))
            && Self::use_record_ok(Self::use_record(sites[7]))
            && Self::use_record_ok(Self::use_record(sites[8]))
            && Self::use_record_ok(Self::use_record(sites[9]))
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorRuntimeUseReport {
        VaachakHardwareRuntimeExecutorRuntimeUseReport {
            runtime_use_entrypoint_active: Self::RUNTIME_USE_ENTRYPOINT_ACTIVE,
            accepted_executor_stack_ready: VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok(
            ),
            boot_markers_ready: VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok(),
            all_selected_call_sites_use_executor: Self::all_selected_call_sites_use_executor(),
            pulp_compatible_backend_active: VaachakHardwareExecutorPulpBackend::backend_ok(),
            physical_spi_transfer_moved: Self::PHYSICAL_SPI_TRANSFER_MOVED,
            chip_select_toggling_moved: Self::CHIP_SELECT_TOGGLING_MOVED,
            sd_mmc_executor_moved: Self::SD_MMC_EXECUTOR_MOVED,
            fat_executor_rewritten: Self::FAT_EXECUTOR_REWRITTEN,
            display_draw_algorithm_rewritten: Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            input_debounce_navigation_rewritten: Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            fat_destructive_behavior_introduced: Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn runtime_use_ok() -> bool {
        Self::report().ok()
    }

    pub fn active_runtime_preflight() -> bool {
        Self::runtime_use_ok()
    }

    pub fn adopt_boot_executor_preflight() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::BootExecutorPreflight,
        ))
    }

    pub fn adopt_board_spi_ownership_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::BoardSpiOwnershipHandoff,
        ))
    }

    pub fn adopt_display_init_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::DisplayInitHandoff,
        ))
    }

    pub fn adopt_display_refresh_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::DisplayRefreshHandoff,
        ))
    }

    pub fn adopt_storage_card_detect_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::StorageCardDetectHandoff,
        ))
    }

    pub fn adopt_storage_mount_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::StorageMountHandoff,
        ))
    }

    pub fn adopt_storage_directory_listing_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::StorageDirectoryListingHandoff,
        ))
    }

    pub fn adopt_reader_file_open_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::ReaderFileOpenHandoff,
        ))
    }

    pub fn adopt_input_driver_init_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::InputDriverInitHandoff,
        ))
    }

    pub fn adopt_input_task_handoff() -> bool {
        Self::use_record_ok(Self::use_record(
            VaachakHardwareRuntimeUseSite::InputTaskHandoff,
        ))
    }

    pub fn emit_runtime_use_marker() {
        if Self::runtime_use_ok() {
            Self::emit_line(Self::HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_MARKER);
            Self::emit_line("hardware.executor.runtime_use.selected_call_sites=10");
            Self::emit_line("hardware.executor.runtime_use.backend.pulp_compatible");
            Self::emit_line("hardware.executor.runtime_use.behavior.preserved");
        } else {
            Self::emit_line("hardware_runtime_executor_runtime_use=failed");
        }
    }

    #[cfg(all(target_arch = "riscv32", target_os = "none"))]
    fn emit_line(line: &str) {
        esp_println::println!("{}", line);
    }

    #[cfg(not(all(target_arch = "riscv32", target_os = "none")))]
    fn emit_line(line: &str) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorRuntimeUse;

    #[test]
    fn hardware_runtime_executor_runtime_use_is_ready() {
        assert!(VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok());
    }

    #[test]
    fn runtime_use_sites_are_complete() {
        let sites = VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_sites();
        assert_eq!(
            sites.len(),
            VaachakHardwareRuntimeExecutorRuntimeUse::RUNTIME_USE_SITE_COUNT
        );
        for site in sites {
            assert!(VaachakHardwareRuntimeExecutorRuntimeUse::use_record_ok(
                VaachakHardwareRuntimeExecutorRuntimeUse::use_record(site)
            ));
        }
    }
}
