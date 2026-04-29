#![cfg_attr(target_arch = "riscv32", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

#[cfg(target_arch = "riscv32")]
extern crate alloc;

#[cfg(target_arch = "riscv32")]
use esp_backtrace as _;

#[cfg(target_arch = "riscv32")]
esp_bootloader_esp_idf::esp_app_desc!();

const PHASE: &str = "bootstrap-phase6-x4-storage-hal-smoke";

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
    use esp_hal::gpio::{Level, Output, OutputConfig};
    use esp_hal::spi;
    use esp_hal::time::Rate;

    esp_println::println!("");
    esp_println::println!("========================================");
    esp_println::println!("VaachakOS X4 storage smoke starting");
    esp_println::println!("phase={}", PHASE);
    esp_println::println!("target=esp32c3 riscv32imc-unknown-none-elf");
    esp_println::println!("phase6=sd-fat-flat-83-smoke");
    esp_println::println!("note=Phase 6 proves SD init, FAT mount, write, and readback");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 32_768);
    esp_println::println!("heap=32K storage-smoke only");

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

    // X4 shares SPI2 between SD and EPD. Phase 6 is storage-only, so keep EPD_CS
    // high with a typed output and use raw GPIO12 as the SD chip-select because
    // GPIO12 is free only in DIO flash mode and has no esp-hal pin type here.
    let _epd_cs_guard = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let sd_cs = unsafe { RawOutputPin::new_output_high(12) };
    esp_println::println!("phase6: EPD_CS GPIO21 high, SD_CS GPIO12 ready");

    esp_println::println!("phase6: configuring SPI2 for SD init at 400kHz");
    esp_println::println!(
        "phase6: pins sclk=GPIO8 mosi=GPIO10 miso=GPIO7 sd_cs=GPIO12 epd_cs=GPIO21-high"
    );

    let slow_cfg = spi::master::Config::default().with_frequency(Rate::from_khz(400));
    let mut spi_raw = spi::master::Spi::new(peripherals.SPI2, slow_cfg)
        .unwrap()
        .with_sck(peripherals.GPIO8)
        .with_mosi(peripherals.GPIO10)
        .with_miso(peripherals.GPIO7);

    let _ = spi_raw.write(&[0xFF; 10]);
    esp_println::println!("phase6: sent 80 idle clocks with both CS lines high");

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(4096);
    let dma_rx_buf = esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_dma_bus = spi_raw
        .with_dma(peripherals.DMA_CH0)
        .with_buffers(dma_rx_buf, dma_tx_buf);
    esp_println::println!("phase6: DMA buffers enabled 4096B tx/rx");

    let sd_spi = ExclusiveDevice::new(spi_dma_bus, sd_cs, Delay::new()).unwrap();

    match run_storage_smoke(sd_spi) {
        Ok(()) => {
            esp_println::println!("VaachakOS X4 storage smoke complete");
            esp_println::println!("phase6=x4-storage-hal-smoke-ok");
        }
        Err(err) => {
            esp_println::println!("VaachakOS X4 storage smoke failed");
            esp_println::println!("phase6=x4-storage-hal-smoke-failed step={}", err.step);
            esp_println::println!("phase6:error={}", err.detail);
        }
    }

    esp_println::println!("========================================");

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(target_arch = "riscv32")]
const SMOKE_FILE: &str = "VOSMOKE.TXT";
#[cfg(target_arch = "riscv32")]
const SMOKE_DIR: &str = "state";
#[cfg(target_arch = "riscv32")]
const SMOKE_BYTES: &[u8] = b"VaachakOS Phase 6 X4 storage smoke\r\nstate/VOSMOKE.TXT\r\n";

#[cfg(target_arch = "riscv32")]
#[derive(Debug, Clone, Copy)]
struct SmokeError {
    step: &'static str,
    detail: &'static str,
}

#[cfg(target_arch = "riscv32")]
impl SmokeError {
    const fn new(step: &'static str, detail: &'static str) -> Self {
        Self { step, detail }
    }
}

