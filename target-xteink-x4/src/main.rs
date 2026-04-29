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
use hal_xteink_x4::{
    LibraryListItem, ReaderPage, X4_LIBRARY_MAX_ITEMS, X4_READER_TEXT_BYTES, X4Input,
    X4Ssd1677Smoke,
};
#[cfg(target_arch = "riscv32")]
use vaachak_core::hal::{ButtonEventKind, ButtonId, InputEvent, InputHal, InputSample};

const PHASE: &str = "bootstrap-phase11-x4-txt-pagination-progress-smoke";

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
    esp_println::println!("VaachakOS X4 TXT reader pagination/progress smoke starting");
    esp_println::println!("phase={}", PHASE);
    esp_println::println!("target=esp32c3 riscv32imc-unknown-none-elf");
    esp_println::println!("phase11=txt-pagination-progress-smoke");
    esp_println::println!("phase11=sd-library-input-reader-pagination-progress-smoke");
    esp_println::println!(
        "note=Phase 11 opens a selected TXT file, paginates, and persists progress"
    );

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut input_adc = unsafe { X4InputAdc::new(&peripherals) };
    esp_println::println!("phase11: input ADC ready row1=GPIO1 row2=GPIO2 power=GPIO3");
    esp_println::println!(
        "phase11: using proven x4-reader-os-rs calibrated thresholds + 10ms poll shape"
    );

    esp_alloc::heap_allocator!(size: 32_768);
    esp_println::println!("heap=32K txt-pagination-progress-smoke only");

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
    esp_println::println!("phase11: EPD_CS GPIO21 high, SD_CS GPIO12 high");

    esp_println::println!("phase11: configuring SPI2 shared bus at 400kHz");
    esp_println::println!(
        "phase11: pins sclk=GPIO8 mosi=GPIO10 miso=GPIO7 sd_cs=GPIO12 epd_cs=GPIO21 dc=GPIO4 rst=GPIO5 busy=GPIO6"
    );

    let slow_cfg = spi::master::Config::default().with_frequency(Rate::from_khz(400));
    let mut spi_raw = spi::master::Spi::new(peripherals.SPI2, slow_cfg)
        .unwrap()
        .with_sck(peripherals.GPIO8)
        .with_mosi(peripherals.GPIO10)
        .with_miso(peripherals.GPIO7);

    let _ = spi_raw.write(&[0xFF; 10]);
    esp_println::println!("phase11: sent 80 idle clocks with both CS lines high");

    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(4096);
    let dma_rx_buf = esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let spi_dma_bus = spi_raw
        .with_dma(peripherals.DMA_CH0)
        .with_buffers(dma_rx_buf, dma_tx_buf);
    esp_println::println!("phase11: DMA buffers enabled 4096B tx/rx");

    let spi_bus = RefCell::new(spi_dma_bus);

    let mut library_entries = [LibraryListItem::EMPTY; X4_LIBRARY_MAX_ITEMS];
    let library_report = {
        let sd_spi = RefCellDevice::new(&spi_bus, sd_cs, Delay::new()).unwrap();
        match run_library_storage_smoke(sd_spi, &mut library_entries) {
            Ok(report) => {
                esp_println::println!(
                    "phase11: library scan ok dir={} count={} total={}",
                    if report.from_books_dir {
                        "BOOKS"
                    } else {
                        "ROOT"
                    },
                    report.count,
                    report.total
                );
                report
            }
            Err(err) => {
                esp_println::println!("phase11: library scan failed step={}", err.step);
                esp_println::println!("phase11:storage-error={}", err.detail);
                LibraryScanReport::empty(false)
            }
        }
    };
    let storage_ok = library_report.storage_ok;
    let library_count = library_report.count;
    let library_total = library_report.total;
    let library_from_books = library_report.from_books_dir;
    let library_items = &library_entries[..library_count];

    // Keep SD deselected before switching the shared bus to the SSD1677.
    unsafe {
        force_gpio_output_high(12);
    }
    esp_println::println!("phase11: SD_CS GPIO12 forced high before display refresh");

    let dc = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let busy = Input::new(
        peripherals.GPIO6,
        InputConfig::default().with_pull(Pull::None),
    );
    let mut delay = Delay::new();

    let epd_spi = RefCellDevice::new(&spi_bus, epd_cs, Delay::new()).unwrap();
    let mut display = X4Ssd1677Smoke::new(epd_spi, dc, rst, busy);

    esp_println::println!("phase11: display init start dma-spidevice");
    display.init(&mut delay);
    esp_println::println!("phase11: display init complete busy={}", display.is_busy());

    let mut selected: u8 = 0;
    let mut mode = UiMode::Library;
    let mut reader_page = ReaderPage::EMPTY;
    let mut reader_session: Option<ReaderSession> = None;

    esp_println::println!(
        "phase11: initial library render start sd_ok={} battery_pct={} selected={} count={} total={}",
        storage_ok,
        battery_pct,
        selected,
        library_count,
        library_total,
    );
    display.draw_phase9_library(
        &mut delay,
        storage_ok,
        battery_pct,
        selected,
        library_items,
        library_total,
        library_from_books,
    );
    esp_println::println!(
        "phase11: initial library refresh complete busy={}",
        display.is_busy()
    );

    esp_println::println!("VaachakOS X4 TXT reader pagination/progress smoke ready");
    esp_println::println!("phase11=x4-txt-pagination-progress-smoke-ready");
    esp_println::println!("========================================");

    let mut input_model = X4Input::default();
    let mut tick_ms: u32 = 0;
    let mut event_count: u32 = 0;

    loop {
        delay.delay_ms(10);
        tick_ms = tick_ms.wrapping_add(10);

        let snapshot = input_adc.sample(tick_ms);
        if snapshot.has_activity() && tick_ms.wrapping_sub(input_adc.last_activity_log_ms()) >= 250
        {
            input_adc.set_last_activity_log_ms(tick_ms);
            esp_println::println!(
                "phase11: adc activity row1={} row2={} power_low={}",
                snapshot.row1_mv,
                snapshot.row2_mv,
                snapshot.sample.power_low,
            );
        }

        macro_rules! process_event {
            ($event:expr) => {{
                match handle_phase11_event(
                    $event,
                    &mut mode,
                    &mut selected,
                    library_items,
                    &mut input_model,
                    &mut event_count,
                ) {
                    Phase11Action::None => {}
                    Phase11Action::RedrawLibrary => {
                        esp_println::println!(
                            "phase11: redraw library selected={} file={}",
                            selected,
                            selected_library_name(library_items, selected),
                        );
                        display.draw_phase9_library(
                            &mut delay,
                            storage_ok,
                            battery_pct,
                            selected,
                            library_items,
                            library_total,
                            library_from_books,
                        );
                        esp_println::println!("phase11: library redraw complete busy={}", display.is_busy());
                    }
                    Phase11Action::OpenSelected => {
                        if let Some(item) = library_items.get(selected as usize).copied() {
                            if !is_txt_reader_supported(item) {
                                esp_println::println!(
                                    "phase11: reader unsupported file={} only TXT/MD in smoke",
                                    item.name_str(),
                                );
                            } else {
                                esp_println::println!(
                                    "phase11: reader open start file={} idx={} size={}",
                                    item.name_str(),
                                    selected,
                                    item.size,
                                );
                                unsafe {
                                    force_gpio_output_high(21);
                                    force_gpio_output_high(12);
                                }
                                let sd_spi = RefCellDevice::new(
                                    &spi_bus,
                                    unsafe { RawOutputPin::new_output_high(12) },
                                    Delay::new(),
                                )
                                .unwrap();
                                match run_txt_reader_page_storage_smoke(
                                    sd_spi,
                                    item,
                                    library_from_books,
                                    None,
                                    &mut reader_page,
                                ) {
                                    Ok(report) => {
                                        mode = UiMode::Reader;
                                        reader_session = Some(ReaderSession { item, from_books_dir: library_from_books });
                                        unsafe { force_gpio_output_high(12); }
                                        esp_println::println!(
                                            "phase11: reader read ok file={} size={} read={} offset={} page={}/{} restored={} progress={} dir={}",
                                            reader_page.name_str(),
                                            report.file_size,
                                            report.read_len,
                                            report.offset,
                                            report.page_index,
                                            report.total_pages,
                                            report.restored,
                                            report.progress_name_str(),
                                            if report.from_books_dir { "BOOKS" } else { "ROOT" },
                                        );
                                        display.draw_phase11_reader(
                                            &mut delay,
                                            storage_ok,
                                            battery_pct,
                                            &reader_page,
                                        );
                                        esp_println::println!(
                                            "phase11: reader refresh complete busy={}",
                                            display.is_busy(),
                                        );
                                        esp_println::println!("phase11=x4-txt-pagination-progress-smoke-ok");
                                    }
                                    Err(err) => {
                                        unsafe { force_gpio_output_high(12); }
                                        esp_println::println!(
                                            "phase11: reader read failed file={} step={} detail={}",
                                            item.name_str(),
                                            err.step,
                                            err.detail,
                                        );
                                    }
                                }
                            }
                        } else {
                            esp_println::println!("phase11: reader open ignored no selected file");
                        }
                    }
                    Phase11Action::ReaderNextPage => {
                        if let Some(session) = reader_session {
                            if let Some(next_offset) = reader_page.next_offset() {
                                esp_println::println!(
                                    "phase11: reader next page file={} from_off={} to_off={}",
                                    session.item.name_str(),
                                    reader_page.offset,
                                    next_offset,
                                );
                                unsafe {
                                    force_gpio_output_high(21);
                                    force_gpio_output_high(12);
                                }
                                let sd_spi = RefCellDevice::new(
                                    &spi_bus,
                                    unsafe { RawOutputPin::new_output_high(12) },
                                    Delay::new(),
                                )
                                .unwrap();
                                match run_txt_reader_page_storage_smoke(
                                    sd_spi,
                                    session.item,
                                    session.from_books_dir,
                                    Some(next_offset),
                                    &mut reader_page,
                                ) {
                                    Ok(report) => {
                                        unsafe { force_gpio_output_high(12); }
                                        esp_println::println!(
                                            "phase11: reader page ok file={} read={} offset={} page={}/{} progress={}",
                                            reader_page.name_str(),
                                            report.read_len,
                                            report.offset,
                                            report.page_index,
                                            report.total_pages,
                                            report.progress_name_str(),
                                        );
                                        display.draw_phase11_reader(&mut delay, storage_ok, battery_pct, &reader_page);
                                        esp_println::println!("phase11: reader page refresh complete busy={}", display.is_busy());
                                        esp_println::println!("phase11=x4-txt-pagination-progress-smoke-ok");
                                    }
                                    Err(err) => {
                                        unsafe { force_gpio_output_high(12); }
                                        esp_println::println!(
                                            "phase11: reader next failed file={} step={} detail={}",
                                            session.item.name_str(),
                                            err.step,
                                            err.detail,
                                        );
                                    }
                                }
                            } else {
                                esp_println::println!(
                                    "phase11: reader next ignored at end file={} page={}/{}",
                                    reader_page.name_str(),
                                    reader_page.page_index,
                                    reader_page.total_pages,
                                );
                            }
                        }
                    }
                    Phase11Action::ReaderPrevPage => {
                        if let Some(session) = reader_session {
                            if let Some(prev_offset) = reader_page.prev_offset() {
                                esp_println::println!(
                                    "phase11: reader prev page file={} from_off={} to_off={}",
                                    session.item.name_str(),
                                    reader_page.offset,
                                    prev_offset,
                                );
                                unsafe {
                                    force_gpio_output_high(21);
                                    force_gpio_output_high(12);
                                }
                                let sd_spi = RefCellDevice::new(
                                    &spi_bus,
                                    unsafe { RawOutputPin::new_output_high(12) },
                                    Delay::new(),
                                )
                                .unwrap();
                                match run_txt_reader_page_storage_smoke(
                                    sd_spi,
                                    session.item,
                                    session.from_books_dir,
                                    Some(prev_offset),
                                    &mut reader_page,
                                ) {
                                    Ok(report) => {
                                        unsafe { force_gpio_output_high(12); }
                                        esp_println::println!(
                                            "phase11: reader page ok file={} read={} offset={} page={}/{} progress={}",
                                            reader_page.name_str(),
                                            report.read_len,
                                            report.offset,
                                            report.page_index,
                                            report.total_pages,
                                            report.progress_name_str(),
                                        );
                                        display.draw_phase11_reader(&mut delay, storage_ok, battery_pct, &reader_page);
                                        esp_println::println!("phase11: reader page refresh complete busy={}", display.is_busy());
                                        esp_println::println!("phase11=x4-txt-pagination-progress-smoke-ok");
                                    }
                                    Err(err) => {
                                        unsafe { force_gpio_output_high(12); }
                                        esp_println::println!(
                                            "phase11: reader prev failed file={} step={} detail={}",
                                            session.item.name_str(),
                                            err.step,
                                            err.detail,
                                        );
                                    }
                                }
                            } else {
                                esp_println::println!(
                                    "phase11: reader prev ignored at start file={} page={}/{}",
                                    reader_page.name_str(),
                                    reader_page.page_index,
                                    reader_page.total_pages,
                                );
                            }
                        }
                    }
                    Phase11Action::ReturnLibrary => {
                        mode = UiMode::Library;
                        esp_println::println!(
                            "phase11: back to library selected={} file={}",
                            selected,
                            selected_library_name(library_items, selected),
                        );
                        display.draw_phase9_library(
                            &mut delay,
                            storage_ok,
                            battery_pct,
                            selected,
                            library_items,
                            library_total,
                            library_from_books,
                        );
                        esp_println::println!("phase11: library restore complete busy={}", display.is_busy());
                    }
                }
            }};
        }

        if let Some(event) = input_model.ingest_sample(snapshot.sample) {
            process_event!(event);
        }

        while let Some(event) = input_model.poll() {
            process_event!(event);
        }
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
                "phase11: calibrated ADC idle row1={} row2={} model=direct-x4-reader-os-rs",
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UiMode {
    Library,
    Reader,
}

#[cfg(target_arch = "riscv32")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase11Action {
    None,
    RedrawLibrary,
    OpenSelected,
    ReaderNextPage,
    ReaderPrevPage,
    ReturnLibrary,
}

