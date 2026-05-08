#![allow(dead_code)]

//! Pulp-backed read-only implementation bridge for Vaachak storage facade.
//!
//! This module adds an adapter boundary only. The generic bridge is pure and can
//! be checked without SD hardware. The embedded X4 backend, enabled for the
//! active firmware target, delegates read/list/size calls to the existing Pulp
//! storage helpers in `x4_kernel::drivers::storage`.
//!
//! It intentionally does not mount or probe SD, arbitrate SPI, drive FAT
//! directly, refresh display hardware, or change reader/file-browser behavior.

use crate::vaachak_x4::io::storage_readonly_adapter::{
    VaachakDirectoryEntry, VaachakDirectoryEntryKind, VaachakReadonlyStorage,
    VaachakReadonlyStorageContractError, VaachakResolvedStoragePaths, VaachakStoragePathRef,
    VaachakStorageReadChunk,
};

pub const STORAGE_READONLY_PULP_BRIDGE_MARKER: &str = "x4-storage-readonly-pulp-bridge-ok";
pub const PULP_READONLY_BRIDGE_OWNER: &str = "target-xteink-x4 Vaachak adapter bridge";
pub const PULP_READONLY_ACTIVE_BACKEND_OWNER: &str = "vendor/pulp-os imported runtime";
pub const PULP_READONLY_MAX_DIR_ENTRIES: usize = 32;
pub const PULP_READONLY_X4_DIR: &str = "_x4";

pub struct VaachakStorageReadonlyPulpBridgeContract;

impl VaachakStorageReadonlyPulpBridgeContract {
    pub const BRIDGE_MARKER: &'static str = STORAGE_READONLY_PULP_BRIDGE_MARKER;
    pub const BRIDGE_OWNER: &'static str = PULP_READONLY_BRIDGE_OWNER;
    pub const ACTIVE_BACKEND_OWNER: &'static str = PULP_READONLY_ACTIVE_BACKEND_OWNER;

    pub const FILE_EXISTS_MAPPING: &'static str = "file_exists -> Pulp file_size helpers";
    pub const READ_FILE_START_MAPPING: &'static str =
        "read_file_start -> Pulp read_file_start helpers";
    pub const READ_CHUNK_MAPPING: &'static str = "read_chunk -> Pulp read_file_chunk helpers";
    pub const LIST_DIRECTORY_MAPPING: &'static str =
        "list_directory_metadata -> Pulp list_*_entries helpers";
    pub const RESOLVE_PATHS_MAPPING: &'static str =
        "resolve_current_storage_paths -> facade PULP_BACKED_ACTIVE_PATHS";

    pub const SD_MOUNT_OR_PROBE_MOVED_TO_BRIDGE: bool = false;
    pub const SD_DRIVER_MOVED_TO_BRIDGE: bool = false;
    pub const FAT_BEHAVIOR_MOVED_TO_BRIDGE: bool = false;
    pub const SPI_ARBITRATION_MOVED_TO_BRIDGE: bool = false;
    pub const DISPLAY_BEHAVIOR_MOVED_TO_BRIDGE: bool = false;
    pub const READER_OR_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;
    pub const WRITABLE_STORAGE_BEHAVIOR_ADDED: bool = false;

    pub const fn physical_behavior_moved() -> bool {
        Self::SD_MOUNT_OR_PROBE_MOVED_TO_BRIDGE
            || Self::SD_DRIVER_MOVED_TO_BRIDGE
            || Self::FAT_BEHAVIOR_MOVED_TO_BRIDGE
            || Self::SPI_ARBITRATION_MOVED_TO_BRIDGE
            || Self::DISPLAY_BEHAVIOR_MOVED_TO_BRIDGE
            || Self::READER_OR_FILE_BROWSER_BEHAVIOR_CHANGED
            || Self::WRITABLE_STORAGE_BEHAVIOR_ADDED
    }

