//! Real USB Serial/JTAG SD bulk-transfer mode.
//!
//! This special mode owns USB_DEVICE for the duration of the transfer.
//! Do not run espflash monitor while using it; host transfer owns the CDC serial stream.

use embassy_time::{Duration, Timer};
use esp_hal::delay::Delay;
use esp_hal::peripherals::USB_DEVICE;
use esp_hal::usb_serial_jtag::UsbSerialJtag;

use crate::apps::usb_transfer::receiver_skeleton::{USB_TRANSFER_MAGIC, USB_TRANSFER_MAX_PAYLOAD};
use crate::apps::usb_transfer::runtime::{
    SdTransferTarget, UsbTransferRuntime, UsbTransferRuntimeError,
};
use crate::apps::widgets::ButtonFeedback;
use crate::board::{Epd, SCREEN_H, SCREEN_W};
use crate::drivers::sdcard::SdStorage;
use crate::drivers::storage;
use crate::drivers::strip::StripBuffer;
use crate::fonts;
use crate::fonts::bitmap::BitmapFont;
use crate::ui::{Alignment, BitmapLabel, CONTENT_TOP, FULL_CONTENT_W, LARGE_MARGIN, Region};

const FRAME_BUF_LEN: usize = 10 + USB_TRANSFER_MAX_PAYLOAD + 4;
const USB_POLL_DELAY_MS: u64 = 2;
const HEADING_X: u16 = LARGE_MARGIN;
const HEADING_W: u16 = FULL_CONTENT_W;
const BODY_X: u16 = LARGE_MARGIN;
const BODY_W: u16 = FULL_CONTENT_W;
const BODY_LINE_GAP: u16 = 10;
const FOOTER_Y: u16 = SCREEN_H - 52;

pub async fn run_usb_transfer_mode(
    usb_device: USB_DEVICE<'static>,
    epd: &mut Epd,
    strip: &mut StripBuffer,
    delay: &mut Delay,
    sd: &SdStorage,
    ui_font_size_idx: u8,
    bumps: &ButtonFeedback,
) {
    let heading = fonts::heading_font(ui_font_size_idx);
    let body = fonts::chrome_font();

    render_screen(
        epd,
        strip,
        delay,
        heading,
        body,
        &[
            "USB Transfer ready",
            "Run send_folder.py with --port",
            "Do not use espflash monitor",
        ],
        Some("Waiting for host..."),
        bumps,
        true,
    )
    .await;

    let mut usb = UsbSerialJtag::new(usb_device);
    let mut runtime = UsbTransferRuntime::new();
    let mut target = UsbSdTarget::new(sd);
    let mut acc = FrameAccumulator::new();

    loop {
        match usb.read_byte() {
            Ok(byte) => {
                if let Some(frame_len) = acc.push(byte) {
                    let result = runtime.accept_raw_frame(acc.frame(frame_len), &mut target);
                    acc.clear();

                    match result {
                        Ok(ack) => {
                            let _ = usb.write(ack);
                            let _ = usb.flush_tx();

                            if runtime.progress().status
                                == crate::apps::usb_transfer::runtime::UsbTransferRuntimeStatus::Complete
                            {
                                render_screen(
                                    epd,
                                    strip,
                                    delay,
                                    heading,
                                    body,
                                    &[
                                        "USB transfer complete",
                                        "Files written to SD",
                                        "Open Reader after exit",
                                    ],
                                    Some("Reboot if host keeps port open"),
                                    bumps,
                                    false,
                                )
                                .await;
                                return;
                            }
                        }
                        Err(_) => {
                            let _ = usb.write(b"ERR\n");
                            let _ = usb.flush_tx();
                            render_screen(
                                epd,
                                strip,
                                delay,
                                heading,
                                body,
                                &[
                                    "USB transfer error",
                                    "Host may retry",
                                    "Check target path/cache",
                                ],
                                Some("Restart transfer from host"),
                                bumps,
                                false,
                            )
                            .await;
                        }
                    }
                }
            }
            Err(nb::Error::WouldBlock) => {
                Timer::after(Duration::from_millis(USB_POLL_DELAY_MS)).await;
            }
            Err(_) => {
                Timer::after(Duration::from_millis(USB_POLL_DELAY_MS)).await;
            }
        }
    }
}

struct FrameAccumulator {
    buf: [u8; FRAME_BUF_LEN],
    len: usize,
}

impl FrameAccumulator {
    const fn new() -> Self {
        Self {
            buf: [0u8; FRAME_BUF_LEN],
            len: 0,
        }
    }

