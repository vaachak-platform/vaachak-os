use vaachak_core::hal::PowerHal;

#[derive(Default)]
pub struct X4Power;

impl PowerHal for X4Power {
    fn battery_pct(&self) -> u8 { 100 }
    fn is_charging(&self) -> bool { false }
    fn deep_sleep(&mut self, _wake_after_ms: u32) -> ! {
        loop {}
    }
    fn light_sleep(&mut self, _ms: u32) {}
}
