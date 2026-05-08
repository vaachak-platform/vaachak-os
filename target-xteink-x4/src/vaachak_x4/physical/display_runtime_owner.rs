#![allow(dead_code)]

use super::display_pulp_backend::VaachakDisplayPulpBackend;
use super::spi_bus_runtime_owner::{
    VaachakSpiBusRuntimeOwner, VaachakSpiRuntimeUser, VaachakSpiTransactionKind,
};

/// Vaachak-owned display runtime ownership entrypoint for Xteink X4.
///
/// This module moves display runtime ownership authority into the Vaachak
/// target layer while keeping SSD1677/e-paper execution in the existing Pulp
/// compatibility backend. It records display identity, geometry, pins,
/// dependency on the shared SPI ownership bridge, and safety metadata. It does
/// not initialize SSD1677, draw pixels, perform full refresh, perform partial
/// refresh, wait on BUSY, or change reader/file browser/storage behavior.
pub struct VaachakDisplayRuntimeOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRuntimeBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayRuntimeOperation {
    PanelIdentityMetadata,
    GeometryMetadata,
    PinMetadata,
    FullRefreshMetadata,
    PartialRefreshMetadata,
    SurfaceRenderMetadata,
    BusyWaitMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayPinMap {
    pub epd_cs_gpio: u8,
    pub epd_dc_gpio: u8,
    pub epd_rst_gpio: u8,
    pub epd_busy_gpio: u8,
    pub spi_sclk_gpio: u8,
    pub spi_mosi_gpio: u8,
    pub spi_miso_gpio: u8,
    pub storage_sd_cs_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayGeometryMap {
    pub native_width: u16,
    pub native_height: u16,
    pub logical_width: u16,
    pub logical_height: u16,
    pub rotation_degrees: u16,
    pub strip_rows: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayOperationOwnership {
    pub operation: VaachakDisplayRuntimeOperation,
    pub backend: VaachakDisplayRuntimeBackend,
    pub ownership_authority: &'static str,
    pub active_executor_owner: &'static str,
    pub shared_spi_dependency_ready: bool,
    pub display_user_registered_on_spi: bool,
    pub behavior_execution_moved_to_vaachak: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayRuntimeOwnershipReport {
    pub display_runtime_ownership_authority_moved_to_vaachak: bool,
    pub active_backend_is_pulp_compatibility: bool,
    pub display_user_registered_on_spi: bool,
    pub shared_spi_dependency_ready: bool,
    pub pin_map_ok: bool,
    pub geometry_map_ok: bool,
    pub backend_bridge_ok: bool,
    pub ssd1677_executor_moved_to_vaachak: bool,
    pub draw_refresh_partial_refresh_moved_to_vaachak: bool,
    pub spi_transaction_executor_moved_to_vaachak: bool,
    pub storage_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakDisplayRuntimeOwnershipReport {
    pub const fn ownership_ok(self) -> bool {
        self.display_runtime_ownership_authority_moved_to_vaachak
            && self.active_backend_is_pulp_compatibility
            && self.display_user_registered_on_spi
            && self.shared_spi_dependency_ready
            && self.pin_map_ok
            && self.geometry_map_ok
            && self.backend_bridge_ok
            && !self.ssd1677_executor_moved_to_vaachak
            && !self.draw_refresh_partial_refresh_moved_to_vaachak
            && !self.spi_transaction_executor_moved_to_vaachak
            && !self.storage_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakDisplayRuntimeOwner {
    pub const DISPLAY_RUNTIME_OWNERSHIP_MARKER: &'static str = "x4-display-runtime-owner-ok";

    pub const DISPLAY_RUNTIME_IDENTITY: &'static str = "xteink-x4-ssd1677-display-runtime";
    pub const DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY: &'static str = "target-xteink-x4 Vaachak layer";
    pub const DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;

    pub const PULP_COMPATIBILITY_BACKEND: VaachakDisplayRuntimeBackend =
        VaachakDisplayRuntimeBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND: VaachakDisplayRuntimeBackend = Self::PULP_COMPATIBILITY_BACKEND;
    pub const ACTIVE_BACKEND_NAME: &'static str = VaachakDisplayPulpBackend::BACKEND_NAME;
    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    pub const DISPLAY_PANEL_NAME: &'static str = "SSD1677 e-paper display";
    pub const DISPLAY_USER_NAME: &'static str = "SSD1677 display";

    pub const EPD_CS_GPIO: u8 = 21;
    pub const EPD_DC_GPIO: u8 = 4;
    pub const EPD_RST_GPIO: u8 = 5;
    pub const EPD_BUSY_GPIO: u8 = 6;

    pub const SPI_SCLK_GPIO: u8 = 8;
    pub const SPI_MOSI_GPIO: u8 = 10;
    pub const SPI_MISO_GPIO: u8 = 7;
    pub const STORAGE_SD_CS_GPIO: u8 = 12;

    pub const NATIVE_WIDTH: u16 = 800;
    pub const NATIVE_HEIGHT: u16 = 480;
    pub const LOGICAL_WIDTH: u16 = 480;
    pub const LOGICAL_HEIGHT: u16 = 800;
    pub const ROTATION_DEGREES: u16 = 270;
    pub const STRIP_ROWS: u16 = 40;

    pub const SSD1677_WRITE_RAM_CMD: u8 = 0x24;
    pub const SSD1677_WRITE_PREVIOUS_RAM_CMD: u8 = 0x26;
    pub const SSD1677_DISPLAY_UPDATE_CONTROL_2_CMD: u8 = 0x22;
    pub const SSD1677_MASTER_ACTIVATION_CMD: u8 = 0x20;

    pub const SHARED_SPI_DEPENDENCY_OWNER: &'static str = "VaachakSpiBusRuntimeOwner";
    pub const DISPLAY_SPI_TRANSACTION_KIND: VaachakSpiTransactionKind =
        VaachakSpiTransactionKind::DisplayRefreshMetadata;

    pub const SSD1677_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BUSY_WAIT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn pin_map() -> VaachakDisplayPinMap {
        VaachakDisplayPinMap {
            epd_cs_gpio: Self::EPD_CS_GPIO,
            epd_dc_gpio: Self::EPD_DC_GPIO,
            epd_rst_gpio: Self::EPD_RST_GPIO,
            epd_busy_gpio: Self::EPD_BUSY_GPIO,
            spi_sclk_gpio: Self::SPI_SCLK_GPIO,
            spi_mosi_gpio: Self::SPI_MOSI_GPIO,
            spi_miso_gpio: Self::SPI_MISO_GPIO,
            storage_sd_cs_gpio: Self::STORAGE_SD_CS_GPIO,
        }
    }

    pub const fn geometry_map() -> VaachakDisplayGeometryMap {
        VaachakDisplayGeometryMap {
            native_width: Self::NATIVE_WIDTH,
            native_height: Self::NATIVE_HEIGHT,
            logical_width: Self::LOGICAL_WIDTH,
            logical_height: Self::LOGICAL_HEIGHT,
            rotation_degrees: Self::ROTATION_DEGREES,
            strip_rows: Self::STRIP_ROWS,
        }
    }

    pub const fn operation_ownership(
        operation: VaachakDisplayRuntimeOperation,
    ) -> VaachakDisplayOperationOwnership {
        VaachakDisplayOperationOwnership {
            operation,
            backend: Self::ACTIVE_BACKEND,
            ownership_authority: Self::DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY,
            active_executor_owner: Self::ACTIVE_EXECUTOR_OWNER,
            shared_spi_dependency_ready: Self::shared_spi_dependency_ready(),
            display_user_registered_on_spi: Self::display_user_registered_on_spi(),
            behavior_execution_moved_to_vaachak: false,
        }
    }

    pub const fn operation_metadata_is_safe(metadata: VaachakDisplayOperationOwnership) -> bool {
        metadata.ownership_authority.len() == Self::DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY.len()
            && metadata.active_executor_owner.len() == Self::ACTIVE_EXECUTOR_OWNER.len()
            && matches!(
                metadata.backend,
                VaachakDisplayRuntimeBackend::PulpCompatibility
            )
            && metadata.shared_spi_dependency_ready
            && metadata.display_user_registered_on_spi
            && !metadata.behavior_execution_moved_to_vaachak
    }

    pub const fn shared_spi_dependency_ready() -> bool {
        VaachakSpiBusRuntimeOwner::ownership_bridge_ok()
    }

    pub const fn display_user_registered_on_spi() -> bool {
        VaachakSpiBusRuntimeOwner::display_user_registered()
            && VaachakSpiBusRuntimeOwner::chip_select_gpio(VaachakSpiRuntimeUser::Display)
                == Self::EPD_CS_GPIO
    }

    pub const fn pin_map_ok() -> bool {
        let pins = Self::pin_map();
        pins.epd_cs_gpio == 21
            && pins.epd_dc_gpio == 4
            && pins.epd_rst_gpio == 5
            && pins.epd_busy_gpio == 6
            && pins.spi_sclk_gpio == 8
            && pins.spi_mosi_gpio == 10
            && pins.spi_miso_gpio == 7
            && pins.storage_sd_cs_gpio == 12
    }

    pub const fn geometry_map_ok() -> bool {
        let geometry = Self::geometry_map();
        geometry.native_width == 800
            && geometry.native_height == 480
            && geometry.logical_width == 480
            && geometry.logical_height == 800
            && geometry.rotation_degrees == 270
            && geometry.strip_rows == 40
            && (geometry.native_width as u32 * geometry.native_height as u32)
                == (geometry.logical_width as u32 * geometry.logical_height as u32)
    }

    pub const fn rendering_execution_moved_to_vaachak() -> bool {
        Self::DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK
            || Self::DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK
            || Self::DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK
            || Self::DISPLAY_BUSY_WAIT_EXECUTOR_MOVED_TO_VAACHAK
    }

    pub const fn report() -> VaachakDisplayRuntimeOwnershipReport {
        VaachakDisplayRuntimeOwnershipReport {
            display_runtime_ownership_authority_moved_to_vaachak:
                Self::DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            active_backend_is_pulp_compatibility: matches!(
                Self::ACTIVE_BACKEND,
                VaachakDisplayRuntimeBackend::PulpCompatibility
            ),
            display_user_registered_on_spi: Self::display_user_registered_on_spi(),
            shared_spi_dependency_ready: Self::shared_spi_dependency_ready(),
            pin_map_ok: Self::pin_map_ok(),
            geometry_map_ok: Self::geometry_map_ok(),
            backend_bridge_ok: VaachakDisplayPulpBackend::bridge_ok(),
            ssd1677_executor_moved_to_vaachak: Self::SSD1677_EXECUTOR_MOVED_TO_VAACHAK,
            draw_refresh_partial_refresh_moved_to_vaachak:
                Self::rendering_execution_moved_to_vaachak(),
            spi_transaction_executor_moved_to_vaachak:
                Self::DISPLAY_SPI_TRANSACTION_EXECUTOR_MOVED_TO_VAACHAK,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn ownership_ok() -> bool {
        Self::report().ownership_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakDisplayRuntimeOperation, VaachakDisplayRuntimeOwner};

    #[test]
    fn display_runtime_ownership_entrypoint_is_active() {
        assert!(VaachakDisplayRuntimeOwner::ownership_ok());
    }

    #[test]
    fn display_pins_and_geometry_are_xteink_x4_values() {
        let pins = VaachakDisplayRuntimeOwner::pin_map();
        assert_eq!(pins.epd_cs_gpio, 21);
        assert_eq!(pins.epd_dc_gpio, 4);
        assert_eq!(pins.epd_rst_gpio, 5);
        assert_eq!(pins.epd_busy_gpio, 6);
        assert_eq!(pins.storage_sd_cs_gpio, 12);

        let geometry = VaachakDisplayRuntimeOwner::geometry_map();
        assert_eq!(geometry.native_width, 800);
        assert_eq!(geometry.native_height, 480);
        assert_eq!(geometry.logical_width, 480);
        assert_eq!(geometry.logical_height, 800);
        assert_eq!(geometry.rotation_degrees, 270);
    }

    #[test]
    fn operation_metadata_stays_pulp_backed() {
        let metadata = VaachakDisplayRuntimeOwner::operation_ownership(
            VaachakDisplayRuntimeOperation::FullRefreshMetadata,
        );
        assert!(VaachakDisplayRuntimeOwner::operation_metadata_is_safe(
            metadata
        ));
        assert!(!metadata.behavior_execution_moved_to_vaachak);
    }
}
