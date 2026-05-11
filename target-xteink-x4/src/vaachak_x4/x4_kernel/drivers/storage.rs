// sd card file operations
//
// all I/O through embedded-sdmmc AsyncVolumeManager; functions are
// synchronous, wrapping async ops with poll_once (SPI bus is blocking
// so every .await resolves immediately)
//
// returns the unified Error type (re-exported as StorageError for
// backward compat); apps receive it through KernelHandle

use core::ops::ControlFlow;

use embedded_sdmmc::{Mode, RawDirectory};

use crate::vaachak_x4::x4_kernel::drivers::sdcard::{SdStorage, SdStorageInner, poll_once};
use crate::vaachak_x4::x4_kernel::error::{Error, ErrorKind};

pub const X4_DIR: &str = "_x4";
pub const TITLES_FILE: &str = "TITLES.BIN";
pub const TITLE_CAP: usize = 64;
pub const MAX_NESTED_STORAGE_PATH_COMPONENTS: usize = 8;

// backward-compatible alias
pub type StorageError = Error;

#[derive(Clone, Copy)]
pub struct DirEntry {
    pub name: [u8; 13],
    pub name_len: u8,
    pub is_dir: bool,
    pub size: u32,
    pub title: [u8; TITLE_CAP],
    pub title_len: u8,
}

impl DirEntry {
    pub const EMPTY: Self = Self {
        name: [0u8; 13],
        name_len: 0,
        is_dir: false,
        size: 0,
        title: [0u8; TITLE_CAP],
        title_len: 0,
    };

    pub fn name_str(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len as usize]).unwrap_or("?")
    }

    pub fn display_name(&self) -> &str {
        let len = (self.title_len & 0x7F) as usize;
        if len > 0 {
            core::str::from_utf8(&self.title[..len]).unwrap_or(self.name_str())
        } else {
            self.name_str()
        }
    }

    pub fn has_real_title(&self) -> bool {
        self.title_len > 0 && self.title_len & 0x80 == 0
    }

    pub fn set_title(&mut self, s: &[u8]) {
        let n = s.len().min(TITLE_CAP);
        self.title[..n].copy_from_slice(&s[..n]);
        self.title_len = n as u8;
    }

    // write a humanized SFN into the title buffer as a soft fallback;
    // does not prevent the title scanner from resolving a real title
    pub fn humanize_sfn(&mut self) {
        let nlen = self.name_len as usize;
        if nlen == 0 || self.has_real_title() {
            return;
        }
        let src = &self.name[..nlen];
        // check if name is all-uppercase (typical 8.3 SFN)
        let all_upper = src.iter().all(|&b| !b.is_ascii_lowercase());
        if !all_upper {
            return; // mixed case: user-supplied LFN, leave as-is
        }
        let n = nlen.min(TITLE_CAP);
        let dot_pos = src.iter().position(|&b| b == b'.').unwrap_or(n);
        for i in 0..n {
            if i == 0 {
                self.title[i] = src[i]; // keep first char uppercase
            } else if i > dot_pos {
                self.title[i] = src[i].to_ascii_lowercase(); // lowercase ext
            } else {
                self.title[i] = src[i].to_ascii_lowercase();
            }
        }
        self.title_len = 0x80 | n as u8;
    }
}

pub struct DirPage {
    pub total: usize,
    pub count: usize,
}

fn ext_eq(name: &[u8], target: &[u8]) -> bool {
    let dot = match name.iter().rposition(|&b| b == b'.') {
        Some(p) => p,
        None => return false,
    };
    let ext = &name[dot + 1..];
    ext.len() == target.len() && ext.eq_ignore_ascii_case(target)
}

fn has_supported_ext(name: &[u8]) -> bool {
    ext_eq(name, b"TXT") || ext_eq(name, b"EPUB") || ext_eq(name, b"EPU") || ext_eq(name, b"MD")
}

// build "NAME.EXT" bytes from a ShortFileName

fn sfn_to_bytes(name: &embedded_sdmmc::ShortFileName, out: &mut [u8; 13]) -> u8 {
    let base = name.base_name();
    let ext = name.extension();
    let mut pos = 0usize;
    let blen = base.len().min(8);
    out[..blen].copy_from_slice(&base[..blen]);
    pos += blen;
    if !ext.is_empty() {
        out[pos] = b'.';
        pos += 1;
        let elen = ext.len().min(3);
        out[pos..pos + elen].copy_from_slice(&ext[..elen]);
        pos += elen;
    }
    pos as u8
}

// file-operation macros; each evaluates to Result<T, Error>
// none use ? internally so caller cleanup is never bypassed

macro_rules! op_file_size {
    ($inner:expr, $dir:expr, $name:expr) => {
        $inner
            .mgr
            .find_directory_entry($dir, $name)
            .await
            .map(|e| e.size)
            .map_err(|_| Error::new(ErrorKind::OpenFile, "file_size"))
    };
}

