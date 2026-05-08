use super::hardware_native_behavior_consolidation::VaachakHardwareNativeBehaviorConsolidation;
use super::hardware_native_behavior_consolidation_cleanup::VaachakHardwareNativeBehaviorConsolidationCleanup;

/// Lower-level physical driver migration plan for Vaachak OS on Xteink X4.
///
/// This is a planning checkpoint, not a new hardware behavior migration. The
/// accepted Vaachak-native behavior layer already owns input event behavior,
/// display refresh command selection, and storage SD/MMC/FAT command-decision
/// behavior. This plan defines the next driver-level extraction order below
/// that layer while preserving the working PulpCompatibility fallback until
/// each physical driver slice passes hardware smoke.
pub struct VaachakPhysicalDriverMigrationPlan;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakPhysicalDriverMigrationTarget {
    InputPhysicalSampling,
    SpiPhysicalTransaction,
    DisplaySsd1677PhysicalRefresh,
    StorageSdMmcBlockDriver,
    StorageFatAlgorithm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakPhysicalDriverMigrationRisk {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPhysicalDriverMigrationStep {
    pub order: u8,
    pub target: VaachakPhysicalDriverMigrationTarget,
    pub deliverable_name: &'static str,
    pub risk: VaachakPhysicalDriverMigrationRisk,
    pub behavior_to_move: &'static str,
    pub pulp_compatibility_to_keep: &'static str,
    pub acceptance_focus: &'static str,
    pub rollback_gate: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPhysicalDriverMigrationPlanReport {
    pub marker: &'static str,
    pub plan_owner: &'static str,
    pub active_low_level_fallback: &'static str,
    pub native_behavior_consolidation_ready: bool,
    pub native_behavior_cleanup_ready: bool,
    pub first_driver_target: VaachakPhysicalDriverMigrationTarget,
    pub first_driver_deliverable: &'static str,
    pub migration_step_count: usize,
    pub input_physical_sampling_selected_first: bool,
    pub spi_physical_transaction_before_storage_or_display_takeover: bool,
    pub display_low_level_refresh_after_spi_driver_gate: bool,
    pub sd_mmc_block_driver_after_spi_driver_gate: bool,
    pub fat_algorithm_driver_last: bool,
    pub rollback_gates_declared: bool,
    pub hardware_smoke_required_for_every_step: bool,
    pub destructive_storage_operations_deferred: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakPhysicalDriverMigrationPlanReport {
    pub const fn ok(self) -> bool {
        self.native_behavior_consolidation_ready
            && self.native_behavior_cleanup_ready
            && self.input_physical_sampling_selected_first
            && self.spi_physical_transaction_before_storage_or_display_takeover
            && self.display_low_level_refresh_after_spi_driver_gate
            && self.sd_mmc_block_driver_after_spi_driver_gate
            && self.fat_algorithm_driver_last
            && self.rollback_gates_declared
            && self.hardware_smoke_required_for_every_step
            && self.destructive_storage_operations_deferred
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && self.migration_step_count == 5
    }
}

impl VaachakPhysicalDriverMigrationPlan {
    pub const PHYSICAL_DRIVER_MIGRATION_PLAN_MARKER: &'static str =
        "physical_driver_migration_plan=ok";
    pub const PLAN_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_LOW_LEVEL_FALLBACK: &'static str = "PulpCompatibility";
    pub const FIRST_NATIVE_PHYSICAL_DRIVER_DELIVERABLE: &'static str =
        "input_physical_sampling_native_driver";
    pub const MIGRATION_STEP_COUNT: usize = 5;

    pub const INPUT_PHYSICAL_SAMPLING_SELECTED_FIRST: bool = true;
    pub const SPI_PHYSICAL_TRANSACTION_BEFORE_STORAGE_OR_DISPLAY_TAKEOVER: bool = true;
    pub const DISPLAY_LOW_LEVEL_REFRESH_AFTER_SPI_DRIVER_GATE: bool = true;
    pub const SD_MMC_BLOCK_DRIVER_AFTER_SPI_DRIVER_GATE: bool = true;
    pub const FAT_ALGORITHM_DRIVER_LAST: bool = true;
    pub const ROLLBACK_GATES_DECLARED: bool = true;
    pub const HARDWARE_SMOKE_REQUIRED_FOR_EVERY_STEP: bool = true;
    pub const DESTRUCTIVE_STORAGE_OPERATIONS_DEFERRED: bool = true;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;

    pub const DRIVER_MIGRATION_STEPS: [VaachakPhysicalDriverMigrationStep; 5] = [
        VaachakPhysicalDriverMigrationStep {
            order: 1,
            target: VaachakPhysicalDriverMigrationTarget::InputPhysicalSampling,
            deliverable_name: "input_physical_sampling_native_driver",
            risk: VaachakPhysicalDriverMigrationRisk::Low,
            behavior_to_move: "ADC ladder sample interpretation and physical button sampling shell",
            pulp_compatibility_to_keep: "known-good Pulp sampling fallback and current navigation dispatch",
            acceptance_focus: "all buttons respond with unchanged direction/back/select behavior",
            rollback_gate: "disable native sampling path and return to PulpCompatibility sampling",
        },
        VaachakPhysicalDriverMigrationStep {
            order: 2,
            target: VaachakPhysicalDriverMigrationTarget::SpiPhysicalTransaction,
            deliverable_name: "spi_physical_transaction_native_driver",
            risk: VaachakPhysicalDriverMigrationRisk::High,
            behavior_to_move: "shared SPI transaction execution shell and chip-select ownership gate",
            pulp_compatibility_to_keep: "existing Pulp SPI transfer implementation until display and SD smoke pass",
            acceptance_focus: "display refresh and SD listing both work after shared SPI handoff",
            rollback_gate: "switch active transaction executor back to PulpCompatibility",
        },
        VaachakPhysicalDriverMigrationStep {
            order: 3,
            target: VaachakPhysicalDriverMigrationTarget::DisplaySsd1677PhysicalRefresh,
            deliverable_name: "display_ssd1677_physical_refresh_native_driver",
            risk: VaachakPhysicalDriverMigrationRisk::Medium,
            behavior_to_move: "SSD1677 refresh command execution wrapper and BUSY/wait ownership shell",
            pulp_compatibility_to_keep: "existing draw buffer and waveform behavior until refresh smoke passes",
            acceptance_focus: "full and partial refresh remain visually unchanged",
            rollback_gate: "return refresh executor to PulpCompatibility display backend",
        },
        VaachakPhysicalDriverMigrationStep {
            order: 4,
            target: VaachakPhysicalDriverMigrationTarget::StorageSdMmcBlockDriver,
            deliverable_name: "storage_sd_mmc_block_native_driver",
            risk: VaachakPhysicalDriverMigrationRisk::Critical,
            behavior_to_move: "SD card block-driver probe/read shell below the Vaachak storage decision layer",
            pulp_compatibility_to_keep: "existing low-level SD/MMC block I/O until repeated mount/list/read smoke passes",
            acceptance_focus: "boot, mount, listing, TXT open, EPUB open, and back navigation pass repeatedly",
            rollback_gate: "return SD/MMC block executor to PulpCompatibility before any write-capable work",
        },
        VaachakPhysicalDriverMigrationStep {
            order: 5,
            target: VaachakPhysicalDriverMigrationTarget::StorageFatAlgorithm,
            deliverable_name: "storage_fat_algorithm_native_driver",
            risk: VaachakPhysicalDriverMigrationRisk::Critical,
            behavior_to_move: "FAT directory traversal, open, read, and cache path access algorithms",
            pulp_compatibility_to_keep: "existing FAT implementation until non-destructive read smoke is stable",
            acceptance_focus: "long filename mapping, directory listing, TXT, EPUB, and state/cache paths remain stable",
            rollback_gate: "keep destructive FAT operations denied and restore PulpCompatibility FAT executor",
        },
    ];

    pub fn native_behavior_prerequisites_ready() -> bool {
        VaachakHardwareNativeBehaviorConsolidation::native_behavior_consolidation_ok()
            && VaachakHardwareNativeBehaviorConsolidationCleanup::cleanup_ok()
    }

    pub fn migration_steps() -> &'static [VaachakPhysicalDriverMigrationStep; 5] {
        &Self::DRIVER_MIGRATION_STEPS
    }

    pub fn report() -> VaachakPhysicalDriverMigrationPlanReport {
        VaachakPhysicalDriverMigrationPlanReport {
            marker: Self::PHYSICAL_DRIVER_MIGRATION_PLAN_MARKER,
            plan_owner: Self::PLAN_OWNER,
            active_low_level_fallback: Self::ACTIVE_LOW_LEVEL_FALLBACK,
            native_behavior_consolidation_ready:
                VaachakHardwareNativeBehaviorConsolidation::native_behavior_consolidation_ok(),
            native_behavior_cleanup_ready:
                VaachakHardwareNativeBehaviorConsolidationCleanup::cleanup_ok(),
            first_driver_target: VaachakPhysicalDriverMigrationTarget::InputPhysicalSampling,
            first_driver_deliverable: Self::FIRST_NATIVE_PHYSICAL_DRIVER_DELIVERABLE,
            migration_step_count: Self::MIGRATION_STEP_COUNT,
            input_physical_sampling_selected_first: Self::INPUT_PHYSICAL_SAMPLING_SELECTED_FIRST,
            spi_physical_transaction_before_storage_or_display_takeover:
                Self::SPI_PHYSICAL_TRANSACTION_BEFORE_STORAGE_OR_DISPLAY_TAKEOVER,
            display_low_level_refresh_after_spi_driver_gate:
                Self::DISPLAY_LOW_LEVEL_REFRESH_AFTER_SPI_DRIVER_GATE,
            sd_mmc_block_driver_after_spi_driver_gate:
                Self::SD_MMC_BLOCK_DRIVER_AFTER_SPI_DRIVER_GATE,
            fat_algorithm_driver_last: Self::FAT_ALGORITHM_DRIVER_LAST,
            rollback_gates_declared: Self::ROLLBACK_GATES_DECLARED,
            hardware_smoke_required_for_every_step: Self::HARDWARE_SMOKE_REQUIRED_FOR_EVERY_STEP,
            destructive_storage_operations_deferred: Self::DESTRUCTIVE_STORAGE_OPERATIONS_DEFERRED,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
        }
    }

    pub fn migration_plan_ok() -> bool {
        Self::report().ok()
    }

    pub fn emit_physical_driver_migration_plan_marker() {
        if Self::migration_plan_ok() {
            Self::emit_line(Self::PHYSICAL_DRIVER_MIGRATION_PLAN_MARKER);
        }
    }

    #[cfg(any(target_arch = "riscv32", target_os = "none"))]
    fn emit_line(line: &str) {
        esp_println::println!("{}", line);
    }

    #[cfg(not(any(target_arch = "riscv32", target_os = "none")))]
    fn emit_line(line: &str) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakPhysicalDriverMigrationPlan;

    #[test]
    fn physical_driver_migration_plan_is_ready() {
        assert!(VaachakPhysicalDriverMigrationPlan::migration_plan_ok());
    }
}
