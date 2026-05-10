#![allow(dead_code)]

use crate::vaachak_x4::contracts::storage_path_helpers::{
    VaachakStateFileKind, VaachakStatePath, VaachakStoragePathHelpers,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStateIoKind {
    Progress,
    Bookmark,
    Theme,
    Metadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageStateIoError {
    InvalidBookId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageStateIoSeamReport {
    pub progress_kind_ok: bool,
    pub bookmark_kind_ok: bool,
    pub theme_kind_ok: bool,
    pub metadata_kind_ok: bool,
    pub path_helpers_used: bool,
    pub physical_storage_io_owned: bool,
    pub reader_cache_io_owned: bool,
}

impl VaachakStorageStateIoSeamReport {
    pub const fn seam_ok(self) -> bool {
        self.progress_kind_ok
            && self.bookmark_kind_ok
            && self.theme_kind_ok
            && self.metadata_kind_ok
            && self.path_helpers_used
            && !self.physical_storage_io_owned
            && !self.reader_cache_io_owned
    }
}

pub trait VaachakStorageStateIo {
    type Error;

    fn read_state(
        &mut self,
        book_id: &[u8],
        kind: VaachakStateIoKind,
        out: &mut [u8],
    ) -> Result<usize, Self::Error>;

    fn write_state(
        &mut self,
        book_id: &[u8],
        kind: VaachakStateIoKind,
        data: &[u8],
    ) -> Result<(), Self::Error>;
}

pub struct VaachakStorageStatePaths;

impl VaachakStateIoKind {
    pub const fn as_file_kind(self) -> VaachakStateFileKind {
        match self {
            Self::Progress => VaachakStateFileKind::Progress,
            Self::Bookmark => VaachakStateFileKind::Bookmark,
            Self::Theme => VaachakStateFileKind::Theme,
            Self::Metadata => VaachakStateFileKind::Metadata,
        }
    }
}

impl VaachakStorageStatePaths {
    pub const IMPLEMENTATION_OWNER: &'static str = "Vaachak-owned storage state IO seam";
    pub const PHYSICAL_STORAGE_IO_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const READER_CACHE_IO_OWNER: &'static str = "Vaachak-owned X4 runtime";
    pub const PHYSICAL_STORAGE_IO_OWNED_BY_BRIDGE: bool = false;
    pub const READER_CACHE_IO_OWNED_BY_BRIDGE: bool = false;

    pub fn state_path(
        book_id: &[u8],
        kind: VaachakStateIoKind,
    ) -> Result<VaachakStatePath, VaachakStorageStateIoError> {
        if !VaachakStoragePathHelpers::is_valid_upper_book_id(book_id) {
            return Err(VaachakStorageStateIoError::InvalidBookId);
        }

        let mut normalized = [0u8; VaachakStoragePathHelpers::BOOK_ID_LEN];
        normalized.copy_from_slice(book_id);

        Ok(VaachakStoragePathHelpers::state_path(
            normalized,
            kind.as_file_kind(),
        ))
    }

    pub fn seam_report() -> VaachakStorageStateIoSeamReport {
        let book_id = b"8A79A61F";

        VaachakStorageStateIoSeamReport {
            progress_kind_ok: Self::state_path(book_id, VaachakStateIoKind::Progress)
                .is_ok_and(|path| path.as_bytes() == b"state/8A79A61F.PRG"),
            bookmark_kind_ok: Self::state_path(book_id, VaachakStateIoKind::Bookmark)
                .is_ok_and(|path| path.as_bytes() == b"state/8A79A61F.BKM"),
            theme_kind_ok: Self::state_path(book_id, VaachakStateIoKind::Theme)
                .is_ok_and(|path| path.as_bytes() == b"state/8A79A61F.THM"),
            metadata_kind_ok: Self::state_path(book_id, VaachakStateIoKind::Metadata)
                .is_ok_and(|path| path.as_bytes() == b"state/8A79A61F.MTA"),
            path_helpers_used: VaachakStoragePathHelpers::active_runtime_adoption_probe(),
            physical_storage_io_owned: Self::PHYSICAL_STORAGE_IO_OWNED_BY_BRIDGE,
            reader_cache_io_owned: Self::READER_CACHE_IO_OWNED_BY_BRIDGE,
        }
    }

    pub fn seam_ok() -> bool {
        Self::seam_report().seam_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakStateIoKind, VaachakStorageStateIoError, VaachakStorageStatePaths};

    #[test]
    fn resolves_state_kinds_through_path_helpers() {
        let book_id = b"8A79A61F";

        assert_eq!(
            VaachakStorageStatePaths::state_path(book_id, VaachakStateIoKind::Progress)
                .unwrap()
                .as_bytes(),
            b"state/8A79A61F.PRG"
        );
        assert_eq!(
            VaachakStorageStatePaths::state_path(book_id, VaachakStateIoKind::Bookmark)
                .unwrap()
                .as_bytes(),
            b"state/8A79A61F.BKM"
        );
        assert_eq!(
            VaachakStorageStatePaths::state_path(book_id, VaachakStateIoKind::Theme)
                .unwrap()
                .as_bytes(),
            b"state/8A79A61F.THM"
        );
        assert_eq!(
            VaachakStorageStatePaths::state_path(book_id, VaachakStateIoKind::Metadata)
                .unwrap()
                .as_bytes(),
            b"state/8A79A61F.MTA"
        );
    }

    #[test]
    fn rejects_non_contract_book_ids() {
        assert_eq!(
            VaachakStorageStatePaths::state_path(b"8a79a61f", VaachakStateIoKind::Progress),
            Err(VaachakStorageStateIoError::InvalidBookId)
        );
        assert_eq!(
            VaachakStorageStatePaths::state_path(b"TOO-SHORT", VaachakStateIoKind::Progress),
            Err(VaachakStorageStateIoError::InvalidBookId)
        );
    }

    #[test]
    fn seam_probe_does_not_claim_physical_io() {
        assert!(VaachakStorageStatePaths::seam_ok());
    }
}