macro_rules! op_read_chunk {
    ($inner:expr, $dir:expr, $name:expr, $offset:expr, $buf:expr) => {
        match $inner
            .mgr
            .open_file_in_dir($dir, $name, Mode::ReadOnly)
            .await
        {
            Err(_) => Err(Error::new(ErrorKind::OpenFile, "read_chunk")),
            Ok(file) => {
                let result = match $inner.mgr.file_seek_from_start(file, $offset) {
                    Ok(()) => $inner
                        .mgr
                        .read(file, $buf)
                        .await
                        .map_err(|_| Error::new(ErrorKind::ReadFailed, "read_chunk")),
                    Err(_) => Err(Error::new(ErrorKind::SeekFailed, "read_chunk")),
                };
                let _ = $inner.mgr.close_file(file).await;
                result
            }
        }
    };
}

macro_rules! op_read_start {
    ($inner:expr, $dir:expr, $name:expr, $buf:expr) => {
        match $inner
            .mgr
            .open_file_in_dir($dir, $name, Mode::ReadOnly)
            .await
        {
            Err(_) => Err(Error::new(ErrorKind::OpenFile, "read_start")),
            Ok(file) => {
                let size = $inner.mgr.file_length(file).unwrap_or(0);
                let result = $inner
                    .mgr
                    .read(file, $buf)
                    .await
                    .map_err(|_| Error::new(ErrorKind::ReadFailed, "read_start"));
                let _ = $inner.mgr.close_file(file).await;
                result.map(|n| (size, n))
            }
        }
    };
}

macro_rules! op_write {
    ($inner:expr, $dir:expr, $name:expr, $data:expr) => {
        match $inner
            .mgr
            .open_file_in_dir($dir, $name, Mode::ReadWriteCreateOrTruncate)
            .await
        {
            Err(_) => Err(Error::new(ErrorKind::OpenFile, "write")),
            Ok(file) => {
                let result = if ($data).is_empty() {
                    Ok(())
                } else {
                    $inner
                        .mgr
                        .write(file, $data)
                        .await
                        .map_err(|_| Error::new(ErrorKind::WriteFailed, "write"))
                };
                let _ = $inner.mgr.close_file(file).await;
                result
            }
        }
    };
}

macro_rules! op_append {
    ($inner:expr, $dir:expr, $name:expr, $data:expr) => {
        match $inner
            .mgr
            .open_file_in_dir($dir, $name, Mode::ReadWriteCreateOrAppend)
            .await
        {
            Err(_) => Err(Error::new(ErrorKind::OpenFile, "append")),
            Ok(file) => {
                let result = if ($data).is_empty() {
                    Ok(())
                } else {
                    $inner
                        .mgr
                        .write(file, $data)
                        .await
                        .map_err(|_| Error::new(ErrorKind::WriteFailed, "append"))
                };
                let _ = $inner.mgr.close_file(file).await;
                result
            }
        }
    };
}

macro_rules! op_delete {
    ($inner:expr, $dir:expr, $name:expr) => {{
        $inner
            .mgr
            .delete_entry_in_dir($dir, $name)
            .await
            .map_err(|_| Error::new(ErrorKind::DeleteFailed, "delete"))
    }};
}

// dir-scoping macros; open subdir, execute body, close handle

macro_rules! in_dir {
    ($inner:expr, $dirname:expr, |$dir:ident| $body:expr) => {
        match $inner.mgr.open_dir($inner.root, $dirname).await {
            Err(_) => Err(Error::new(ErrorKind::OpenDir, "in_dir")),
            Ok($dir) => {
                let _r = $body;
                let _ = $inner.mgr.close_dir($dir);
                _r
            }
        }
    };
}

macro_rules! in_subdir {
    ($inner:expr, $d1:expr, $d2:expr, |$dir:ident| $body:expr) => {
        match $inner.mgr.open_dir($inner.root, $d1).await {
            Err(_) => Err(Error::new(ErrorKind::OpenDir, "in_subdir")),
            Ok(_mid) => match $inner.mgr.open_dir(_mid, $d2).await {
                Err(_) => {
                    let _ = $inner.mgr.close_dir(_mid);
                    Err(Error::new(ErrorKind::OpenDir, "in_subdir"))
                }
                Ok($dir) => {
                    let _r = $body;
                    let _ = $inner.mgr.close_dir($dir);
                    let _ = $inner.mgr.close_dir(_mid);
                    _r
                }
            },
        }
    };
}

fn borrow(sd: &SdStorage) -> core::result::Result<core::cell::RefMut<'_, SdStorageInner>, Error> {
    sd.borrow_inner()
        .ok_or(Error::new(ErrorKind::NoCard, "storage::borrow"))
}

