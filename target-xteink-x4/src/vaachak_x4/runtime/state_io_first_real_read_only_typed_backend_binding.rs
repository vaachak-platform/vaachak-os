// Phase 37F — State I/O First Real Read-Only Typed Backend Binding.
//
// This module is the first runtime-facing binding point between Vaachak typed
// state records and a concrete backend-provided read operation. It remains
// deliberately narrow: it builds canonical X4 state paths and invokes a
// caller-supplied read-only backend trait. It does not own SD/FAT/SPI/display/
// input/power behavior and it does not expose mutation operations.

#![allow(dead_code)]

pub const PHASE_37F_STATE_IO_FIRST_REAL_READ_ONLY_TYPED_BACKEND_BINDING_MARKER: &str =
    "phase37f=x4-state-io-first-real-read-only-typed-backend-binding-ok";

pub const PHASE_37F_STATE_DIR: &[u8] = b"state/";
pub const PHASE_37F_MAX_RENDERED_STATE_PATH_LEN: usize = 18;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37fTypedStateRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl Phase37fTypedStateRecordKind {
    pub const fn extension(self) -> &'static [u8; 3] {
        match self {
            Self::Progress => b"PRG",
            Self::Theme => b"THM",
            Self::Metadata => b"MTA",
            Self::Bookmark => b"BKM",
            Self::BookmarkIndex => b"TXT",
        }
    }

    pub const fn requires_book_id(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }

    pub const fn canonical_name_for_index(self) -> Option<&'static [u8; 9]> {
        match self {
            Self::BookmarkIndex => Some(b"BMIDX.TXT"),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase37fTypedStateRecordRef {
    pub kind: Phase37fTypedStateRecordKind,
    pub book_id: Option<u32>,
}

impl Phase37fTypedStateRecordRef {
    pub const fn for_book(kind: Phase37fTypedStateRecordKind, book_id: u32) -> Self {
        Self {
            kind,
            book_id: Some(book_id),
        }
    }

    pub const fn bookmark_index() -> Self {
        Self {
            kind: Phase37fTypedStateRecordKind::BookmarkIndex,
            book_id: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37fPathRenderError {
    BufferTooSmall,
    MissingBookId,
    UnexpectedBookId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase37fReadOnlyBackendStatus {
    Found { bytes_read: usize },
    NotFound,
    BufferTooSmall { required_capacity: usize },
    BackendUnavailable,
    UnsupportedRecordKind,
    InvalidRequest,
    CorruptRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase37fReadOnlyBackendOutcome {
    pub record_ref: Phase37fTypedStateRecordRef,
    pub rendered_path_len: usize,
    pub status: Phase37fReadOnlyBackendStatus,
}

pub trait Phase37fReadOnlyTypedStateBackend {
    fn read_typed_state_record(
        &self,
        rendered_path: &[u8],
        record_kind: Phase37fTypedStateRecordKind,
        output: &mut [u8],
    ) -> Phase37fReadOnlyBackendStatus;
}

pub fn phase37f_render_typed_state_path(
    record_ref: Phase37fTypedStateRecordRef,
    output: &mut [u8],
) -> Result<usize, Phase37fPathRenderError> {
    let mut cursor = append_bytes(output, 0, PHASE_37F_STATE_DIR)
        .map_err(|_| Phase37fPathRenderError::BufferTooSmall)?;

    if let Some(index_name) = record_ref.kind.canonical_name_for_index() {
        if record_ref.book_id.is_some() {
            return Err(Phase37fPathRenderError::UnexpectedBookId);
        }

        cursor = append_bytes(output, cursor, index_name)
            .map_err(|_| Phase37fPathRenderError::BufferTooSmall)?;
        return Ok(cursor);
    }

    let book_id = match record_ref.book_id {
        Some(book_id) => book_id,
        None => return Err(Phase37fPathRenderError::MissingBookId),
    };

    let mut hex = [0_u8; 8];
    render_hex8(book_id, &mut hex);
    cursor =
        append_bytes(output, cursor, &hex).map_err(|_| Phase37fPathRenderError::BufferTooSmall)?;
    cursor =
        append_bytes(output, cursor, b".").map_err(|_| Phase37fPathRenderError::BufferTooSmall)?;
    cursor = append_bytes(output, cursor, record_ref.kind.extension())
        .map_err(|_| Phase37fPathRenderError::BufferTooSmall)?;

    Ok(cursor)
}

pub fn phase37f_read_typed_state_record<B: Phase37fReadOnlyTypedStateBackend>(
    backend: &B,
    record_ref: Phase37fTypedStateRecordRef,
    output: &mut [u8],
) -> Phase37fReadOnlyBackendOutcome {
    let mut path = [0_u8; PHASE_37F_MAX_RENDERED_STATE_PATH_LEN];
    let rendered_path_len = match phase37f_render_typed_state_path(record_ref, &mut path) {
        Ok(rendered_path_len) => rendered_path_len,
        Err(_) => {
            return Phase37fReadOnlyBackendOutcome {
                record_ref,
                rendered_path_len: 0,
                status: Phase37fReadOnlyBackendStatus::InvalidRequest,
            };
        }
    };

    let status =
        backend.read_typed_state_record(&path[..rendered_path_len], record_ref.kind, output);

    Phase37fReadOnlyBackendOutcome {
        record_ref,
        rendered_path_len,
        status,
    }
}

pub fn phase37f_is_backend_status_read_only(status: Phase37fReadOnlyBackendStatus) -> bool {
    matches!(
        status,
        Phase37fReadOnlyBackendStatus::Found { .. }
            | Phase37fReadOnlyBackendStatus::NotFound
            | Phase37fReadOnlyBackendStatus::BufferTooSmall { .. }
            | Phase37fReadOnlyBackendStatus::BackendUnavailable
            | Phase37fReadOnlyBackendStatus::UnsupportedRecordKind
            | Phase37fReadOnlyBackendStatus::InvalidRequest
            | Phase37fReadOnlyBackendStatus::CorruptRecord
    )
}

pub fn phase37f_supported_record_kinds() -> &'static [Phase37fTypedStateRecordKind] {
    &[
        Phase37fTypedStateRecordKind::Progress,
        Phase37fTypedStateRecordKind::Theme,
        Phase37fTypedStateRecordKind::Metadata,
        Phase37fTypedStateRecordKind::Bookmark,
        Phase37fTypedStateRecordKind::BookmarkIndex,
    ]
}

fn append_bytes(output: &mut [u8], cursor: usize, bytes: &[u8]) -> Result<usize, ()> {
    let end = cursor.checked_add(bytes.len()).ok_or(())?;
    if end > output.len() {
        return Err(());
    }

    output[cursor..end].copy_from_slice(bytes);
    Ok(end)
}

fn render_hex8(value: u32, output: &mut [u8; 8]) {
    let mut shift: u32 = 28;
    for byte in output.iter_mut() {
        let nibble = ((value >> shift) & 0x0f) as u8;
        *byte = match nibble {
            0..=9 => b'0' + nibble,
            _ => b'A' + (nibble - 10),
        };
        shift = shift.saturating_sub(4);
    }
}
