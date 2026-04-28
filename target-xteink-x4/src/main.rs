use hal_xteink_x4::X4Hal;
use vaachak_core::VaachakOs;

fn main() {
    // Host-friendly bootstrap placeholder.
    // Real embedded boot/runtime wiring will replace this once the first X4 slice is extracted.
    let hal = X4Hal::new_placeholder();
    let mut os = VaachakOs::new(hal);
    os.boot();
    println!("vaachak-os skeleton: target-xteink-x4 placeholder boot complete");
}