fn validate_nested_storage_path(path: &str) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    if path.starts_with('/') || path.ends_with('/') || path.contains("//") {
        return Err(Error::new(ErrorKind::InvalidData, "nested_path_shape"));
    }

    let mut count = 0usize;
    for component in path.split('/') {
        if component.is_empty()
            || component == "."
            || component == ".."
            || component
                .as_bytes()
                .iter()
                .any(|&b| b == b'\\' || b == b':')
        {
            return Err(Error::new(ErrorKind::InvalidData, "nested_path_component"));
        }
        count += 1;
        if count > MAX_NESTED_STORAGE_PATH_COMPONENTS {
            return Err(Error::new(ErrorKind::InvalidData, "nested_path_depth"));
        }
    }

    Ok(())
}

async fn open_nested_storage_dir(
    inner: &mut SdStorageInner,
    path: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<(RawDirectory, bool)> {
    if path.is_empty() {
        return Ok((inner.root, false));
    }
    validate_nested_storage_path(path)?;

    let mut current = inner.root;
    let mut current_is_child = false;
    let mut depth = 0usize;

    for component in path.split('/') {
        if depth >= MAX_NESTED_STORAGE_PATH_COMPONENTS {
            close_nested_storage_dir(inner, current, current_is_child);
            return Err(Error::new(
                ErrorKind::OpenDir,
                "open_nested_storage_dir_depth",
            ));
        }

        match inner.mgr.open_dir(current, component).await {
            Ok(next) => {
                close_nested_storage_dir(inner, current, current_is_child);
                current = next;
                current_is_child = true;
                depth += 1;
            }
            Err(_) => {
                close_nested_storage_dir(inner, current, current_is_child);
                return Err(Error::new(ErrorKind::OpenDir, "open_nested_storage_dir"));
            }
        }
    }

    Ok((current, current_is_child))
}

fn close_nested_storage_dir(inner: &mut SdStorageInner, dir: RawDirectory, should_close: bool) {
    if should_close {
        let _ = inner.mgr.close_dir(dir);
    }
}

// root file operations

pub fn file_size(sd: &SdStorage, name: &str) -> crate::vaachak_x4::x4_kernel::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        op_file_size!(inner, inner.root, name)
    })
}

pub fn read_file_chunk(
    sd: &SdStorage,
    name: &str,
    offset: u32,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        op_read_chunk!(inner, inner.root, name, offset, buf)
    })
}

pub fn read_file_start(
    sd: &SdStorage,
    name: &str,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        op_read_start!(inner, inner.root, name, buf)
    })
}

pub fn write_file(
    sd: &SdStorage,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        op_write!(inner, inner.root, name, data)
    })
}

pub fn append_root_file(
    sd: &SdStorage,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        op_append!(inner, inner.root, name, data)
    })
}

pub fn delete_file(sd: &SdStorage, name: &str) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        op_delete!(inner, inner.root, name)
    })
}

// directory listing

pub fn list_root_files(
    sd: &SdStorage,
    buf: &mut [DirEntry],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        let mut count = 0usize;
        let mut total = 0usize;

        inner
            .mgr
            .iterate_dir(inner.root, |entry| {
                if entry.attributes.is_volume() || entry.attributes.is_directory() {
                    return ControlFlow::Continue(());
                }

                let mut name_buf = [0u8; 13];
                let name_len = sfn_to_bytes(&entry.name, &mut name_buf);
                let sfn = &name_buf[..name_len as usize];

                if sfn.is_empty() || sfn[0] == b'.' || sfn[0] == b'_' {
                    return ControlFlow::Continue(());
                }
                if !has_supported_ext(sfn) {
                    return ControlFlow::Continue(());
                }

                total += 1;

                if count < buf.len() {
                    buf[count] = DirEntry {
                        name: name_buf,
                        name_len,
                        is_dir: false,
                        size: entry.size,
                        title: [0u8; TITLE_CAP],
                        title_len: 0,
                    };
                    count += 1;
                }
                ControlFlow::Continue(())
            })
            .await
            .map_err(|_| Error::new(ErrorKind::ReadFailed, "list_root_files"))?;

        if total > count {
            log::warn!(
                "dir: {} supported files on SD, only {} fit in buffer (max {})",
                total,
                count,
                buf.len(),
            );
        }
        Ok(count)
    })
}

