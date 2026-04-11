#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

use embedded_hal::delay::DelayNs;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    main,
    time::{Duration, Instant},
};
use esp_println::println;
use vaachak_board_x4::{NATIVE_HEIGHT, STRIP_BYTES, STRIP_HEIGHT, X4Board, X4StripTarget};

struct SpinDelay;

fn idle_forever() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn log_and_idle<E: core::fmt::Debug>(step: &str, err: E) -> ! {
    println!("bring-up failed at {step}: {err:?}");
    println!("serial alive; idle loop");
    idle_forever()
}

impl DelayNs for SpinDelay {
    fn delay_ns(&mut self, ns: u32) {
        let micros = (ns as u64).div_ceil(1_000);
        let start = Instant::now();
        while start.elapsed() < Duration::from_micros(micros) {
            core::hint::spin_loop();
        }
    }
}

#[main]
fn main() -> ! {
    println!("board init start");
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut delay = SpinDelay;

    let board = X4Board::new(peripherals);
    let mut display = board.into_display();
    let mut strip = [0xFF; STRIP_BYTES];

    println!("VaachakOS X4 bring-up starting");

    let mut trace = |message: &'static str| println!("{message}");

    if let Err(err) = display.init_with_trace(&mut delay, &mut trace) {
        log_and_idle("display init", err);
    }

    if let Err(err) = display.begin_full_frame_with_trace(&mut trace) {
        log_and_idle("frame setup", err);
    }

    println!("frame write start");
    for native_y in (0..NATIVE_HEIGHT).step_by(STRIP_HEIGHT as usize) {
        let mut target = X4StripTarget::new(native_y, &mut strip);
        vaachak_core::draw_bringup_screen(&mut target);
        if let Err(err) = display.write_native_strip(target.bytes()) {
            log_and_idle("frame write", err);
        }
    }
    println!("frame write end");

    if let Err(err) = display.write_full_red_plane_with_trace(&mut trace) {
        log_and_idle("red plane clear", err);
    }

    println!("refresh start");
    if let Err(err) = display.refresh_full_with_trace(&mut delay, &mut trace) {
        log_and_idle("full refresh", err);
    }
    println!("refresh complete");

    println!("Bring-up screen rendered");
    println!("panel sleep skipped for bring-up stability");

    idle_forever()
}
