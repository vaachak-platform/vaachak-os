pub mod book_id;
pub mod book_identity;
pub mod bookmark;
pub mod progress;
pub mod reader_file;
pub mod reader_meta;
pub mod reader_runtime;
pub mod storage_layout;
pub mod theme;

pub use book_id::{BookId, BookIdScheme, FNV1A32_OFFSET, FNV1A32_PRIME, fnv1a32};
pub use book_identity::BookIdentity;
pub use bookmark::{BookmarkIndexRecord, ReaderBookmark};
pub use progress::ReaderProgress;
pub use reader_file::{
    LIBRARY_DISPLAY_NAME_MAX, LIBRARY_KIND_LABEL_MAX, LIBRARY_PATH_MAX, LibraryEntry,
    LibraryScanPolicy, ReaderFileKind,
};
pub use reader_meta::{BookFormat, ReaderMeta};
pub use reader_runtime::{ReaderNavAction, ReaderPageState, ReaderSessionState, ReaderUiMode};
pub use storage_layout::{ReaderStoragePaths, StorageLayout, StorageLayoutKind};
pub use theme::{ReaderThemePreset, ThemeContrast, ThemeKind};