pub fn list_root_entries(
    sd: &SdStorage,
    buf: &mut [DirEntry],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        let mut count = 0usize;
        let mut total = 0usize;

        inner
            .mgr
            .iterate_dir(inner.root, |entry| {
                if entry.attributes.is_volume() {
                    return ControlFlow::Continue(());
                }

                let mut name_buf = [0u8; 13];
                let name_len = sfn_to_bytes(&entry.name, &mut name_buf);
                let sfn = &name_buf[..name_len as usize];

                if sfn.is_empty() || sfn[0] == b'.' {
                    return ControlFlow::Continue(());
                }

                total += 1;

                if count < buf.len() {
                    buf[count] = DirEntry {
                        name: name_buf,
                        name_len,
                        is_dir: entry.attributes.is_directory(),
                        size: entry.size,
                        title: [0u8; TITLE_CAP],
                        title_len: 0,
                    };
                    count += 1;
                }

                ControlFlow::Continue(())
            })
            .await
            .map_err(|_| Error::new(ErrorKind::ReadFailed, "list_root_entries"))?;

        if total > count {
            log::warn!(
                "sd-manager: {} entries on SD, only {} fit in buffer",
                total,
                count
            );
        }

        Ok(count)
    })
}

pub fn list_dir_entries(
    sd: &SdStorage,
    dir_name: &str,
    buf: &mut [DirEntry],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        let dir = match inner.mgr.open_dir(inner.root, dir_name).await {
            Ok(d) => d,
            Err(_) => return Err(Error::new(ErrorKind::OpenDir, "list_dir_entries")),
        };

        let mut count = 0usize;
        let mut total = 0usize;

        let result = inner
            .mgr
            .iterate_dir(dir, |entry| {
                if entry.attributes.is_volume() {
                    return ControlFlow::Continue(());
                }

                let mut name_buf = [0u8; 13];
                let name_len = sfn_to_bytes(&entry.name, &mut name_buf);
                let sfn = &name_buf[..name_len as usize];

                if sfn.is_empty() || sfn[0] == b'.' {
                    return ControlFlow::Continue(());
                }

                total += 1;

                if count < buf.len() {
                    buf[count] = DirEntry {
                        name: name_buf,
                        name_len,
                        is_dir: entry.attributes.is_directory(),
                        size: entry.size,
                        title: [0u8; TITLE_CAP],
                        title_len: 0,
                    };
                    count += 1;
                }

                ControlFlow::Continue(())
            })
            .await
            .map_err(|_| Error::new(ErrorKind::ReadFailed, "list_dir_entries"))
            .map(|_| count);

        let _ = inner.mgr.close_dir(dir);

        if result.is_ok() && total > count {
            log::warn!(
                "sd-manager: {} entries in {}, only {} fit in buffer",
                total,
                dir_name,
                count
            );
        }

        result
    })
}

pub fn delete_file_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_delete!(inner, dir_h, name))
    })
}

pub fn list_subdir_entries(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    buf: &mut [DirEntry],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        in_subdir!(inner, dir, subdir, |dir_h| {
            let mut count = 0usize;
            let mut total = 0usize;

            let result = inner
                .mgr
                .iterate_dir(dir_h, |entry| {
                    if entry.attributes.is_volume() {
                        return ControlFlow::Continue(());
                    }

                    let mut name_buf = [0u8; 13];
                    let name_len = sfn_to_bytes(&entry.name, &mut name_buf);
                    let sfn = &name_buf[..name_len as usize];

                    if sfn.is_empty() || sfn[0] == b'.' {
                        return ControlFlow::Continue(());
                    }

                    total += 1;

                    if count < buf.len() {
                        buf[count] = DirEntry {
                            name: name_buf,
                            name_len,
                            is_dir: entry.attributes.is_directory(),
                            size: entry.size,
                            title: [0u8; TITLE_CAP],
                            title_len: 0,
                        };
                        count += 1;
                    }

                    ControlFlow::Continue(())
                })
                .await
                .map_err(|_| Error::new(ErrorKind::ReadFailed, "list_subdir_entries"))
                .map(|_| count);

            if result.is_ok() && total > count {
                log::warn!(
                    "sd-manager: {} entries in {}/{}, only {} fit in buffer",
                    total,
                    dir,
                    subdir,
                    count
                );
            }

            result
        })
    })
}

pub fn list_path_entries(
    sd: &SdStorage,
    path: &str,
    buf: &mut [DirEntry],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let (dir, should_close) = open_nested_storage_dir(inner, path).await?;

        let mut count = 0usize;
        let mut total = 0usize;
        let result = inner
            .mgr
            .iterate_dir(dir, |entry| {
                if entry.attributes.is_volume() {
                    return ControlFlow::Continue(());
                }

                let mut name_buf = [0u8; 13];
                let name_len = sfn_to_bytes(&entry.name, &mut name_buf);
                let sfn = &name_buf[..name_len as usize];
                if sfn.is_empty() || sfn[0] == b'.' {
                    return ControlFlow::Continue(());
                }

                total += 1;
                if count < buf.len() {
                    buf[count] = DirEntry {
                        name: name_buf,
                        name_len,
                        is_dir: entry.attributes.is_directory(),
                        size: entry.size,
                        title: [0u8; TITLE_CAP],
                        title_len: 0,
                    };
                    count += 1;
                }

                ControlFlow::Continue(())
            })
            .await
            .map_err(|_| Error::new(ErrorKind::ReadFailed, "list_path_entries"))
            .map(|_| count);

        close_nested_storage_dir(inner, dir, should_close);

        if result.is_ok() && total > count {
            log::warn!(
                "sd-manager: {} entries in {}, only {} fit in buffer",
                total,
                path,
                count
            );
        }

        result
    })
}

