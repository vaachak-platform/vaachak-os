#![cfg_attr(target_arch = "riscv32", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

#[cfg(target_arch = "riscv32")]
extern crate alloc;

#[cfg(target_arch = "riscv32")]
use esp_backtrace as _;

#[cfg(target_arch = "riscv32")]
esp_bootloader_esp_idf::esp_app_desc!();

const PHASE: &str = "bootstrap-phase5-x4-display-hal-smoke";

#[cfg(not(target_arch = "riscv32"))]
fn main() {
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
    use embedded_hal_bus::spi::ExclusiveDevice;
    use esp_hal::clock::CpuClock;
    use esp_hal::delay::Delay;
    use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
    use esp_hal::spi;
    use esp_hal::time::Rate;
    use hal_xteink_x4::X4Ssd1677Smoke;

    esp_println::println!("");
    esp_println::println!("========================================");
    esp_println::println!("VaachakOS X4 display smoke starting");
    esp_println::println!("phase={}", PHASE);
    esp_println::println!("target=esp32c3 riscv32imc-unknown-none-elf");
    esp_println::println!("phase5.4=ported-dma-spidevice-display-path");
    esp_println::println!(
        "note=Phase 5.4 uses the proven DMA SpiDevice shape from x4-reader-os-rs"
    );

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 16_384);
    esp_println::println!("heap=16K boot-smoke only");

    let hal = hal_xteink_x4::X4Hal::new_placeholder();
    let mut os = vaachak_core::VaachakOs::new(hal);

    match os.boot_storage_display_power() {
        Ok(report) => {
            esp_println::println!(
                "model display logical={}x{} native={}x{} rot={:?} strip_rows={}",
                report.display.logical_width,
                report.display.logical_height,
                report.display.native_width,
                report.display.native_height,
                report.display.rotation,
                report.display.strip_rows,
            );
            esp_println::println!(
                "model bus shared_sd_epd={} probe={}kHz runtime={}MHz",
                report.display_bus.shared_sd_epd_bus,
                report.display_bus.probe_khz,
                report.display_bus.runtime_mhz,
            );
            esp_println::println!(
                "model storage state={:?} card_bytes={:?}",
                report.storage.state,
                report.storage.card_size_bytes,
            );
            esp_println::println!(
                "model power battery_mv={} pct={}",
                report.battery_mv,
                report.battery_pct,
            );
        }
        Err(err) => {
            esp_println::println!("VaachakOS model boot smoke failed: {}", err);
        }
    }

    // X4 shares SPI2 between ePaper and SD. Keep SD_CS high and issue the same
    // 80 idle clocks before DMA conversion that the working x4-reader-os-rs
    // board bring-up uses.
    unsafe {
        force_gpio_output_high(12);
    }
    esp_println::println!("phase5.4: forced shared-bus SD_CS GPIO12 high");

    esp_println::println!("phase5.4: configuring SPI2 at 400kHz, then DMA device for EPD");
    esp_println::println!(
        "phase5: pins sclk=GPIO8 mosi=GPIO10 miso=GPIO7 epd_cs=GPIO21 dc=GPIO4 rst=GPIO5 busy=GPIO6 sd_cs=GPIO12-high"
    );

    let slow_cfg = spi::master::Config::default().with_frequency(Rate::from_khz(400));
    let mut spi_raw = spi::master::Spi::new(peripherals.SPI2, slow_cfg)
        .unwrap()
        .with_sck(peripherals.GPIO8)
        .with_mosi(peripherals.GPIO10)
        .with_miso(peripherals.GPIO7);

    let _ = spi_raw.write(&[0xFF; 10]);
    esp_println::println!("phase5.4: sent 80 idle clocks with CS high");

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(4096);
    let dma_rx_buf = esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_dma_bus = spi_raw
        .with_dma(peripherals.DMA_CH0)
        .with_buffers(dma_rx_buf, dma_tx_buf);
    esp_println::println!("phase5.4: DMA buffers enabled 4096B tx/rx");

    let epd_cs = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let busy = Input::new(
        peripherals.GPIO6,
        InputConfig::default().with_pull(Pull::None),
    );
    let mut delay = Delay::new();

    let epd_spi = ExclusiveDevice::new(spi_dma_bus, epd_cs, Delay::new()).unwrap();
    let mut display = X4Ssd1677Smoke::new(epd_spi, dc, rst, busy);

    esp_println::println!("phase5: display init start dma-spidevice");
    display.init(&mut delay);
    esp_println::println!("phase5: display init complete busy={}", display.is_busy());

    esp_println::println!("phase5: full-frame smoke draw start dma-spidevice");
    display.draw_phase5_smoke(&mut delay);
    esp_println::println!(
        "phase5: full-frame smoke refresh complete busy={}",
        display.is_busy()
    );

    esp_println::println!("VaachakOS X4 display smoke complete");
    esp_println::println!("phase5.4=ssd1677-full-frame-smoke-dma-spidevice-ok");
    esp_println::println!("========================================");

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(target_arch = "riscv32")]
unsafe fn force_gpio_output_high(pin: u8) {
    const GPIO_OUT_W1TS: u32 = 0x6000_4008;
    const GPIO_ENABLE_W1TS: u32 = 0x6000_4024;
    const IO_MUX_BASE: u32 = 0x6000_9000;
    const IO_MUX_PIN_STRIDE: u32 = 0x04;
    const GPIO_FUNC_OUT_SEL_BASE: u32 = 0x6000_4554;

    let mask = 1u32 << pin;
    let mux_reg = (IO_MUX_BASE + pin as u32 * IO_MUX_PIN_STRIDE) as *mut u32;
    let out_sel = (GPIO_FUNC_OUT_SEL_BASE + pin as u32 * 4) as *mut u32;

    unsafe {
        let val = mux_reg.read_volatile();
        let val = (val & !(0b111 << 12)) | (1 << 12);
        mux_reg.write_volatile(val);

        out_sel.write_volatile(0x80);
        (GPIO_ENABLE_W1TS as *mut u32).write_volatile(mask);
        (GPIO_OUT_W1TS as *mut u32).write_volatile(mask);
    }
}
