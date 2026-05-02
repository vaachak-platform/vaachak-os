#![allow(dead_code)]

use super::storage_state::{
    VaachakStateIoKind, VaachakStorageStateIo, VaachakStorageStateIoError, VaachakStorageStatePaths,
};
use crate::vaachak_x4::contracts::storage_path_helpers::VaachakStatePath;

pub trait VaachakStorageStatePathIo {
    type Error;

    fn read_state_path(
        &mut self,
        path: &VaachakStatePath,
        out: &mut [u8],
    ) -> Result<usize, Self::Error>;

    fn write_state_path(&mut self, path: &VaachakStatePath, data: &[u8])
    -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageStateAdapterError<E> {
    Contract(VaachakStorageStateIoError),
    Backend(E),
}

pub struct VaachakStorageStateIoAdapter<B> {
    backend: B,
}

impl<B> VaachakStorageStateIoAdapter<B> {
    pub const fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn into_backend(self) -> B {
        self.backend
    }
}

impl<B> VaachakStorageStateIo for VaachakStorageStateIoAdapter<B>
where
    B: VaachakStorageStatePathIo,
{
    type Error = VaachakStorageStateAdapterError<B::Error>;

    fn read_state(
        &mut self,
        book_id: &[u8],
        kind: VaachakStateIoKind,
        out: &mut [u8],
    ) -> Result<usize, Self::Error> {
        let path =
            VaachakStorageStatePaths::state_path(book_id, kind).map_err(Self::Error::Contract)?;
        self.backend
            .read_state_path(&path, out)
            .map_err(Self::Error::Backend)
    }

    fn write_state(
        &mut self,
        book_id: &[u8],
        kind: VaachakStateIoKind,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let path =
            VaachakStorageStatePaths::state_path(book_id, kind).map_err(Self::Error::Contract)?;
        self.backend
            .write_state_path(&path, data)
            .map_err(Self::Error::Backend)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        VaachakStorageStateAdapterError, VaachakStorageStateIoAdapter, VaachakStorageStatePathIo,
    };
    use crate::vaachak_x4::contracts::storage_path_helpers::VaachakStatePath;
    use crate::vaachak_x4::io::storage_state::{
        VaachakStateIoKind, VaachakStorageStateIo, VaachakStorageStateIoError,
    };

    struct ProbeBackend {
        last_path: VaachakStatePath,
    }

    impl Default for ProbeBackend {
        fn default() -> Self {
            Self {
                last_path: VaachakStatePath::empty(),
            }
        }
    }

    impl VaachakStorageStatePathIo for ProbeBackend {
        type Error = ();

        fn read_state_path(
            &mut self,
            path: &VaachakStatePath,
            _out: &mut [u8],
        ) -> Result<usize, Self::Error> {
            self.last_path = *path;
            Ok(0)
        }

        fn write_state_path(
            &mut self,
            path: &VaachakStatePath,
            _data: &[u8],
        ) -> Result<(), Self::Error> {
            self.last_path = *path;
            Ok(())
        }
    }

    #[test]
    fn adapter_resolves_semantic_kind_before_delegating() {
        let mut adapter = VaachakStorageStateIoAdapter::new(ProbeBackend::default());
        let mut out = [];

        assert_eq!(
            adapter.read_state(b"8A79A61F", VaachakStateIoKind::Bookmark, &mut out),
            Ok(0)
        );
        assert_eq!(
            adapter.backend().last_path.as_bytes(),
            b"state/8A79A61F.BKM"
        );
    }

    #[test]
    fn adapter_rejects_invalid_book_id_before_backend() {
        let mut adapter = VaachakStorageStateIoAdapter::new(ProbeBackend::default());
        let mut out = [];

        assert_eq!(
            adapter.read_state(b"8a79a61f", VaachakStateIoKind::Progress, &mut out),
            Err(VaachakStorageStateAdapterError::Contract(
                VaachakStorageStateIoError::InvalidBookId
            ))
        );
        assert!(adapter.backend().last_path.is_empty());
    }
}
