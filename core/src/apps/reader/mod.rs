//! Reader app-level contracts.
//!
//! Bootstrap Phase 1 intentionally moves persistence models into
//! `crate::models`. This module should grow reader behavior/state-machine code,
//! not own book identity, progress, bookmark, theme, or storage path formats.

pub use crate::models::{
    BookFormat, BookId, BookIdScheme, BookmarkIndexRecord, ReaderBookmark, ReaderMeta,
    ReaderProgress, ReaderStoragePaths, ReaderThemePreset, StorageLayout, StorageLayoutKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReaderOpenMode {
    Continue,
    FromLibrary,
    FromBookmark,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderOpenRequest {
    pub book_id: BookId,
    pub mode: ReaderOpenMode,
    pub progress: Option<ReaderProgress>,
}

impl ReaderOpenRequest {
    pub fn continue_from(progress: ReaderProgress) -> Self {
        Self {
            book_id: progress.book_id.clone(),
            mode: ReaderOpenMode::Continue,
            progress: Some(progress),
        }
    }

    pub fn from_library(book_id: BookId) -> Self {
        Self {
            book_id,
            mode: ReaderOpenMode::FromLibrary,
            progress: None,
        }
    }
}
