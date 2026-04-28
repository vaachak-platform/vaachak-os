use heapless::Vec;
use vaachak_core::hal::{DirEntry, OpenMode, StorageError, StorageHal};

#[derive(Default)]
pub struct X4Storage;

pub struct X4File;
pub struct X4Dir;

impl StorageHal for X4Storage {
    type File = X4File;
    type Dir = X4Dir;

    fn sd_present(&self) -> bool { false }

    fn open(&mut self, _path: &str, _mode: OpenMode) -> Result<Self::File, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn open_dir(&mut self, _path: &str) -> Result<Self::Dir, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn mkdir(&mut self, _path: &str) -> Result<(), StorageError> {
        Err(StorageError::Unsupported)
    }

    fn remove(&mut self, _path: &str) -> Result<(), StorageError> {
        Err(StorageError::Unsupported)
    }

    fn exists(&self, _path: &str) -> bool { false }

    fn nvs_get(&self, _key: &str, _buf: &mut [u8]) -> Result<usize, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn nvs_set(&mut self, _key: &str, _val: &[u8]) -> Result<(), StorageError> {
        Err(StorageError::Unsupported)
    }
}