#[cfg(target_arch = "riscv32")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ReaderSession {
    item: LibraryListItem,
    from_books_dir: bool,
}

#[cfg(target_arch = "riscv32")]
fn handle_phase11_event(
    event: InputEvent,
    mode: &mut UiMode,
    selected: &mut u8,
    items: &[LibraryListItem],
    input: &mut X4Input,
    event_count: &mut u32,
) -> Phase11Action {
    *event_count = event_count.wrapping_add(1);
    esp_println::println!(
        "phase11: input event #{} button={} kind={:?} mode={:?}",
        *event_count,
        event.button.name(),
        event.kind,
        *mode,
    );

    match *mode {
        UiMode::Library => handle_library_mode_event(event, selected, items, input),
        UiMode::Reader => handle_reader_mode_event(event, input),
    }
}

#[cfg(target_arch = "riscv32")]
fn handle_library_mode_event(
    event: InputEvent,
    selected: &mut u8,
    items: &[LibraryListItem],
    input: &mut X4Input,
) -> Phase11Action {
    let max_selected = items.len().saturating_sub(1).min(4) as u8;

    match event.kind {
        ButtonEventKind::Press | ButtonEventKind::Repeat => match event.button {
            ButtonId::Up | ButtonId::Left => {
                *selected = (*selected).saturating_sub(1);
                if items.is_empty() {
                    Phase11Action::None
                } else {
                    Phase11Action::RedrawLibrary
                }
            }
            ButtonId::Down | ButtonId::Right => {
                *selected = (*selected + 1).min(max_selected);
                if items.is_empty() {
                    Phase11Action::None
                } else {
                    Phase11Action::RedrawLibrary
                }
            }
            ButtonId::Select => {
                esp_println::println!(
                    "phase11: select file={} idx={}",
                    selected_library_name(items, *selected),
                    *selected,
                );
                Phase11Action::OpenSelected
            }
            ButtonId::Back => {
                esp_println::println!("phase11: back pressed on Library smoke");
                Phase11Action::None
            }
            ButtonId::Power => {
                esp_println::println!("phase11: power pressed on Library smoke");
                Phase11Action::None
            }
        },
        ButtonEventKind::LongPress => {
            esp_println::println!(
                "phase11: long press consumed button={} file={}",
                event.button.name(),
                selected_library_name(items, *selected),
            );
            input.reset_hold_state();
            Phase11Action::None
        }
        ButtonEventKind::Release => Phase11Action::None,
    }
}