    fn frame(&self, len: usize) -> &[u8] {
        &self.buf[..len]
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn push(&mut self, byte: u8) -> Option<usize> {
        if self.len >= self.buf.len() {
            self.clear();
        }

        self.buf[self.len] = byte;
        self.len += 1;

        self.resync_magic();

        if self.len < 10 {
            return None;
        }

        let payload_len =
            u32::from_le_bytes([self.buf[6], self.buf[7], self.buf[8], self.buf[9]]) as usize;

        if payload_len > USB_TRANSFER_MAX_PAYLOAD {
            self.clear();
            return None;
        }

        let total = 10 + payload_len + 4;
        if total > self.buf.len() {
            self.clear();
            return None;
        }

        if self.len >= total { Some(total) } else { None }
    }

    fn resync_magic(&mut self) {
        while self.len > 0 {
            let prefix_len = self.len.min(USB_TRANSFER_MAGIC.len());
            if self.buf[..prefix_len] == USB_TRANSFER_MAGIC[..prefix_len] {
                return;
            }

            self.buf.copy_within(1..self.len, 0);
            self.len -= 1;
        }
    }
}

struct UsbSdTarget<'a> {
    sd: &'a SdStorage,
    expected_offset: u32,
    active_path: [u8; 96],
    active_path_len: usize,
}

impl<'a> UsbSdTarget<'a> {
    const fn new(sd: &'a SdStorage) -> Self {
        Self {
            sd,
            expected_offset: 0,
            active_path: [0u8; 96],
            active_path_len: 0,
        }
    }

    fn set_active_path(&mut self, path: &[u8]) {
        let n = path.len().min(self.active_path.len());
        self.active_path[..n].copy_from_slice(&path[..n]);
        self.active_path_len = n;
    }

    fn active_path(&self) -> &[u8] {
        &self.active_path[..self.active_path_len]
    }
}

impl SdTransferTarget for UsbSdTarget<'_> {
    fn ensure_dir(&mut self, path: &[u8]) -> Result<(), UsbTransferRuntimeError> {
        let mut parts = PathParts::new();
        let count = parts.parse(path)?;

        match count {
            0 => Ok(()),
            1 => {
                let _ = storage::ensure_dir(self.sd, parts.get(0));
                Ok(())
            }
            2 => {
                let _ = storage::ensure_dir(self.sd, parts.get(0));
                let _ = storage::ensure_dir_in_dir(self.sd, parts.get(0), parts.get(1));
                Ok(())
            }
            _ => Err(UsbTransferRuntimeError::InvalidPath),
        }
    }

    fn begin_file(
        &mut self,
        path: &[u8],
        _size: u32,
        _crc32: u32,
    ) -> Result<(), UsbTransferRuntimeError> {
        let mut parts = PathParts::new();
        let count = parts.parse(path)?;

        match count {
            1 => {
                let _ = storage::delete_file(self.sd, parts.get(0));
            }
            2 => {
                let _ = storage::ensure_dir(self.sd, parts.get(0));
                let _ = storage::delete_file_in_dir(self.sd, parts.get(0), parts.get(1));
            }
            3 => {
                let _ = storage::ensure_dir(self.sd, parts.get(0));
                let _ = storage::ensure_dir_in_dir(self.sd, parts.get(0), parts.get(1));
                let _ = storage::delete_file_in_subdir(
                    self.sd,
                    parts.get(0),
                    parts.get(1),
                    parts.get(2),
                );
            }
            _ => return Err(UsbTransferRuntimeError::InvalidPath),
        }

        self.expected_offset = 0;
        self.set_active_path(path);
        Ok(())
    }

    fn write_chunk(
        &mut self,
        path: &[u8],
        offset: u32,
        data: &[u8],
    ) -> Result<(), UsbTransferRuntimeError> {
        if path != self.active_path() || offset != self.expected_offset {
            return Err(UsbTransferRuntimeError::InvalidChunk);
        }

        let mut parts = PathParts::new();
        let count = parts.parse(path)?;

        match count {
            1 => {
                if offset == 0 {
                    storage::write_file(self.sd, parts.get(0), data)
                        .map_err(|_| UsbTransferRuntimeError::Storage)?;
                } else {
                    storage::append_root_file(self.sd, parts.get(0), data)
                        .map_err(|_| UsbTransferRuntimeError::Storage)?;
                }
            }
            2 => {
                if offset == 0 {
                    let _ = storage::ensure_dir(self.sd, parts.get(0));
                    storage::write_file_in_dir(self.sd, parts.get(0), parts.get(1), data)
                        .map_err(|_| UsbTransferRuntimeError::Storage)?;
                } else {
                    storage::append_file_in_dir(self.sd, parts.get(0), parts.get(1), data)
                        .map_err(|_| UsbTransferRuntimeError::Storage)?;
                }
            }
            3 => {
                if offset == 0 {
                    let _ = storage::ensure_dir(self.sd, parts.get(0));
                    let _ = storage::ensure_dir_in_dir(self.sd, parts.get(0), parts.get(1));
                    storage::write_file_in_subdir(
                        self.sd,
                        parts.get(0),
                        parts.get(1),
                        parts.get(2),
                        data,
                    )
                    .map_err(|_| UsbTransferRuntimeError::Storage)?;
                } else {
                    storage::append_file_in_subdir(
                        self.sd,
                        parts.get(0),
                        parts.get(1),
                        parts.get(2),
                        data,
                    )
                    .map_err(|_| UsbTransferRuntimeError::Storage)?;
                }
            }
            _ => return Err(UsbTransferRuntimeError::InvalidPath),
        }

        self.expected_offset = self.expected_offset.saturating_add(data.len() as u32);
        Ok(())
    }

    fn finish_file(
        &mut self,
        path: &[u8],
        size: u32,
        _crc32: u32,
    ) -> Result<(), UsbTransferRuntimeError> {
        if path != self.active_path() || self.expected_offset != size {
            return Err(UsbTransferRuntimeError::InvalidChunk);
        }

        self.expected_offset = 0;
        self.active_path_len = 0;
        Ok(())
    }
}

