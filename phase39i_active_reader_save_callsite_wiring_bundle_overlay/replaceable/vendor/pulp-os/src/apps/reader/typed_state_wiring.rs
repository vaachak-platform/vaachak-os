//! Phase 39I — Active Reader Typed-State Save Callsite Wiring.
//!
//! This module is intentionally Pulp-local because the active reader save
//! callsites live inside the imported Pulp reader crate.
//!
//! It centralizes active reader writes under one typed-state facade while still
//! delegating to the existing `KernelHandle` file APIs. This avoids introducing
//! a dependency from `vendor/pulp-os` back to `target-xteink-x4`.
//!
//! Covered active reader state files:
//! - `state/<BOOKID>.PRG`
//! - `state/<BOOKID>.THM`
//! - `state/<BOOKID>.MTA`
//! - `state/<BOOKID>.BKM`
//! - `state/BMIDX.TXT`
//! - `state/recent.txt` remains routed through the same facade as reader state
//!
//! This does not move SD/FAT/SPI/display/input/power ownership. It only makes
//! the active reader save callsites route through a single typed-state seam.

#![allow(dead_code)]

use crate::apps::reader_state;
use crate::error::Result;
use crate::kernel::KernelHandle;

pub const PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_WIRING_MARKER: &str =
    "phase39i=x4-active-reader-save-callsite-wiring-bundle-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39iReaderStateWriteKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
    Recent,
    UnknownStateFile,
    NonStateSubdir,
}

impl Phase39iReaderStateWriteKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Progress => "progress",
            Self::Theme => "theme",
            Self::Metadata => "metadata",
            Self::Bookmark => "bookmark",
            Self::BookmarkIndex => "bookmark-index",
            Self::Recent => "recent",
            Self::UnknownStateFile => "unknown-state-file",
            Self::NonStateSubdir => "non-state-subdir",
        }
    }

    pub const fn is_typed_record(self) -> bool {
        matches!(
            self,
            Self::Progress | Self::Theme | Self::Metadata | Self::Bookmark | Self::BookmarkIndex
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39iReaderStateWriteReport {
    pub kind: Phase39iReaderStateWriteKind,
    pub bytes: usize,
    pub ok: bool,
}

impl Phase39iReaderStateWriteReport {
    pub const fn accepted(self) -> bool {
        self.ok
    }
}

/// Centralized active reader `state/` directory creation.
///
/// This intentionally delegates to the existing KernelHandle API so hardware and
/// filesystem behavior remain unchanged.
pub fn ensure_state_dir(k: &mut KernelHandle<'_>) -> Result<()> {
    k.ensure_app_subdir(reader_state::STATE_DIR)
}

/// Centralized active reader subdir write facade.
///
/// Phase 39I patching rewrites active reader `k.write_app_subdir(...)` callsites
/// to call this function. Behavior remains delegated to KernelHandle.
pub fn write_app_subdir(
    k: &mut KernelHandle<'_>,
    dir: &str,
    name: &str,
    data: &[u8],
) -> Result<()> {
    let kind = classify_write(dir, name);
    let result = k.write_app_subdir(dir, name, data);

    match &result {
        Ok(()) => {
            log::debug!(
                "phase39i: reader state write kind={} path={}/{} bytes={} ok=true",
                kind.label(),
                dir,
                name,
                data.len()
            );
        }
        Err(e) => {
            log::warn!(
                "phase39i: reader state write kind={} path={}/{} bytes={} ok=false err={}",
                kind.label(),
                dir,
                name,
                data.len(),
                e
            );
        }
    }

    result
}

pub fn write_progress_record(k: &mut KernelHandle<'_>, name: &str, data: &[u8]) -> Result<()> {
    write_app_subdir(k, reader_state::STATE_DIR, name, data)
}

pub fn write_theme_record(k: &mut KernelHandle<'_>, name: &str, data: &[u8]) -> Result<()> {
    write_app_subdir(k, reader_state::STATE_DIR, name, data)
}

pub fn write_metadata_record(k: &mut KernelHandle<'_>, name: &str, data: &[u8]) -> Result<()> {
    write_app_subdir(k, reader_state::STATE_DIR, name, data)
}

pub fn write_bookmark_record(k: &mut KernelHandle<'_>, name: &str, data: &[u8]) -> Result<()> {
    write_app_subdir(k, reader_state::STATE_DIR, name, data)
}

pub fn write_bookmark_index(k: &mut KernelHandle<'_>, data: &[u8]) -> Result<()> {
    write_app_subdir(k, reader_state::STATE_DIR, reader_state::BOOKMARKS_INDEX_FILE, data)
}

pub fn classify_write(dir: &str, name: &str) -> Phase39iReaderStateWriteKind {
    if dir != reader_state::STATE_DIR {
        return Phase39iReaderStateWriteKind::NonStateSubdir;
    }

    classify_state_file_name(name)
}

pub fn classify_state_file_name(name: &str) -> Phase39iReaderStateWriteKind {
    if name.eq_ignore_ascii_case(reader_state::BOOKMARKS_INDEX_FILE) {
        return Phase39iReaderStateWriteKind::BookmarkIndex;
    }

    if name.eq_ignore_ascii_case(reader_state::RECENT_RECORD_FILE) {
        return Phase39iReaderStateWriteKind::Recent;
    }

    let bytes = name.as_bytes();
    if bytes.len() < 5 {
        return Phase39iReaderStateWriteKind::UnknownStateFile;
    }

    let ext = &bytes[bytes.len() - 4..];
    if ext.eq_ignore_ascii_case(reader_state::PROGRESS_RECORD_EXT.as_bytes()) {
        Phase39iReaderStateWriteKind::Progress
    } else if ext.eq_ignore_ascii_case(reader_state::THEME_RECORD_EXT.as_bytes()) {
        Phase39iReaderStateWriteKind::Theme
    } else if ext.eq_ignore_ascii_case(reader_state::META_RECORD_EXT.as_bytes()) {
        Phase39iReaderStateWriteKind::Metadata
    } else if ext.eq_ignore_ascii_case(b".BKM") {
        Phase39iReaderStateWriteKind::Bookmark
    } else {
        Phase39iReaderStateWriteKind::UnknownStateFile
    }
}

pub fn phase39i_marker() -> &'static str {
    PHASE_39I_ACTIVE_READER_SAVE_CALLSITE_WIRING_MARKER
}