#[cfg(target_arch = "riscv32")]
fn handle_reader_mode_event(event: InputEvent, input: &mut X4Input) -> Phase11Action {
    match event.kind {
        ButtonEventKind::Press | ButtonEventKind::Repeat => match event.button {
            ButtonId::Back | ButtonId::Left => Phase11Action::ReturnLibrary,
            ButtonId::Down | ButtonId::Right | ButtonId::Select => Phase11Action::ReaderNextPage,
            ButtonId::Up => Phase11Action::ReaderPrevPage,
            ButtonId::Power => {
                esp_println::println!(
                    "phase11: power pressed on TXT reader pagination/progress smoke"
                );
                Phase11Action::None
            }
        },
        ButtonEventKind::LongPress => {
            esp_println::println!(
                "phase11: reader long press consumed button={}",
                event.button.name()
            );
            input.reset_hold_state();
            Phase11Action::None
        }
        ButtonEventKind::Release => Phase11Action::None,
    }
}

#[cfg(target_arch = "riscv32")]
fn selected_library_name(items: &[LibraryListItem], selected: u8) -> &str {
    items
        .get(selected as usize)
        .map(LibraryListItem::name_str)
        .unwrap_or("<none>")
}

#[cfg(target_arch = "riscv32")]
fn is_txt_reader_supported(item: LibraryListItem) -> bool {
    ext_eq(item.name_bytes(), b"TXT") || ext_eq(item.name_bytes(), b"MD")
}

