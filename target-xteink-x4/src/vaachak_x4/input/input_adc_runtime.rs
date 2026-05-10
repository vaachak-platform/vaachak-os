#![allow(dead_code)]

use super::input_semantics_runtime::VaachakPhysicalButton;

pub struct VaachakInputAdcRuntimeBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakAdcLadderRow {
    Row1Gpio1,
    Row2Gpio2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakAdcButtonBand {
    pub center_mv: u16,
    pub tolerance_mv: u16,
    pub button: VaachakPhysicalButton,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputTimingPolicy {
    pub oversample_count: u32,
    pub debounce_window_ms: u64,
    pub long_press_window_ms: u64,
    pub repeat_interval_ms: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputAdcRuntimeReport {
    pub row1_classification_ok: bool,
    pub row2_classification_ok: bool,
    pub boundary_rejection_ok: bool,
    pub timing_policy_ok: bool,
    pub physical_adc_sampling_owned: bool,
    pub debounce_loop_owned: bool,
}

impl VaachakInputAdcRuntimeReport {
    pub const fn preflight_ok(self) -> bool {
        self.row1_classification_ok
            && self.row2_classification_ok
            && self.boundary_rejection_ok
            && self.timing_policy_ok
            && !self.physical_adc_sampling_owned
            && !self.debounce_loop_owned
    }
}

impl VaachakInputAdcRuntimeBridge {
    pub const IMPLEMENTATION_OWNER: &'static str = "Vaachak-owned input ADC classification facade";
    pub const PHYSICAL_ADC_SAMPLING_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const DEBOUNCE_LOOP_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const PHYSICAL_ADC_SAMPLING_OWNED_BY_BRIDGE: bool = false;
    pub const DEBOUNCE_LOOP_OWNED_BY_BRIDGE: bool = false;

    pub const ROW1_GPIO: u8 = 1;
    pub const ROW2_GPIO: u8 = 2;
    pub const POWER_GPIO: u8 = 3;
    pub const DEFAULT_TOLERANCE_MV: u16 = 150;
    pub const LOW_RAIL_TOLERANCE_MV: u16 = 50;

    pub const ROW1_BANDS: [VaachakAdcButtonBand; 4] = [
        VaachakAdcButtonBand {
            center_mv: 3,
            tolerance_mv: Self::LOW_RAIL_TOLERANCE_MV,
            button: VaachakPhysicalButton::Right,
        },
        VaachakAdcButtonBand {
            center_mv: 1113,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakPhysicalButton::Left,
        },
        VaachakAdcButtonBand {
            center_mv: 1984,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakPhysicalButton::Confirm,
        },
        VaachakAdcButtonBand {
            center_mv: 2556,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakPhysicalButton::Back,
        },
    ];

    pub const ROW2_BANDS: [VaachakAdcButtonBand; 2] = [
        VaachakAdcButtonBand {
            center_mv: 3,
            tolerance_mv: Self::LOW_RAIL_TOLERANCE_MV,
            button: VaachakPhysicalButton::VolDown,
        },
        VaachakAdcButtonBand {
            center_mv: 1659,
            tolerance_mv: Self::DEFAULT_TOLERANCE_MV,
            button: VaachakPhysicalButton::VolUp,
        },
    ];

    pub const TIMING_POLICY: VaachakInputTimingPolicy = VaachakInputTimingPolicy {
        oversample_count: 4,
        debounce_window_ms: 15,
        long_press_window_ms: 1000,
        repeat_interval_ms: 150,
    };

    pub fn active_runtime_preflight() -> bool {
        Self::preflight_report().preflight_ok()
    }

    pub fn preflight_report() -> VaachakInputAdcRuntimeReport {
        VaachakInputAdcRuntimeReport {
            row1_classification_ok: Self::row1_classification_ok(),
            row2_classification_ok: Self::row2_classification_ok(),
            boundary_rejection_ok: Self::boundary_rejection_ok(),
            timing_policy_ok: Self::timing_policy_ok(),
            physical_adc_sampling_owned: Self::PHYSICAL_ADC_SAMPLING_OWNED_BY_BRIDGE,
            debounce_loop_owned: Self::DEBOUNCE_LOOP_OWNED_BY_BRIDGE,
        }
    }

    pub const fn classify_mv(
        row: VaachakAdcLadderRow,
        millivolts: u16,
    ) -> Option<VaachakPhysicalButton> {
        let bands: &[VaachakAdcButtonBand] = match row {
            VaachakAdcLadderRow::Row1Gpio1 => &Self::ROW1_BANDS,
            VaachakAdcLadderRow::Row2Gpio2 => &Self::ROW2_BANDS,
        };

        let mut idx = 0;
        while idx < bands.len() {
            let band = bands[idx];
            let low = band.center_mv.saturating_sub(band.tolerance_mv);
            let high = band.center_mv.saturating_add(band.tolerance_mv);
            if millivolts >= low && millivolts <= high {
                return Some(band.button);
            }
            idx += 1;
        }

        None
    }

    fn row1_classification_ok() -> bool {
        Self::classify_mv(VaachakAdcLadderRow::Row1Gpio1, 3) == Some(VaachakPhysicalButton::Right)
            && Self::classify_mv(VaachakAdcLadderRow::Row1Gpio1, 1113)
                == Some(VaachakPhysicalButton::Left)
            && Self::classify_mv(VaachakAdcLadderRow::Row1Gpio1, 1984)
                == Some(VaachakPhysicalButton::Confirm)
            && Self::classify_mv(VaachakAdcLadderRow::Row1Gpio1, 2556)
                == Some(VaachakPhysicalButton::Back)
    }

    fn row2_classification_ok() -> bool {
        Self::classify_mv(VaachakAdcLadderRow::Row2Gpio2, 3) == Some(VaachakPhysicalButton::VolDown)
            && Self::classify_mv(VaachakAdcLadderRow::Row2Gpio2, 1659)
                == Some(VaachakPhysicalButton::VolUp)
    }

    fn boundary_rejection_ok() -> bool {
        Self::classify_mv(VaachakAdcLadderRow::Row1Gpio1, 54).is_none()
            && Self::classify_mv(VaachakAdcLadderRow::Row1Gpio1, 960).is_none()
            && Self::classify_mv(VaachakAdcLadderRow::Row2Gpio2, 1540).is_none()
            && Self::classify_mv(VaachakAdcLadderRow::Row2Gpio2, 1900).is_none()
    }

    const fn timing_policy_ok() -> bool {
        Self::TIMING_POLICY.oversample_count == 4
            && Self::TIMING_POLICY.debounce_window_ms == 15
            && Self::TIMING_POLICY.long_press_window_ms == 1000
            && Self::TIMING_POLICY.repeat_interval_ms == 150
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakAdcLadderRow, VaachakInputAdcRuntimeBridge};
    use crate::vaachak_x4::input::input_semantics_runtime::VaachakPhysicalButton;

    #[test]
    fn adc_classification_probe_is_pure_and_valid() {
        assert!(VaachakInputAdcRuntimeBridge::active_runtime_preflight());
    }

    #[test]
    fn classifies_known_ladder_centers() {
        assert_eq!(
            VaachakInputAdcRuntimeBridge::classify_mv(VaachakAdcLadderRow::Row1Gpio1, 1984),
            Some(VaachakPhysicalButton::Confirm)
        );
        assert_eq!(
            VaachakInputAdcRuntimeBridge::classify_mv(VaachakAdcLadderRow::Row2Gpio2, 1659),
            Some(VaachakPhysicalButton::VolUp)
        );
    }
}