pub fn ensure_path(sd: &SdStorage, path: &str) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    let path = path.trim_matches('/');
    if path.is_empty() {
        return Ok(());
    }
    validate_nested_storage_path(path)?;

    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let mut current = inner.root;
        let mut current_is_child = false;
        let mut depth = 0usize;

        for component in path.split('/') {
            if depth >= MAX_NESTED_STORAGE_PATH_COMPONENTS {
                close_nested_storage_dir(inner, current, current_is_child);
                return Err(Error::new(ErrorKind::WriteFailed, "ensure_path_depth"));
            }

            let child = match inner.mgr.open_dir(current, component).await {
                Ok(child) => child,
                Err(_) => {
                    match inner.mgr.make_dir_in_dir(current, component).await {
                        Ok(()) | Err(embedded_sdmmc::Error::DirAlreadyExists) => {}
                        Err(_) => {
                            close_nested_storage_dir(inner, current, current_is_child);
                            return Err(Error::new(ErrorKind::WriteFailed, "ensure_path"));
                        }
                    }
                    match inner.mgr.open_dir(current, component).await {
                        Ok(child) => child,
                        Err(_) => {
                            close_nested_storage_dir(inner, current, current_is_child);
                            return Err(Error::new(ErrorKind::OpenDir, "ensure_path_open"));
                        }
                    }
                }
            };

            close_nested_storage_dir(inner, current, current_is_child);
            current = child;
            current_is_child = true;
            depth += 1;
        }

        close_nested_storage_dir(inner, current, current_is_child);
        Ok(())
    })
}

pub fn file_size_in_path(
    sd: &SdStorage,
    path: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let (dir, should_close) = open_nested_storage_dir(inner, path).await?;
        let result = op_file_size!(inner, dir, name);
        close_nested_storage_dir(inner, dir, should_close);
        result
    })
}

pub fn read_file_start_in_path(
    sd: &SdStorage,
    path: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let (dir, should_close) = open_nested_storage_dir(inner, path).await?;
        let result = op_read_start!(inner, dir, name, buf);
        close_nested_storage_dir(inner, dir, should_close);
        result
    })
}

pub fn read_file_chunk_in_path(
    sd: &SdStorage,
    path: &str,
    name: &str,
    offset: u32,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let (dir, should_close) = open_nested_storage_dir(inner, path).await?;
        let result = op_read_chunk!(inner, dir, name, offset, buf);
        close_nested_storage_dir(inner, dir, should_close);
        result
    })
}

pub fn write_file_in_path(
    sd: &SdStorage,
    path: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let (dir, should_close) = open_nested_storage_dir(inner, path).await?;
        let result = op_write!(inner, dir, name, data);
        close_nested_storage_dir(inner, dir, should_close);
        result
    })
}

pub fn append_file_in_path(
    sd: &SdStorage,
    path: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let (dir, should_close) = open_nested_storage_dir(inner, path).await?;
        let result = op_append!(inner, dir, name, data);
        close_nested_storage_dir(inner, dir, should_close);
        result
    })
}

pub fn delete_file_in_path(
    sd: &SdStorage,
    path: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let (dir, should_close) = open_nested_storage_dir(inner, path).await?;
        let result = op_delete!(inner, dir, name);
        close_nested_storage_dir(inner, dir, should_close);
        result
    })
}

// directory management

pub fn ensure_dir(sd: &SdStorage, name: &str) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    // two poll_once calls so the large make_dir future never shares
    // a stack frame with open_dir, halving peak stack usage
    let exists = poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        match inner.mgr.open_dir(inner.root, name).await {
            Ok(dir) => {
                let _ = inner.mgr.close_dir(dir);
                Ok::<_, Error>(true)
            }
            Err(_) => Ok(false),
        }
    })?;

    if exists {
        return Ok(());
    }

    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        match inner.mgr.make_dir_in_dir(inner.root, name).await {
            Ok(()) => Ok(()),
            Err(embedded_sdmmc::Error::DirAlreadyExists) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::WriteFailed, "ensure_dir")),
        }
    })
}

// single-directory file operations

pub fn write_file_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_write!(inner, dir_h, name, data))
    })
}

pub fn append_file_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_append!(inner, dir_h, name, data))
    })
}

pub fn file_size_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_file_size!(inner, dir_h, name))
    })
}