    pub const fn active_runtime_preflight() -> StorageReadonlyPulpBridgePreflight {
        StorageReadonlyPulpBridgePreflight {
            bridge_marker_present: !Self::BRIDGE_MARKER.is_empty(),
            active_backend_is_pulp: !Self::ACTIVE_BACKEND_OWNER.is_empty(),
            physical_behavior_moved: Self::physical_behavior_moved(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageReadonlyPulpBridgePreflight {
    pub bridge_marker_present: bool,
    pub active_backend_is_pulp: bool,
    pub physical_behavior_moved: bool,
}

impl StorageReadonlyPulpBridgePreflight {
    pub const fn ok(self) -> bool {
        self.bridge_marker_present && self.active_backend_is_pulp && !self.physical_behavior_moved
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PulpReadonlyStorageBridgeError<E> {
    Contract(VaachakReadonlyStorageContractError),
    Backend(E),
    NonUtf8Path,
    UnsupportedPath,
    NotFile,
    NotDirectory,
    OffsetTooLarge,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PulpReadonlyDirectoryEntry {
    pub name: [u8; VaachakDirectoryEntry::MAX_NAME_LEN],
    pub name_len: usize,
    pub is_dir: bool,
    pub size_bytes: u64,
}

impl PulpReadonlyDirectoryEntry {
    pub const EMPTY: Self = Self {
        name: [0; VaachakDirectoryEntry::MAX_NAME_LEN],
        name_len: 0,
        is_dir: false,
        size_bytes: 0,
    };

    pub fn from_name(name: &[u8], is_dir: bool, size_bytes: u64) -> Option<Self> {
        if name.is_empty() || name.len() > VaachakDirectoryEntry::MAX_NAME_LEN {
            return None;
        }

        let mut out = Self::EMPTY;
        out.name[..name.len()].copy_from_slice(name);
        out.name_len = name.len();
        out.is_dir = is_dir;
        out.size_bytes = size_bytes;
        Some(out)
    }

    pub fn name_bytes(&self) -> &[u8] {
        &self.name[..self.name_len]
    }

    pub fn to_vaachak_entry(self) -> Option<VaachakDirectoryEntry> {
        let kind = if self.is_dir {
            VaachakDirectoryEntryKind::Directory
        } else {
            VaachakDirectoryEntryKind::File
        };
        let size = if self.is_dir {
            None
        } else {
            Some(self.size_bytes)
        };
        VaachakDirectoryEntry::from_name(self.name_bytes(), kind, size)
    }
}

/// Minimal backend contract consumed by the bridge.
///
/// The active embedded implementation delegates to Pulp storage helpers. Tests
/// and host/static checks can use a fake backend without constructing SD
/// hardware, mounted FAT state, SPI, or display objects.
pub trait PulpReadonlyStorageBackend {
    type Error;

    fn file_size_root(&mut self, name: &str) -> Result<u32, Self::Error>;
    fn file_size_in_dir(&mut self, dir: &str, name: &str) -> Result<u32, Self::Error>;
    fn file_size_in_subdir(
        &mut self,
        dir: &str,
        subdir: &str,
        name: &str,
    ) -> Result<u32, Self::Error>;

    fn read_file_start_root(
        &mut self,
        name: &str,
        out: &mut [u8],
    ) -> Result<(u32, usize), Self::Error>;
    fn read_file_start_in_dir(
        &mut self,
        dir: &str,
        name: &str,
        out: &mut [u8],
    ) -> Result<(u32, usize), Self::Error>;
    fn read_file_start_in_subdir(
        &mut self,
        dir: &str,
        subdir: &str,
        name: &str,
        out: &mut [u8],
    ) -> Result<(u32, usize), Self::Error>;

    fn read_file_chunk_root(
        &mut self,
        name: &str,
        offset: u32,
        out: &mut [u8],
    ) -> Result<usize, Self::Error>;
    fn read_file_chunk_in_dir(
        &mut self,
        dir: &str,
        name: &str,
        offset: u32,
        out: &mut [u8],
    ) -> Result<usize, Self::Error>;
    fn read_file_chunk_in_subdir(
        &mut self,
        dir: &str,
        subdir: &str,
        name: &str,
        offset: u32,
        out: &mut [u8],
    ) -> Result<usize, Self::Error>;

    fn list_root_entries(
        &mut self,
        out: &mut [PulpReadonlyDirectoryEntry],
    ) -> Result<usize, Self::Error>;
    fn list_dir_entries(
        &mut self,
        dir: &str,
        out: &mut [PulpReadonlyDirectoryEntry],
    ) -> Result<usize, Self::Error>;
    fn list_subdir_entries(
        &mut self,
        dir: &str,
        subdir: &str,
        out: &mut [PulpReadonlyDirectoryEntry],
    ) -> Result<usize, Self::Error>;
}

pub struct PulpReadonlyStorageBridge<B> {
    backend: B,
}

impl<B> PulpReadonlyStorageBridge<B> {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PulpReadonlyPath<'a> {
    Root,
    RootFile(&'a str),
    RootDirectory(&'a str),
    DirFile {
        dir: &'a str,
        name: &'a str,
    },
    Subdir {
        dir: &'a str,
        subdir: &'a str,
    },
    SubdirFile {
        dir: &'a str,
        subdir: &'a str,
        name: &'a str,
    },
}

fn normalize_pulp_dir_name(name: &str) -> &str {
    if name.eq_ignore_ascii_case("_x4") {
        PULP_READONLY_X4_DIR
    } else {
        name
    }
}

fn parse_storage_path<'a, E>(
    path: VaachakStoragePathRef<'a>,
    for_directory: bool,
) -> Result<PulpReadonlyPath<'a>, PulpReadonlyStorageBridgeError<E>> {
    let path = path
        .as_str()
        .ok_or(PulpReadonlyStorageBridgeError::NonUtf8Path)?;
    let rest = path.strip_prefix('/').unwrap_or(path);

    if rest.is_empty() {
        return if for_directory {
            Ok(PulpReadonlyPath::Root)
        } else {
            Err(PulpReadonlyStorageBridgeError::NotFile)
        };
    }

    let mut parts = [""; 3];
    let mut count = 0usize;
    for part in rest.split('/') {
        if part.is_empty() {
            return Err(PulpReadonlyStorageBridgeError::UnsupportedPath);
        }
        if count >= parts.len() {
            return Err(PulpReadonlyStorageBridgeError::UnsupportedPath);
        }
        parts[count] = part;
        count += 1;
    }

    match (count, for_directory) {
        (1, true) => Ok(PulpReadonlyPath::RootDirectory(normalize_pulp_dir_name(
            parts[0],
        ))),
        (1, false) => Ok(PulpReadonlyPath::RootFile(parts[0])),
        (2, true) => Ok(PulpReadonlyPath::Subdir {
            dir: normalize_pulp_dir_name(parts[0]),
            subdir: parts[1],
        }),
        (2, false) => Ok(PulpReadonlyPath::DirFile {
            dir: normalize_pulp_dir_name(parts[0]),
            name: parts[1],
        }),
        (3, true) => Err(PulpReadonlyStorageBridgeError::NotDirectory),
        (3, false) => Ok(PulpReadonlyPath::SubdirFile {
            dir: normalize_pulp_dir_name(parts[0]),
            subdir: parts[1],
            name: parts[2],
        }),
        _ => Err(PulpReadonlyStorageBridgeError::UnsupportedPath),
    }
}

impl<B> PulpReadonlyStorageBridge<B>
where
    B: PulpReadonlyStorageBackend,
{
    fn file_size(
        &mut self,
        path: PulpReadonlyPath<'_>,
    ) -> Result<u32, PulpReadonlyStorageBridgeError<B::Error>> {
        match path {
            PulpReadonlyPath::RootFile(name) => self
                .backend
                .file_size_root(name)
                .map_err(PulpReadonlyStorageBridgeError::Backend),
            PulpReadonlyPath::DirFile { dir, name } => self
                .backend
                .file_size_in_dir(dir, name)
                .map_err(PulpReadonlyStorageBridgeError::Backend),
            PulpReadonlyPath::SubdirFile { dir, subdir, name } => self
                .backend
                .file_size_in_subdir(dir, subdir, name)
                .map_err(PulpReadonlyStorageBridgeError::Backend),
            PulpReadonlyPath::Root
            | PulpReadonlyPath::RootDirectory(_)
            | PulpReadonlyPath::Subdir { .. } => Err(PulpReadonlyStorageBridgeError::NotFile),
        }
    }

    fn read_start(
        &mut self,
        path: PulpReadonlyPath<'_>,
        out: &mut [u8],
    ) -> Result<VaachakStorageReadChunk, PulpReadonlyStorageBridgeError<B::Error>> {
        if out.is_empty() {
            return Err(PulpReadonlyStorageBridgeError::Contract(
                VaachakReadonlyStorageContractError::EmptyOutputBuffer,
            ));
        }

        let (size, bytes_read) = match path {
            PulpReadonlyPath::RootFile(name) => self.backend.read_file_start_root(name, out),
            PulpReadonlyPath::DirFile { dir, name } => {
                self.backend.read_file_start_in_dir(dir, name, out)
            }
            PulpReadonlyPath::SubdirFile { dir, subdir, name } => self
                .backend
                .read_file_start_in_subdir(dir, subdir, name, out),
            PulpReadonlyPath::Root
            | PulpReadonlyPath::RootDirectory(_)
            | PulpReadonlyPath::Subdir { .. } => {
                return Err(PulpReadonlyStorageBridgeError::NotFile);
            }
        }
        .map_err(PulpReadonlyStorageBridgeError::Backend)?;

        Ok(VaachakStorageReadChunk::from_read(
            0,
            bytes_read,
            bytes_read as u64 >= size as u64,
        ))
    }

    fn read_at(
        &mut self,
        path: PulpReadonlyPath<'_>,
        offset: u64,
        out: &mut [u8],
    ) -> Result<VaachakStorageReadChunk, PulpReadonlyStorageBridgeError<B::Error>> {
        if offset > u32::MAX as u64 {
            return Err(PulpReadonlyStorageBridgeError::OffsetTooLarge);
        }
        if out.is_empty() {
            return Err(PulpReadonlyStorageBridgeError::Contract(
                VaachakReadonlyStorageContractError::EmptyOutputBuffer,
            ));
        }

        let file_size = self.file_size(path)?;
        if offset >= file_size as u64 {
            return Ok(VaachakStorageReadChunk::empty_at(offset, true));
        }

        let offset32 = offset as u32;
        let bytes_read = match path {
            PulpReadonlyPath::RootFile(name) => {
                self.backend.read_file_chunk_root(name, offset32, out)
            }
            PulpReadonlyPath::DirFile { dir, name } => self
                .backend
                .read_file_chunk_in_dir(dir, name, offset32, out),
            PulpReadonlyPath::SubdirFile { dir, subdir, name } => self
                .backend
                .read_file_chunk_in_subdir(dir, subdir, name, offset32, out),
            PulpReadonlyPath::Root
            | PulpReadonlyPath::RootDirectory(_)
            | PulpReadonlyPath::Subdir { .. } => {
                return Err(PulpReadonlyStorageBridgeError::NotFile);
            }
        }
        .map_err(PulpReadonlyStorageBridgeError::Backend)?;

        Ok(VaachakStorageReadChunk::from_read(
            offset,
            bytes_read,
            offset + bytes_read as u64 >= file_size as u64,
        ))
    }

    fn list_entries(
        &mut self,
        path: PulpReadonlyPath<'_>,
        out: &mut [VaachakDirectoryEntry],
    ) -> Result<usize, PulpReadonlyStorageBridgeError<B::Error>> {
        if out.is_empty() {
            return Err(PulpReadonlyStorageBridgeError::Contract(
                VaachakReadonlyStorageContractError::DirectoryEntryBufferTooSmall,
            ));
        }

        let mut scratch = [PulpReadonlyDirectoryEntry::EMPTY; PULP_READONLY_MAX_DIR_ENTRIES];
        let limit = core::cmp::min(out.len(), scratch.len());
        let count = match path {
            PulpReadonlyPath::Root => self.backend.list_root_entries(&mut scratch[..limit]),
            PulpReadonlyPath::RootDirectory(dir) => {
                self.backend.list_dir_entries(dir, &mut scratch[..limit])
            }
            PulpReadonlyPath::Subdir { dir, subdir } => {
                self.backend
                    .list_subdir_entries(dir, subdir, &mut scratch[..limit])
            }
            PulpReadonlyPath::RootFile(_)
            | PulpReadonlyPath::DirFile { .. }
            | PulpReadonlyPath::SubdirFile { .. } => {
                return Err(PulpReadonlyStorageBridgeError::NotDirectory);
            }
        }
        .map_err(PulpReadonlyStorageBridgeError::Backend)?;

        let mut written = 0usize;
        let capped = core::cmp::min(count, limit);
        while written < capped {
            if let Some(entry) = scratch[written].to_vaachak_entry() {
                out[written] = entry;
                written += 1;
            } else {
                break;
            }
        }

        Ok(written)
    }
}

impl<B> VaachakReadonlyStorage for PulpReadonlyStorageBridge<B>
where
    B: PulpReadonlyStorageBackend,
{
    type Error = PulpReadonlyStorageBridgeError<B::Error>;

    fn file_exists(&mut self, path: VaachakStoragePathRef<'_>) -> Result<bool, Self::Error> {
        let path = parse_storage_path(path, false)?;
        Ok(self.file_size(path).is_ok())
    }

    fn read_file_start(
        &mut self,
        path: VaachakStoragePathRef<'_>,
        out: &mut [u8],
    ) -> Result<VaachakStorageReadChunk, Self::Error> {
        let path = parse_storage_path(path, false)?;
        self.read_start(path, out)
    }

    fn read_chunk(
        &mut self,
        path: VaachakStoragePathRef<'_>,
        offset: u64,
        out: &mut [u8],
    ) -> Result<VaachakStorageReadChunk, Self::Error> {
        let path = parse_storage_path(path, false)?;
        self.read_at(path, offset, out)
    }

    fn list_directory_metadata(
        &mut self,
        path: VaachakStoragePathRef<'_>,
        out: &mut [VaachakDirectoryEntry],
    ) -> Result<usize, Self::Error> {
        let path = parse_storage_path(path, true)?;
        self.list_entries(path, out)
    }

    fn resolve_current_storage_paths(&self) -> VaachakResolvedStoragePaths<'static> {
        VaachakResolvedStoragePaths::PULP_BACKED_ACTIVE_PATHS
    }
}

#[cfg(target_arch = "riscv32")]
pub struct X4PulpReadonlyStorageBackend<'a> {
    sd: &'a x4_kernel::drivers::sdcard::SdStorage,
}

#[cfg(target_arch = "riscv32")]
impl<'a> X4PulpReadonlyStorageBackend<'a> {
    pub const fn new(sd: &'a x4_kernel::drivers::sdcard::SdStorage) -> Self {
        Self { sd }
    }

    pub const fn sd(&self) -> &'a x4_kernel::drivers::sdcard::SdStorage {
        self.sd
    }
}

#[cfg(target_arch = "riscv32")]
impl PulpReadonlyStorageBackend for X4PulpReadonlyStorageBackend<'_> {
    type Error = x4_kernel::error::Error;

    fn file_size_root(&mut self, name: &str) -> Result<u32, Self::Error> {
        x4_kernel::drivers::storage::file_size(self.sd, name)
    }

    fn file_size_in_dir(&mut self, dir: &str, name: &str) -> Result<u32, Self::Error> {
        x4_kernel::drivers::storage::file_size_in_dir(self.sd, dir, name)
    }

    fn file_size_in_subdir(
        &mut self,
        dir: &str,
        subdir: &str,
        name: &str,
    ) -> Result<u32, Self::Error> {
        x4_kernel::drivers::storage::file_size_in_subdir(self.sd, dir, subdir, name)
    }

    fn read_file_start_root(
        &mut self,
        name: &str,
        out: &mut [u8],
    ) -> Result<(u32, usize), Self::Error> {
        x4_kernel::drivers::storage::read_file_start(self.sd, name, out)
    }

    fn read_file_start_in_dir(
        &mut self,
        dir: &str,
        name: &str,
        out: &mut [u8],
    ) -> Result<(u32, usize), Self::Error> {
        x4_kernel::drivers::storage::read_file_start_in_dir(self.sd, dir, name, out)
    }

    fn read_file_start_in_subdir(
        &mut self,
        dir: &str,
        subdir: &str,
        name: &str,
        out: &mut [u8],
    ) -> Result<(u32, usize), Self::Error> {
        x4_kernel::drivers::storage::read_file_start_in_subdir(self.sd, dir, subdir, name, out)
    }

    fn read_file_chunk_root(
        &mut self,
        name: &str,
        offset: u32,
        out: &mut [u8],
    ) -> Result<usize, Self::Error> {
        x4_kernel::drivers::storage::read_file_chunk(self.sd, name, offset, out)
    }

    fn read_file_chunk_in_dir(
        &mut self,
        dir: &str,
        name: &str,
        offset: u32,
        out: &mut [u8],
    ) -> Result<usize, Self::Error> {
        x4_kernel::drivers::storage::read_file_chunk_in_dir(self.sd, dir, name, offset, out)
    }

    fn read_file_chunk_in_subdir(
        &mut self,
        dir: &str,
        subdir: &str,
        name: &str,
        offset: u32,
        out: &mut [u8],
    ) -> Result<usize, Self::Error> {
        x4_kernel::drivers::storage::read_file_chunk_in_subdir(
            self.sd, dir, subdir, name, offset, out,
        )
    }

    fn list_root_entries(
        &mut self,
        out: &mut [PulpReadonlyDirectoryEntry],
    ) -> Result<usize, Self::Error> {
        let mut raw = [x4_kernel::drivers::storage::DirEntry::EMPTY; PULP_READONLY_MAX_DIR_ENTRIES];
        let limit = core::cmp::min(out.len(), raw.len());
        let count = x4_kernel::drivers::storage::list_root_entries(self.sd, &mut raw[..limit])?;
        copy_pulp_entries(&raw[..core::cmp::min(count, limit)], out)
    }

    fn list_dir_entries(
        &mut self,
        dir: &str,
        out: &mut [PulpReadonlyDirectoryEntry],
    ) -> Result<usize, Self::Error> {
        let mut raw = [x4_kernel::drivers::storage::DirEntry::EMPTY; PULP_READONLY_MAX_DIR_ENTRIES];
        let limit = core::cmp::min(out.len(), raw.len());
        let count = x4_kernel::drivers::storage::list_dir_entries(self.sd, dir, &mut raw[..limit])?;
        copy_pulp_entries(&raw[..core::cmp::min(count, limit)], out)
    }

    fn list_subdir_entries(
        &mut self,
        dir: &str,
        subdir: &str,
        out: &mut [PulpReadonlyDirectoryEntry],
    ) -> Result<usize, Self::Error> {
        let mut raw = [x4_kernel::drivers::storage::DirEntry::EMPTY; PULP_READONLY_MAX_DIR_ENTRIES];
        let limit = core::cmp::min(out.len(), raw.len());
        let count = x4_kernel::drivers::storage::list_subdir_entries(
            self.sd,
            dir,
            subdir,
            &mut raw[..limit],
        )?;
        copy_pulp_entries(&raw[..core::cmp::min(count, limit)], out)
    }
}

#[cfg(target_arch = "riscv32")]
fn copy_pulp_entries(
    raw: &[x4_kernel::drivers::storage::DirEntry],
    out: &mut [PulpReadonlyDirectoryEntry],
) -> Result<usize, x4_kernel::error::Error> {
    let mut count = 0usize;
    while count < raw.len() && count < out.len() {
        let src = raw[count];
        let name = &src.name[..src.name_len as usize];
        if let Some(entry) =
            PulpReadonlyDirectoryEntry::from_name(name, src.is_dir, src.size as u64)
        {
            out[count] = entry;
            count += 1;
        } else {
            break;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::{
        PulpReadonlyDirectoryEntry, PulpReadonlyStorageBackend, PulpReadonlyStorageBridge,
        VaachakStorageReadonlyPulpBridgeContract,
    };
    use crate::vaachak_x4::io::storage_readonly_adapter::{
        VaachakDirectoryEntry, VaachakReadonlyStorage, VaachakStoragePathRef,
    };

    #[derive(Default)]
    struct FakePulpBackend {
        last_dir: &'static str,
        last_subdir: &'static str,
        last_name: &'static str,
        last_offset: u32,
    }

    impl PulpReadonlyStorageBackend for FakePulpBackend {
        type Error = ();

        fn file_size_root(&mut self, name: &str) -> Result<u32, Self::Error> {
            self.last_name = if name == "BOOK.TXT" {
                "BOOK.TXT"
            } else {
                "missing"
            };
            if name == "BOOK.TXT" { Ok(8) } else { Err(()) }
        }

        fn file_size_in_dir(&mut self, dir: &str, name: &str) -> Result<u32, Self::Error> {
            self.last_dir = if dir == "state" { "state" } else { "other" };
            self.last_name = if name == "BMIDX.TXT" {
                "BMIDX.TXT"
            } else {
                "missing"
            };
            if dir == "state" && name == "BMIDX.TXT" {
                Ok(4)
            } else {
                Err(())
            }
        }

        fn file_size_in_subdir(
            &mut self,
            dir: &str,
            subdir: &str,
            name: &str,
        ) -> Result<u32, Self::Error> {
            self.last_dir = if dir == "sleep" { "sleep" } else { "other" };
            self.last_subdir = if subdir == "daily" { "daily" } else { "other" };
            self.last_name = if name == "MON.BMP" {
                "MON.BMP"
            } else {
                "missing"
            };
            if dir == "sleep" && subdir == "daily" && name == "MON.BMP" {
                Ok(9)
            } else {
                Err(())
            }
        }

        fn read_file_start_root(
            &mut self,
            name: &str,
            out: &mut [u8],
        ) -> Result<(u32, usize), Self::Error> {
            self.file_size_root(name)?;
            out[..4].copy_from_slice(b"book");
            Ok((8, 4))
        }

        fn read_file_start_in_dir(
            &mut self,
            dir: &str,
            name: &str,
            out: &mut [u8],
        ) -> Result<(u32, usize), Self::Error> {
            self.file_size_in_dir(dir, name)?;
            out[..4].copy_from_slice(b"bmid");
            Ok((4, 4))
        }

        fn read_file_start_in_subdir(
            &mut self,
            dir: &str,
            subdir: &str,
            name: &str,
            out: &mut [u8],
        ) -> Result<(u32, usize), Self::Error> {
            self.file_size_in_subdir(dir, subdir, name)?;
            out[..3].copy_from_slice(b"bmp");
            Ok((9, 3))
        }

        fn read_file_chunk_root(
            &mut self,
            name: &str,
            offset: u32,
            out: &mut [u8],
        ) -> Result<usize, Self::Error> {
            self.file_size_root(name)?;
            self.last_offset = offset;
            out[0] = b'R';
            Ok(1)
        }

        fn read_file_chunk_in_dir(
            &mut self,
            dir: &str,
            name: &str,
            offset: u32,
            out: &mut [u8],
        ) -> Result<usize, Self::Error> {
            self.file_size_in_dir(dir, name)?;
            self.last_offset = offset;
            out[0] = b'D';
            Ok(1)
        }

        fn read_file_chunk_in_subdir(
            &mut self,
            dir: &str,
            subdir: &str,
            name: &str,
            offset: u32,
            out: &mut [u8],
        ) -> Result<usize, Self::Error> {
            self.file_size_in_subdir(dir, subdir, name)?;
            self.last_offset = offset;
            out[0] = b'S';
            Ok(1)
        }

        fn list_root_entries(
            &mut self,
            out: &mut [PulpReadonlyDirectoryEntry],
        ) -> Result<usize, Self::Error> {
            out[0] = PulpReadonlyDirectoryEntry::from_name(b"BOOK.TXT", false, 8).unwrap();
            Ok(1)
        }

        fn list_dir_entries(
            &mut self,
            dir: &str,
            out: &mut [PulpReadonlyDirectoryEntry],
        ) -> Result<usize, Self::Error> {
            self.last_dir = if dir == "state" { "state" } else { "other" };
            out[0] = PulpReadonlyDirectoryEntry::from_name(b"BMIDX.TXT", false, 4).unwrap();
            Ok(1)
        }

        fn list_subdir_entries(
            &mut self,
            dir: &str,
            subdir: &str,
            out: &mut [PulpReadonlyDirectoryEntry],
        ) -> Result<usize, Self::Error> {
            self.last_dir = if dir == "sleep" { "sleep" } else { "other" };
            self.last_subdir = if subdir == "daily" { "daily" } else { "other" };
            out[0] = PulpReadonlyDirectoryEntry::from_name(b"MON.BMP", false, 9).unwrap();
            Ok(1)
        }
    }

    #[test]
    fn bridge_maps_root_file_reads_to_backend() {
        let mut bridge = PulpReadonlyStorageBridge::new(FakePulpBackend::default());
        let path = VaachakStoragePathRef::from_str("/BOOK.TXT").unwrap();
        let mut out = [0u8; 8];

        assert_eq!(bridge.file_exists(path), Ok(true));
        let chunk = bridge.read_file_start(path, &mut out).unwrap();
        assert_eq!(chunk.bytes_read, 4);
        assert_eq!(&out[..4], b"book");

        let chunk = bridge.read_chunk(path, 2, &mut out).unwrap();
        assert_eq!(chunk.offset, 2);
        assert_eq!(bridge.backend().last_offset, 2);
    }

    #[test]
    fn bridge_maps_directory_and_subdirectory_paths() {
        let mut bridge = PulpReadonlyStorageBridge::new(FakePulpBackend::default());
        let path = VaachakStoragePathRef::from_str("/sleep/daily/MON.BMP").unwrap();
        let mut out = [0u8; 8];

        let chunk = bridge.read_file_start(path, &mut out).unwrap();
        assert_eq!(chunk.bytes_read, 3);
        assert_eq!(bridge.backend().last_dir, "sleep");
        assert_eq!(bridge.backend().last_subdir, "daily");
    }

    #[test]
    fn bridge_lists_metadata_without_mutating_contracts() {
        let mut bridge = PulpReadonlyStorageBridge::new(FakePulpBackend::default());
        let mut out = [VaachakDirectoryEntry::empty(); 2];

        assert_eq!(
            bridge.list_directory_metadata(VaachakStoragePathRef::from_str("/").unwrap(), &mut out),
            Ok(1)
        );
        assert_eq!(out[0].name_bytes(), b"BOOK.TXT");
    }

    #[test]
    fn bridge_preflight_keeps_physical_behavior_imported() {
        let report = VaachakStorageReadonlyPulpBridgeContract::active_runtime_preflight();
        assert!(report.ok());
        assert!(!VaachakStorageReadonlyPulpBridgeContract::physical_behavior_moved());
    }
}