#[cfg(target_arch = "riscv32")]
const SMOKE_FILE: &str = "VOSMOKE.TXT";
#[cfg(target_arch = "riscv32")]
const SMOKE_DIR: &str = "state";
#[cfg(target_arch = "riscv32")]
const SMOKE_BYTES: &[u8] =
    b"VaachakOS Phase 11 X4 TXT reader pagination/progress smoke\r\nstate/VOSMOKE.TXT\r\n";

#[cfg(target_arch = "riscv32")]
#[derive(Debug, Clone, Copy)]
struct LibraryScanReport {
    storage_ok: bool,
    count: usize,
    total: usize,
    from_books_dir: bool,
}

#[cfg(target_arch = "riscv32")]
impl LibraryScanReport {
    const fn empty(storage_ok: bool) -> Self {
        Self {
            storage_ok,
            count: 0,
            total: 0,
            from_books_dir: false,
        }
    }
}

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
fn run_library_storage_smoke<SPI>(
    sd_spi: SPI,
    entries: &mut [LibraryListItem],
) -> Result<LibraryScanReport, SmokeError>
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

    esp_println::println!("phase11: sd init start");
    let mut card_bytes = None;
    for attempt in 1..=5 {
        match sd.num_bytes() {
            Ok(size) => {
                card_bytes = Some(size);
                esp_println::println!(
                    "phase11: sd initialised attempt={} bytes={} mb={}",
                    attempt,
                    size,
                    size / 1024 / 1024
                );
                break;
            }
            Err(_) => {
                esp_println::println!("phase11: sd init attempt={} failed", attempt);
                sd.mark_card_uninit();
                embedded_hal::delay::DelayNs::delay_ms(&mut esp_hal::delay::Delay::new(), 50);
            }
        }
    }

    let card_bytes = card_bytes.ok_or(SmokeError::new("sd_init", "card did not respond"))?;
    esp_println::println!("phase11: sd card bytes={}", card_bytes);

    let adapter = BlockDeviceAdapter(sd);
    let mut mgr: AsyncVolumeManager<_, NullTimeSource, 4, 4, 1> =
        AsyncVolumeManager::new(adapter, NullTimeSource);

    let vol: RawVolume = poll_once(mgr.open_raw_volume(VolumeIdx(0)))
        .map_err(|_| SmokeError::new("open_volume", "volume 0 open failed"))?;
    let root: RawDirectory = mgr
        .open_root_dir(vol)
        .map_err(|_| SmokeError::new("open_root", "root directory open failed"))?;
    esp_println::println!("phase11: sd mounted volume=0 root=open");

    let state_dir = match poll_once(mgr.open_dir(root, SMOKE_DIR)) {
        Ok(dir) => dir,
        Err(_) => {
            esp_println::println!("phase11: creating {} directory", SMOKE_DIR);
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
        "phase11: wrote state/{} bytes={}",
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
        "phase11: readback ok state/{} size={} read={}",
        SMOKE_FILE,
        size,
        read_len
    );

    let (mut count, mut total, mut from_books_dir) = (0usize, 0usize, false);

    if let Ok(books_dir) = poll_once(mgr.open_dir(root, "BOOKS")) {
        esp_println::println!("phase11: scanning BOOKS directory");
        let scan = scan_supported_files(&mut mgr, books_dir, entries, "BOOKS")?;
        let _ = mgr.close_dir(books_dir);
        if scan.total > 0 {
            count = scan.count;
            total = scan.total;
            from_books_dir = true;
        } else {
            esp_println::println!("phase11: BOOKS has no supported files, falling back to root");
        }
    } else {
        esp_println::println!("phase11: BOOKS directory not found, scanning root");
    }

    if total == 0 {
        let scan = scan_supported_files(&mut mgr, root, entries, "ROOT")?;
        count = scan.count;
        total = scan.total;
        from_books_dir = false;
    }

    for (idx, item) in entries.iter().take(count).enumerate() {
        esp_println::println!(
            "phase11: file[{}]={} size={}",
            idx,
            item.name_str(),
            item.size,
        );
    }

    let _ = mgr.close_dir(state_dir);
    let _ = mgr.close_dir(root);
    let _ = poll_once(mgr.close_volume(vol));
    esp_println::println!("phase11: volume closed cleanly");

    Ok(LibraryScanReport {
        storage_ok: true,
        count,
        total,
        from_books_dir,
    })
}

