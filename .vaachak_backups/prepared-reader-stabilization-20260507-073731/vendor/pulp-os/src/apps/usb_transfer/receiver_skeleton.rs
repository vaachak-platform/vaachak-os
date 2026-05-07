#![allow(dead_code)]

//! USB Serial SD Bulk Transfer receiver skeleton.
//!
//! This module is intentionally not wired into the runtime yet.
//! It defines the parser/state-machine pieces that the future USB Transfer
//! app will connect to the ESP32-C3 USB serial/JTAG stream and SD storage.

use core::cmp;

pub const USB_TRANSFER_MAGIC: &[u8; 5] = b"VUSB1";
pub const USB_TRANSFER_MAX_PATH: usize = 96;
pub const USB_TRANSFER_MAX_PAYLOAD: usize = 8192;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbTransferFrameType {
    Hello = 1,
    Mkdir = 2,
    Begin = 3,
    Chunk = 4,
    End = 5,
    Done = 6,
    Abort = 7,
}

impl UsbTransferFrameType {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Hello),
            2 => Some(Self::Mkdir),
            3 => Some(Self::Begin),
            4 => Some(Self::Chunk),
            5 => Some(Self::End),
            6 => Some(Self::Done),
            7 => Some(Self::Abort),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbTransferError {
    BadMagic,
    BadFrameType,
    PayloadTooLarge,
    CrcMismatch,
    InvalidPath,
    InvalidUtf8,
    MissingField,
    UnexpectedFrame,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbTransferState {
    Idle,
    Ready,
    ReceivingFile,
    Complete,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbTransferFrame<'a> {
    pub frame_type: UsbTransferFrameType,
    pub payload: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveFile {
    pub path: [u8; USB_TRANSFER_MAX_PATH],
    pub path_len: usize,
    pub size: u32,
    pub received: u32,
    pub expected_crc32: u32,
    pub running_crc32: u32,
}

impl ActiveFile {
    pub const fn empty() -> Self {
        Self {
            path: [0u8; USB_TRANSFER_MAX_PATH],
            path_len: 0,
            size: 0,
            received: 0,
            expected_crc32: 0,
            running_crc32: 0,
        }
    }

    pub fn path(&self) -> &[u8] {
        &self.path[..self.path_len]
    }
}

#[derive(Debug)]
pub struct UsbTransferReceiver {
    pub state: UsbTransferState,
    pub active: ActiveFile,
    pub files_started: u32,
    pub files_completed: u32,
    pub bytes_received: u32,
}

impl UsbTransferReceiver {
    pub const fn new() -> Self {
        Self {
            state: UsbTransferState::Idle,
            active: ActiveFile::empty(),
            files_started: 0,
            files_completed: 0,
            bytes_received: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn accept_frame(&mut self, frame: UsbTransferFrame<'_>) -> Result<(), UsbTransferError> {
        match frame.frame_type {
            UsbTransferFrameType::Hello => {
                self.state = UsbTransferState::Ready;
                Ok(())
            }
            UsbTransferFrameType::Mkdir => {
                if !matches!(
                    self.state,
                    UsbTransferState::Ready | UsbTransferState::ReceivingFile
                ) {
                    return Err(UsbTransferError::UnexpectedFrame);
                }

                let path = extract_json_string(frame.payload, b"path")?;
                validate_sd_path(path)?;
                Ok(())
            }
            UsbTransferFrameType::Begin => {
                if !matches!(self.state, UsbTransferState::Ready) {
                    return Err(UsbTransferError::UnexpectedFrame);
                }

                let path = extract_json_string(frame.payload, b"path")?;
                validate_sd_path(path)?;

                let size = extract_json_u32(frame.payload, b"size")?;
                let crc32 = extract_json_hex_u32(frame.payload, b"crc32")?;

                let mut active = ActiveFile::empty();
                active.path_len = cmp::min(path.len(), active.path.len());
                active.path[..active.path_len].copy_from_slice(&path[..active.path_len]);
                active.size = size;
                active.expected_crc32 = crc32;
                active.running_crc32 = 0xFFFF_FFFF;

                self.active = active;
                self.files_started = self.files_started.saturating_add(1);
                self.state = UsbTransferState::ReceivingFile;
                Ok(())
            }
            UsbTransferFrameType::Chunk => {
                if self.state != UsbTransferState::ReceivingFile {
                    return Err(UsbTransferError::UnexpectedFrame);
                }

                let split = frame
                    .payload
                    .iter()
                    .position(|&b| b == b'\n')
                    .ok_or(UsbTransferError::MissingField)?;

                let header = &frame.payload[..split];
                let data = &frame.payload[split + 1..];

                let length = extract_json_u32(header, b"length")?;
                let chunk_crc = extract_json_hex_u32(header, b"crc32")?;

                if length as usize != data.len() {
                    return Err(UsbTransferError::MissingField);
                }

                if crc32(data) != chunk_crc {
                    return Err(UsbTransferError::CrcMismatch);
                }

                self.active.received = self.active.received.saturating_add(data.len() as u32);
                self.bytes_received = self.bytes_received.saturating_add(data.len() as u32);
                Ok(())
            }
            UsbTransferFrameType::End => {
                if self.state != UsbTransferState::ReceivingFile {
                    return Err(UsbTransferError::UnexpectedFrame);
                }

                if self.active.received != self.active.size {
                    self.state = UsbTransferState::Failed;
                    return Err(UsbTransferError::UnexpectedFrame);
                }

                self.files_completed = self.files_completed.saturating_add(1);
                self.active = ActiveFile::empty();
                self.state = UsbTransferState::Ready;
                Ok(())
            }
            UsbTransferFrameType::Done => {
                if !matches!(self.state, UsbTransferState::Ready | UsbTransferState::Idle) {
                    return Err(UsbTransferError::UnexpectedFrame);
                }

                self.state = UsbTransferState::Complete;
                Ok(())
            }
            UsbTransferFrameType::Abort => {
                self.state = UsbTransferState::Failed;
                Ok(())
            }
        }
    }
}

pub fn parse_frame(input: &[u8]) -> Result<UsbTransferFrame<'_>, UsbTransferError> {
    if input.len() < 14 {
        return Err(UsbTransferError::MissingField);
    }

    if &input[..5] != USB_TRANSFER_MAGIC {
        return Err(UsbTransferError::BadMagic);
    }

    let frame_type =
        UsbTransferFrameType::from_u8(input[5]).ok_or(UsbTransferError::BadFrameType)?;

    let len = u32::from_le_bytes([input[6], input[7], input[8], input[9]]) as usize;
    if len > USB_TRANSFER_MAX_PAYLOAD {
        return Err(UsbTransferError::PayloadTooLarge);
    }

    let expected_total = 10usize
        .checked_add(len)
        .and_then(|v| v.checked_add(4))
        .ok_or(UsbTransferError::PayloadTooLarge)?;

    if input.len() < expected_total {
        return Err(UsbTransferError::MissingField);
    }

    let crc_offset = 10 + len;
    let actual_crc = u32::from_le_bytes([
        input[crc_offset],
        input[crc_offset + 1],
        input[crc_offset + 2],
        input[crc_offset + 3],
    ]);
    let expected_crc = crc32(&input[..crc_offset]);

    if actual_crc != expected_crc {
        return Err(UsbTransferError::CrcMismatch);
    }

    Ok(UsbTransferFrame {
        frame_type,
        payload: &input[10..10 + len],
    })
}

pub fn validate_sd_path(path: &[u8]) -> Result<(), UsbTransferError> {
    if path.is_empty() || path[0] != b'/' {
        return Err(UsbTransferError::InvalidPath);
    }

    if path.len() > USB_TRANSFER_MAX_PATH {
        return Err(UsbTransferError::InvalidPath);
    }

    let mut component_len = 0usize;
    let mut component_count = 0usize;
    let mut prev_slash = false;

    for (idx, &b) in path.iter().enumerate() {
        if b == b'\\' || b == b':' || b < 0x20 {
            return Err(UsbTransferError::InvalidPath);
        }

        if b == b'/' {
            if idx != 0 && prev_slash {
                return Err(UsbTransferError::InvalidPath);
            }

            if component_len > 0 {
                component_count += 1;
                component_len = 0;
            }

            prev_slash = true;
            continue;
        }

        prev_slash = false;
        component_len += 1;

        if component_len > 32 {
            return Err(UsbTransferError::InvalidPath);
        }
    }

    if component_len > 0 {
        component_count += 1;
    }

    if component_count > 4 {
        return Err(UsbTransferError::InvalidPath);
    }

    for component in path.split(|&b| b == b'/') {
        if component.is_empty() {
            continue;
        }

        if component == b"." || component == b".." {
            return Err(UsbTransferError::InvalidPath);
        }
    }

    Ok(())
}

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;

    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }

    !crc
}

fn extract_json_string<'a>(payload: &'a [u8], key: &[u8]) -> Result<&'a [u8], UsbTransferError> {
    let needle = make_json_key_needle(key)?;
    let pos = find_subsequence(payload, &needle).ok_or(UsbTransferError::MissingField)?;
    let mut start = pos + needle.len();

    while start < payload.len() && payload[start] == b' ' {
        start += 1;
    }

    if start >= payload.len() || payload[start] != b'"' {
        return Err(UsbTransferError::MissingField);
    }

    start += 1;
    let end = payload[start..]
        .iter()
        .position(|&b| b == b'"')
        .ok_or(UsbTransferError::MissingField)?;

    Ok(&payload[start..start + end])
}

