#![allow(dead_code)]

/// Pulp compatibility backend for the Vaachak display runtime owner.
///
/// This backend is deliberately descriptive. It records that the active
/// SSD1677/e-paper executor remains the existing imported Pulp runtime. It must
/// not initialize the display, send SSD1677 commands, draw pixels, perform full
/// or partial refreshes, wait on BUSY, or touch shared SPI chip-select lines.
pub struct VaachakDisplayPulpBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayPulpBackendReport {
    pub active_hardware_executor: bool,
    pub active_ssd1677_executor_owner: &'static str,
    pub active_draw_executor_owner: &'static str,
    pub active_full_refresh_executor_owner: &'static str,
    pub active_partial_refresh_executor_owner: &'static str,
    pub active_busy_wait_executor_owner: &'static str,
    pub active_rotation_executor_owner: &'static str,
    pub active_strip_render_executor_owner: &'static str,
    pub ssd1677_executor_moved_to_vaachak: bool,
    pub draw_executor_moved_to_vaachak: bool,
    pub refresh_executor_moved_to_vaachak: bool,
    pub partial_refresh_executor_moved_to_vaachak: bool,
    pub spi_transaction_executor_moved_to_vaachak: bool,
    pub storage_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakDisplayPulpBackendReport {
    pub const fn bridge_ok(self) -> bool {
        self.active_hardware_executor
            && self.active_ssd1677_executor_owner.len()
                == VaachakDisplayPulpBackend::ACTIVE_SSD1677_EXECUTOR_OWNER.len()
            && self.active_draw_executor_owner.len()
                == VaachakDisplayPulpBackend::ACTIVE_DRAW_EXECUTOR_OWNER.len()
            && self.active_full_refresh_executor_owner.len()
                == VaachakDisplayPulpBackend::ACTIVE_FULL_REFRESH_EXECUTOR_OWNER.len()
            && self.active_partial_refresh_executor_owner.len()
                == VaachakDisplayPulpBackend::ACTIVE_PARTIAL_REFRESH_EXECUTOR_OWNER.len()
            && self.active_busy_wait_executor_owner.len()
                == VaachakDisplayPulpBackend::ACTIVE_BUSY_WAIT_EXECUTOR_OWNER.len()
            && self.active_rotation_executor_owner.len()
                == VaachakDisplayPulpBackend::ACTIVE_ROTATION_EXECUTOR_OWNER.len()
            && self.active_strip_render_executor_owner.len()
                == VaachakDisplayPulpBackend::ACTIVE_STRIP_RENDER_EXECUTOR_OWNER.len()
            && !self.ssd1677_executor_moved_to_vaachak
            && !self.draw_executor_moved_to_vaachak
            && !self.refresh_executor_moved_to_vaachak
            && !self.partial_refresh_executor_moved_to_vaachak
            && !self.spi_transaction_executor_moved_to_vaachak
            && !self.storage_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakDisplayPulpBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const ACTIVE_HARDWARE_EXECUTOR: bool = true;

    pub const ACTIVE_SSD1677_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_DRAW_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_FULL_REFRESH_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_PARTIAL_REFRESH_EXECUTOR_OWNER: &'static str =
        "vendor/pulp-os imported runtime";
    pub const ACTIVE_BUSY_WAIT_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_ROTATION_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_STRIP_RENDER_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const SSD1677_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DRAW_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn report() -> VaachakDisplayPulpBackendReport {
        VaachakDisplayPulpBackendReport {
            active_hardware_executor: Self::ACTIVE_HARDWARE_EXECUTOR,
            active_ssd1677_executor_owner: Self::ACTIVE_SSD1677_EXECUTOR_OWNER,
            active_draw_executor_owner: Self::ACTIVE_DRAW_EXECUTOR_OWNER,
            active_full_refresh_executor_owner: Self::ACTIVE_FULL_REFRESH_EXECUTOR_OWNER,
            active_partial_refresh_executor_owner: Self::ACTIVE_PARTIAL_REFRESH_EXECUTOR_OWNER,
            active_busy_wait_executor_owner: Self::ACTIVE_BUSY_WAIT_EXECUTOR_OWNER,
            active_rotation_executor_owner: Self::ACTIVE_ROTATION_EXECUTOR_OWNER,
            active_strip_render_executor_owner: Self::ACTIVE_STRIP_RENDER_EXECUTOR_OWNER,
            ssd1677_executor_moved_to_vaachak: Self::SSD1677_EXECUTOR_MOVED_TO_VAACHAK,
            draw_executor_moved_to_vaachak: Self::DRAW_EXECUTOR_MOVED_TO_VAACHAK,
            refresh_executor_moved_to_vaachak: Self::REFRESH_EXECUTOR_MOVED_TO_VAACHAK,
            partial_refresh_executor_moved_to_vaachak:
                Self::PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK,
            spi_transaction_executor_moved_to_vaachak:
                Self::SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn bridge_ok() -> bool {
        Self::report().bridge_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakDisplayPulpBackend;

    #[test]
    fn pulp_backend_keeps_active_display_executor() {
        assert!(VaachakDisplayPulpBackend::bridge_ok());
        assert_eq!(VaachakDisplayPulpBackend::BACKEND_NAME, "PulpCompatibility");
        assert!(VaachakDisplayPulpBackend::ACTIVE_HARDWARE_EXECUTOR);
    }
}