#[cfg(target_arch = "riscv32")]
#[derive(Debug, Clone, Copy)]
struct ReaderReadReport {
    file_size: u32,
    read_len: usize,
    offset: u32,
    page_index: u16,
    total_pages: u16,
    restored: bool,
    from_books_dir: bool,
    progress_name: [u8; 12],
}

#[cfg(target_arch = "riscv32")]
impl ReaderReadReport {
    fn progress_name_str(&self) -> &str {
        core::str::from_utf8(&self.progress_name).unwrap_or("????????.PRG")
    }
}

#[cfg(target_arch = "riscv32")]
fn run_txt_reader_page_storage_smoke<SPI>(
    sd_spi: SPI,
    item: LibraryListItem,
    from_books_dir: bool,
    requested_offset: Option<u32>,
    page: &mut ReaderPage,
) -> Result<ReaderReadReport, SmokeError>
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

    let progress_name = progress_file_name_for(item);
    let progress_name_str = core::str::from_utf8(&progress_name).unwrap_or("????????.PRG");

    let sd = SdCard::new(sd_spi, esp_hal::delay::Delay::new());
    let adapter = BlockDeviceAdapter(sd);
    let mut mgr: AsyncVolumeManager<_, NullTimeSource, 4, 4, 1> =
        AsyncVolumeManager::new(adapter, NullTimeSource);

    let vol: RawVolume = poll_once(mgr.open_raw_volume(VolumeIdx(0)))
        .map_err(|_| SmokeError::new("reader_open_volume", "volume 0 open failed"))?;
    let root: RawDirectory = mgr
        .open_root_dir(vol)
        .map_err(|_| SmokeError::new("reader_open_root", "root directory open failed"))?;

    let state_dir = ensure_state_dir(&mut mgr, root)?;

    let restored_offset = if let Some(offset) = requested_offset {
        offset
    } else {
        read_progress_offset(&mut mgr, state_dir, progress_name_str).unwrap_or(0)
    };
    let restored = requested_offset.is_none() && restored_offset > 0;
    if restored {
        esp_println::println!(
            "phase11: progress loaded state/{} offset={} file={}",
            progress_name_str,
            restored_offset,
            item.name_str(),
        );
    } else if requested_offset.is_none() {
        esp_println::println!(
            "phase11: progress default state/{} offset=0 file={}",
            progress_name_str,
            item.name_str(),
        );
    }

    let read_dir = if from_books_dir {
        poll_once(mgr.open_dir(root, "BOOKS"))
            .map_err(|_| SmokeError::new("reader_open_books", "BOOKS directory open failed"))?
    } else {
        root
    };

    let file = poll_once(mgr.open_file_in_dir(read_dir, item.name_str(), Mode::ReadOnly))
        .map_err(|_| SmokeError::new("reader_open_file", "selected file open failed"))?;
    let file_size = mgr.file_length(file).unwrap_or(item.size);
    let total_pages = txt_total_pages(file_size);
    let max_offset = if file_size == 0 {
        0
    } else {
        ((file_size - 1) / X4_READER_TEXT_BYTES as u32) * X4_READER_TEXT_BYTES as u32
    };
    let offset = restored_offset.min(max_offset);

    let mut skip = offset;
    let mut skip_buf = [0u8; 128];
    while skip > 0 {
        let want = skip.min(skip_buf.len() as u32) as usize;
        let got = poll_once(mgr.read(file, &mut skip_buf[..want]))
            .map_err(|_| SmokeError::new("reader_skip", "skip read failed"))?;
        if got == 0 {
            break;
        }
        skip = skip.saturating_sub(got as u32);
    }

    let mut read_buf = [0u8; X4_READER_TEXT_BYTES];
    let read_len = poll_once(mgr.read(file, &mut read_buf))
        .map_err(|_| SmokeError::new("reader_read_file", "selected file read failed"))?;
    poll_once(mgr.close_file(file))
        .map_err(|_| SmokeError::new("reader_close_file", "reader close file failed"))?;

    let page_index = ((offset / X4_READER_TEXT_BYTES as u32) + 1).min(u16::MAX as u32) as u16;
    *page = ReaderPage::new_paged(
        item.name_bytes(),
        file_size,
        offset,
        page_index,
        total_pages,
        &read_buf[..read_len],
    );

    write_progress_record(
        &mut mgr,
        state_dir,
        progress_name_str,
        item,
        offset,
        page_index,
        total_pages,
        file_size,
    )?;

    if from_books_dir {
        let _ = mgr.close_dir(read_dir);
    }
    let _ = mgr.close_dir(state_dir);
    let _ = mgr.close_dir(root);
    let _ = poll_once(mgr.close_volume(vol));

    Ok(ReaderReadReport {
        file_size,
        read_len,
        offset,
        page_index,
        total_pages,
        restored,
        from_books_dir,
        progress_name,
    })
}