#[cfg(target_arch = "riscv32")]
fn run_storage_smoke<SPI>(sd_spi: SPI) -> Result<(), SmokeError>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    use embedded_sdmmc::{
        AsyncBlockDevice, AsyncVolumeManager, Block, BlockCount, BlockDevice, BlockIdx, Mode,
        RawDirectory, RawVolume, SdCard, TimeSource, Timestamp, VolumeIdx,
    };

    struct BlockDeviceAdapter<D: BlockDevice>(D);

    impl<D: BlockDevice> AsyncBlockDevice for BlockDeviceAdapter<D> {
        type Error = D::Error;

        async fn read(
            &mut self,
            blocks: &mut [Block],
            start_block_idx: BlockIdx,
        ) -> Result<(), Self::Error> {
            self.0.read(blocks, start_block_idx)
        }

        async fn write(
            &mut self,
            blocks: &[Block],
            start_block_idx: BlockIdx,
        ) -> Result<(), Self::Error> {
            self.0.write(blocks, start_block_idx)
        }

        async fn num_blocks(&mut self) -> Result<BlockCount, Self::Error> {
            self.0.num_blocks()
        }
    }

    struct NullTimeSource;

    impl TimeSource for NullTimeSource {
        fn get_timestamp(&self) -> Timestamp {
            Timestamp {
                year_since_1970: 56,
                zero_indexed_month: 0,
                zero_indexed_day: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
        }
    }

    let sd = SdCard::new(sd_spi, esp_hal::delay::Delay::new());

    esp_println::println!("phase6: sd init start");
    let mut card_bytes = None;
    for attempt in 1..=5 {
        match sd.num_bytes() {
            Ok(size) => {
                card_bytes = Some(size);
                esp_println::println!(
                    "phase6: sd initialised attempt={} bytes={} mb={}",
                    attempt,
                    size,
                    size / 1024 / 1024
                );
                break;
            }
            Err(_) => {
                esp_println::println!("phase6: sd init attempt={} failed", attempt);
                sd.mark_card_uninit();
                embedded_hal::delay::DelayNs::delay_ms(&mut esp_hal::delay::Delay::new(), 50);
            }
        }
    }

    let card_bytes = card_bytes.ok_or(SmokeError::new("sd_init", "card did not respond"))?;
    esp_println::println!("phase6: sd card bytes={}", card_bytes);

    let adapter = BlockDeviceAdapter(sd);
    let mut mgr: AsyncVolumeManager<_, NullTimeSource, 4, 4, 1> =
        AsyncVolumeManager::new(adapter, NullTimeSource);

    let vol: RawVolume = poll_once(mgr.open_raw_volume(VolumeIdx(0)))
        .map_err(|_| SmokeError::new("open_volume", "volume 0 open failed"))?;
    let root: RawDirectory = mgr
        .open_root_dir(vol)
        .map_err(|_| SmokeError::new("open_root", "root directory open failed"))?;
    esp_println::println!("phase6: sd mounted volume=0 root=open");

    let state_dir = match poll_once(mgr.open_dir(root, SMOKE_DIR)) {
        Ok(dir) => dir,
        Err(_) => {
            esp_println::println!("phase6: creating {} directory", SMOKE_DIR);
            match poll_once(mgr.make_dir_in_dir(root, SMOKE_DIR)) {
                Ok(()) | Err(embedded_sdmmc::Error::DirAlreadyExists) => {}
                Err(_) => return Err(SmokeError::new("mkdir_state", "failed to create state dir")),
            }
            poll_once(mgr.open_dir(root, SMOKE_DIR))
                .map_err(|_| SmokeError::new("open_state", "state dir open failed after mkdir"))?
        }
    };

    let file =
        poll_once(mgr.open_file_in_dir(state_dir, SMOKE_FILE, Mode::ReadWriteCreateOrTruncate))
            .map_err(|_| SmokeError::new("open_write", "open VOSMOKE.TXT failed"))?;
    poll_once(mgr.write(file, SMOKE_BYTES))
        .map_err(|_| SmokeError::new("write_file", "write VOSMOKE.TXT failed"))?;
    poll_once(mgr.close_file(file))
        .map_err(|_| SmokeError::new("close_write", "close after write failed"))?;
    esp_println::println!(
        "phase6: wrote state/{} bytes={}",
        SMOKE_FILE,
        SMOKE_BYTES.len()
    );

    let file = poll_once(mgr.open_file_in_dir(state_dir, SMOKE_FILE, Mode::ReadOnly))
        .map_err(|_| SmokeError::new("open_read", "open VOSMOKE.TXT read failed"))?;
    let size = mgr.file_length(file).unwrap_or(0);
    let mut read_buf = [0u8; 96];
    let read_len = poll_once(mgr.read(file, &mut read_buf))
        .map_err(|_| SmokeError::new("read_file", "read VOSMOKE.TXT failed"))?;
    poll_once(mgr.close_file(file))
        .map_err(|_| SmokeError::new("close_read", "close after read failed"))?;

    if size < SMOKE_BYTES.len() as u32 || read_len < SMOKE_BYTES.len() {
        return Err(SmokeError::new("readback_len", "readback too short"));
    }
    if &read_buf[..SMOKE_BYTES.len()] != SMOKE_BYTES {
        return Err(SmokeError::new("readback_cmp", "readback mismatch"));
    }
    esp_println::println!(
        "phase6: readback ok state/{} size={} read={}",
        SMOKE_FILE,
        size,
        read_len
    );

    let _ = mgr.close_dir(state_dir);
    let _ = mgr.close_dir(root);
    let _ = poll_once(mgr.close_volume(vol));
    esp_println::println!("phase6: volume closed cleanly");

    Ok(())
}

