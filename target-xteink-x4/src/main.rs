#![cfg_attr(target_arch = "riscv32", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

#[cfg(target_arch = "riscv32")]
extern crate alloc;

#[cfg(target_arch = "riscv32")]
use esp_backtrace as _;

#[cfg(target_arch = "riscv32")]
esp_bootloader_esp_idf::esp_app_desc!();

const PHASE: &str = "bootstrap-phase4-x4-target-boot-smoke";

#[cfg(not(target_arch = "riscv32"))]
fn main() {
    // Host-friendly smoke path retained so `cargo test/check/clippy --workspace`
    // stays useful on the development machine. The real X4 firmware entry point
    // is compiled only for riscv32imc-unknown-none-elf below.
    let hal = hal_xteink_x4::X4Hal::new_placeholder();
    let mut os = vaachak_core::VaachakOs::new(hal);

    match os.boot_storage_display_power() {
        Ok(report) => {
            println!(
                concat!(
                    "VaachakOS X4 host bootstrap smoke\n",
                    "phase={}\n",
                    "logical={}x{} native={}x{} rot={:?} strip_rows={}\n",
                    "shared_bus={} probe={}kHz runtime={}MHz\n",
                    "storage={:?} card_bytes={:?}\n",
                    "battery={}mV pct={}"
                ),
                PHASE,
                report.display.logical_width,
                report.display.logical_height,
                report.display.native_width,
                report.display.native_height,
                report.display.rotation,
                report.display.strip_rows,
                report.display_bus.shared_sd_epd_bus,
                report.display_bus.probe_khz,
                report.display_bus.runtime_mhz,
                report.storage.state,
                report.storage.card_size_bytes,
                report.battery_mv,
                report.battery_pct,
            );
        }
        Err(err) => {
            eprintln!("bootstrap failed: {err}");
            std::process::exit(1);
        }
    }
}

#[cfg(target_arch = "riscv32")]
#[esp_hal::main]
fn main() -> ! {
    use esp_hal::clock::CpuClock;

    // Do not use the `log` facade for the first hardware smoke. The logger can
    // be filtered out depending on environment/build settings. Direct
    // `esp_println` output is the acceptance signal for Phase 4.4.
    esp_println::println!("");
    esp_println::println!("========================================");
    esp_println::println!("VaachakOS X4 boot smoke starting");
    esp_println::println!("phase={}", PHASE);
    esp_println::println!("target=esp32c3 riscv32imc-unknown-none-elf");
    esp_println::println!("note=display is intentionally not initialized in Phase 4");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    // Small boot-smoke heap only. This is not the final X4 runtime allocator.
    // The working x4-reader-os-rs firmware uses a larger heap plus reclaimed
    // RAM; keep Phase 4 intentionally small until real runtime migration starts.
    esp_alloc::heap_allocator!(size: 16_384);
    esp_println::println!("heap=16K boot-smoke only");

    let hal = hal_xteink_x4::X4Hal::new_placeholder();
    let mut os = vaachak_core::VaachakOs::new(hal);

    match os.boot_storage_display_power() {
        Ok(report) => {
            esp_println::println!(
                "display logical={}x{} native={}x{} rot={:?} strip_rows={}",
                report.display.logical_width,
                report.display.logical_height,
                report.display.native_width,
                report.display.native_height,
                report.display.rotation,
                report.display.strip_rows,
            );
            esp_println::println!(
                "bus shared_sd_epd={} probe={}kHz runtime={}MHz",
                report.display_bus.shared_sd_epd_bus,
                report.display_bus.probe_khz,
                report.display_bus.runtime_mhz,
            );
            esp_println::println!(
                "storage state={:?} card_bytes={:?}",
                report.storage.state,
                report.storage.card_size_bytes,
            );
            esp_println::println!(
                "power battery_mv={} pct={}",
                report.battery_mv,
                report.battery_pct,
            );
            esp_println::println!("VaachakOS X4 boot smoke complete");
        }
        Err(err) => {
            esp_println::println!("VaachakOS X4 boot smoke failed: {}", err);
        }
    }

    esp_println::println!("phase4.4=serial-direct-print-ok");
    esp_println::println!("========================================");

    loop {
        core::hint::spin_loop();
    }
}
