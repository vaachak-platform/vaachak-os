#![allow(dead_code)]

use super::display_physical_ssd1677_native_driver::VaachakSsd1677PhysicalNativeDriver;
use super::input_physical_sampling_native_driver::VaachakInputPhysicalSamplingNativeDriver;
use super::spi_physical_native_driver::VaachakSpiPhysicalNativeDriver;
use super::storage_fat_algorithm_native_driver::VaachakStorageFatAlgorithmNativeDriver;
use super::storage_physical_sd_mmc_native_driver::VaachakStoragePhysicalSdMmcNativeDriver;

/// Canonical Vaachak-owned hardware physical migration map for Xteink X4.
///
/// This checkpoint consolidates the accepted full/native physical migrations:
/// SPI, SSD1677 display, SD/MMC physical storage, FAT filesystem algorithms,
/// and native input sampling interpretation. It intentionally does not delete
/// older Pulp source trees; it records that the active Vaachak runtime ownership
/// map no longer selects Pulp for SPI/display/SD/MMC/FAT physical ownership.
/// The input path still keeps a Pulp-compatible ADC/GPIO read fallback until the
/// target HAL read executor is wired, while Vaachak owns sample interpretation.
pub struct VaachakHardwarePhysicalFullMigrationConsolidation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwarePhysicalMigrationBackend {
    VaachakNativeSpiPhysicalDriver,
    VaachakNativeSsd1677PhysicalDriver,
    VaachakNativeSdMmcPhysicalDriver,
    VaachakNativeFatAlgorithmDriver,
    VaachakPhysicalSamplingWithPulpAdcGpioReadFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwarePhysicalMigrationDomain {
    SpiBus,
    DisplaySsd1677,
    StorageSdMmc,
    StorageFatAlgorithm,
    InputPhysicalSampling,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakPhysicalMigrationDomainStatus {
    pub domain: VaachakHardwarePhysicalMigrationDomain,
    pub active_backend: VaachakHardwarePhysicalMigrationBackend,
    pub ownership_moved_to_vaachak: bool,
    pub imported_pulp_runtime_active: bool,
    pub pulp_fallback_enabled: bool,
    pub target_hal_boundary_remaining: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwarePhysicalMigrationMap {
    pub marker: &'static str,
    pub ownership_layer: &'static str,
    pub spi_backend_name: &'static str,
    pub display_backend_name: &'static str,
    pub storage_sd_mmc_backend_name: &'static str,
    pub storage_fat_backend_name: &'static str,
    pub input_sampling_backend_name: &'static str,
    pub spi_full_migration_ok: bool,
    pub display_full_migration_ok: bool,
    pub storage_sd_mmc_full_migration_ok: bool,
    pub storage_fat_full_migration_ok: bool,
    pub input_physical_sampling_native_ok: bool,
    pub imported_pulp_spi_runtime_active: bool,
    pub imported_pulp_display_runtime_active: bool,
    pub imported_pulp_sd_mmc_runtime_active: bool,
    pub imported_pulp_fat_runtime_active: bool,
    pub input_adc_gpio_read_fallback_remains: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
}

impl VaachakPhysicalMigrationDomainStatus {
    pub const fn ok(self) -> bool {
        self.ownership_moved_to_vaachak
            && !self.imported_pulp_runtime_active
            && !self.pulp_fallback_enabled
    }

    pub const fn input_sampling_ok(self) -> bool {
        self.ownership_moved_to_vaachak
            && !self.imported_pulp_runtime_active
            && self.pulp_fallback_enabled
            && self.target_hal_boundary_remaining
    }
}

impl VaachakHardwarePhysicalMigrationMap {
    pub fn ok(self) -> bool {
        self.spi_full_migration_ok
            && self.display_full_migration_ok
            && self.storage_sd_mmc_full_migration_ok
            && self.storage_fat_full_migration_ok
            && self.input_physical_sampling_native_ok
            && !self.imported_pulp_spi_runtime_active
            && !self.imported_pulp_display_runtime_active
            && !self.imported_pulp_sd_mmc_runtime_active
            && !self.imported_pulp_fat_runtime_active
            && self.input_adc_gpio_read_fallback_remains
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
    }
}

impl VaachakHardwarePhysicalFullMigrationConsolidation {
    pub const MARKER: &'static str = "hardware_physical_full_migration_consolidation=ok";
    pub const OWNERSHIP_LAYER: &'static str = "target-xteink-x4 Vaachak layer";

    pub const SPI_BACKEND_NAME: &'static str = VaachakSpiPhysicalNativeDriver::ACTIVE_BACKEND_NAME;
    pub const DISPLAY_BACKEND_NAME: &'static str =
        VaachakSsd1677PhysicalNativeDriver::ACTIVE_BACKEND_NAME;
    pub const STORAGE_SD_MMC_BACKEND_NAME: &'static str =
        VaachakStoragePhysicalSdMmcNativeDriver::ACTIVE_BACKEND_NAME;
    pub const STORAGE_FAT_BACKEND_NAME: &'static str =
        VaachakStorageFatAlgorithmNativeDriver::ACTIVE_BACKEND_NAME;
    pub const INPUT_SAMPLING_BACKEND_NAME: &'static str =
        VaachakInputPhysicalSamplingNativeDriver::ACTIVE_BACKEND_NAME;

    pub const SPI_STATUS: VaachakPhysicalMigrationDomainStatus = VaachakPhysicalMigrationDomainStatus {
        domain: VaachakHardwarePhysicalMigrationDomain::SpiBus,
        active_backend: VaachakHardwarePhysicalMigrationBackend::VaachakNativeSpiPhysicalDriver,
        ownership_moved_to_vaachak: VaachakSpiPhysicalNativeDriver::SPI_FULLY_MIGRATED_TO_VAACHAK,
        imported_pulp_runtime_active: VaachakSpiPhysicalNativeDriver::IMPORTED_PULP_SPI_RUNTIME_ACTIVE,
        pulp_fallback_enabled: VaachakSpiPhysicalNativeDriver::PULP_SPI_TRANSFER_FALLBACK_ENABLED,
        target_hal_boundary_remaining:
            VaachakSpiPhysicalNativeDriver::LOW_LEVEL_HAL_PERIPHERAL_CALLS_REMAIN_TARGET_HAL_BOUNDARY,
    };

    pub const DISPLAY_STATUS: VaachakPhysicalMigrationDomainStatus =
        VaachakPhysicalMigrationDomainStatus {
            domain: VaachakHardwarePhysicalMigrationDomain::DisplaySsd1677,
            active_backend:
                VaachakHardwarePhysicalMigrationBackend::VaachakNativeSsd1677PhysicalDriver,
            ownership_moved_to_vaachak:
                VaachakSsd1677PhysicalNativeDriver::DISPLAY_PHYSICAL_FULLY_MIGRATED_TO_VAACHAK,
            imported_pulp_runtime_active:
                VaachakSsd1677PhysicalNativeDriver::IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE,
            pulp_fallback_enabled:
                VaachakSsd1677PhysicalNativeDriver::PULP_DISPLAY_EXECUTOR_FALLBACK_ENABLED,
            target_hal_boundary_remaining:
                VaachakSsd1677PhysicalNativeDriver::TARGET_HAL_PIN_SPI_BOUNDARY_REMAINS,
        };

    pub const STORAGE_SD_MMC_STATUS: VaachakPhysicalMigrationDomainStatus =
        VaachakPhysicalMigrationDomainStatus {
            domain: VaachakHardwarePhysicalMigrationDomain::StorageSdMmc,
            active_backend:
                VaachakHardwarePhysicalMigrationBackend::VaachakNativeSdMmcPhysicalDriver,
            ownership_moved_to_vaachak:
                VaachakStoragePhysicalSdMmcNativeDriver::SD_MMC_PHYSICAL_FULLY_MIGRATED_TO_VAACHAK,
            imported_pulp_runtime_active:
                VaachakStoragePhysicalSdMmcNativeDriver::IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE,
            pulp_fallback_enabled:
                VaachakStoragePhysicalSdMmcNativeDriver::PULP_SD_MMC_EXECUTOR_FALLBACK_ENABLED,
            target_hal_boundary_remaining:
                VaachakStoragePhysicalSdMmcNativeDriver::TARGET_HAL_SPI_BOUNDARY_REMAINS,
        };

    pub const STORAGE_FAT_STATUS: VaachakPhysicalMigrationDomainStatus =
        VaachakPhysicalMigrationDomainStatus {
            domain: VaachakHardwarePhysicalMigrationDomain::StorageFatAlgorithm,
            active_backend:
                VaachakHardwarePhysicalMigrationBackend::VaachakNativeFatAlgorithmDriver,
            ownership_moved_to_vaachak:
                VaachakStorageFatAlgorithmNativeDriver::FAT_ALGORITHM_FULLY_MIGRATED_TO_VAACHAK,
            imported_pulp_runtime_active:
                VaachakStorageFatAlgorithmNativeDriver::IMPORTED_PULP_FAT_RUNTIME_ACTIVE,
            pulp_fallback_enabled:
                VaachakStorageFatAlgorithmNativeDriver::PULP_FAT_ALGORITHM_FALLBACK_ENABLED,
            target_hal_boundary_remaining: true,
        };

    pub const INPUT_SAMPLING_STATUS: VaachakPhysicalMigrationDomainStatus =
        VaachakPhysicalMigrationDomainStatus {
            domain: VaachakHardwarePhysicalMigrationDomain::InputPhysicalSampling,
            active_backend:
                VaachakHardwarePhysicalMigrationBackend::VaachakPhysicalSamplingWithPulpAdcGpioReadFallback,
            ownership_moved_to_vaachak: VaachakInputPhysicalSamplingNativeDriver::RAW_ADC_LADDER_SAMPLE_INTERPRETATION_MOVED_TO_VAACHAK,
            imported_pulp_runtime_active: false,
            pulp_fallback_enabled:
                !VaachakInputPhysicalSamplingNativeDriver::ADC_PERIPHERAL_READ_EXECUTOR_MOVED_TO_VAACHAK,
            target_hal_boundary_remaining: true,
        };

    pub fn migration_map() -> VaachakHardwarePhysicalMigrationMap {
        VaachakHardwarePhysicalMigrationMap {
            marker: Self::MARKER,
            ownership_layer: Self::OWNERSHIP_LAYER,
            spi_backend_name: Self::SPI_BACKEND_NAME,
            display_backend_name: Self::DISPLAY_BACKEND_NAME,
            storage_sd_mmc_backend_name: Self::STORAGE_SD_MMC_BACKEND_NAME,
            storage_fat_backend_name: Self::STORAGE_FAT_BACKEND_NAME,
            input_sampling_backend_name: Self::INPUT_SAMPLING_BACKEND_NAME,
            spi_full_migration_ok: VaachakSpiPhysicalNativeDriver::full_migration_ok(),
            display_full_migration_ok: VaachakSsd1677PhysicalNativeDriver::full_migration_ok(),
            storage_sd_mmc_full_migration_ok:
                VaachakStoragePhysicalSdMmcNativeDriver::full_migration_ok(),
            storage_fat_full_migration_ok:
                VaachakStorageFatAlgorithmNativeDriver::full_migration_ok(),
            input_physical_sampling_native_ok:
                VaachakInputPhysicalSamplingNativeDriver::native_physical_sampling_ok(),
            imported_pulp_spi_runtime_active:
                VaachakSpiPhysicalNativeDriver::IMPORTED_PULP_SPI_RUNTIME_ACTIVE,
            imported_pulp_display_runtime_active:
                VaachakSsd1677PhysicalNativeDriver::IMPORTED_PULP_SSD1677_RUNTIME_ACTIVE,
            imported_pulp_sd_mmc_runtime_active:
                VaachakStoragePhysicalSdMmcNativeDriver::IMPORTED_PULP_SD_MMC_RUNTIME_ACTIVE,
            imported_pulp_fat_runtime_active:
                VaachakStorageFatAlgorithmNativeDriver::IMPORTED_PULP_FAT_RUNTIME_ACTIVE,
            input_adc_gpio_read_fallback_remains:
                !VaachakInputPhysicalSamplingNativeDriver::ADC_PERIPHERAL_READ_EXECUTOR_MOVED_TO_VAACHAK,
            reader_file_browser_ux_changed: false,
            app_navigation_behavior_changed: false,
        }
    }

    pub fn consolidation_ok() -> bool {
        Self::SPI_STATUS.ok()
            && Self::DISPLAY_STATUS.ok()
            && Self::STORAGE_SD_MMC_STATUS.ok()
            && Self::STORAGE_FAT_STATUS.ok()
            && Self::INPUT_SAMPLING_STATUS.input_sampling_ok()
            && Self::migration_map().ok()
    }
}