pub fn file_size_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_file_size!(
            inner, dir_h, name
        ))
    })
}

pub fn read_file_chunk_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    offset: u32,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_read_chunk!(
            inner, dir_h, name, offset, buf
        ))
    })
}

pub fn read_file_start_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_read_start!(inner, dir_h, name, buf))
    })
}

pub fn read_file_chunk_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    offset: u32,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_read_chunk!(
            inner, dir_h, name, offset, buf
        ))
    })
}

pub fn read_file_start_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_read_start!(
            inner, dir_h, name, buf
        ))
    })
}

/// Read the start of a file under a fixed three-level directory path.
///
/// This is used by the first Lua app proof to read
/// `/VAACHAK/APPS/<app_id>/MAIN.LUA` without adding recursive SD scanning or
/// changing raw SD/FAT/SPI behavior.
pub fn read_file_start_in_three_subdir(
    sd: &SdStorage,
    dir1: &str,
    dir2: &str,
    dir3: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let d1 = inner
            .mgr
            .open_dir(inner.root, dir1)
            .await
            .map_err(|_| Error::new(ErrorKind::OpenDir, "read_file_start_in_three_subdir"))?;
        let d2 = match inner.mgr.open_dir(d1, dir2).await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(d1);
                return Err(Error::new(
                    ErrorKind::OpenDir,
                    "read_file_start_in_three_subdir",
                ));
            }
        };
        let d3 = match inner.mgr.open_dir(d2, dir3).await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(d2);
                let _ = inner.mgr.close_dir(d1);
                return Err(Error::new(
                    ErrorKind::OpenDir,
                    "read_file_start_in_three_subdir",
                ));
            }
        };

        let result = op_read_start!(inner, d3, name, buf);
        let _ = inner.mgr.close_dir(d3);
        let _ = inner.mgr.close_dir(d2);
        let _ = inner.mgr.close_dir(d1);
        result
    })
}

/// Read the start of a file under a fixed four-level directory path.
///
/// This is used by SD-loaded Lua apps that keep app data one level below
/// the physical app folder, for example:
/// `/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT`.
///
/// It intentionally opens a fixed-depth path and does not add recursive SD
/// scanning or change raw SD/FAT/SPI behavior.
pub fn read_file_start_in_four_subdir(
    sd: &SdStorage,
    dir1: &str,
    dir2: &str,
    dir3: &str,
    dir4: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        let d1 = inner
            .mgr
            .open_dir(inner.root, dir1)
            .await
            .map_err(|_| Error::new(ErrorKind::OpenDir, "read_file_start_in_four_subdir"))?;
        let d2 = match inner.mgr.open_dir(d1, dir2).await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(d1);
                return Err(Error::new(
                    ErrorKind::OpenDir,
                    "read_file_start_in_four_subdir",
                ));
            }
        };
        let d3 = match inner.mgr.open_dir(d2, dir3).await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(d2);
                let _ = inner.mgr.close_dir(d1);
                return Err(Error::new(
                    ErrorKind::OpenDir,
                    "read_file_start_in_four_subdir",
                ));
            }
        };
        let d4 = match inner.mgr.open_dir(d3, dir4).await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(d3);
                let _ = inner.mgr.close_dir(d2);
                let _ = inner.mgr.close_dir(d1);
                return Err(Error::new(
                    ErrorKind::OpenDir,
                    "read_file_start_in_four_subdir",
                ));
            }
        };

        let result = op_read_start!(inner, d4, name, buf);
        let _ = inner.mgr.close_dir(d4);
        let _ = inner.mgr.close_dir(d3);
        let _ = inner.mgr.close_dir(d2);
        let _ = inner.mgr.close_dir(d1);
        result
    })
}

/// Read the start of a file under the canonical Lua app data path:
/// `/VAACHAK/APPS/<APP>/DATA/<NAME>`.
///
/// This helper is intentionally fixed-depth and read-only. It does not add
/// recursive SD scanning, does not change raw SD/FAT/SPI behavior, and exists
/// only to avoid treating `DATA/Y2026.TXT` as a single 8.3 filename.
pub fn read_file_start_in_vaachak_lua_app_data_file(
    sd: &SdStorage,
    app_folder: &str,
    data_dir: &str,
    name: &str,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        let vaachak = inner
            .mgr
            .open_dir(inner.root, "VAACHAK")
            .await
            .map_err(|_| Error::new(ErrorKind::OpenDir, "lua_app_data:VAACHAK"))?;

        let apps = match inner.mgr.open_dir(vaachak, "APPS").await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(vaachak);
                return Err(Error::new(ErrorKind::OpenDir, "lua_app_data:APPS"));
            }
        };

        let app = match inner.mgr.open_dir(apps, app_folder).await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(apps);
                let _ = inner.mgr.close_dir(vaachak);
                return Err(Error::new(ErrorKind::OpenDir, "lua_app_data:APP"));
            }
        };

        let data = match inner.mgr.open_dir(app, data_dir).await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(app);
                let _ = inner.mgr.close_dir(apps);
                let _ = inner.mgr.close_dir(vaachak);
                return Err(Error::new(ErrorKind::OpenDir, "lua_app_data:DATA"));
            }
        };

        let result = op_read_start!(inner, data, name, buf);

        let _ = inner.mgr.close_dir(data);
        let _ = inner.mgr.close_dir(app);
        let _ = inner.mgr.close_dir(apps);
        let _ = inner.mgr.close_dir(vaachak);

        result
    })
}

