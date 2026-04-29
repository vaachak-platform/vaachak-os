#![cfg_attr(target_arch = "riscv32", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

#[cfg(target_arch = "riscv32")]
extern crate alloc;

#[cfg(target_arch = "riscv32")]
use esp_backtrace as _;

#[cfg(target_arch = "riscv32")]
esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(target_arch = "riscv32")]
use core::cell::RefCell;
#[cfg(target_arch = "riscv32")]
use embedded_hal::delay::DelayNs;
#[cfg(target_arch = "riscv32")]
use embedded_hal_bus::spi::RefCellDevice;
#[cfg(target_arch = "riscv32")]
use esp_hal::Blocking;
#[cfg(target_arch = "riscv32")]
use esp_hal::analog::adc::{Adc, AdcCalCurve, AdcConfig, AdcPin, Attenuation};
#[cfg(target_arch = "riscv32")]
use esp_hal::clock::CpuClock;
#[cfg(target_arch = "riscv32")]
use esp_hal::delay::Delay;
#[cfg(target_arch = "riscv32")]
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
#[cfg(target_arch = "riscv32")]
use esp_hal::peripherals::{ADC1, GPIO1, GPIO2};
#[cfg(target_arch = "riscv32")]
use esp_hal::spi;
#[cfg(target_arch = "riscv32")]
use esp_hal::time::Rate;
#[cfg(target_arch = "riscv32")]
use hal_xteink_x4::{X4Input, X4Ssd1677Smoke};
#[cfg(target_arch = "riscv32")]
use vaachak_core::hal::{ButtonEventKind, ButtonId, InputEvent, InputHal, InputSample};

