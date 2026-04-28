pub trait PowerHal {
    fn battery_pct(&self) -> u8;
    fn is_charging(&self) -> bool;
    fn deep_sleep(&mut self, wake_after_ms: u32) -> !;
    fn light_sleep(&mut self, ms: u32);
}