/// Read `/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT` using explicit path segments.
///
/// This intentionally avoids generic slash-containing file names and avoids
/// recursive scanning. It mirrors the path that Wi-Fi Transfer lists on the SD
/// card and is used only by the Lua Panchang runtime app.
pub fn read_vaachak_apps_panchang_y2026_start(
    sd: &SdStorage,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<(u32, usize)> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        let vaachak = inner
            .mgr
            .open_dir(inner.root, "VAACHAK")
            .await
            .map_err(|_| Error::new(ErrorKind::OpenDir, "panchang_y2026:VAACHAK"))?;

        let apps = match inner.mgr.open_dir(vaachak, "APPS").await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(vaachak);
                return Err(Error::new(ErrorKind::OpenDir, "panchang_y2026:APPS"));
            }
        };

        let panchang = match inner.mgr.open_dir(apps, "PANCHANG").await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(apps);
                let _ = inner.mgr.close_dir(vaachak);
                return Err(Error::new(ErrorKind::OpenDir, "panchang_y2026:PANCHANG"));
            }
        };

        let data = match inner.mgr.open_dir(panchang, "DATA").await {
            Ok(dir) => dir,
            Err(_) => {
                let _ = inner.mgr.close_dir(panchang);
                let _ = inner.mgr.close_dir(apps);
                let _ = inner.mgr.close_dir(vaachak);
                return Err(Error::new(ErrorKind::OpenDir, "panchang_y2026:DATA"));
            }
        };

        let result = op_read_start!(inner, data, "Y2026.TXT", buf);

        let _ = inner.mgr.close_dir(data);
        let _ = inner.mgr.close_dir(panchang);
        let _ = inner.mgr.close_dir(apps);
        let _ = inner.mgr.close_dir(vaachak);

        result
    })
}

pub fn ensure_dir_in_dir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    let exists = poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        in_dir!(inner, dir, |dir_h| {
            match inner.mgr.open_dir(dir_h, name).await {
                Ok(child) => {
                    let _ = inner.mgr.close_dir(child);
                    Ok::<_, Error>(true)
                }
                Err(_) => Ok(false),
            }
        })
    })?;

    if exists {
        return Ok(());
    }

    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        in_dir!(inner, dir, |dir_h| {
            match inner.mgr.make_dir_in_dir(dir_h, name).await {
                Ok(()) => Ok(()),
                Err(embedded_sdmmc::Error::DirAlreadyExists) => Ok(()),
                Err(_) => Err(Error::new(ErrorKind::WriteFailed, "ensure_dir_in_dir")),
            }
        })
    })
}

pub fn ensure_dir_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    let exists = poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        in_subdir!(inner, dir, subdir, |dir_h| {
            match inner.mgr.open_dir(dir_h, name).await {
                Ok(child) => {
                    let _ = inner.mgr.close_dir(child);
                    Ok::<_, Error>(true)
                }
                Err(_) => Ok(false),
            }
        })
    })?;

    if exists {
        return Ok(());
    }

    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;

        in_subdir!(inner, dir, subdir, |dir_h| {
            match inner.mgr.make_dir_in_dir(dir_h, name).await {
                Ok(()) => Ok(()),
                Err(embedded_sdmmc::Error::DirAlreadyExists) => Ok(()),
                Err(_) => Err(Error::new(ErrorKind::WriteFailed, "ensure_dir_in_subdir")),
            }
        })
    })
}

pub fn write_file_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_write!(
            inner, dir_h, name, data
        ))
    })
}

pub fn append_file_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_append!(
            inner, dir_h, name, data
        ))
    })
}

pub fn delete_file_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_delete!(inner, dir_h, name))
    })
}

// async boot path (runs inside the real executor)

pub async fn ensure_x4_dir_async(
    sd: &SdStorage,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    let mut guard = borrow(sd)?;
    let inner = &mut *guard;

    if let Ok(dir) = inner.mgr.open_dir(inner.root, X4_DIR).await {
        let _ = inner.mgr.close_dir(dir);
        return Ok(());
    }
    match inner.mgr.make_dir_in_dir(inner.root, X4_DIR).await {
        Ok(()) => Ok(()),
        Err(embedded_sdmmc::Error::DirAlreadyExists) => Ok(()),
        Err(_) => Err(Error::new(ErrorKind::WriteFailed, "ensure_x4_dir_async")),
    }
}

