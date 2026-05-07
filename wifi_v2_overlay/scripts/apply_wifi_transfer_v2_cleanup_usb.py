from pathlib import Path
import re
import sys

root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()

storage = root / "vendor/pulp-os/kernel/src/drivers/storage.rs"
upload = root / "vendor/pulp-os/src/apps/upload.rs"
upload_html = root / "vendor/pulp-os/assets/upload.html"
home = root / "vendor/pulp-os/src/apps/home.rs"
apps_mod = root / "vendor/pulp-os/src/apps/mod.rs"
manager = root / "vendor/pulp-os/src/apps/manager.rs"
usb_readme = root / "tools/usb_transfer/README.md"

errors: list[str] = []


def read(path: Path) -> str:
    if not path.exists():
        errors.append(f"missing file: {path}")
        return ""
    return path.read_text()


def write(path: Path, text: str) -> None:
    path.write_text(text)


def insert_before_required(text: str, anchor: str, addition: str, label: str) -> str:
    if anchor not in text:
        errors.append(f"could not find anchor for {label}: {anchor!r}")
        return text
    return text.replace(anchor, addition.rstrip() + "\n\n" + anchor, 1)


def remove_match_arm(text: str, arm_prefix: str) -> str:
    start = text.find(arm_prefix)
    if start < 0:
        return text

    brace = text.find("{", start)
    if brace < 0:
        return text

    depth = 0
    end = None
    for i in range(brace, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                break

    if end is None:
        return text

    while end < len(text) and text[end] in " \t\r\n,":
        end += 1

    return text[:start] + text[end:]


# ---------------------------------------------------------------------
# 1. Storage helpers needed by chunked nested writes.
# ---------------------------------------------------------------------

s = read(storage)

if s:
    storage_additions: list[str] = []

    if "pub fn file_size_in_dir(" not in s:
        storage_additions.append(r'''
pub fn file_size_in_dir(sd: &SdStorage, dir: &str, name: &str) -> crate::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_file_size!(inner, dir_h, name))
    })
}
''')

    if "pub fn file_size_in_subdir(" not in s:
        storage_additions.append(r'''
pub fn file_size_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
) -> crate::error::Result<u32> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_file_size!(
            inner, dir_h, name
        ))
    })
}
''')

    if "pub fn ensure_dir_in_dir(" not in s:
        storage_additions.append(r'''
pub fn ensure_dir_in_dir(sd: &SdStorage, dir: &str, name: &str) -> crate::error::Result<()> {
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
''')

    if "pub fn write_file_in_subdir(" not in s:
        storage_additions.append(r'''
pub fn write_file_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    data: &[u8],
) -> crate::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_write!(
            inner, dir_h, name, data
        ))
    })
}
''')

    if "pub fn append_file_in_subdir(" not in s:
        storage_additions.append(r'''
pub fn append_file_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
    data: &[u8],
) -> crate::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_append!(
            inner, dir_h, name, data
        ))
    })
}
''')

    if "pub fn delete_file_in_dir(" not in s:
        storage_additions.append(r'''
pub fn delete_file_in_dir(sd: &SdStorage, dir: &str, name: &str) -> crate::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_dir!(inner, dir, |dir_h| op_delete!(inner, dir_h, name))
    })
}
''')

    if "pub fn delete_file_in_subdir(" not in s:
        storage_additions.append(r'''
pub fn delete_file_in_subdir(
    sd: &SdStorage,
    dir: &str,
    subdir: &str,
    name: &str,
) -> crate::error::Result<()> {
    poll_once(async {
        let mut guard = borrow(sd)?;
        let inner = &mut *guard;
        in_subdir!(inner, dir, subdir, |dir_h| op_delete!(
            inner, dir_h, name
        ))
    })
}
''')

    if storage_additions:
        s = insert_before_required(
            s,
            "\n// async boot path",
            "\n".join(storage_additions),
            "Wi-Fi v2 storage helpers",
        )

    write(storage, s)


# ---------------------------------------------------------------------
# 2. Wi-Fi Transfer v2 endpoints.
# ---------------------------------------------------------------------

u = read(upload)

if u:
    if "const WIFI_V2_MAX_PATH" not in u:
        old = "const DIR_LIST_MAX: usize = 64;\n"
        new = (
            "const DIR_LIST_MAX: usize = 64;\n\n"
            "const WIFI_V2_MAX_PATH: usize = 96;\n"
            "const WIFI_V2_MAX_CHUNK: usize = 1536;\n"
        )
        if old not in u:
            errors.append("could not find DIR_LIST_MAX constant")
        else:
            u = u.replace(old, new, 1)

    if "let request_target = wifi_v2_request_target(request_line);" not in u:
        old = "    let path = extract_path(request_line);\n"
        new = (
            "    let path = extract_path(request_line);\n"
            "    let request_target = wifi_v2_request_target(request_line);\n"
        )
        if old not in u:
            errors.append("could not find extract_path request line")
        else:
            u = u.replace(old, new, 1)

    route_block = r'''
    if is_get && path == b"/v2/stat" {
        handle_v2_stat(&mut socket, sd, request_target).await;
        close_socket(&mut socket).await;
        return ServerEvent::Nothing;
    }

    if is_post && path == b"/v2/mkdir" {
        handle_v2_mkdir(&mut socket, sd, request_target).await;
        close_socket(&mut socket).await;
        return ServerEvent::Nothing;
    }

    if is_post && path == b"/v2/chunk" {
        handle_v2_chunk(&mut socket, sd, headers, initial_body, request_target).await;
        close_socket(&mut socket).await;
        return ServerEvent::Nothing;
    }

'''
    if 'path == b"/v2/chunk"' not in u:
        anchor = '    if is_post && path == b"/upload" {\n'
        if anchor not in u:
            errors.append("could not find /upload route anchor")
        else:
            u = u.replace(anchor, route_block + anchor, 1)

    v2_helpers = r'''
async fn handle_v2_stat(socket: &mut TcpSocket<'_>, sd: &SdStorage, target: &[u8]) {
    let mut path_buf = [0u8; WIFI_V2_MAX_PATH];
    let path_len = match query_path(target, b"p", &mut path_buf) {
        Ok(n) => n,
        Err(e) => {
            send_error_response(socket, e).await;
            return;
        }
    };

    let Some(path) = core::str::from_utf8(&path_buf[..path_len]).ok() else {
        send_error_response(socket, "bad path").await;
        return;
    };

    let mut out = [0u8; 96];
    let mut pos = 0usize;

    match wifi_v2_file_size(sd, path) {
        Ok(size) => {
            let prefix = b"{\"exists\":true,\"size\":";
            out[..prefix.len()].copy_from_slice(prefix);
            pos += prefix.len();
            pos += fmt_u32(size, &mut out[pos..]);
            out[pos] = b'}';
            pos += 1;
        }
        Err(_) => {
            let body = b"{\"exists\":false,\"size\":0}";
            out[..body.len()].copy_from_slice(body);
            pos = body.len();
        }
    }

    let _ = socket.write_all(HTTP_200_JSON).await;
    let _ = socket.write_all(&out[..pos]).await;
    let _ = socket.flush().await;
}

async fn handle_v2_mkdir(socket: &mut TcpSocket<'_>, sd: &SdStorage, target: &[u8]) {
    let mut path_buf = [0u8; WIFI_V2_MAX_PATH];
    let path_len = match query_path(target, b"p", &mut path_buf) {
        Ok(n) => n,
        Err(e) => {
            send_error_response(socket, e).await;
            return;
        }
    };

    let Some(path) = core::str::from_utf8(&path_buf[..path_len]).ok() else {
        send_error_response(socket, "bad path").await;
        return;
    };

    match wifi_v2_ensure_dir(sd, path) {
        Ok(()) => send_ok_json(socket).await,
        Err(_) => send_error_response(socket, "mkdir failed").await,
    }
}

async fn handle_v2_chunk(
    socket: &mut TcpSocket<'_>,
    sd: &SdStorage,
    headers: &[u8],
    initial_body: &[u8],
    target: &[u8],
) {
    let mut path_buf = [0u8; WIFI_V2_MAX_PATH];
    let path_len = match query_path(target, b"p", &mut path_buf) {
        Ok(n) => n,
        Err(e) => {
            send_error_response(socket, e).await;
            return;
        }
    };

    let Some(path) = core::str::from_utf8(&path_buf[..path_len]).ok() else {
        send_error_response(socket, "bad path").await;
        return;
    };

    let offset = match query_u32(target, b"o") {
        Ok(v) => v,
        Err(e) => {
            send_error_response(socket, e).await;
            return;
        }
    };

    let content_len = extract_content_length(headers).unwrap_or(0);
    if content_len == 0 || content_len > WIFI_V2_MAX_CHUNK {
        send_error_response(socket, "bad chunk size").await;
        return;
    }

    let mut body = [0u8; WIFI_V2_MAX_CHUNK];
    let have = initial_body.len().min(content_len).min(body.len());
    body[..have].copy_from_slice(&initial_body[..have]);
    let mut body_len = have;

    while body_len < content_len {
        match socket.read(&mut body[body_len..content_len]).await {
            Ok(0) => break,
            Ok(n) => body_len += n,
            Err(_) => {
                send_error_response(socket, "read failed").await;
                return;
            }
        }
    }

    if body_len != content_len {
        send_error_response(socket, "short chunk").await;
        return;
    }

    let chunk = &body[..body_len];

    match wifi_v2_write_chunk(sd, path, offset, chunk) {
        Ok(size) => {
            let mut out = [0u8; 96];
            let mut pos = 0usize;
            let prefix = b"{\"ok\":true,\"size\":";
            out[..prefix.len()].copy_from_slice(prefix);
            pos += prefix.len();
            pos += fmt_u32(size, &mut out[pos..]);
            out[pos] = b'}';
            pos += 1;

            let _ = socket.write_all(HTTP_200_JSON).await;
            let _ = socket.write_all(&out[..pos]).await;
            let _ = socket.flush().await;
        }
        Err(e) => send_error_response(socket, e).await,
    }
}

async fn send_ok_json(socket: &mut TcpSocket<'_>) {
    let _ = socket.write_all(HTTP_200_JSON).await;
    let _ = socket.write_all(b"{\"ok\":true}").await;
    let _ = socket.flush().await;
}

fn wifi_v2_write_chunk(
    sd: &SdStorage,
    path: &str,
    offset: u32,
    data: &[u8],
) -> Result<u32, &'static str> {
    let current = wifi_v2_file_size(sd, path).unwrap_or(0);
    let next = offset.saturating_add(data.len() as u32);

    if offset == 0 {
        let _ = wifi_v2_delete_file(sd, path);
        wifi_v2_write_file(sd, path, data).map_err(|_| "write failed")?;
        return Ok(data.len() as u32);
    }

    if current >= next {
        return Ok(current);
    }

    if current != offset {
        return Err("resume offset mismatch");
    }

    wifi_v2_append_file(sd, path, data).map_err(|_| "append failed")?;
    Ok(next)
}

fn wifi_v2_file_size(sd: &SdStorage, path: &str) -> crate::error::Result<u32> {
    let parts = WifiV2PathParts::parse(path).map_err(|_| {
        crate::error::Error::new(crate::error::ErrorKind::InvalidData, "wifi_v2_size_path")
    })?;

    match parts.count {
        1 => storage::file_size(sd, parts.get(0)),
        2 => storage::file_size_in_dir(sd, parts.get(0), parts.get(1)),
        3 => storage::file_size_in_subdir(sd, parts.get(0), parts.get(1), parts.get(2)),
        _ => Err(crate::error::Error::new(
            crate::error::ErrorKind::InvalidData,
            "wifi_v2_size_depth",
        )),
    }
}

fn wifi_v2_write_file(sd: &SdStorage, path: &str, data: &[u8]) -> crate::error::Result<()> {
    let parts = WifiV2PathParts::parse(path).map_err(|_| {
        crate::error::Error::new(crate::error::ErrorKind::InvalidData, "wifi_v2_write_path")
    })?;

    match parts.count {
        1 => storage::write_file(sd, parts.get(0), data),
        2 => {
            let _ = storage::ensure_dir(sd, parts.get(0));
            storage::write_file_in_dir(sd, parts.get(0), parts.get(1), data)
        }
        3 => {
            let _ = storage::ensure_dir(sd, parts.get(0));
            let _ = storage::ensure_dir_in_dir(sd, parts.get(0), parts.get(1));
            storage::write_file_in_subdir(sd, parts.get(0), parts.get(1), parts.get(2), data)
        }
        _ => Err(crate::error::Error::new(
            crate::error::ErrorKind::InvalidData,
            "wifi_v2_write_depth",
        )),
    }
}

fn wifi_v2_append_file(sd: &SdStorage, path: &str, data: &[u8]) -> crate::error::Result<()> {
    let parts = WifiV2PathParts::parse(path).map_err(|_| {
        crate::error::Error::new(crate::error::ErrorKind::InvalidData, "wifi_v2_append_path")
    })?;

    match parts.count {
        1 => storage::append_root_file(sd, parts.get(0), data),
        2 => storage::append_file_in_dir(sd, parts.get(0), parts.get(1), data),
        3 => storage::append_file_in_subdir(sd, parts.get(0), parts.get(1), parts.get(2), data),
        _ => Err(crate::error::Error::new(
            crate::error::ErrorKind::InvalidData,
            "wifi_v2_append_depth",
        )),
    }
}

fn wifi_v2_delete_file(sd: &SdStorage, path: &str) -> crate::error::Result<()> {
    let parts = WifiV2PathParts::parse(path).map_err(|_| {
        crate::error::Error::new(crate::error::ErrorKind::InvalidData, "wifi_v2_delete_path")
    })?;

    match parts.count {
        1 => storage::delete_file(sd, parts.get(0)),
        2 => storage::delete_file_in_dir(sd, parts.get(0), parts.get(1)),
        3 => storage::delete_file_in_subdir(sd, parts.get(0), parts.get(1), parts.get(2)),
        _ => Err(crate::error::Error::new(
            crate::error::ErrorKind::InvalidData,
            "wifi_v2_delete_depth",
        )),
    }
}

fn wifi_v2_ensure_dir(sd: &SdStorage, path: &str) -> crate::error::Result<()> {
    if path.is_empty() {
        return Ok(());
    }

    let parts = WifiV2PathParts::parse(path).map_err(|_| {
        crate::error::Error::new(crate::error::ErrorKind::InvalidData, "wifi_v2_mkdir_path")
    })?;

    match parts.count {
        1 => storage::ensure_dir(sd, parts.get(0)),
        2 => {
            let _ = storage::ensure_dir(sd, parts.get(0));
            storage::ensure_dir_in_dir(sd, parts.get(0), parts.get(1))
        }
        _ => Err(crate::error::Error::new(
            crate::error::ErrorKind::InvalidData,
            "wifi_v2_mkdir_depth",
        )),
    }
}

struct WifiV2PathParts<'a> {
    raw: &'a str,
    starts: [usize; 3],
    lens: [usize; 3],
    count: usize,
}

impl<'a> WifiV2PathParts<'a> {
    fn parse(path: &'a str) -> Result<Self, &'static str> {
        let raw = path.trim_matches('/');
        if raw.is_empty() {
            return Err("empty path");
        }

        let mut out = Self {
            raw,
            starts: [0; 3],
            lens: [0; 3],
            count: 0,
        };

        let bytes = raw.as_bytes();
        let mut start = 0usize;

        for i in 0..=bytes.len() {
            if i == bytes.len() || bytes[i] == b'/' {
                if out.count >= out.starts.len() || i == start {
                    return Err("bad path depth");
                }

                let part = &raw[start..i];
                validate_wifi_v2_component(part.as_bytes())?;

                out.starts[out.count] = start;
                out.lens[out.count] = i - start;
                out.count += 1;
                start = i + 1;
            }
        }

        Ok(out)
    }

    fn get(&self, idx: usize) -> &str {
        let start = self.starts[idx];
        let len = self.lens[idx];
        &self.raw[start..start + len]
    }
}

fn validate_wifi_v2_component(part: &[u8]) -> Result<(), &'static str> {
    if part.is_empty() || part.len() > 13 || part == b"." || part == b".." {
        return Err("bad path component");
    }

    for &b in part {
        let ok = b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'~');
        if !ok {
            return Err("bad path character");
        }
    }

    Ok(())
}

fn query_path(target: &[u8], key: &[u8], out: &mut [u8]) -> Result<usize, &'static str> {
    let len = query_value_decoded(target, key, out)?;
    let mut tmp = [0u8; WIFI_V2_MAX_PATH];
    tmp[..len].copy_from_slice(&out[..len]);
    normalize_wifi_v2_path(&tmp[..len], out)
}

fn query_u32(target: &[u8], key: &[u8]) -> Result<u32, &'static str> {
    let mut buf = [0u8; 16];
    let len = query_value_decoded(target, key, &mut buf)?;
    let mut value = 0u32;

    for &b in &buf[..len] {
        if !b.is_ascii_digit() {
            return Err("bad number");
        }
        value = value
            .saturating_mul(10)
            .saturating_add((b - b'0') as u32);
    }

    Ok(value)
}

fn query_value_decoded(
    target: &[u8],
    key: &[u8],
    out: &mut [u8],
) -> Result<usize, &'static str> {
    let Some(qpos) = target.iter().position(|&b| b == b'?') else {
        return Err("missing query");
    };

    let query = &target[qpos + 1..];
    let mut start = 0usize;

    while start < query.len() {
        let end = query[start..]
            .iter()
            .position(|&b| b == b'&')
            .map(|p| start + p)
            .unwrap_or(query.len());
        let pair = &query[start..end];

        if pair.starts_with(key) && pair.get(key.len()) == Some(&b'=') {
            return wifi_v2_url_decode_to_buf(&pair[key.len() + 1..], out).ok_or("bad encoding");
        }

        start = end.saturating_add(1);
    }

    Err("missing query key")
}

fn normalize_wifi_v2_path(raw: &[u8], out: &mut [u8]) -> Result<usize, &'static str> {
    let mut src = wifi_v2_trim_ascii(raw);

    while src.first() == Some(&b'/') {
        src = &src[1..];
    }
    while src.last() == Some(&b'/') {
        src = &src[..src.len() - 1];
    }

    if src.is_empty() || src.len() >= out.len() {
        return Err("bad path length");
    }

    let mut pos = 0usize;
    let mut comp_start = 0usize;
    let mut comp_count = 0usize;

    for i in 0..=src.len() {
        if i == src.len() || src[i] == b'/' {
            let comp = &src[comp_start..i];
            validate_wifi_v2_component(comp)?;

            if comp_count >= 3 {
                return Err("path too deep");
            }

            if comp_count > 0 {
                out[pos] = b'/';
                pos += 1;
            }

            out[pos..pos + comp.len()].copy_from_slice(comp);
            pos += comp.len();
            comp_count += 1;
            comp_start = i + 1;
        }
    }

    Ok(pos)
}

fn wifi_v2_url_decode_to_buf(input: &[u8], out: &mut [u8]) -> Option<usize> {
    let mut i = 0usize;
    let mut pos = 0usize;

    while i < input.len() {
        if pos >= out.len() {
            return None;
        }

        match input[i] {
            b'%' => {
                if i + 2 >= input.len() {
                    return None;
                }
                let hi = wifi_v2_hex_val(input[i + 1])?;
                let lo = wifi_v2_hex_val(input[i + 2])?;
                out[pos] = (hi << 4) | lo;
                pos += 1;
                i += 3;
            }
            b'+' => {
                out[pos] = b' ';
                pos += 1;
                i += 1;
            }
            b => {
                out[pos] = b;
                pos += 1;
                i += 1;
            }
        }
    }

    Some(pos)
}

fn wifi_v2_hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn wifi_v2_trim_ascii(input: &[u8]) -> &[u8] {
    let mut start = 0usize;
    let mut end = input.len();

    while start < end && input[start].is_ascii_whitespace() {
        start += 1;
    }
    while end > start && input[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &input[start..end]
}

fn wifi_v2_request_target(line: &[u8]) -> &[u8] {
    let start = match line.iter().position(|&b| b == b' ') {
        Some(p) => p + 1,
        None => return b"/",
    };

    let rest = &line[start..];
    let end = rest.iter().position(|&b| b == b' ').unwrap_or(rest.len());
    &rest[..end]
}

'''
    if "async fn handle_v2_chunk(" not in u:
        anchor = "\nasync fn handle_upload(\n"
        if anchor not in u:
            errors.append("could not find handle_upload anchor")
        else:
            u = u.replace(anchor, "\n" + v2_helpers + anchor, 1)

    write(upload, u)


# ---------------------------------------------------------------------
# 3. Replace browser page with chunked folder uploader.
# ---------------------------------------------------------------------

upload_page = r'''<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>X4 Wi-Fi Transfer v2</title>
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>
body{font-family:system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;margin:20px;max-width:980px}
h1{font-size:24px;margin-bottom:4px}.card{border:1px solid #bbb;border-radius:10px;padding:14px;margin:14px 0}
label{display:block;font-weight:600;margin-top:10px}input,button{font:inherit;padding:8px;margin-top:6px}
input[type=text],input[type=number]{width:100%;box-sizing:border-box}button{cursor:pointer}button:disabled{opacity:.5;cursor:not-allowed}
progress{width:100%;height:20px}pre{background:#f6f6f6;border:1px solid #ddd;border-radius:8px;padding:10px;white-space:pre-wrap;max-height:300px;overflow:auto}
.small{color:#555;font-size:13px}.row{display:flex;gap:10px;flex-wrap:wrap}.row>*{flex:1}
</style>
</head>
<body>
<h1>X4 Wi-Fi Transfer v2</h1>
<div class="small">Chunked folder upload with resume. Use this for prepared cache folders such as <code>/FCACHE/15D1296A</code>.</div>

<div class="card">
  <label>Target folder on SD</label>
  <input id="target" type="text" value="/FCACHE/15D1296A">
  <div class="small">Select the local cache folder itself, for example <code>/tmp/FCACHE/15D1296A</code>. If folder selection is unsupported, select all files inside the folder.</div>

  <label>Chunk size</label>
  <input id="chunkSize" type="number" min="256" max="1536" value="1024">

  <label>Folder/files</label>
  <input id="files" type="file" webkitdirectory directory multiple>
  <div class="row"><button id="start">Upload / Resume</button><button id="stop" disabled>Stop</button></div>
  <p id="summary">No files selected.</p>
  <progress id="progress" value="0" max="1"></progress>
</div>

<div class="card"><h3>Status</h3><pre id="log"></pre></div>

<script>
const $ = (id) => document.getElementById(id);
let stopRequested = false;
function log(line){const el=$("log");el.textContent+=line+"\n";el.scrollTop=el.scrollHeight;}
function cleanTarget(path){path=(path||"").trim().replaceAll("\\","/");if(!path.startsWith("/"))path="/"+path;path=path.replace(/\/+/g,"/");if(path.length>1&&path.endsWith("/"))path=path.slice(0,-1);return path;}
function stripRootFolder(file){const rel=file.webkitRelativePath||file.name;const parts=rel.split("/").filter(Boolean);if(parts.length>1)parts.shift();return parts.join("/")||file.name;}
function targetPathFor(file){return cleanTarget($("target").value)+"/"+stripRootFolder(file);}
async function apiJson(url,options){const res=await fetch(url,options||{});const text=await res.text();if(!res.ok)throw new Error(text||("HTTP "+res.status));try{return JSON.parse(text);}catch(_){return {};}}
async function mkdir(path){await apiJson("/v2/mkdir?p="+encodeURIComponent(path),{method:"POST"});}
async function stat(path){return await apiJson("/v2/stat?p="+encodeURIComponent(path));}
async function sendChunk(path,offset,blob){const url="/v2/chunk?p="+encodeURIComponent(path)+"&o="+offset;return await apiJson(url,{method:"POST",body:blob});}
async function sendChunkWithRetry(path,offset,blob,nextOffset){for(let attempt=1;attempt<=4;attempt++){try{return await sendChunk(path,offset,blob);}catch(err){const st=await stat(path).catch(()=>({exists:false,size:0}));if(st.exists&&st.size>=nextOffset){log("  retry recovered by stat: "+path+" size="+st.size);return st;}if(attempt===4)throw err;log("  retry "+attempt+" after error: "+err.message);await new Promise((resolve)=>setTimeout(resolve,250*attempt));}}}
async function uploadFile(file,index,total,state){const path=targetPathFor(file);const chunkSize=Math.max(256,Math.min(1536,parseInt($("chunkSize").value||"1024",10)));const existing=await stat(path).catch(()=>({exists:false,size:0}));if(existing.exists&&existing.size===file.size){log("skip complete ["+index+"/"+total+"] "+path);state.done+=file.size;$("progress").value=state.done;return;}let offset=existing.exists&&existing.size<file.size?existing.size:0;if(offset>0)log("resume ["+index+"/"+total+"] "+path+" from "+offset);else log("upload ["+index+"/"+total+"] "+path+" ("+file.size+" bytes)");while(offset<file.size){if(stopRequested)throw new Error("stopped by user");const next=Math.min(offset+chunkSize,file.size);const blob=file.slice(offset,next);await sendChunkWithRetry(path,offset,blob,next);const wrote=next-offset;offset=next;state.done+=wrote;$("progress").value=state.done;$("summary").textContent=path+" "+offset+"/"+file.size;}}
async function startUpload(){stopRequested=false;$("start").disabled=true;$("stop").disabled=false;$("log").textContent="";const files=Array.from($("files").files||[]);if(!files.length){log("No files selected.");$("start").disabled=false;$("stop").disabled=true;return;}const target=cleanTarget($("target").value);$("target").value=target;const dirs=[];const parts=target.split("/").filter(Boolean);if(parts.length>=1)dirs.push("/"+parts[0]);if(parts.length>=2)dirs.push("/"+parts[0]+"/"+parts[1]);try{for(const dir of dirs){log("mkdir "+dir);await mkdir(dir).catch((err)=>log("  mkdir warning: "+err.message));}const totalBytes=files.reduce((sum,f)=>sum+f.size,0);const state={done:0};$("progress").max=totalBytes||1;$("progress").value=0;log("files="+files.length+" bytes="+totalBytes);for(let i=0;i<files.length;i++){await uploadFile(files[i],i+1,files.length,state);}$("summary").textContent="Upload complete.";log("complete");}catch(err){$("summary").textContent="Upload stopped/failed: "+err.message;log("ERROR: "+err.message);log("Press Upload / Resume to continue from the device file sizes.");}finally{$("start").disabled=false;$("stop").disabled=true;}}
$("start").addEventListener("click",startUpload);$("stop").addEventListener("click",()=>{stopRequested=true;$("stop").disabled=true;});$("files").addEventListener("change",()=>{const files=Array.from($("files").files||[]);const bytes=files.reduce((sum,f)=>sum+f.size,0);$("summary").textContent=files.length+" files selected, "+bytes+" bytes.";});
</script>
</body>
</html>
'''

if upload_html.exists():
    write(upload_html, upload_page)
else:
    errors.append(f"missing file: {upload_html}")


# ---------------------------------------------------------------------
# 4. Hide USB Transfer option and stop exporting experimental USB module.
# ---------------------------------------------------------------------

h = read(home)
if h:
    h = re.sub(r'(\s*5\s*=>\s*)3(,\s*)', r'\g<1>2\2', h, count=1)
    h = h.replace('            (5, 2) => "USB Transfer",\n', "")
    h = h.replace('            (5, 2) => "Bulk SD transfer",\n', "")

    h = h.replace(
        '''            // Tools
            (5, 0) => Transition::Push(AppId::Files),
            (5, 1) => {
                self.open_placeholder(
                    "QR Generator",
                    "Placeholder",
                    ReturnTarget::CategoryItems,
                    ctx,
                );
                Transition::None
            }
            (5, _) => Transition::Push(AppId::UsbTransfer),
''',
        '''            // Tools
            (5, 0) => Transition::Push(AppId::Files),
            (5, _) => {
                self.open_placeholder(
                    "QR Generator",
                    "Placeholder",
                    ReturnTarget::CategoryItems,
                    ctx,
                );
                Transition::None
            }
''',
    )

    h = h.replace('''            (5, 2) => Transition::Push(AppId::UsbTransfer),\n''', "")
    h = h.replace(
        '''            (5, _) => Transition::Push(AppId::UsbTransfer),
''',
        '''            (5, _) => {
                self.open_placeholder(
                    "QR Generator",
                    "Placeholder",
                    ReturnTarget::CategoryItems,
                    ctx,
                );
                Transition::None
            }
''',
    )

    if "AppId::UsbTransfer" in h:
        errors.append("home.rs still references AppId::UsbTransfer after cleanup")

    write(home, h)

am = read(apps_mod)
if am:
    am = am.replace("pub mod usb_transfer;\n", "")
    am = re.sub(r'\n\s*// USB[^\n]*\n\s*UsbTransfer,\n', "\n", am)
    am = re.sub(r'\n\s*// USB bulk transfer[^\n]*\n\s*UsbTransfer,\n', "\n", am)
    am = am.replace("    UsbTransfer,\n", "")
    write(apps_mod, am)

mgr = read(manager)
if mgr:
    mgr = mgr.replace(" | AppId::UsbTransfer", "")
    mgr = mgr.replace("AppId::UsbTransfer | ", "")
    mgr = remove_match_arm(mgr, "            AppId::UsbTransfer =>")
    mgr = remove_match_arm(mgr, "                AppId::UsbTransfer =>")
    if "AppId::UsbTransfer" in mgr:
        errors.append("manager.rs still references AppId::UsbTransfer after cleanup")
    write(manager, mgr)

if usb_readme.exists():
    text = usb_readme.read_text()
    if "## Status: experimental and hidden" not in text:
        text += r'''

## Status: experimental and hidden

USB bulk transfer was tested as a custom USB Serial/JTAG protocol. It is not exposed in the X4 Tools menu now.

Use Wi-Fi Transfer v2 for large prepared cache uploads:

```text
Apps > Network > Wi-Fi Transfer
Browser page: X4 Wi-Fi Transfer v2
```

Reason:

```text
- X4/ESP32-C3 does not support normal USB mass-storage file transfer.
- The custom USB serial path conflicts with monitor/console use and is still experimental.
- Chunked Wi-Fi upload with resume is the supported large-transfer path.
```
'''
        write(usb_readme, text)

if errors:
    print("Wi-Fi Transfer v2 patch failed:", file=sys.stderr)
    for err in errors:
        print(f"- {err}", file=sys.stderr)
    print("\nInspect with:", file=sys.stderr)
    print("rg -n \"UsbTransfer|/v2/chunk|handle_v2|WIFI_V2\" vendor/pulp-os/src/apps vendor/pulp-os/kernel/src/drivers/storage.rs", file=sys.stderr)
    sys.exit(1)

print("Wi-Fi Transfer v2 chunked upload/resume applied; USB option hidden")