fn extract_json_u32(payload: &[u8], key: &[u8]) -> Result<u32, UsbTransferError> {
    let needle = make_json_key_needle(key)?;
    let pos = find_subsequence(payload, &needle).ok_or(UsbTransferError::MissingField)?;
    let mut start = pos + needle.len();

    while start < payload.len() && payload[start] == b' ' {
        start += 1;
    }

    let mut value = 0u32;
    let mut any = false;

    while start < payload.len() {
        let b = payload[start];
        if !b.is_ascii_digit() {
            break;
        }

        any = true;
        value = value
            .checked_mul(10)
            .and_then(|v| v.checked_add(u32::from(b - b'0')))
            .ok_or(UsbTransferError::MissingField)?;
        start += 1;
    }

    if any {
        Ok(value)
    } else {
        Err(UsbTransferError::MissingField)
    }
}

fn extract_json_hex_u32(payload: &[u8], key: &[u8]) -> Result<u32, UsbTransferError> {
    let raw = extract_json_string(payload, key)?;
    let mut value = 0u32;

    for &b in raw {
        let digit = match b {
            b'0'..=b'9' => u32::from(b - b'0'),
            b'a'..=b'f' => u32::from(b - b'a' + 10),
            b'A'..=b'F' => u32::from(b - b'A' + 10),
            _ => return Err(UsbTransferError::MissingField),
        };

        value = value
            .checked_mul(16)
            .and_then(|v| v.checked_add(digit))
            .ok_or(UsbTransferError::MissingField)?;
    }

    Ok(value)
}