// _x4 subdirectory operations

pub fn ensure_x4_subdir(
    sd: &SdStorage,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    let exists = poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |x4_h| {
            match inner.mgr.open_dir(x4_h, name).await {
                Ok(sub) => {
                    let _ = inner.mgr.close_dir(sub);
                    Ok::<_, Error>(true)
                }
                Err(_) => Ok(false),
            }
        })
    })?;

    if exists {
        return Ok(());
    }

    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |x4_h| {
            match inner.mgr.make_dir_in_dir(x4_h, name).await {
                Ok(()) => Ok::<_, Error>(()),
                Err(embedded_sdmmc::Error::DirAlreadyExists) => Ok(()),
                Err(_) => Err(Error::new(ErrorKind::WriteFailed, "ensure_x4_subdir")),
            }
        })
    })
}

pub fn write_in_x4_subdir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, X4_DIR, dir, |sub_h| op_write!(
            inner, sub_h, name, data
        ))
    })
}

pub fn append_in_x4_subdir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, X4_DIR, dir, |sub_h| op_append!(
            inner, sub_h, name, data
        ))
    })
}

pub fn read_chunk_in_x4_subdir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
    offset: u32,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, X4_DIR, dir, |sub_h| op_read_chunk!(
            inner, sub_h, name, offset, buf
        ))
    })
}

pub fn file_size_in_x4_subdir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, X4_DIR, dir, |sub_h| op_file_size!(
            inner, sub_h, name
        ))
    })
}

pub fn delete_in_x4_subdir(
    sd: &SdStorage,
    dir: &str,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, X4_DIR, dir, |sub_h| op_delete!(inner, sub_h, name))
    })
}

// _x4/ direct file operations (cache files live directly in _x4/)

pub fn read_chunk_in_x4(
    sd: &SdStorage,
    name: &str,
    offset: u32,
    buf: &mut [u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<usize> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |dir_h| op_read_chunk!(
            inner, dir_h, name, offset, buf
        ))
    })
}

pub fn write_in_x4(
    sd: &SdStorage,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |dir_h| op_write!(inner, dir_h, name, data))
    })
}

pub fn append_in_x4(
    sd: &SdStorage,
    name: &str,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |dir_h| op_append!(inner, dir_h, name, data))
    })
}

pub fn file_size_in_x4(
    sd: &SdStorage,
    name: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |dir_h| op_file_size!(inner, dir_h, name))
    })
}

pub fn delete_in_x4(sd: &SdStorage, name: &str) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |dir_h| op_delete!(inner, dir_h, name))
    })
}

// seek+write: open existing file, seek to offset, write data, close
// used to update the chapter offset table after all chapters are appended
pub fn write_at_in_x4(
    sd: &SdStorage,
    name: &str,
    offset: u32,
    data: &[u8],
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, X4_DIR, |dir_h| {
            match inner
                .mgr
                .open_file_in_dir(dir_h, name, Mode::ReadWriteCreateOrAppend)
                .await
            {
                Err(_) => Err(Error::new(ErrorKind::OpenFile, "write_at")),
                Ok(file) => {
                    let result = match inner.mgr.file_seek_from_start(file, offset) {
                        Ok(()) => inner
                            .mgr
                            .write(file, data)
                            .await
                            .map_err(|_| Error::new(ErrorKind::WriteFailed, "write_at")),
                        Err(_) => Err(Error::new(ErrorKind::SeekFailed, "write_at")),
                    };
                    let _ = inner.mgr.close_file(file).await;
                    result
                }
            }
        })
    })
}

// title mapping

// append a title line to _x4/TITLES.BIN
pub fn save_title(
    sd: &SdStorage,
    filename: &str,
    title: &str,
) -> crate::vaachak_x4::x4_kernel::error::Result<()> {
    let name_bytes = filename.as_bytes();
    let title_bytes = title.as_bytes();
    let title_len = title_bytes.len().min(TITLE_CAP);
    let line_len = name_bytes.len() + 1 + title_len + 1; // name + \t + title + \n
    if line_len > 128 {
        return Err(Error::new(
            ErrorKind::WriteFailed,
            "save_title: line too long",
        ));
    }
    let mut line = [0u8; 128];
    line[..name_bytes.len()].copy_from_slice(name_bytes);
    line[name_bytes.len()] = b'\t';
    line[name_bytes.len() + 1..name_bytes.len() + 1 + title_len]
        .copy_from_slice(&title_bytes[..title_len]);
    line[name_bytes.len() + 1 + title_len] = b'\n';

    append_file_in_dir(sd, X4_DIR, TITLES_FILE, &line[..line_len])
}