const PHASE: &str = "bootstrap-phase8-x4-input-navigation-smoke";

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
    esp_println::println!("");
    esp_println::println!("========================================");
    esp_println::println!("VaachakOS X4 input navigation smoke starting");
    esp_println::println!("phase={}", PHASE);
    esp_println::println!("target=esp32c3 riscv32imc-unknown-none-elf");
    esp_println::println!("phase8=input-navigation-home-smoke");
    esp_println::println!("phase8.5=ported-x4-reader-os-rs-poll-shape");
    esp_println::println!(
        "note=Phase 8 adds ADC ladder input navigation to the minimal Home screen"
    );

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut input_adc = unsafe { X4InputAdc::new(&peripherals) };
    esp_println::println!("phase8: input ADC ready row1=GPIO1 row2=GPIO2 power=GPIO3");
    esp_println::println!(
        "phase8.5: using proven x4-reader-os-rs calibrated thresholds + 10ms poll shape"
    );

    esp_alloc::heap_allocator!(size: 32_768);
    esp_println::println!("heap=32K input-navigation-smoke only");

    let hal = hal_xteink_x4::X4Hal::new_placeholder();
    let mut os = vaachak_core::VaachakOs::new(hal);
    let mut battery_pct = 0u8;

    match os.boot_storage_display_power() {
        Ok(report) => {
            battery_pct = report.battery_pct;
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

    // X4 shares SPI2 between SD and EPD. Phase 8 proves both hardware paths in
    // one firmware using a bus manager so ownership is explicit:
    // - EPD_CS high while SD owns the bus.
    // - SD_CS high while the display owns the bus.
    let epd_cs = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let sd_cs = unsafe { RawOutputPin::new_output_high(12) };
    esp_println::println!("phase8: EPD_CS GPIO21 high, SD_CS GPIO12 high");

    esp_println::println!("phase8: configuring SPI2 shared bus at 400kHz");
    esp_println::println!(
        "phase8: pins sclk=GPIO8 mosi=GPIO10 miso=GPIO7 sd_cs=GPIO12 epd_cs=GPIO21 dc=GPIO4 rst=GPIO5 busy=GPIO6"
    );

    let slow_cfg = spi::master::Config::default().with_frequency(Rate::from_khz(400));
    let mut spi_raw = spi::master::Spi::new(peripherals.SPI2, slow_cfg)
        .unwrap()
        .with_sck(peripherals.GPIO8)
        .with_mosi(peripherals.GPIO10)
        .with_miso(peripherals.GPIO7);

    let _ = spi_raw.write(&[0xFF; 10]);
    esp_println::println!("phase8: sent 80 idle clocks with both CS lines high");

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(4096);
    let dma_rx_buf = esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_dma_bus = spi_raw
        .with_dma(peripherals.DMA_CH0)
        .with_buffers(dma_rx_buf, dma_tx_buf);
    esp_println::println!("phase8: DMA buffers enabled 4096B tx/rx");

    let spi_bus = RefCell::new(spi_dma_bus);

    let storage_ok = {
        let sd_spi = RefCellDevice::new(&spi_bus, sd_cs, Delay::new()).unwrap();
        match run_storage_smoke(sd_spi) {
            Ok(()) => {
                esp_println::println!("phase8: storage smoke ok");
                true
            }
            Err(err) => {
                esp_println::println!("phase8: storage smoke failed step={}", err.step);
                esp_println::println!("phase8:storage-error={}", err.detail);
                false
            }
        }
    };

    // Keep SD deselected before switching the shared bus to the SSD1677.
    unsafe {
        force_gpio_output_high(12);
    }
    esp_println::println!("phase8: SD_CS GPIO12 forced high before display refresh");

    let dc = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let busy = Input::new(
        peripherals.GPIO6,
        InputConfig::default().with_pull(Pull::None),
    );
    let mut delay = Delay::new();

    let epd_spi = RefCellDevice::new(&spi_bus, epd_cs, Delay::new()).unwrap();
    let mut display = X4Ssd1677Smoke::new(epd_spi, dc, rst, busy);

    esp_println::println!("phase8: display init start dma-spidevice");
    display.init(&mut delay);
    esp_println::println!("phase8: display init complete busy={}", display.is_busy());

    let mut selected: u8 = 0;
    esp_println::println!(
        "phase8: initial home render start sd_ok={} battery_pct={} selected={} item={}",
        storage_ok,
        battery_pct,
        selected,
        menu_item_name(selected),
    );
    display.draw_phase8_home(&mut delay, storage_ok, battery_pct, selected);
    esp_println::println!(
        "phase8: initial home refresh complete busy={}",
        display.is_busy()
    );

    esp_println::println!("VaachakOS X4 input navigation smoke ready");
    esp_println::println!("phase8=x4-input-navigation-smoke-ready");
    esp_println::println!("========================================");

    let mut input_model = X4Input::default();
    let mut tick_ms: u32 = 0;
    let mut event_count: u32 = 0;

    loop {
        // Match the proven x4-reader-os-rs input task cadence.
        // The previous Phase 8 path sampled every 20ms and then called a
        // timer-only tick in the same loop. That tick fed `stable` back into
        // the debounce state and reset short-lived candidates before they
        // could cross the 15ms debounce window.
        delay.delay_ms(10);
        tick_ms = tick_ms.wrapping_add(10);

        let snapshot = input_adc.sample(tick_ms);
        if snapshot.has_activity() && tick_ms.wrapping_sub(input_adc.last_activity_log_ms()) >= 250
        {
            input_adc.set_last_activity_log_ms(tick_ms);
            esp_println::println!(
                "phase8.5: adc activity row1={} row2={} power_low={}",
                snapshot.row1_mv,
                snapshot.row2_mv,
                snapshot.sample.power_low,
            );
        }

        if let Some(event) = input_model.ingest_sample(snapshot.sample) {
            if handle_nav_event(event, &mut selected, &mut input_model, &mut event_count) {
                esp_println::println!(
                    "phase8: redraw selected={} item={}",
                    selected,
                    menu_item_name(selected),
                );
                display.draw_phase8_home(&mut delay, storage_ok, battery_pct, selected);
                esp_println::println!("phase8: redraw complete busy={}", display.is_busy());
                esp_println::println!("phase8=x4-input-navigation-smoke-ok");
            }
        }

        while let Some(event) = input_model.poll() {
            if handle_nav_event(event, &mut selected, &mut input_model, &mut event_count) {
                esp_println::println!(
                    "phase8: redraw selected={} item={}",
                    selected,
                    menu_item_name(selected),
                );
                display.draw_phase8_home(&mut delay, storage_ok, battery_pct, selected);
                esp_println::println!("phase8: redraw complete busy={}", display.is_busy());
                esp_println::println!("phase8=x4-input-navigation-smoke-ok");
            }
        }

        // Do not call `tick()` here. For this smoke phase, navigation only
        // needs Press/Release. LongPress/Repeat will be reintroduced after
        // the runtime has a dedicated input task, mirroring x4-reader-os-rs.
    }
}

#[cfg(target_arch = "riscv32")]
struct X4InputAdc {
    adc: Adc<'static, ADC1<'static>, Blocking>,
    row1: AdcPin<GPIO1<'static>, ADC1<'static>, AdcCalCurve<ADC1<'static>>>,
    row2: AdcPin<GPIO2<'static>, ADC1<'static>, AdcCalCurve<ADC1<'static>>>,
    power: Input<'static>,
    idle_logged: bool,
    last_activity_log_ms: u32,
}

#[cfg(target_arch = "riscv32")]
#[derive(Clone, Copy)]
struct X4AdcSnapshot {
    row1_mv: u16,
    row2_mv: u16,
    sample: InputSample,
}

#[cfg(target_arch = "riscv32")]
impl X4AdcSnapshot {
    fn has_activity(self) -> bool {
        self.sample.power_low || self.sample.row1_mv < 2850 || self.sample.row2_mv < 2850
    }
}

#[cfg(target_arch = "riscv32")]
impl X4InputAdc {
    unsafe fn new(p: &esp_hal::peripherals::Peripherals) -> Self {
        let mut adc_cfg = AdcConfig::new();
        let row1 = adc_cfg.enable_pin_with_cal::<_, AdcCalCurve<ADC1>>(
            unsafe { p.GPIO1.clone_unchecked() },
            Attenuation::_11dB,
        );
        let row2 = adc_cfg.enable_pin_with_cal::<_, AdcCalCurve<ADC1>>(
            unsafe { p.GPIO2.clone_unchecked() },
            Attenuation::_11dB,
        );
        let adc = Adc::new(unsafe { p.ADC1.clone_unchecked() }, adc_cfg);
        let power = Input::new(
            unsafe { p.GPIO3.clone_unchecked() },
            InputConfig::default().with_pull(Pull::Up),
        );

        Self {
            adc,
            row1,
            row2,
            power,
            idle_logged: false,
            last_activity_log_ms: 0,
        }
    }

    fn sample(&mut self, at_ms: u32) -> X4AdcSnapshot {
        // This intentionally mirrors the working x4-reader-os-rs input path.
        // esp-hal AdcCalCurve readings are already in the calibrated ladder
        // space used by the proven X4 thresholds:
        // row1 ~= 3/1113/1984/2556 and row2 ~= 3/1659.
        // Do not normalize these values to a synthetic 0..4095 range.
        let row1_mv = self.read_row1_mv();
        let row2_mv = self.read_row2_mv();
        let power_low = self.power.is_low();

        if !self.idle_logged && row1_mv > 2500 && row2_mv > 2500 {
            self.idle_logged = true;
            esp_println::println!(
                "phase8.5: calibrated ADC idle row1={} row2={} model=direct-x4-reader-os-rs",
                row1_mv,
                row2_mv,
            );
        }

        X4AdcSnapshot {
            row1_mv,
            row2_mv,
            sample: InputSample::new(row1_mv, row2_mv, power_low, at_ms),
        }
    }

    fn last_activity_log_ms(&self) -> u32 {
        self.last_activity_log_ms
    }

    fn set_last_activity_log_ms(&mut self, at_ms: u32) {
        self.last_activity_log_ms = at_ms;
    }

    fn read_row1_mv(&mut self) -> u16 {
        let mut sum = 0u32;
        for _ in 0..4 {
            sum += nb::block!(self.adc.read_oneshot(&mut self.row1)).unwrap_or(4095) as u32;
        }
        (sum / 4) as u16
    }

    fn read_row2_mv(&mut self) -> u16 {
        let mut sum = 0u32;
        for _ in 0..4 {
            sum += nb::block!(self.adc.read_oneshot(&mut self.row2)).unwrap_or(4095) as u32;
        }
        (sum / 4) as u16
    }
}

#[cfg(target_arch = "riscv32")]
fn handle_nav_event(
    event: InputEvent,
    selected: &mut u8,
    input: &mut X4Input,
    event_count: &mut u32,
) -> bool {
    *event_count = event_count.wrapping_add(1);
    esp_println::println!(
        "phase8: input event #{} button={} kind={:?}",
        *event_count,
        event.button.name(),
        event.kind,
    );

    match event.kind {
        ButtonEventKind::Press | ButtonEventKind::Repeat => match event.button {
            ButtonId::Up | ButtonId::Left => {
                *selected = (*selected).saturating_sub(1);
                true
            }
            ButtonId::Down | ButtonId::Right => {
                *selected = (*selected + 1).min(3);
                true
            }
            ButtonId::Select => {
                esp_println::println!(
                    "phase8: select item={} idx={}",
                    menu_item_name(*selected),
                    *selected,
                );
                false
            }
            ButtonId::Back => {
                esp_println::println!("phase8: back pressed on Home smoke");
                false
            }
            ButtonId::Power => {
                esp_println::println!("phase8: power pressed on Home smoke");
                false
            }
        },
        ButtonEventKind::LongPress => {
            esp_println::println!(
                "phase8: long press consumed button={} item={}",
                event.button.name(),
                menu_item_name(*selected),
            );
            input.reset_hold_state();
            false
        }
        ButtonEventKind::Release => false,
    }
}

#[cfg(target_arch = "riscv32")]
fn menu_item_name(selected: u8) -> &'static str {
    match selected.min(3) {
        0 => "Continue",
        1 => "Library",
        2 => "Settings",
        _ => "System",
    }
}