#[cfg(target_arch = "riscv32")]
fn poll_once<T>(fut: impl core::future::Future<Output = T>) -> T {
    use core::pin::pin;
    use core::task::{Context, Poll, Waker};

    let waker: &Waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut fut = pin!(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("poll_once: future pended -- SPI must be blocking"),
    }
}

#[cfg(target_arch = "riscv32")]
struct RawOutputPin {
    pin: u8,
}

#[cfg(target_arch = "riscv32")]
impl RawOutputPin {
    unsafe fn new_output_high(pin: u8) -> Self {
        let mut p = Self { pin };
        let _ = embedded_hal::digital::OutputPin::set_high(&mut p);
        p.configure_output();
        let _ = embedded_hal::digital::OutputPin::set_high(&mut p);
        p
    }

    fn configure_output(&mut self) {
        const IO_MUX_BASE: u32 = 0x6000_9000;
        const IO_MUX_PIN_STRIDE: u32 = 0x04;
        const GPIO_FUNC_OUT_SEL_BASE: u32 = 0x6000_4554;
        const GPIO_ENABLE_W1TS: u32 = 0x6000_4024;

        let mux_reg = (IO_MUX_BASE + self.pin as u32 * IO_MUX_PIN_STRIDE) as *mut u32;
        let out_sel = (GPIO_FUNC_OUT_SEL_BASE + self.pin as u32 * 4) as *mut u32;
        let mask = 1u32 << self.pin;

        unsafe {
            let val = mux_reg.read_volatile();
            let val = (val & !(0b111 << 12)) | (1 << 12);
            mux_reg.write_volatile(val);
            out_sel.write_volatile(0x80);
            (GPIO_ENABLE_W1TS as *mut u32).write_volatile(mask);
        }
    }
}

#[cfg(target_arch = "riscv32")]
impl embedded_hal::digital::ErrorType for RawOutputPin {
    type Error = core::convert::Infallible;
}

#[cfg(target_arch = "riscv32")]
impl embedded_hal::digital::OutputPin for RawOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        const GPIO_OUT_W1TC: u32 = 0x6000_400c;
        let mask = 1u32 << self.pin;
        unsafe {
            (GPIO_OUT_W1TC as *mut u32).write_volatile(mask);
        }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        const GPIO_OUT_W1TS: u32 = 0x6000_4008;
        let mask = 1u32 << self.pin;
        unsafe {
            (GPIO_OUT_W1TS as *mut u32).write_volatile(mask);
        }
        Ok(())
    }
}
