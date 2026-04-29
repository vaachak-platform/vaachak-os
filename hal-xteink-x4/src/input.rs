use vaachak_core::hal::{ButtonEventKind, ButtonId, ButtonThreshold, InputEvent, InputHal};

pub const DEFAULT_TOLERANCE_MV: u16 = 150;

pub const ROW1_THRESHOLDS: [ButtonThreshold; 4] = [
    ButtonThreshold {
        center_mv: 3,
        tolerance_mv: 50,
        logical: ButtonId::Right,
    },
    ButtonThreshold {
        center_mv: 1113,
        tolerance_mv: DEFAULT_TOLERANCE_MV,
        logical: ButtonId::Left,
    },
    ButtonThreshold {
        center_mv: 1984,
        tolerance_mv: DEFAULT_TOLERANCE_MV,
        logical: ButtonId::Select,
    },
    ButtonThreshold {
        center_mv: 2556,
        tolerance_mv: DEFAULT_TOLERANCE_MV,
        logical: ButtonId::Back,
    },
];

pub const ROW2_THRESHOLDS: [ButtonThreshold; 2] = [
    ButtonThreshold {
        center_mv: 3,
        tolerance_mv: 50,
        logical: ButtonId::Down,
    },
    ButtonThreshold {
        center_mv: 1659,
        tolerance_mv: DEFAULT_TOLERANCE_MV,
        logical: ButtonId::Up,
    },
];

#[derive(Default)]
pub struct X4Input {
    hold_consumed: bool,
    queued: Option<InputEvent>,
}

impl X4Input {
    pub fn inject_adc_snapshot(
        &mut self,
        row1_mv: u16,
        row2_mv: u16,
        power_low: bool,
    ) -> Option<InputEvent> {
        let button = if power_low {
            Some(ButtonId::Power)
        } else {
            decode_thresholds(row1_mv, &ROW1_THRESHOLDS)
                .or_else(|| decode_thresholds(row2_mv, &ROW2_THRESHOLDS))
        }?;

        let evt = InputEvent {
            button,
            kind: ButtonEventKind::Press,
        };
        self.queued = Some(evt);
        self.poll()
    }

    pub fn hold_consumed(&self) -> bool {
        self.hold_consumed
    }
}

impl InputHal for X4Input {
    fn poll(&mut self) -> Option<InputEvent> {
        self.queued.take()
    }

    fn reset_hold_state(&mut self) {
        self.hold_consumed = true;
    }

    fn row1_thresholds(&self) -> &[ButtonThreshold] {
        &ROW1_THRESHOLDS
    }

    fn row2_thresholds(&self) -> &[ButtonThreshold] {
        &ROW2_THRESHOLDS
    }
}

fn decode_thresholds(mv: u16, thresholds: &[ButtonThreshold]) -> Option<ButtonId> {
    thresholds.iter().find_map(|threshold| {
        let low = threshold.center_mv.saturating_sub(threshold.tolerance_mv);
        let high = threshold.center_mv.saturating_add(threshold.tolerance_mv);
        if mv >= low && mv <= high {
            Some(threshold.logical)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use vaachak_core::hal::InputHal;

    #[test]
    fn row1_thresholds_preserve_bottom_left_cluster_mapping() {
        let mut input = X4Input::default();
        assert_eq!(
            input.inject_adc_snapshot(3, 4095, false).unwrap().button,
            ButtonId::Right
        );
        assert_eq!(
            input.inject_adc_snapshot(1113, 4095, false).unwrap().button,
            ButtonId::Left
        );
        assert_eq!(
            input.inject_adc_snapshot(1984, 4095, false).unwrap().button,
            ButtonId::Select
        );
        assert_eq!(
            input.inject_adc_snapshot(2556, 4095, false).unwrap().button,
            ButtonId::Back
        );
    }

    #[test]
    fn row2_thresholds_map_vertical_buttons() {
        let mut input = X4Input::default();
        assert_eq!(
            input.inject_adc_snapshot(4095, 3, false).unwrap().button,
            ButtonId::Down
        );
        assert_eq!(
            input.inject_adc_snapshot(4095, 1659, false).unwrap().button,
            ButtonId::Up
        );
    }

    #[test]
    fn power_button_wins_over_adc_ladders() {
        let mut input = X4Input::default();
        assert_eq!(
            input.inject_adc_snapshot(1984, 1659, true).unwrap().button,
            ButtonId::Power
        );
    }

    #[test]
    fn hold_reset_state_is_recorded() {
        let mut input = X4Input::default();
        input.reset_hold_state();
        assert!(input.hold_consumed());
    }
}