#[cfg(target_arch = "riscv32")]
fn txt_total_pages(file_size: u32) -> u16 {
    let pages = if file_size == 0 {
        1
    } else {
        (file_size + X4_READER_TEXT_BYTES as u32 - 1) / X4_READER_TEXT_BYTES as u32
    };
    pages.min(u16::MAX as u32).max(1) as u16
}

#[cfg(target_arch = "riscv32")]
fn progress_file_name_for(item: LibraryListItem) -> [u8; 12] {
    let mut hash = 0x811c9dc5u32;
    for &b in item.name_bytes() {
        hash ^= b.to_ascii_uppercase() as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    for b in item.size.to_le_bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x01000193);
    }

    let mut out = [0u8; 12];
    write_hex8_upper(hash, &mut out[..8]);
    out[8] = b'.';
    out[9] = b'P';
    out[10] = b'R';
    out[11] = b'G';
    out
}

#[cfg(target_arch = "riscv32")]
fn write_hex8_upper(mut value: u32, out: &mut [u8]) {
    let mut i = 8usize;
    while i > 0 {
        i -= 1;
        let nibble = (value & 0x0f) as u8;
        out[i] = if nibble < 10 {
            b'0' + nibble
        } else {
            b'A' + (nibble - 10)
        };
        value >>= 4;
    }
}

