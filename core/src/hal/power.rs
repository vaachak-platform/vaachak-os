#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChargeState {
    Unknown,
    NotCharging,
    Charging,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatteryReading {
    pub adc_mv: u16,
    pub battery_mv: u16,
    pub percentage: u8,
}

pub trait PowerHal {
    /// Raw ADC-domain battery measurement in millivolts.
    fn adc_mv(&mut self) -> u16;

    /// Actual estimated battery voltage in millivolts.
    fn battery_mv(&mut self) -> u16;

    /// Normalized battery percentage for UI/status use.
    fn battery_pct(&mut self) -> u8;

    fn reading(&mut self) -> BatteryReading {
        BatteryReading {
            adc_mv: self.adc_mv(),
            battery_mv: self.battery_mv(),
            percentage: self.battery_pct(),
        }
    }

    fn charge_state(&self) -> ChargeState {
        ChargeState::Unknown
    }

    fn is_charging(&self) -> bool {
        matches!(self.charge_state(), ChargeState::Charging)
    }

    fn deep_sleep(&mut self, wake_after_ms: u32) -> !;
    fn light_sleep(&mut self, ms: u32);
}