#[cfg(target_arch = "riscv32")]
const SMOKE_FILE: &str = "VOSMOKE.TXT";
#[cfg(target_arch = "riscv32")]
const SMOKE_DIR: &str = "state";
#[cfg(target_arch = "riscv32")]
const SMOKE_BYTES: &[u8] = b"VaachakOS Phase 8 X4 minimal home smoke\r\nstate/VOSMOKE.TXT\r\n";

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

    esp_println::println!("phase8: sd init start");
    let mut card_bytes = None;
    for attempt in 1..=5 {
        match sd.num_bytes() {
            Ok(size) => {
                card_bytes = Some(size);
                esp_println::println!(
                    "phase8: sd initialised attempt={} bytes={} mb={}",
                    attempt,
                    size,
                    size / 1024 / 1024
                );
                break;
            }
            Err(_) => {
                esp_println::println!("phase8: sd init attempt={} failed", attempt);
                sd.mark_card_uninit();
                embedded_hal::delay::DelayNs::delay_ms(&mut esp_hal::delay::Delay::new(), 50);
            }
        }
    }

    let card_bytes = card_bytes.ok_or(SmokeError::new("sd_init", "card did not respond"))?;
    esp_println::println!("phase8: sd card bytes={}", card_bytes);

    let adapter = BlockDeviceAdapter(sd);
    let mut mgr: AsyncVolumeManager<_, NullTimeSource, 4, 4, 1> =
        AsyncVolumeManager::new(adapter, NullTimeSource);

    let vol: RawVolume = poll_once(mgr.open_raw_volume(VolumeIdx(0)))
        .map_err(|_| SmokeError::new("open_volume", "volume 0 open failed"))?;
    let root: RawDirectory = mgr
        .open_root_dir(vol)
        .map_err(|_| SmokeError::new("open_root", "root directory open failed"))?;
    esp_println::println!("phase8: sd mounted volume=0 root=open");

    let state_dir = match poll_once(mgr.open_dir(root, SMOKE_DIR)) {
        Ok(dir) => dir,
        Err(_) => {
            esp_println::println!("phase8: creating {} directory", SMOKE_DIR);
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
        "phase8: wrote state/{} bytes={}",
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
        "phase8: readback ok state/{} size={} read={}",
        SMOKE_FILE,
        size,
        read_len
    );

    let _ = mgr.close_dir(state_dir);
    let _ = mgr.close_dir(root);
    let _ = poll_once(mgr.close_volume(vol));
    esp_println::println!("phase8: volume closed cleanly");

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
