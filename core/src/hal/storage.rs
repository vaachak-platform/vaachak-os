use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
    Create,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirEntry {
    pub name: heapless::String<64>,
    pub is_dir: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageError {
    NotFound,
    Io,
    Unsupported,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::NotFound => f.write_str("not found"),
            StorageError::Io => f.write_str("i/o error"),
            StorageError::Unsupported => f.write_str("unsupported"),
        }
    }
}

pub trait StorageHal {
    type File;
    type Dir;

    fn sd_present(&self) -> bool;
    fn open(&mut self, path: &str, mode: OpenMode) -> Result<Self::File, StorageError>;
    fn open_dir(&mut self, path: &str) -> Result<Self::Dir, StorageError>;
    fn mkdir(&mut self, path: &str) -> Result<(), StorageError>;
    fn remove(&mut self, path: &str) -> Result<(), StorageError>;
    fn exists(&self, path: &str) -> bool;

    fn nvs_get(&self, key: &str, buf: &mut [u8]) -> Result<usize, StorageError>;
    fn nvs_set(&mut self, key: &str, val: &[u8]) -> Result<(), StorageError>;
}