#[cfg(target_arch = "riscv32")]
fn ensure_state_dir<D, T, const MAX_DIRS: usize, const MAX_FILES: usize, const MAX_VOLS: usize>(
    mgr: &mut embedded_sdmmc::AsyncVolumeManager<D, T, MAX_DIRS, MAX_FILES, MAX_VOLS>,
    root: embedded_sdmmc::RawDirectory,
) -> Result<embedded_sdmmc::RawDirectory, SmokeError>
where
    D: embedded_sdmmc::AsyncBlockDevice,
    T: embedded_sdmmc::TimeSource,
{
    match poll_once(mgr.open_dir(root, SMOKE_DIR)) {
        Ok(dir) => Ok(dir),
        Err(_) => {
            match poll_once(mgr.make_dir_in_dir(root, SMOKE_DIR)) {
                Ok(()) | Err(embedded_sdmmc::Error::DirAlreadyExists) => {}
                Err(_) => {
                    return Err(SmokeError::new(
                        "reader_mkdir_state",
                        "failed to create state dir",
                    ));
                }
            }
            poll_once(mgr.open_dir(root, SMOKE_DIR))
                .map_err(|_| SmokeError::new("reader_open_state", "state dir open failed"))
        }
    }
}

#[cfg(target_arch = "riscv32")]
fn read_progress_offset<
    D,
    T,
    const MAX_DIRS: usize,
    const MAX_FILES: usize,
    const MAX_VOLS: usize,
>(
    mgr: &mut embedded_sdmmc::AsyncVolumeManager<D, T, MAX_DIRS, MAX_FILES, MAX_VOLS>,
    state_dir: embedded_sdmmc::RawDirectory,
    progress_name: &str,
) -> Option<u32>
where
    D: embedded_sdmmc::AsyncBlockDevice,
    T: embedded_sdmmc::TimeSource,
{
    let file =
        poll_once(mgr.open_file_in_dir(state_dir, progress_name, embedded_sdmmc::Mode::ReadOnly))
            .ok()?;
    let mut buf = [0u8; 96];
    let len = poll_once(mgr.read(file, &mut buf)).ok()?;
    let _ = poll_once(mgr.close_file(file));
    parse_offset_record(&buf[..len])
}

#[cfg(target_arch = "riscv32")]
fn parse_offset_record(bytes: &[u8]) -> Option<u32> {
    let key = b"offset=";
    let pos = bytes.windows(key.len()).position(|w| w == key)? + key.len();
    let mut value = 0u32;
    let mut saw_digit = false;
    for &b in &bytes[pos..] {
        if !b.is_ascii_digit() {
            break;
        }
        saw_digit = true;
        value = value.saturating_mul(10).saturating_add((b - b'0') as u32);
    }
    saw_digit.then_some(value)
}

#[cfg(target_arch = "riscv32")]
#[allow(clippy::too_many_arguments)]
fn write_progress_record<
    D,
    T,
    const MAX_DIRS: usize,
    const MAX_FILES: usize,
    const MAX_VOLS: usize,
>(
    mgr: &mut embedded_sdmmc::AsyncVolumeManager<D, T, MAX_DIRS, MAX_FILES, MAX_VOLS>,
    state_dir: embedded_sdmmc::RawDirectory,
    progress_name: &str,
    item: LibraryListItem,
    offset: u32,
    page_index: u16,
    total_pages: u16,
    file_size: u32,
) -> Result<(), SmokeError>
where
    D: embedded_sdmmc::AsyncBlockDevice,
    T: embedded_sdmmc::TimeSource,
{
    let mut buf = [0u8; 128];
    let mut pos = 0usize;
    append_lit(&mut buf, &mut pos, b"version=phase11\n");
    append_lit(&mut buf, &mut pos, b"file=");
    append_bytes(&mut buf, &mut pos, item.name_bytes());
    append_lit(&mut buf, &mut pos, b"\nsize=");
    append_u32(&mut buf, &mut pos, file_size);
    append_lit(&mut buf, &mut pos, b"\noffset=");
    append_u32(&mut buf, &mut pos, offset);
    append_lit(&mut buf, &mut pos, b"\npage=");
    append_u32(&mut buf, &mut pos, page_index as u32);
    append_lit(&mut buf, &mut pos, b"\ntotal=");
    append_u32(&mut buf, &mut pos, total_pages as u32);
    append_lit(&mut buf, &mut pos, b"\n");

    let file = poll_once(mgr.open_file_in_dir(
        state_dir,
        progress_name,
        embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
    ))
    .map_err(|_| SmokeError::new("progress_open", "open progress file failed"))?;
    poll_once(mgr.write(file, &buf[..pos]))
        .map_err(|_| SmokeError::new("progress_write", "write progress file failed"))?;
    poll_once(mgr.close_file(file))
        .map_err(|_| SmokeError::new("progress_close", "close progress file failed"))?;
    esp_println::println!(
        "phase11: progress wrote state/{} offset={} page={}/{} file={}",
        progress_name,
        offset,
        page_index,
        total_pages,
        item.name_str(),
    );
    Ok(())
}

