#![allow(dead_code)]

/// Vaachak-owned pure storage path helper layer.
///
/// The current implementation intentionally extracts only deterministic path/name logic.
/// Physical SD/SPI/filesystem IO remains owned by the imported Pulp runtime.
pub struct VaachakStoragePathHelpers;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct VaachakStoragePathAdoptionReport {
    pub state_dir_ok: bool,
    pub progress_path_ok: bool,
    pub bookmark_path_ok: bool,
    pub theme_path_ok: bool,
    pub metadata_path_ok: bool,
    pub bookmark_index_path_ok: bool,
    pub validation_helpers_ok: bool,
    pub physical_io_moved: bool,
}

impl VaachakStoragePathAdoptionReport {
    pub const fn adoption_ok(self) -> bool {
        self.state_dir_ok
            && self.progress_path_ok
            && self.bookmark_path_ok
            && self.theme_path_ok
            && self.metadata_path_ok
            && self.bookmark_index_path_ok
            && self.validation_helpers_ok
            && !self.physical_io_moved
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VaachakStateFileKind {
    Progress,
    Bookmark,
    Theme,
    Metadata,
}

impl VaachakStateFileKind {
    pub const fn extension(self) -> &'static [u8; 3] {
        match self {
            Self::Progress => b"PRG",
            Self::Bookmark => b"BKM",
            Self::Theme => b"THM",
            Self::Metadata => b"MTA",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct VaachakStatePath {
    bytes: [u8; VaachakStoragePathHelpers::MAX_STATE_PATH_LEN],
    len: usize,
}

impl VaachakStatePath {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; VaachakStoragePathHelpers::MAX_STATE_PATH_LEN],
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(self.as_bytes()).ok()
    }
}

impl VaachakStoragePathHelpers {
    pub const IMPLEMENTATION_OWNER: &'static str = "Vaachak-owned pure path helpers";
    pub const PHYSICAL_IO_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const STORAGE_PATH_HELPERS_MARKER: &'static str = "x4-storage-path-helpers-ok";
    pub const STORAGE_PATH_ADOPTION_CHECK: &'static str = "storage-path-helper-adoption";
    pub const PHYSICAL_STORAGE_IO_MOVED_TO_BOUNDARY: bool = false;

    pub const STATE_DIR: &'static [u8] = b"state";
    pub const STATE_DIR_STR: &'static str = "state";
    pub const PATH_SEPARATOR: u8 = b'/';
    pub const BOOK_ID_LEN: usize = 8;
    pub const EXT_LEN: usize = 3;
    pub const STATE_FILE_NAME_LEN: usize = 12; // 8.3: XXXXXXXX.EXT
    pub const MAX_STATE_PATH_LEN: usize = 24; // state/ + 8.3 + safety margin

    pub const BOOKMARK_INDEX_FILE: &'static str = "BMIDX.TXT";
    pub const PROGRESS_EXTENSION: &'static str = "PRG";
    pub const BOOKMARK_EXTENSION: &'static str = "BKM";
    pub const THEME_EXTENSION: &'static str = "THM";
    pub const METADATA_EXTENSION: &'static str = "MTA";

    /// Validation marker. This is the only validation marker emitted by the
    /// Vaachak facade after previous development-development logging is quieted.
    #[cfg(target_arch = "riscv32")]
    pub fn emit_storage_path_helpers_marker() {
        esp_println::println!("{}", Self::STORAGE_PATH_HELPERS_MARKER);
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_storage_path_helpers_marker() {}

    pub fn smoke_ok() -> bool {
        Self::is_valid_book_id(b"8A79A61F")
            && Self::is_supported_state_extension(b"PRG")
            && Self::is_supported_state_extension(b"BKM")
            && Self::is_supported_state_extension(b"THM")
            && Self::is_supported_state_extension(b"MTA")
            && Self::is_reserved_state_file(b"BMIDX.TXT")
            && Self::state_file_name(*b"8A79A61F", VaachakStateFileKind::Progress).as_bytes()
                == b"8A79A61F.PRG"
    }

    pub fn storage_path_adoption_report() -> VaachakStoragePathAdoptionReport {
        let book_id = *b"8A79A61F";

        VaachakStoragePathAdoptionReport {
            state_dir_ok: Self::STATE_DIR == b"state" && Self::STATE_DIR_STR == "state",
            progress_path_ok: Self::progress_path(book_id).as_bytes() == b"state/8A79A61F.PRG",
            bookmark_path_ok: Self::bookmark_path(book_id).as_bytes() == b"state/8A79A61F.BKM",
            theme_path_ok: Self::theme_path(book_id).as_bytes() == b"state/8A79A61F.THM",
            metadata_path_ok: Self::metadata_path(book_id).as_bytes() == b"state/8A79A61F.MTA",
            bookmark_index_path_ok: Self::bookmark_index_path().as_bytes() == b"state/BMIDX.TXT",
            validation_helpers_ok: Self::is_valid_upper_book_id(b"8A79A61F")
                && !Self::is_valid_upper_book_id(b"8a79a61f")
                && Self::is_supported_state_extension(b"prg")
                && Self::is_reserved_state_file(b"bMiDx.TxT"),
            physical_io_moved: Self::PHYSICAL_STORAGE_IO_MOVED_TO_BOUNDARY,
        }
    }

    pub fn active_runtime_adoption_probe() -> bool {
        Self::storage_path_adoption_report().adoption_ok()
    }

    pub fn state_file_name(
        book_id: [u8; Self::BOOK_ID_LEN],
        kind: VaachakStateFileKind,
    ) -> VaachakStatePath {
        let mut out = VaachakStatePath::empty();
        out.bytes[..Self::BOOK_ID_LEN].copy_from_slice(&book_id);
        out.bytes[Self::BOOK_ID_LEN] = b'.';
        let ext = kind.extension();
        out.bytes[Self::BOOK_ID_LEN + 1..Self::BOOK_ID_LEN + 1 + Self::EXT_LEN]
            .copy_from_slice(ext);
        out.len = Self::STATE_FILE_NAME_LEN;
        out
    }

    pub fn state_file_name_from_str(
        book_id: &str,
        kind: VaachakStateFileKind,
    ) -> Option<VaachakStatePath> {
        let bytes = book_id.as_bytes();
        if !Self::is_valid_upper_book_id(bytes) {
            return None;
        }

        let mut id = [0u8; Self::BOOK_ID_LEN];
        id.copy_from_slice(bytes);
        Some(Self::state_file_name(id, kind))
    }

    pub fn state_path(
        book_id: [u8; Self::BOOK_ID_LEN],
        kind: VaachakStateFileKind,
    ) -> VaachakStatePath {
        let file = Self::state_file_name(book_id, kind);
        let mut out = VaachakStatePath::empty();
        let mut pos = 0usize;

        out.bytes[pos..pos + Self::STATE_DIR.len()].copy_from_slice(Self::STATE_DIR);
        pos += Self::STATE_DIR.len();
        out.bytes[pos] = Self::PATH_SEPARATOR;
        pos += 1;
        out.bytes[pos..pos + file.len()].copy_from_slice(file.as_bytes());
        pos += file.len();
        out.len = pos;
        out
    }

    pub fn progress_path(book_id: [u8; Self::BOOK_ID_LEN]) -> VaachakStatePath {
        Self::state_path(book_id, VaachakStateFileKind::Progress)
    }

    pub fn bookmark_path(book_id: [u8; Self::BOOK_ID_LEN]) -> VaachakStatePath {
        Self::state_path(book_id, VaachakStateFileKind::Bookmark)
    }

    pub fn theme_path(book_id: [u8; Self::BOOK_ID_LEN]) -> VaachakStatePath {
        Self::state_path(book_id, VaachakStateFileKind::Theme)
    }

    pub fn metadata_path(book_id: [u8; Self::BOOK_ID_LEN]) -> VaachakStatePath {
        Self::state_path(book_id, VaachakStateFileKind::Metadata)
    }

    pub fn bookmark_index_path() -> VaachakStatePath {
        let mut out = VaachakStatePath::empty();
        let mut pos = 0usize;
        out.bytes[pos..pos + Self::STATE_DIR.len()].copy_from_slice(Self::STATE_DIR);
        pos += Self::STATE_DIR.len();
        out.bytes[pos] = Self::PATH_SEPARATOR;
        pos += 1;
        out.bytes[pos..pos + Self::BOOKMARK_INDEX_FILE.len()]
            .copy_from_slice(Self::BOOKMARK_INDEX_FILE.as_bytes());
        pos += Self::BOOKMARK_INDEX_FILE.len();
        out.len = pos;
        out
    }

    pub fn is_valid_book_id(bytes: &[u8]) -> bool {
        bytes.len() == Self::BOOK_ID_LEN && bytes.iter().copied().all(Self::is_hex_ascii)
    }

    pub fn is_valid_upper_book_id(bytes: &[u8]) -> bool {
        bytes.len() == Self::BOOK_ID_LEN && bytes.iter().copied().all(Self::is_upper_hex_ascii)
    }

    pub fn is_supported_state_extension(bytes: &[u8]) -> bool {
        bytes.eq_ignore_ascii_case(Self::PROGRESS_EXTENSION.as_bytes())
            || bytes.eq_ignore_ascii_case(Self::BOOKMARK_EXTENSION.as_bytes())
            || bytes.eq_ignore_ascii_case(Self::THEME_EXTENSION.as_bytes())
            || bytes.eq_ignore_ascii_case(Self::METADATA_EXTENSION.as_bytes())
    }

    pub fn is_reserved_state_file(bytes: &[u8]) -> bool {
        bytes.eq_ignore_ascii_case(Self::BOOKMARK_INDEX_FILE.as_bytes())
    }

    const fn is_hex_ascii(b: u8) -> bool {
        b.is_ascii_hexdigit()
    }

    const fn is_upper_hex_ascii(b: u8) -> bool {
        b.is_ascii_digit() || (b >= b'A' && b <= b'F')
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakStateFileKind, VaachakStoragePathHelpers};

    #[test]
    fn storage_path_adoption_probe_uses_pure_helpers() {
        assert!(VaachakStoragePathHelpers::active_runtime_adoption_probe());
    }

    #[test]
    fn builds_expected_state_paths() {
        let book_id = *b"8A79A61F";
        assert_eq!(
            VaachakStoragePathHelpers::progress_path(book_id).as_bytes(),
            b"state/8A79A61F.PRG"
        );
        assert_eq!(
            VaachakStoragePathHelpers::bookmark_path(book_id).as_bytes(),
            b"state/8A79A61F.BKM"
        );
        assert_eq!(
            VaachakStoragePathHelpers::theme_path(book_id).as_bytes(),
            b"state/8A79A61F.THM"
        );
        assert_eq!(
            VaachakStoragePathHelpers::metadata_path(book_id).as_bytes(),
            b"state/8A79A61F.MTA"
        );
        assert_eq!(
            VaachakStoragePathHelpers::bookmark_index_path().as_bytes(),
            b"state/BMIDX.TXT"
        );
    }

    #[test]
    fn validates_uppercase_state_record_ids() {
        assert!(VaachakStoragePathHelpers::is_valid_upper_book_id(
            b"8A79A61F"
        ));
        assert!(!VaachakStoragePathHelpers::is_valid_upper_book_id(
            b"8a79a61f"
        ));
        assert_eq!(
            VaachakStoragePathHelpers::state_file_name_from_str(
                "8A79A61F",
                VaachakStateFileKind::Progress
            )
            .unwrap()
            .as_bytes(),
            b"8A79A61F.PRG"
        );
    }
}
