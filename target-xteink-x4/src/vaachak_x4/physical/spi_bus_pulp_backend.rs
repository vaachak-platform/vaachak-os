#![allow(dead_code)]

/// Pulp compatibility backend for the Vaachak-owned SPI runtime owner.
///
/// This module deliberately does not initialize SPI, perform transactions,
/// toggle chip-select lines, probe or mount SD, access FAT, or draw/refresh the
/// display. It only records that the existing imported Pulp runtime is still the
/// active hardware executor behind the new Vaachak-owned SPI ownership entrypoint.
pub struct VaachakSpiPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiPulpBackendRole {
    CompatibilityExecutor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiPulpBackendReport {
    pub backend_active: bool,
    pub active_hardware_executor_is_pulp: bool,
    pub spi_executor_owned_by_vaachak: bool,
    pub sd_executor_owned_by_vaachak: bool,
    pub display_executor_owned_by_vaachak: bool,
    pub arbitration_policy_moved_to_vaachak: bool,
    pub sd_probe_mount_moved_to_vaachak: bool,
    pub sd_fat_moved_to_vaachak: bool,
    pub display_rendering_moved_to_vaachak: bool,
}

impl VaachakSpiPulpBackendReport {
    pub const fn bridge_ok(self) -> bool {
        self.backend_active
            && self.active_hardware_executor_is_pulp
            && !self.spi_executor_owned_by_vaachak
            && !self.sd_executor_owned_by_vaachak
            && !self.display_executor_owned_by_vaachak
            && !self.arbitration_policy_moved_to_vaachak
            && !self.sd_probe_mount_moved_to_vaachak
            && !self.sd_fat_moved_to_vaachak
            && !self.display_rendering_moved_to_vaachak
    }
}

impl VaachakSpiPulpBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const BACKEND_ROLE: VaachakSpiPulpBackendRole =
        VaachakSpiPulpBackendRole::CompatibilityExecutor;

    pub const ACTIVE_HARDWARE_EXECUTOR: bool = true;
    pub const ACTIVE_HARDWARE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const PHYSICAL_SPI_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const PHYSICAL_SD_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const PHYSICAL_DISPLAY_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const SPI_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const SD_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const ARBITRATION_POLICY_MOVED_TO_VAACHAK: bool = false;
    pub const SD_PROBE_MOUNT_MOVED_TO_VAACHAK: bool = false;
    pub const SD_FAT_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_RENDERING_MOVED_TO_VAACHAK: bool = false;

    pub const fn report() -> VaachakSpiPulpBackendReport {
        VaachakSpiPulpBackendReport {
            backend_active: Self::ACTIVE_HARDWARE_EXECUTOR,
            active_hardware_executor_is_pulp: true,
            spi_executor_owned_by_vaachak: Self::SPI_EXECUTOR_MOVED_TO_VAACHAK,
            sd_executor_owned_by_vaachak: Self::SD_EXECUTOR_MOVED_TO_VAACHAK,
            display_executor_owned_by_vaachak: Self::DISPLAY_EXECUTOR_MOVED_TO_VAACHAK,
            arbitration_policy_moved_to_vaachak: Self::ARBITRATION_POLICY_MOVED_TO_VAACHAK,
            sd_probe_mount_moved_to_vaachak: Self::SD_PROBE_MOUNT_MOVED_TO_VAACHAK,
            sd_fat_moved_to_vaachak: Self::SD_FAT_MOVED_TO_VAACHAK,
            display_rendering_moved_to_vaachak: Self::DISPLAY_RENDERING_MOVED_TO_VAACHAK,
        }
    }

    pub const fn bridge_ok() -> bool {
        Self::report().bridge_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakSpiPulpBackend;

    #[test]
    fn pulp_backend_remains_active_executor() {
        assert!(VaachakSpiPulpBackend::bridge_ok());
        assert_eq!(
            VaachakSpiPulpBackend::ACTIVE_HARDWARE_EXECUTOR_OWNER,
            "vendor/pulp-os imported runtime"
        );
    }
}