#[cfg(target_arch = "riscv32")]
fn append_lit(out: &mut [u8], pos: &mut usize, bytes: &[u8]) {
    append_bytes(out, pos, bytes);
}

#[cfg(target_arch = "riscv32")]
fn append_bytes(out: &mut [u8], pos: &mut usize, bytes: &[u8]) {
    let space = out.len().saturating_sub(*pos);
    let n = bytes.len().min(space);
    out[*pos..*pos + n].copy_from_slice(&bytes[..n]);
    *pos += n;
}

#[cfg(target_arch = "riscv32")]
fn append_u32(out: &mut [u8], pos: &mut usize, value: u32) {
    let mut tmp = [0u8; 10];
    let mut n = value;
    let mut len = 0usize;
    if n == 0 {
        tmp[0] = b'0';
        len = 1;
    } else {
        while n > 0 && len < tmp.len() {
            tmp[len] = b'0' + (n % 10) as u8;
            n /= 10;
            len += 1;
        }
    }
    while len > 0 {
        len -= 1;
        append_bytes(out, pos, &tmp[len..len + 1]);
    }
}

#[cfg(target_arch = "riscv32")]
#[derive(Debug, Clone, Copy)]
struct ScanCounts {
    count: usize,
    total: usize,
}

#[cfg(target_arch = "riscv32")]
fn scan_supported_files<
    D,
    T,
    const MAX_DIRS: usize,
    const MAX_FILES: usize,
    const MAX_VOLS: usize,
>(
    mgr: &mut embedded_sdmmc::AsyncVolumeManager<D, T, MAX_DIRS, MAX_FILES, MAX_VOLS>,
    dir: embedded_sdmmc::RawDirectory,
    entries: &mut [LibraryListItem],
    label: &'static str,
) -> Result<ScanCounts, SmokeError>
where
    D: embedded_sdmmc::AsyncBlockDevice,
    T: embedded_sdmmc::TimeSource,
{
    use core::ops::ControlFlow;

    for slot in entries.iter_mut() {
        *slot = LibraryListItem::EMPTY;
    }

    let mut count = 0usize;
    let mut total = 0usize;
    let max_visible = entries.len().min(5);

    poll_once(mgr.iterate_dir(dir, |entry| {
        if entry.attributes.is_volume() || entry.attributes.is_directory() {
            return ControlFlow::Continue(());
        }

        let mut name_buf = [0u8; 13];
        let name_len = sfn_to_bytes(&entry.name, &mut name_buf);
        let name = &name_buf[..name_len as usize];

        if name.is_empty() || name[0] == b'.' || name[0] == b'_' || !has_supported_ext(name) {
            return ControlFlow::Continue(());
        }

        total += 1;
        if count < max_visible {
            entries[count] = LibraryListItem::new(name, entry.size);
            count += 1;
        }

        ControlFlow::Continue(())
    }))
    .map_err(|_| SmokeError::new("scan_dir", label))?;

    esp_println::println!(
        "phase11: scan dir={} visible={} total={}",
        label,
        count,
        total,
    );

    Ok(ScanCounts { count, total })
}

#[cfg(target_arch = "riscv32")]
fn ext_eq(name: &[u8], target: &[u8]) -> bool {
    let dot = match name.iter().rposition(|&b| b == b'.') {
        Some(p) => p,
        None => return false,
    };
    let ext = &name[dot + 1..];
    ext.len() == target.len() && ext.eq_ignore_ascii_case(target)
}

#[cfg(target_arch = "riscv32")]
fn has_supported_ext(name: &[u8]) -> bool {
    ext_eq(name, b"TXT") || ext_eq(name, b"EPUB") || ext_eq(name, b"EPU") || ext_eq(name, b"MD")
}

#[cfg(target_arch = "riscv32")]
fn sfn_to_bytes(name: &embedded_sdmmc::ShortFileName, out: &mut [u8; 13]) -> u8 {
    let base = name.base_name();
    let ext = name.extension();
    let mut pos = 0usize;
    let blen = base.len().min(8);
    out[..blen].copy_from_slice(&base[..blen]);
    pos += blen;
    if !ext.is_empty() && pos < out.len() {
        out[pos] = b'.';
        pos += 1;
        let elen = ext.len().min(3).min(out.len().saturating_sub(pos));
        out[pos..pos + elen].copy_from_slice(&ext[..elen]);
        pos += elen;
    }
    pos as u8
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
