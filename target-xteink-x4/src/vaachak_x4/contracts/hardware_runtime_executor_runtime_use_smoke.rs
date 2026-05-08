#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_executor_runtime_use::{
    VaachakHardwareRuntimeExecutorRuntimeUse, VaachakHardwareRuntimeUseSite,
};

/// Smoke contract for the first runtime-use adoption of the consolidated
/// Vaachak hardware executor.
pub struct VaachakHardwareRuntimeExecutorRuntimeUseSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorRuntimeUseSmokeReport {
    pub runtime_use_marker_present: bool,
    pub runtime_use_entrypoint_ready: bool,
    pub runtime_use_site_count_ok: bool,
    pub boot_preflight_wired: bool,
    pub spi_handoff_wired: bool,
    pub display_handoff_wired: bool,
    pub storage_handoff_wired: bool,
    pub reader_file_open_handoff_wired: bool,
    pub input_handoff_wired: bool,
    pub behavior_preserved: bool,
}

impl VaachakHardwareRuntimeExecutorRuntimeUseSmokeReport {
    pub const fn ok(self) -> bool {
        self.runtime_use_marker_present
            && self.runtime_use_entrypoint_ready
            && self.runtime_use_site_count_ok
            && self.boot_preflight_wired
            && self.spi_handoff_wired
            && self.display_handoff_wired
            && self.storage_handoff_wired
            && self.reader_file_open_handoff_wired
            && self.input_handoff_wired
            && self.behavior_preserved
    }
}

impl VaachakHardwareRuntimeExecutorRuntimeUseSmoke {
    pub const HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_SMOKE_MARKER: &'static str =
        "hardware_runtime_executor_runtime_use_smoke=ok";
    pub const REQUIRED_RUNTIME_USE_SITE_COUNT: usize = 10;

    pub const fn use_site_ok(site: VaachakHardwareRuntimeUseSite) -> bool {
        VaachakHardwareRuntimeExecutorRuntimeUse::use_record_ok(
            VaachakHardwareRuntimeExecutorRuntimeUse::use_record(site),
        )
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorRuntimeUseSmokeReport {
        VaachakHardwareRuntimeExecutorRuntimeUseSmokeReport {
            runtime_use_marker_present:
                VaachakHardwareRuntimeExecutorRuntimeUse::HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_MARKER.len()
                    > 0,
            runtime_use_entrypoint_ready:
                VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok(),
            runtime_use_site_count_ok:
                VaachakHardwareRuntimeExecutorRuntimeUse::RUNTIME_USE_SITE_COUNT
                    == Self::REQUIRED_RUNTIME_USE_SITE_COUNT,
            boot_preflight_wired: Self::use_site_ok(
                VaachakHardwareRuntimeUseSite::BootExecutorPreflight,
            ),
            spi_handoff_wired: Self::use_site_ok(
                VaachakHardwareRuntimeUseSite::BoardSpiOwnershipHandoff,
            ),
            display_handoff_wired: Self::use_site_ok(
                VaachakHardwareRuntimeUseSite::DisplayInitHandoff,
            ) && Self::use_site_ok(VaachakHardwareRuntimeUseSite::DisplayRefreshHandoff),
            storage_handoff_wired: Self::use_site_ok(
                VaachakHardwareRuntimeUseSite::StorageCardDetectHandoff,
            ) && Self::use_site_ok(VaachakHardwareRuntimeUseSite::StorageMountHandoff)
                && Self::use_site_ok(
                    VaachakHardwareRuntimeUseSite::StorageDirectoryListingHandoff,
                ),
            reader_file_open_handoff_wired: Self::use_site_ok(
                VaachakHardwareRuntimeUseSite::ReaderFileOpenHandoff,
            ),
            input_handoff_wired: Self::use_site_ok(
                VaachakHardwareRuntimeUseSite::InputDriverInitHandoff,
            ) && Self::use_site_ok(VaachakHardwareRuntimeUseSite::InputTaskHandoff),
            behavior_preserved: !VaachakHardwareRuntimeExecutorRuntimeUse::PHYSICAL_SPI_TRANSFER_MOVED
                && !VaachakHardwareRuntimeExecutorRuntimeUse::CHIP_SELECT_TOGGLING_MOVED
                && !VaachakHardwareRuntimeExecutorRuntimeUse::SD_MMC_EXECUTOR_MOVED
                && !VaachakHardwareRuntimeExecutorRuntimeUse::FAT_EXECUTOR_REWRITTEN
                && !VaachakHardwareRuntimeExecutorRuntimeUse::DISPLAY_DRAW_ALGORITHM_REWRITTEN
                && !VaachakHardwareRuntimeExecutorRuntimeUse::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN
                && !VaachakHardwareRuntimeExecutorRuntimeUse::READER_FILE_BROWSER_UX_CHANGED
                && !VaachakHardwareRuntimeExecutorRuntimeUse::APP_NAVIGATION_BEHAVIOR_CHANGED
                && !VaachakHardwareRuntimeExecutorRuntimeUse::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn smoke_ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorRuntimeUseSmoke;

    #[test]
    fn hardware_runtime_executor_runtime_use_smoke_is_ready() {
        assert!(VaachakHardwareRuntimeExecutorRuntimeUseSmoke::smoke_ok());
    }
}
