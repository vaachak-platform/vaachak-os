#![allow(dead_code)]

pub struct VaachakSpiBusRuntimeBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSharedSpiDevice {
    Display,
    Storage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakSpiBusMode {
    SdProbeSlow,
    OperationalFast,
    DisplayRefresh,
    StorageIo,
    DisplayBusyBackgroundStorage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiPinContract {
    pub sclk_gpio: u8,
    pub mosi_gpio: u8,
    pub miso_gpio: u8,
    pub epd_cs_gpio: u8,
    pub sd_cs_gpio: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiTimingContract {
    pub sd_probe_khz: u32,
    pub operational_mhz: u32,
    pub dma_channel: u8,
    pub dma_tx_bytes: usize,
    pub dma_rx_bytes: usize,
    pub sd_init_before_epd_traffic: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiSelectionState {
    pub display_selected: bool,
    pub storage_selected: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiBusRuntimeReport {
    pub pins_ok: bool,
    pub timing_ok: bool,
    pub selection_rules_ok: bool,
    pub mode_rules_ok: bool,
    pub physical_spi_owned: bool,
    pub physical_sd_owned: bool,
    pub physical_display_owned: bool,
}

impl VaachakSpiBusRuntimeReport {
    pub const fn preflight_ok(self) -> bool {
        self.pins_ok
            && self.timing_ok
            && self.selection_rules_ok
            && self.mode_rules_ok
            && !self.physical_spi_owned
            && !self.physical_sd_owned
            && !self.physical_display_owned
    }
}

impl VaachakSpiBusRuntimeBridge {
    pub const IMPLEMENTATION_OWNER: &'static str = "Vaachak-owned SPI arbitration facade";
    pub const PHYSICAL_SPI_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const PHYSICAL_SD_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const PHYSICAL_DISPLAY_OWNER: &'static str = "Vaachak-owned X4 runtime";

    pub const PHYSICAL_SPI_OWNED_BY_BRIDGE: bool = false;
    pub const PHYSICAL_SD_OWNED_BY_BRIDGE: bool = false;
    pub const PHYSICAL_DISPLAY_OWNED_BY_BRIDGE: bool = false;

    pub const PINS: VaachakSpiPinContract = VaachakSpiPinContract {
        sclk_gpio: 8,
        mosi_gpio: 10,
        miso_gpio: 7,
        epd_cs_gpio: 21,
        sd_cs_gpio: 12,
    };

    pub const TIMING: VaachakSpiTimingContract = VaachakSpiTimingContract {
        sd_probe_khz: 400,
        operational_mhz: 20,
        dma_channel: 0,
        dma_tx_bytes: 4096,
        dma_rx_bytes: 4096,
        sd_init_before_epd_traffic: true,
    };

    pub fn active_runtime_preflight() -> bool {
        Self::preflight_report().preflight_ok()
    }

    pub fn preflight_report() -> VaachakSpiBusRuntimeReport {
        VaachakSpiBusRuntimeReport {
            pins_ok: Self::pins_ok(),
            timing_ok: Self::timing_ok(),
            selection_rules_ok: Self::selection_rules_ok(),
            mode_rules_ok: Self::mode_rules_ok(),
            physical_spi_owned: Self::PHYSICAL_SPI_OWNED_BY_BRIDGE,
            physical_sd_owned: Self::PHYSICAL_SD_OWNED_BY_BRIDGE,
            physical_display_owned: Self::PHYSICAL_DISPLAY_OWNED_BY_BRIDGE,
        }
    }

    pub const fn chip_select_gpio(device: VaachakSharedSpiDevice) -> u8 {
        match device {
            VaachakSharedSpiDevice::Display => Self::PINS.epd_cs_gpio,
            VaachakSharedSpiDevice::Storage => Self::PINS.sd_cs_gpio,
        }
    }

    pub const fn selection_is_valid(selection: VaachakSpiSelectionState) -> bool {
        !(selection.display_selected && selection.storage_selected)
    }

    pub const fn mode_allows_storage_io(mode: VaachakSpiBusMode) -> bool {
        matches!(
            mode,
            VaachakSpiBusMode::SdProbeSlow
                | VaachakSpiBusMode::OperationalFast
                | VaachakSpiBusMode::StorageIo
                | VaachakSpiBusMode::DisplayBusyBackgroundStorage
        )
    }

    pub const fn mode_allows_display_io(mode: VaachakSpiBusMode) -> bool {
        matches!(
            mode,
            VaachakSpiBusMode::OperationalFast | VaachakSpiBusMode::DisplayRefresh
        )
    }

    fn pins_ok() -> bool {
        Self::PINS.sclk_gpio == 8
            && Self::PINS.mosi_gpio == 10
            && Self::PINS.miso_gpio == 7
            && Self::chip_select_gpio(VaachakSharedSpiDevice::Display) == 21
            && Self::chip_select_gpio(VaachakSharedSpiDevice::Storage) == 12
    }

    fn timing_ok() -> bool {
        Self::TIMING.sd_probe_khz == 400
            && Self::TIMING.operational_mhz == 20
            && Self::TIMING.dma_channel == 0
            && Self::TIMING.dma_tx_bytes == 4096
            && Self::TIMING.dma_rx_bytes == 4096
            && Self::TIMING.sd_init_before_epd_traffic
    }

    fn selection_rules_ok() -> bool {
        Self::selection_is_valid(VaachakSpiSelectionState {
            display_selected: false,
            storage_selected: false,
        }) && Self::selection_is_valid(VaachakSpiSelectionState {
            display_selected: true,
            storage_selected: false,
        }) && Self::selection_is_valid(VaachakSpiSelectionState {
            display_selected: false,
            storage_selected: true,
        }) && !Self::selection_is_valid(VaachakSpiSelectionState {
            display_selected: true,
            storage_selected: true,
        })
    }

    fn mode_rules_ok() -> bool {
        Self::mode_allows_storage_io(VaachakSpiBusMode::SdProbeSlow)
            && Self::mode_allows_storage_io(VaachakSpiBusMode::DisplayBusyBackgroundStorage)
            && Self::mode_allows_storage_io(VaachakSpiBusMode::StorageIo)
            && Self::mode_allows_display_io(VaachakSpiBusMode::DisplayRefresh)
            && Self::mode_allows_display_io(VaachakSpiBusMode::OperationalFast)
            && !Self::mode_allows_display_io(VaachakSpiBusMode::SdProbeSlow)
            && !Self::mode_allows_storage_io(VaachakSpiBusMode::DisplayRefresh)
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakSharedSpiDevice, VaachakSpiBusRuntimeBridge, VaachakSpiSelectionState};

    #[test]
    fn shared_spi_probe_is_pure_and_valid() {
        assert!(VaachakSpiBusRuntimeBridge::active_runtime_preflight());
    }

    #[test]
    fn rejects_dual_chip_select() {
        assert!(!VaachakSpiBusRuntimeBridge::selection_is_valid(
            VaachakSpiSelectionState {
                display_selected: true,
                storage_selected: true,
            }
        ));
    }

    #[test]
    fn reports_expected_chip_selects() {
        assert_eq!(
            VaachakSpiBusRuntimeBridge::chip_select_gpio(VaachakSharedSpiDevice::Display),
            21
        );
        assert_eq!(
            VaachakSpiBusRuntimeBridge::chip_select_gpio(VaachakSharedSpiDevice::Storage),
            12
        );
    }
}