fn make_json_key_needle(key: &[u8]) -> Result<[u8; 40], UsbTransferError> {
    if key.len() + 3 > 40 {
        return Err(UsbTransferError::MissingField);
    }

    let mut out = [0u8; 40];
    out[0] = b'"';
    out[1..1 + key.len()].copy_from_slice(key);
    out[1 + key.len()] = b'"';
    out[2 + key.len()] = b':';
    Ok(out)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let needle = trim_nul_tail(needle);

    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }

    haystack.windows(needle.len()).position(|w| w == needle)
}

fn trim_nul_tail(input: &[u8]) -> &[u8] {
    let mut end = input.len();
    while end > 0 && input[end - 1] == 0 {
        end -= 1;
    }
    &input[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_safe_paths() {
        assert!(validate_sd_path(b"/FCACHE/15D1296A/P000.VRN").is_ok());
        assert!(validate_sd_path(b"/YEARLY_H.TXT").is_ok());
    }

    #[test]
    fn rejects_traversal() {
        assert!(validate_sd_path(b"/../SECRET").is_err());
        assert!(validate_sd_path(b"/FCACHE/../SECRET").is_err());
        assert!(validate_sd_path(b"/FCACHE\\BAD").is_err());
        assert!(validate_sd_path(b"/FCACHE:BAD").is_err());
    }

    #[test]
    fn crc32_matches_standard_vector() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }
}
