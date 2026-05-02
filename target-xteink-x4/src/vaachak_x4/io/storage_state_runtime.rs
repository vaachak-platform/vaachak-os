#![allow(dead_code)]

use super::reader_state_runtime::VaachakReaderStateRuntimeBridge;
use super::storage_state::{VaachakStateIoKind, VaachakStorageStateIo, VaachakStorageStatePaths};
use super::storage_state_adapter::{VaachakStorageStateIoAdapter, VaachakStorageStatePathIo};
use crate::vaachak_x4::contracts::storage_path_helpers::VaachakStatePath;

pub struct VaachakStorageStateRuntimeBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageStateRuntimePreflightReport {
    pub progress_path_ok: bool,
    pub bookmark_path_ok: bool,
    pub theme_path_ok: bool,
    pub metadata_path_ok: bool,
    pub adapter_probe_ok: bool,
    pub physical_storage_io_owned: bool,
}

impl VaachakStorageStateRuntimePreflightReport {
    pub const fn preflight_ok(self) -> bool {
        self.progress_path_ok
            && self.bookmark_path_ok
            && self.theme_path_ok
            && self.metadata_path_ok
            && self.adapter_probe_ok
            && !self.physical_storage_io_owned
    }
}

struct VaachakStorageStatePathProbe {
    observed: [bool; 4],
}

impl VaachakStorageStatePathProbe {
    const fn new() -> Self {
        Self {
            observed: [false; 4],
        }
    }

    const fn observed_all(&self) -> bool {
        self.observed[0] && self.observed[1] && self.observed[2] && self.observed[3]
    }

    fn observe(&mut self, path: &VaachakStatePath) {
        match path.as_bytes() {
            b"state/8A79A61F.PRG" => self.observed[0] = true,
            b"state/8A79A61F.BKM" => self.observed[1] = true,
            b"state/8A79A61F.THM" => self.observed[2] = true,
            b"state/8A79A61F.MTA" => self.observed[3] = true,
            _ => {}
        }
    }
}

impl VaachakStorageStatePathIo for VaachakStorageStatePathProbe {
    type Error = core::convert::Infallible;

    fn read_state_path(
        &mut self,
        path: &VaachakStatePath,
        _out: &mut [u8],
    ) -> Result<usize, Self::Error> {
        self.observe(path);
        Ok(0)
    }

    fn write_state_path(
        &mut self,
        path: &VaachakStatePath,
        _data: &[u8],
    ) -> Result<(), Self::Error> {
        self.observe(path);
        Ok(())
    }
}

impl VaachakStorageStateRuntimeBridge {
    pub const PHYSICAL_STORAGE_IO_OWNED_IN_PHASE35B: bool = false;
    pub const PRE_HEAP_RUNTIME_PREFLIGHT_ALLOCATES: bool = false;
    pub const ALLOC_RUNTIME_PREFLIGHT_REQUIRES_HEAP: bool = true;

    pub fn active_runtime_preflight() -> bool {
        Self::preflight_report().preflight_ok()
    }

    pub fn active_runtime_alloc_preflight() -> bool {
        VaachakReaderStateRuntimeBridge::active_runtime_preflight()
    }

    pub fn preflight_report() -> VaachakStorageStateRuntimePreflightReport {
        let book_id = b"8A79A61F";
        let mut out = [];
        let mut adapter = VaachakStorageStateIoAdapter::new(VaachakStorageStatePathProbe::new());

        let progress_adapter_ok = adapter
            .read_state(book_id, VaachakStateIoKind::Progress, &mut out)
            .is_ok();
        let bookmark_adapter_ok = adapter
            .read_state(book_id, VaachakStateIoKind::Bookmark, &mut out)
            .is_ok();
        let theme_adapter_ok = adapter
            .write_state(book_id, VaachakStateIoKind::Theme, &[])
            .is_ok();
        let metadata_adapter_ok = adapter
            .write_state(book_id, VaachakStateIoKind::Metadata, &[])
            .is_ok();

        VaachakStorageStateRuntimePreflightReport {
            progress_path_ok: Self::path_matches(
                book_id,
                VaachakStateIoKind::Progress,
                b"state/8A79A61F.PRG",
            ),
            bookmark_path_ok: Self::path_matches(
                book_id,
                VaachakStateIoKind::Bookmark,
                b"state/8A79A61F.BKM",
            ),
            theme_path_ok: Self::path_matches(
                book_id,
                VaachakStateIoKind::Theme,
                b"state/8A79A61F.THM",
            ),
            metadata_path_ok: Self::path_matches(
                book_id,
                VaachakStateIoKind::Metadata,
                b"state/8A79A61F.MTA",
            ),
            adapter_probe_ok: progress_adapter_ok
                && bookmark_adapter_ok
                && theme_adapter_ok
                && metadata_adapter_ok
                && adapter.backend().observed_all(),
            physical_storage_io_owned: Self::PHYSICAL_STORAGE_IO_OWNED_IN_PHASE35B,
        }
    }

    fn path_matches(book_id: &[u8], kind: VaachakStateIoKind, expected: &[u8]) -> bool {
        VaachakStorageStatePaths::state_path(book_id, kind)
            .is_ok_and(|path| path.as_bytes() == expected)
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageStateRuntimeBridge;

    #[test]
    fn active_runtime_preflight_is_path_only_and_valid() {
        assert!(VaachakStorageStateRuntimeBridge::active_runtime_preflight());
    }
}