struct PathParts<'a> {
    raw: &'a str,
    starts: [usize; 4],
    lens: [usize; 4],
    count: usize,
}

impl<'a> PathParts<'a> {
    const fn new() -> Self {
        Self {
            raw: "",
            starts: [0; 4],
            lens: [0; 4],
            count: 0,
        }
    }

    fn parse(&mut self, path: &'a [u8]) -> Result<usize, UsbTransferRuntimeError> {
        let raw = core::str::from_utf8(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;
        let raw = raw.trim_matches('/');

        self.raw = raw;
        self.count = 0;

        if raw.is_empty() {
            return Ok(0);
        }

        let bytes = raw.as_bytes();
        let mut start = 0usize;

        for i in 0..=bytes.len() {
            if i == bytes.len() || bytes[i] == b'/' {
                if self.count >= self.starts.len() || i == start {
                    return Err(UsbTransferRuntimeError::InvalidPath);
                }

                let part = &raw[start..i];
                if part == "." || part == ".." || part.contains('\\') || part.contains(':') {
                    return Err(UsbTransferRuntimeError::InvalidPath);
                }

                self.starts[self.count] = start;
                self.lens[self.count] = i - start;
                self.count += 1;
                start = i + 1;
            }
        }

        Ok(self.count)
    }

    fn get(&self, idx: usize) -> &str {
        let start = self.starts[idx];
        let len = self.lens[idx];
        &self.raw[start..start + len]
    }
}

async fn render_screen(
    epd: &mut Epd,
    strip: &mut StripBuffer,
    delay: &mut Delay,
    heading: &'static BitmapFont,
    body: &'static BitmapFont,
    lines: &[&str],
    footer: Option<&str>,
    bumps: &ButtonFeedback,
    full_refresh: bool,
) {
    let heading_h = heading.line_height;
    let body_h = body.line_height;
    let body_stride = body_h + BODY_LINE_GAP;

    let heading_region = Region::new(HEADING_X, CONTENT_TOP + 12, HEADING_W, heading_h);

    let body_area_top = CONTENT_TOP + 12 + heading_h + 40;
    let body_area_bottom = FOOTER_Y.saturating_sub(20);
    let body_area_h = body_area_bottom.saturating_sub(body_area_top);
    let total_body_h = if lines.is_empty() {
        0
    } else {
        (lines.len() as u16 - 1) * body_stride + body_h
    };
    let body_start_y = body_area_top + body_area_h.saturating_sub(total_body_h) / 2;

    let footer_region = Region::new(BODY_X, FOOTER_Y, BODY_W, body_h);

    let draw = |s: &mut StripBuffer| {
        BitmapLabel::new(heading_region, "USB Transfer", heading)
            .alignment(Alignment::CenterLeft)
            .draw(s)
            .unwrap();

        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                continue;
            }

            let y = body_start_y + (i as u16) * body_stride;
            let region = Region::new(BODY_X, y, BODY_W, body_h);
            BitmapLabel::new(region, line, body)
                .alignment(Alignment::Center)
                .draw(s)
                .unwrap();
        }

        if let Some(text) = footer {
            BitmapLabel::new(footer_region, text, body)
                .alignment(Alignment::Center)
                .draw(s)
                .unwrap();
        }

        bumps.draw(s);
    };

    if full_refresh {
        epd.full_refresh_async(strip, delay, &draw).await;
    } else {
        epd.partial_refresh_async(strip, delay, 0, 0, SCREEN_W, SCREEN_H, &draw)
            .await;
    }
}
