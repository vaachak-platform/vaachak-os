//! Device-side runtime for USB Serial SD Bulk Transfer.
//!
//! This is the integration layer above the frame parser/state machine and below
//! the future hardware USB serial byte stream.
//!
//! It intentionally depends on a small `SdTransferTarget` trait so it can be
//! tested and later wired to the real SD storage layer without coupling the
//! protocol parser to a concrete storage implementation.

use super::receiver_skeleton::{
    USB_TRANSFER_MAX_PATH, UsbTransferError, UsbTransferFrameType, UsbTransferReceiver, crc32,
    parse_frame, validate_sd_path,
};

pub const USB_TRANSFER_ACK_OK: &[u8] = b"OK\n";
pub const USB_TRANSFER_ACK_ERR: &[u8] = b"ERR\n";
pub const USB_TRANSFER_READ_BUF: usize = 9216;
pub const USB_TRANSFER_MAX_FILE_BYTES: u32 = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbTransferRuntimeError {
    Protocol(UsbTransferError),
    Storage,
    InvalidPath,
    InvalidChunk,
    MissingField,
    FileTooLarge,
    CrcMismatch,
    UnexpectedFrame,
}

impl From<UsbTransferError> for UsbTransferRuntimeError {
    fn from(value: UsbTransferError) -> Self {
        Self::Protocol(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbTransferRuntimeStatus {
    Idle,
    Ready,
    Receiving,
    Complete,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbTransferProgress {
    pub status: UsbTransferRuntimeStatus,
    pub files_started: u32,
    pub files_completed: u32,
    pub bytes_received: u32,
    pub current_path: [u8; USB_TRANSFER_MAX_PATH],
    pub current_path_len: usize,
    pub current_file_size: u32,
    pub current_file_received: u32,
}

impl UsbTransferProgress {
    pub const fn empty() -> Self {
        Self {
            status: UsbTransferRuntimeStatus::Idle,
            files_started: 0,
            files_completed: 0,
            bytes_received: 0,
            current_path: [0u8; USB_TRANSFER_MAX_PATH],
            current_path_len: 0,
            current_file_size: 0,
            current_file_received: 0,
        }
    }

    pub fn current_path(&self) -> &[u8] {
        &self.current_path[..self.current_path_len]
    }
}

pub trait SdTransferTarget {
    fn ensure_dir(&mut self, path: &[u8]) -> Result<(), UsbTransferRuntimeError>;
    fn begin_file(
        &mut self,
        path: &[u8],
        size: u32,
        crc32: u32,
    ) -> Result<(), UsbTransferRuntimeError>;
    fn write_chunk(
        &mut self,
        path: &[u8],
        offset: u32,
        data: &[u8],
    ) -> Result<(), UsbTransferRuntimeError>;
    fn finish_file(
        &mut self,
        path: &[u8],
        size: u32,
        crc32: u32,
    ) -> Result<(), UsbTransferRuntimeError>;
}

#[derive(Debug)]
pub struct UsbTransferRuntime {
    receiver: UsbTransferReceiver,
    progress: UsbTransferProgress,
    active_crc32: u32,
}

impl UsbTransferRuntime {
    pub const fn new() -> Self {
        Self {
            receiver: UsbTransferReceiver::new(),
            progress: UsbTransferProgress::empty(),
            active_crc32: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn progress(&self) -> UsbTransferProgress {
        self.progress
    }

    pub fn accept_raw_frame<T: SdTransferTarget>(
        &mut self,
        raw: &[u8],
        sd: &mut T,
    ) -> Result<&'static [u8], UsbTransferRuntimeError> {
        match parse_frame(raw) {
            Ok(frame) => {
                self.accept_frame(frame.frame_type, frame.payload, sd)?;
                Ok(USB_TRANSFER_ACK_OK)
            }
            Err(err) => {
                self.progress.status = UsbTransferRuntimeStatus::Failed;
                Err(err.into())
            }
        }
    }

    pub fn accept_frame<T: SdTransferTarget>(
        &mut self,
        frame_type: UsbTransferFrameType,
        payload: &[u8],
        sd: &mut T,
    ) -> Result<(), UsbTransferRuntimeError> {
        match frame_type {
            UsbTransferFrameType::Hello => {
                self.receiver
                    .accept_frame(super::receiver_skeleton::UsbTransferFrame {
                        frame_type,
                        payload,
                    })?;
                self.progress.status = UsbTransferRuntimeStatus::Ready;
                Ok(())
            }
            UsbTransferFrameType::Mkdir => {
                let path = extract_json_string(payload, b"path")?;
                validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;
                sd.ensure_dir(path)?;
                self.receiver
                    .accept_frame(super::receiver_skeleton::UsbTransferFrame {
                        frame_type,
                        payload,
                    })?;
                self.progress.status = UsbTransferRuntimeStatus::Ready;
                Ok(())
            }
            UsbTransferFrameType::Begin => {
                let path = extract_json_string(payload, b"path")?;
                validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;

                let size = extract_json_u32(payload, b"size")?;
                if size > USB_TRANSFER_MAX_FILE_BYTES {
                    return Err(UsbTransferRuntimeError::FileTooLarge);
                }

                let expected_crc = extract_json_hex_u32(payload, b"crc32")?;
                // BEGIN is a metadata/preflight frame. Some FAT paths cannot
                // safely create/truncate an empty file here, so storage errors
                // are intentionally deferred to CHUNK offset 0 where data is
                // actually written.
                let _ = sd.begin_file(path, size, expected_crc);

                self.receiver
                    .accept_frame(super::receiver_skeleton::UsbTransferFrame {
                        frame_type,
                        payload,
                    })?;

                self.active_crc32 = 0xFFFF_FFFF;
                self.progress.status = UsbTransferRuntimeStatus::Receiving;
                self.progress.files_started = self.progress.files_started.saturating_add(1);
                self.progress.current_file_size = size;
                self.progress.current_file_received = 0;
                self.copy_current_path(path);

                Ok(())
            }
            UsbTransferFrameType::Chunk => {
                let split = payload
                    .iter()
                    .position(|&b| b == b'\n')
                    .ok_or(UsbTransferRuntimeError::MissingField)?;

                let header = &payload[..split];
                let data = &payload[split + 1..];

                let path = extract_json_string(header, b"path")?;
                let offset = extract_json_u32(header, b"offset")?;
                let length = extract_json_u32(header, b"length")?;
                let chunk_crc = extract_json_hex_u32(header, b"crc32")?;

                validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;

                if length as usize != data.len() {
                    return Err(UsbTransferRuntimeError::InvalidChunk);
                }

                if crc32(data) != chunk_crc {
                    return Err(UsbTransferRuntimeError::CrcMismatch);
                }

                sd.write_chunk(path, offset, data)?;

                self.receiver
                    .accept_frame(super::receiver_skeleton::UsbTransferFrame {
                        frame_type,
                        payload,
                    })?;

                self.active_crc32 = crc32_update(self.active_crc32, data);
                self.progress.bytes_received = self
                    .progress
                    .bytes_received
                    .saturating_add(data.len() as u32);
                self.progress.current_file_received = offset.saturating_add(data.len() as u32);

                Ok(())
            }
            UsbTransferFrameType::End => {
                let path = extract_json_string(payload, b"path")?;
                validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;

                let size = extract_json_u32(payload, b"size")?;
                let expected_crc = extract_json_hex_u32(payload, b"crc32")?;
                let actual_crc = crc32_finalize(self.active_crc32);

                if actual_crc != expected_crc {
                    self.progress.status = UsbTransferRuntimeStatus::Failed;
                    return Err(UsbTransferRuntimeError::CrcMismatch);
                }

                sd.finish_file(path, size, expected_crc)?;

                self.receiver
                    .accept_frame(super::receiver_skeleton::UsbTransferFrame {
                        frame_type,
                        payload,
                    })?;

                self.progress.files_completed = self.progress.files_completed.saturating_add(1);
                self.progress.status = UsbTransferRuntimeStatus::Ready;
                self.progress.current_file_size = 0;
                self.progress.current_file_received = 0;
                self.progress.current_path_len = 0;
                self.active_crc32 = 0;

                Ok(())
            }
            UsbTransferFrameType::Done => {
                self.receiver
                    .accept_frame(super::receiver_skeleton::UsbTransferFrame {
                        frame_type,
                        payload,
                    })?;
                self.progress.status = UsbTransferRuntimeStatus::Complete;
                Ok(())
            }
            UsbTransferFrameType::Abort => {
                self.receiver
                    .accept_frame(super::receiver_skeleton::UsbTransferFrame {
                        frame_type,
                        payload,
                    })?;
                self.progress.status = UsbTransferRuntimeStatus::Failed;
                Ok(())
            }
        }
    }

    fn copy_current_path(&mut self, path: &[u8]) {
        let len = path.len().min(self.progress.current_path.len());
        self.progress.current_path[..len].copy_from_slice(&path[..len]);
        self.progress.current_path_len = len;
    }
}

fn crc32_update(mut state: u32, data: &[u8]) -> u32 {
    for &byte in data {
        state ^= u32::from(byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(state & 1);
            state = (state >> 1) ^ (0xEDB8_8320 & mask);
        }
    }

    state
}

fn crc32_finalize(state: u32) -> u32 {
    !state
}

fn extract_json_string<'a>(
    payload: &'a [u8],
    key: &[u8],
) -> Result<&'a [u8], UsbTransferRuntimeError> {
    let needle = make_json_key_needle(key)?;
    let pos = find_subsequence(payload, &needle).ok_or(UsbTransferRuntimeError::MissingField)?;
    let mut start = pos + needle.len();

    while start < payload.len() && payload[start] == b' ' {
        start += 1;
    }

    if start >= payload.len() || payload[start] != b'"' {
        return Err(UsbTransferRuntimeError::MissingField);
    }

    start += 1;
    let end = payload[start..]
        .iter()
        .position(|&b| b == b'"')
        .ok_or(UsbTransferRuntimeError::MissingField)?;

    Ok(&payload[start..start + end])
}

fn extract_json_u32(payload: &[u8], key: &[u8]) -> Result<u32, UsbTransferRuntimeError> {
    let needle = make_json_key_needle(key)?;
    let pos = find_subsequence(payload, &needle).ok_or(UsbTransferRuntimeError::MissingField)?;
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
            .ok_or(UsbTransferRuntimeError::MissingField)?;
        start += 1;
    }

    if any {
        Ok(value)
    } else {
        Err(UsbTransferRuntimeError::MissingField)
    }
}

fn extract_json_hex_u32(payload: &[u8], key: &[u8]) -> Result<u32, UsbTransferRuntimeError> {
    let raw = extract_json_string(payload, key)?;
    let mut value = 0u32;

    for &b in raw {
        let digit = match b {
            b'0'..=b'9' => u32::from(b - b'0'),
            b'a'..=b'f' => u32::from(b - b'a' + 10),
            b'A'..=b'F' => u32::from(b - b'A' + 10),
            _ => return Err(UsbTransferRuntimeError::MissingField),
        };

        value = value
            .checked_mul(16)
            .and_then(|v| v.checked_add(digit))
            .ok_or(UsbTransferRuntimeError::MissingField)?;
    }

    Ok(value)
}

fn make_json_key_needle(key: &[u8]) -> Result<[u8; 40], UsbTransferRuntimeError> {
    if key.len() + 3 > 40 {
        return Err(UsbTransferRuntimeError::MissingField);
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

    struct MemoryTarget {
        dirs: u32,
        begins: u32,
        chunks: u32,
        ends: u32,
        bytes: u32,
    }

    impl MemoryTarget {
        const fn new() -> Self {
            Self {
                dirs: 0,
                begins: 0,
                chunks: 0,
                ends: 0,
                bytes: 0,
            }
        }
    }

    impl SdTransferTarget for MemoryTarget {
        fn ensure_dir(&mut self, path: &[u8]) -> Result<(), UsbTransferRuntimeError> {
            validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;
            self.dirs += 1;
            Ok(())
        }

        fn begin_file(
            &mut self,
            path: &[u8],
            _size: u32,
            _crc32: u32,
        ) -> Result<(), UsbTransferRuntimeError> {
            validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;
            self.begins += 1;
            Ok(())
        }

        fn write_chunk(
            &mut self,
            path: &[u8],
            _offset: u32,
            data: &[u8],
        ) -> Result<(), UsbTransferRuntimeError> {
            validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;
            self.chunks += 1;
            self.bytes += data.len() as u32;
            Ok(())
        }

        fn finish_file(
            &mut self,
            path: &[u8],
            _size: u32,
            _crc32: u32,
        ) -> Result<(), UsbTransferRuntimeError> {
            validate_sd_path(path).map_err(|_| UsbTransferRuntimeError::InvalidPath)?;
            self.ends += 1;
            Ok(())
        }
    }

    #[test]
    fn runtime_accepts_mkdir() {
        let mut runtime = UsbTransferRuntime::new();
        let mut target = MemoryTarget::new();

        runtime
            .accept_frame(
                UsbTransferFrameType::Hello,
                br#"{"version":1}"#,
                &mut target,
            )
            .unwrap();
        runtime
            .accept_frame(
                UsbTransferFrameType::Mkdir,
                br#"{"path":"/FCACHE/15D1296A"}"#,
                &mut target,
            )
            .unwrap();

        assert_eq!(target.dirs, 1);
        assert_eq!(runtime.progress().status, UsbTransferRuntimeStatus::Ready);
    }

    #[test]
    fn runtime_rejects_bad_mkdir_path() {
        let mut runtime = UsbTransferRuntime::new();
        let mut target = MemoryTarget::new();

        runtime
            .accept_frame(
                UsbTransferFrameType::Hello,
                br#"{"version":1}"#,
                &mut target,
            )
            .unwrap();

        assert!(
            runtime
                .accept_frame(
                    UsbTransferFrameType::Mkdir,
                    br#"{"path":"/FCACHE/../BAD"}"#,
                    &mut target,
                )
                .is_err()
        );
    }
}
