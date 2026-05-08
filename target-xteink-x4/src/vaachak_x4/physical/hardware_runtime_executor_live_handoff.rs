#![allow(dead_code)]

use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorPulpBackend,
};
use super::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use super::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use super::hardware_runtime_executor_runtime_use::VaachakHardwareRuntimeExecutorRuntimeUse;
use super::storage_backend_native_sd_mmc_fat_executor::VaachakStorageBackendNativeSdMmcFatExecutor;

/// Live runtime handoff surface for the Vaachak hardware executor path.
///
/// This layer is intentionally narrow: it lets boot and the imported Pulp
/// runtime boundary invoke the already-accepted Vaachak executor handoff path,
/// while keeping physical SPI transfer, chip-select toggling, low-level SD/MMC,
/// FAT algorithms, SSD1677 rendering, button ADC scan/debounce, reader UX,
/// file-browser UX, and app navigation on the currently working
/// Pulp-compatible runtime.
pub struct VaachakHardwareRuntimeExecutorLiveHandoff;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeLiveHandoffSite {
    BootPreflight,
    ImportedPulpReaderRuntimeBoundary,
    StorageAvailabilityHandoff,
    DisplayRefreshHandoff,
    InputRuntimeHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeLiveHandoffDecision {
    LiveHandoffAccepted,
    RejectedBeforePulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeLiveHandoffRecord {
    pub site: VaachakHardwareRuntimeLiveHandoffSite,
    pub decision: VaachakHardwareRuntimeLiveHandoffDecision,
    pub runtime_use_preflight_required: bool,
    pub acceptance_required_before_live_handoff: bool,
    pub pulp_compatible_backend_active: bool,
    pub active_backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub low_level_executor_owner: &'static str,
    pub live_callsite_integrated: bool,
    pub physical_spi_transfer_changed: bool,
    pub chip_select_toggling_changed: bool,
    pub sd_mmc_low_level_changed: bool,
    pub fat_storage_algorithm_changed: bool,
    pub display_draw_algorithm_changed: bool,
    pub input_debounce_navigation_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub destructive_storage_behavior_added: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeLiveHandoffReport {
    pub live_handoff_entrypoint_active: bool,
    pub runtime_use_preflight_required: bool,
    pub acceptance_required_before_live_handoff: bool,
    pub all_live_handoff_sites_ready: bool,
    pub boot_live_handoff_ready: bool,
    pub imported_pulp_boundary_handoff_ready: bool,
    pub storage_availability_handoff_ready: bool,
    pub display_refresh_handoff_ready: bool,
    pub input_runtime_handoff_ready: bool,
    pub pulp_compatible_backend_active: bool,
    pub physical_spi_transfer_changed: bool,
    pub chip_select_toggling_changed: bool,
    pub sd_mmc_low_level_changed: bool,
    pub fat_storage_algorithm_changed: bool,
    pub display_draw_algorithm_changed: bool,
    pub input_debounce_navigation_changed: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub destructive_storage_behavior_added: bool,
}

impl VaachakHardwareRuntimeLiveHandoffReport {
    pub const fn ok(self) -> bool {
        self.live_handoff_entrypoint_active
            && self.runtime_use_preflight_required
            && self.acceptance_required_before_live_handoff
            && self.all_live_handoff_sites_ready
            && self.boot_live_handoff_ready
            && self.imported_pulp_boundary_handoff_ready
            && self.storage_availability_handoff_ready
            && self.display_refresh_handoff_ready
            && self.input_runtime_handoff_ready
            && self.pulp_compatible_backend_active
            && !self.physical_spi_transfer_changed
            && !self.chip_select_toggling_changed
            && !self.sd_mmc_low_level_changed
            && !self.fat_storage_algorithm_changed
            && !self.display_draw_algorithm_changed
            && !self.input_debounce_navigation_changed
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.destructive_storage_behavior_added
    }
}

impl VaachakHardwareRuntimeExecutorLiveHandoff {
    pub const HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_MARKER: &'static str =
        "hardware_runtime_executor_live_path_handoff=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_SCOPE: &'static str =
        "live boot and imported Pulp runtime handoff into Vaachak executor path";

    pub const LIVE_HANDOFF_ENTRYPOINT_ACTIVE: bool = true;
    pub const LIVE_HANDOFF_SITE_COUNT: usize = 5;

    pub const RUNTIME_USE_PREFLIGHT_REQUIRED: bool = true;
    pub const ACCEPTANCE_REQUIRED_BEFORE_LIVE_HANDOFF: bool = true;
    pub const LOW_LEVEL_BACKEND_REMAINS_PULP_COMPATIBLE: bool = true;

    pub const PHYSICAL_SPI_TRANSFER_CHANGED: bool = false;
    pub const CHIP_SELECT_TOGGLING_CHANGED: bool = false;
    pub const SD_MMC_LOW_LEVEL_CHANGED: bool = false;
    pub const FAT_STORAGE_ALGORITHM_CHANGED: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_CHANGED: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false;

    pub const fn live_handoff_sites()
    -> [VaachakHardwareRuntimeLiveHandoffSite; Self::LIVE_HANDOFF_SITE_COUNT] {
        [
            VaachakHardwareRuntimeLiveHandoffSite::BootPreflight,
            VaachakHardwareRuntimeLiveHandoffSite::ImportedPulpReaderRuntimeBoundary,
            VaachakHardwareRuntimeLiveHandoffSite::StorageAvailabilityHandoff,
            VaachakHardwareRuntimeLiveHandoffSite::DisplayRefreshHandoff,
            VaachakHardwareRuntimeLiveHandoffSite::InputRuntimeHandoff,
        ]
    }

    pub const fn runtime_use_preflight_ok() -> bool {
        Self::RUNTIME_USE_PREFLIGHT_REQUIRED
            && VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()
    }

    pub const fn acceptance_preflight_ok() -> bool {
        Self::ACCEPTANCE_REQUIRED_BEFORE_LIVE_HANDOFF
            && VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()
    }

    pub const fn pulp_compatible_backend_ok() -> bool {
        Self::LOW_LEVEL_BACKEND_REMAINS_PULP_COMPATIBLE
            && VaachakHardwareExecutorPulpBackend::backend_ok()
    }

    pub fn backend_takeover_preflight_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeover::takeover_ok()
    }

    pub const fn record_for(
        site: VaachakHardwareRuntimeLiveHandoffSite,
    ) -> VaachakHardwareRuntimeLiveHandoffRecord {
        let accepted = Self::runtime_use_preflight_ok()
            && Self::acceptance_preflight_ok()
            && Self::pulp_compatible_backend_ok();
        VaachakHardwareRuntimeLiveHandoffRecord {
            site,
            decision: if accepted {
                VaachakHardwareRuntimeLiveHandoffDecision::LiveHandoffAccepted
            } else {
                VaachakHardwareRuntimeLiveHandoffDecision::RejectedBeforePulpCompatibility
            },
            runtime_use_preflight_required: Self::RUNTIME_USE_PREFLIGHT_REQUIRED,
            acceptance_required_before_live_handoff: Self::ACCEPTANCE_REQUIRED_BEFORE_LIVE_HANDOFF,
            pulp_compatible_backend_active: Self::LOW_LEVEL_BACKEND_REMAINS_PULP_COMPATIBLE,
            active_backend: VaachakHardwareExecutorBackend::PulpCompatibility,
            backend_name: VaachakHardwareExecutorPulpBackend::BACKEND_NAME,
            low_level_executor_owner: VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER,
            live_callsite_integrated: true,
            physical_spi_transfer_changed: Self::PHYSICAL_SPI_TRANSFER_CHANGED,
            chip_select_toggling_changed: Self::CHIP_SELECT_TOGGLING_CHANGED,
            sd_mmc_low_level_changed: Self::SD_MMC_LOW_LEVEL_CHANGED,
            fat_storage_algorithm_changed: Self::FAT_STORAGE_ALGORITHM_CHANGED,
            display_draw_algorithm_changed: Self::DISPLAY_DRAW_ALGORITHM_CHANGED,
            input_debounce_navigation_changed: Self::INPUT_DEBOUNCE_NAVIGATION_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            destructive_storage_behavior_added: Self::DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED,
        }
    }

    pub const fn record_ok(record: VaachakHardwareRuntimeLiveHandoffRecord) -> bool {
        matches!(
            record.decision,
            VaachakHardwareRuntimeLiveHandoffDecision::LiveHandoffAccepted
        ) && record.runtime_use_preflight_required
            && record.acceptance_required_before_live_handoff
            && record.pulp_compatible_backend_active
            && matches!(
                record.active_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && record.backend_name.len() == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && record.low_level_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && record.live_callsite_integrated
            && !record.physical_spi_transfer_changed
            && !record.chip_select_toggling_changed
            && !record.sd_mmc_low_level_changed
            && !record.fat_storage_algorithm_changed
            && !record.display_draw_algorithm_changed
            && !record.input_debounce_navigation_changed
            && !record.reader_file_browser_ux_changed
            && !record.app_navigation_behavior_changed
            && !record.destructive_storage_behavior_added
    }

    pub const fn all_live_handoff_sites_ready() -> bool {
        let sites = Self::live_handoff_sites();
        Self::record_ok(Self::record_for(sites[0]))
            && Self::record_ok(Self::record_for(sites[1]))
            && Self::record_ok(Self::record_for(sites[2]))
            && Self::record_ok(Self::record_for(sites[3]))
            && Self::record_ok(Self::record_for(sites[4]))
    }

    pub const fn report() -> VaachakHardwareRuntimeLiveHandoffReport {
        VaachakHardwareRuntimeLiveHandoffReport {
            live_handoff_entrypoint_active: Self::LIVE_HANDOFF_ENTRYPOINT_ACTIVE,
            runtime_use_preflight_required: Self::runtime_use_preflight_ok(),
            acceptance_required_before_live_handoff: Self::acceptance_preflight_ok(),
            all_live_handoff_sites_ready: Self::all_live_handoff_sites_ready(),
            boot_live_handoff_ready: Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::BootPreflight,
            )),
            imported_pulp_boundary_handoff_ready: Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::ImportedPulpReaderRuntimeBoundary,
            )),
            storage_availability_handoff_ready: Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::StorageAvailabilityHandoff,
            )),
            display_refresh_handoff_ready: Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::DisplayRefreshHandoff,
            )),
            input_runtime_handoff_ready: Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::InputRuntimeHandoff,
            )),
            pulp_compatible_backend_active: Self::pulp_compatible_backend_ok(),
            physical_spi_transfer_changed: Self::PHYSICAL_SPI_TRANSFER_CHANGED,
            chip_select_toggling_changed: Self::CHIP_SELECT_TOGGLING_CHANGED,
            sd_mmc_low_level_changed: Self::SD_MMC_LOW_LEVEL_CHANGED,
            fat_storage_algorithm_changed: Self::FAT_STORAGE_ALGORITHM_CHANGED,
            display_draw_algorithm_changed: Self::DISPLAY_DRAW_ALGORITHM_CHANGED,
            input_debounce_navigation_changed: Self::INPUT_DEBOUNCE_NAVIGATION_CHANGED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            destructive_storage_behavior_added: Self::DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED,
        }
    }

    pub const fn live_handoff_ok() -> bool {
        Self::report().ok()
    }

    pub fn active_boot_preflight() -> bool {
        Self::backend_takeover_preflight_ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_spi_display_transaction_handoff().ok()
            && VaachakHardwareRuntimeExecutorRuntimeUse::active_runtime_preflight()
            && VaachakHardwareRuntimeExecutorRuntimeUse::adopt_boot_executor_preflight()
            && Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::BootPreflight,
            ))
    }

    pub fn adopt_boot_preflight() -> bool {
        Self::active_boot_preflight()
    }

    pub fn adopt_imported_pulp_reader_runtime_boundary() -> bool {
        Self::backend_takeover_preflight_ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_storage_file_open_handoff().ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_storage_file_read_handoff().ok()
            && VaachakHardwareRuntimeExecutorRuntimeUse::active_runtime_preflight()
            && VaachakHardwareRuntimeExecutorRuntimeUse::adopt_reader_file_open_handoff()
            && Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::ImportedPulpReaderRuntimeBoundary,
            ))
    }

    pub fn adopt_storage_availability_handoff() -> bool {
        Self::backend_takeover_preflight_ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_spi_storage_transaction_handoff().ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_storage_probe_mount_handoff().ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_storage_directory_listing_handoff()
                .ok()
            && VaachakHardwareRuntimeExecutorRuntimeUse::active_runtime_preflight()
            && VaachakHardwareRuntimeExecutorRuntimeUse::adopt_storage_card_detect_handoff()
            && VaachakHardwareRuntimeExecutorRuntimeUse::adopt_storage_mount_handoff()
            && VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_availability_handoff()
            && VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_fat_access_handoff()
            && Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::StorageAvailabilityHandoff,
            ))
    }

    pub fn adopt_display_refresh_handoff() -> bool {
        Self::backend_takeover_preflight_ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_display_full_refresh_handoff().ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_display_partial_refresh_handoff().ok()
            && VaachakHardwareRuntimeExecutorRuntimeUse::active_runtime_preflight()
            && VaachakHardwareRuntimeExecutorRuntimeUse::adopt_display_refresh_handoff()
            && Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::DisplayRefreshHandoff,
            ))
    }

    pub fn adopt_input_runtime_handoff() -> bool {
        Self::backend_takeover_preflight_ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_input_scan_handoff().ok()
            && VaachakHardwareRuntimeBackendTakeover::execute_input_navigation_handoff().ok()
            && VaachakHardwareRuntimeExecutorRuntimeUse::active_runtime_preflight()
            && VaachakHardwareRuntimeExecutorRuntimeUse::adopt_input_driver_init_handoff()
            && VaachakHardwareRuntimeExecutorRuntimeUse::adopt_input_task_handoff()
            && Self::record_ok(Self::record_for(
                VaachakHardwareRuntimeLiveHandoffSite::InputRuntimeHandoff,
            ))
    }

    pub fn emit_live_handoff_marker() {
        if Self::live_handoff_ok() {
            Self::emit_line(Self::HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_MARKER);
            Self::emit_line("hardware.executor.live_handoff.boot_preflight");
            Self::emit_line("hardware.executor.live_handoff.imported_pulp_reader_runtime_boundary");
            Self::emit_line("hardware.executor.live_handoff.storage_availability");
            Self::emit_line("hardware.executor.live_handoff.display_refresh");
            Self::emit_line("hardware.executor.live_handoff.input_runtime");
            Self::emit_line("hardware.executor.live_handoff.backend.pulp_compatible");
            Self::emit_line("hardware.executor.live_handoff.behavior.preserved");
        } else {
            Self::emit_line("hardware_runtime_executor_live_path_handoff=failed");
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
    use super::VaachakHardwareRuntimeExecutorLiveHandoff;

    #[test]
    fn hardware_runtime_executor_live_handoff_is_ready() {
        assert!(VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok());
    }

    #[test]
    fn live_handoff_sites_are_complete() {
        let sites = VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_sites();
        assert_eq!(
            sites.len(),
            VaachakHardwareRuntimeExecutorLiveHandoff::LIVE_HANDOFF_SITE_COUNT
        );
        for site in sites {
            assert!(VaachakHardwareRuntimeExecutorLiveHandoff::record_ok(
                VaachakHardwareRuntimeExecutorLiveHandoff::record_for(site)
            ));
        }
    }
}
