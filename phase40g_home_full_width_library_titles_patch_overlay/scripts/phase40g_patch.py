from pathlib import Path
import sys
home, files, dircache, out, patched_list = map(Path, sys.argv[1:])
root = Path.cwd()
patched, notes = [], []

def write_changed(path, text, orig, note):
    if text != orig:
        path.write_text(text)
        patched.append(str(path.relative_to(root)))
        notes.append(note)

# home.rs: full-width current title preview
text = home.read_text(); orig = text
if "PHASE40G_HOME_RECENT_PREVIEW_W" not in text:
    text = text.replace("const RECENT_BUF_LEN: usize = 160;\n", "const RECENT_BUF_LEN: usize = 160;\n\nconst PHASE40G_HOME_RECENT_PREVIEW_X: u16 = LARGE_MARGIN;\nconst PHASE40G_HOME_RECENT_PREVIEW_W: u16 = FULL_CONTENT_W;\nconst PHASE40G_HOME_RECENT_PREVIEW_LINES: u16 = 1;\n", 1)
text = text.replace("        body_line_h + RECENT_PREVIEW_GAP\n", "        body_line_h * PHASE40G_HOME_RECENT_PREVIEW_LINES + RECENT_PREVIEW_GAP\n", 1)
text = text.replace("""        let title_region = Region::new(
            ITEM_X,
            CONTENT_TOP + 8,
            ITEM_W,
            self.ui_fonts.heading.line_height,
        );""", """        let title_region = Region::new(
            LARGE_MARGIN,
            CONTENT_TOP + 8,
            FULL_CONTENT_W,
            self.ui_fonts.heading.line_height,
        );""", 1)
text = text.replace("""    fn recent_preview_region(&self) -> Region {
        Region::new(
            ITEM_X,
            CONTENT_TOP + 8 + self.ui_fonts.heading.line_height + 6,
            ITEM_W,
            self.ui_fonts.body.line_height,
        )
    }""", """    fn recent_preview_region(&self) -> Region {
        Region::new(
            PHASE40G_HOME_RECENT_PREVIEW_X,
            CONTENT_TOP + 8 + self.ui_fonts.heading.line_height + 6,
            PHASE40G_HOME_RECENT_PREVIEW_W,
            self.ui_fonts.body.line_height * PHASE40G_HOME_RECENT_PREVIEW_LINES,
        )
    }""", 1)
text = text.replace("BitmapDynLabel::<48>::new(self.recent_preview_region(), self.ui_fonts.body)", "BitmapDynLabel::<96>::new(self.recent_preview_region(), self.ui_fonts.body)")
if "phase40g=x4-home-full-width-library-title-patch-ok" not in text:
    text = text.replace("// launcher screen: menu, bookmarks browser\n", "// launcher screen: menu, bookmarks browser\n// phase40g=x4-home-full-width-library-title-patch-ok\n", 1)
write_changed(home, text, orig, "home.rs: full-width Home current title preview")

# dir_cache.rs: EPU + TXT title candidates
text = dircache.read_text(); orig = text
if "PHASE40G_DIR_TITLE_KIND_EPUB" not in text:
    text = text.replace("const MAX_DIR_ENTRIES: usize = 128;\n", "const MAX_DIR_ENTRIES: usize = 128;\n\npub const PHASE40G_DIR_TITLE_KIND_EPUB: u8 = 1;\npub const PHASE40G_DIR_TITLE_KIND_TEXT: u8 = 2;\n", 1)
if "fn phase40g_is_text_title_name" not in text:
    text = text.replace("impl DirCache {", """fn phase40g_is_text_title_name(name: &[u8]) -> bool {
    name.len() >= 4
        && name[name.len() - 4] == b'.'
        && name[name.len() - 3..].eq_ignore_ascii_case(b"TXT")
}

impl DirCache {""", 1)
old = """    pub fn next_untitled_epub(&self, from: usize) -> Option<(usize, [u8; 13], u8)> {
        for i in from..self.count {
            let e = &self.entries[i];
            if e.has_real_title() || e.is_dir {
                continue;
            }
            let name = e.name_str().as_bytes();
            if name.len() >= 5
                && name[name.len() - 5..name.len() - 4] == [b'.']
                && name[name.len() - 4..].eq_ignore_ascii_case(b"EPUB")
            {
                return Some((i, e.name, e.name_len));
            }
        }
        None
    }
"""
new = """    pub fn next_untitled_epub(&self, from: usize) -> Option<(usize, [u8; 13], u8)> {
        for i in from..self.count {
            let e = &self.entries[i];
            if e.has_real_title() || e.is_dir {
                continue;
            }
            let name = e.name_str().as_bytes();
            if phase38i_is_epub_or_epu_name(name) {
                return Some((i, e.name, e.name_len));
            }
        }
        None
    }

    pub fn next_untitled_reader_title(&self, from: usize) -> Option<(usize, [u8; 13], u8, u8)> {
        for i in from..self.count {
            let e = &self.entries[i];
            if e.has_real_title() || e.is_dir {
                continue;
            }
            let name = e.name_str().as_bytes();
            if phase38i_is_epub_or_epu_name(name) {
                return Some((i, e.name, e.name_len, PHASE40G_DIR_TITLE_KIND_EPUB));
            }
            if phase40g_is_text_title_name(name) {
                return Some((i, e.name, e.name_len, PHASE40G_DIR_TITLE_KIND_TEXT));
            }
        }
        None
    }
"""
if old in text:
    text = text.replace(old, new, 1)
elif "next_untitled_reader_title" not in text:
    raise SystemExit("dir_cache.rs: could not patch next_untitled_epub")
if "phase40g=x4-home-full-width-library-title-patch-ok" not in text:
    text = text.replace("// directory listing cache: sorted entries with title resolution\n", "// directory listing cache: sorted entries with title resolution\n// phase40g=x4-home-full-width-library-title-patch-ok\n", 1)
write_changed(dircache, text, orig, "dir_cache.rs: EPU/TXT title candidates")

# files.rs: scanner dispatch + TXT title extraction
text = files.read_text(); orig = text
if "const PHASE40G_DIR_TITLE_KIND_TEXT: u8 = 2;" not in text:
    text = text.replace("const MAX_PAGE_SIZE: usize = 14;\n", "const MAX_PAGE_SIZE: usize = 14;\nconst PHASE40G_DIR_TITLE_KIND_TEXT: u8 = 2;\nconst PHASE40G_TEXT_TITLE_SCAN_BYTES: usize = 768;\nconst PHASE40G_TEXT_TITLE_MAX_BYTES: usize = 96;\n", 1)
text = text.replace("if let Some(dirty) = scan_one_epub_title(k, self.title_scan_idx) {", "if let Some(dirty) = scan_one_reader_title(k, self.title_scan_idx) {", 1)
helper = r'''
fn phase40g_is_ascii_space(byte: u8) -> bool { matches!(byte, b' ' | b'\t' | b'\r' | b'\n') }

fn phase40g_contains_icase(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() { return false; }
    let limit = haystack.len() - needle.len();
    let mut start = 0usize;
    while start <= limit {
        let mut matched = true;
        let mut i = 0usize;
        while i < needle.len() {
            if !haystack[start + i].eq_ignore_ascii_case(&needle[i]) { matched = false; break; }
            i += 1;
        }
        if matched { return true; }
        start += 1;
    }
    false
}

fn phase40g_skip_text_title_line(line: &[u8]) -> bool {
    line.is_empty()
        || phase40g_contains_icase(line, b"project gutenberg")
        || phase40g_contains_icase(line, b"ebook")
        || phase40g_contains_icase(line, b"produced by")
        || phase40g_contains_icase(line, b"transcribed by")
        || phase40g_contains_icase(line, b"***")
}

fn phase40g_copy_text_title(line: &[u8], out: &mut [u8]) -> usize {
    let mut start = 0usize;
    let mut end = line.len();
    while start < end && phase40g_is_ascii_space(line[start]) { start += 1; }
    while end > start && phase40g_is_ascii_space(line[end - 1]) { end -= 1; }
    let title_prefix = b"Title:";
    if end.saturating_sub(start) > title_prefix.len()
        && line[start..start + title_prefix.len()].eq_ignore_ascii_case(title_prefix)
    {
        start += title_prefix.len();
        while start < end && phase40g_is_ascii_space(line[start]) { start += 1; }
    }
    let mut written = 0usize;
    let mut prev_space = false;
    let max = out.len().min(PHASE40G_TEXT_TITLE_MAX_BYTES);
    let mut i = start;
    while i < end && written < max {
        let byte = line[i];
        let normalized = if matches!(byte, b'_' | b'-') { b' ' } else { byte };
        if phase40g_is_ascii_space(normalized) {
            if written > 0 && !prev_space { out[written] = b' '; written += 1; }
            prev_space = true;
        } else if normalized.is_ascii() && !normalized.is_ascii_control() {
            out[written] = normalized;
            written += 1;
            prev_space = false;
        }
        i += 1;
    }
    while written > 0 && out[written - 1] == b' ' { written -= 1; }
    written
}

fn phase40g_extract_text_title(data: &[u8], out: &mut [u8]) -> usize {
    let mut start = 0usize;
    let mut lines_seen = 0usize;
    while start < data.len() && lines_seen < 40 {
        let end = data[start..].iter().position(|&b| b == b'\n').map(|p| start + p).unwrap_or(data.len());
        let mut line = &data[start..end];
        if line.ends_with(b"\r") { line = &line[..line.len() - 1]; }
        let mut trimmed_start = 0usize;
        let mut trimmed_end = line.len();
        while trimmed_start < trimmed_end && phase40g_is_ascii_space(line[trimmed_start]) { trimmed_start += 1; }
        while trimmed_end > trimmed_start && phase40g_is_ascii_space(line[trimmed_end - 1]) { trimmed_end -= 1; }
        let trimmed = &line[trimmed_start..trimmed_end];
        if !phase40g_skip_text_title_line(trimmed) {
            let len = phase40g_copy_text_title(trimmed, out);
            if len >= 3 { return len; }
        }
        start = end.saturating_add(1);
        lines_seen += 1;
    }
    0
}

fn scan_one_text_title(k: &mut KernelHandle<'_>, idx: usize, name: &str, next_idx: usize) -> Option<TitleScanResult> {
    let mut buf = [0u8; PHASE40G_TEXT_TITLE_SCAN_BYTES];
    let mut title = [0u8; PHASE40G_TEXT_TITLE_MAX_BYTES];
    let result = (|| -> crate::error::Result<usize> {
        let n = k.read_chunk(name, 0, &mut buf)?;
        let title_len = phase40g_extract_text_title(&buf[..n], &mut title);
        if title_len == 0 { return Err(Error::new(ErrorKind::InvalidData, "text_title_scan: no title")); }
        let title_str = core::str::from_utf8(&title[..title_len]).map_err(|_| Error::new(ErrorKind::BadEncoding, "text_title_scan: title"))?;
        log::info!("titles: {} -> \"{}\" (text)", name, title_str);
        let _ = k.save_title(name, title_str);
        k.dir_cache_mut().set_entry_title(idx, &title[..title_len]);
        Ok(title_len)
    })();
    if let Err(e) = &result { log::warn!("titles: {} text title failed: {}", name, e); }
    Some(TitleScanResult { next_idx, resolved: result.is_ok() })
}

'''
if "fn phase40g_extract_text_title" not in text:
    text = text.replace("fn phase38i_is_epub_or_epu_name(name: &[u8]) -> bool {", helper + "fn phase38i_is_epub_or_epu_name(name: &[u8]) -> bool {", 1)
old_sig = '''fn scan_one_epub_title(k: &mut KernelHandle<'_>, from: usize) -> Option<TitleScanResult> {
    let (idx, name_buf, name_len) = k.dir_cache_mut().next_untitled_epub(from)?;
    let name = core::str::from_utf8(&name_buf[..name_len as usize]).unwrap_or("");
    let next_idx = idx + 1;

    log::info!("titles: scanning {} (idx {})", name, idx);
'''
new_sig = '''fn scan_one_reader_title(k: &mut KernelHandle<'_>, from: usize) -> Option<TitleScanResult> {
    let (idx, name_buf, name_len, title_kind) = k.dir_cache_mut().next_untitled_reader_title(from)?;
    let name = core::str::from_utf8(&name_buf[..name_len as usize]).unwrap_or("");
    let next_idx = idx + 1;

    if title_kind == PHASE40G_DIR_TITLE_KIND_TEXT {
        return scan_one_text_title(k, idx, name, next_idx);
    }

    log::info!("titles: scanning {} (idx {})", name, idx);
'''
if old_sig in text:
    text = text.replace(old_sig, new_sig, 1)
elif "fn scan_one_reader_title" not in text:
    raise SystemExit("files.rs: could not patch scan_one_epub_title signature")
if "phase40g=x4-home-full-width-library-title-patch-ok" not in text:
    text = text.replace("// paginated file browser for SD card root directory\n", "// paginated file browser for SD card root directory\n// phase40g=x4-home-full-width-library-title-patch-ok\n", 1)
write_changed(files, text, orig, "files.rs: EPU/TXT title scanner")

patched_list.write_text("\n".join(patched) + ("\n" if patched else ""))
status = "ACCEPTED" if patched else "REJECTED"
with out.open("w") as f:
    f.write("# Phase 40G Home/Library Title Patch\n")
    f.write(f"status={status}\n")
    f.write(f"reason={'HomeAndLibraryTitleSourcesPatched' if patched else 'NoFilesPatched'}\n")
    f.write(f"patched_files={len(patched)}\n")
    f.write("changes_home_title_layout=true\nchanges_library_title_resolution=true\nchanges_footer_labels=false\nchanges_input_mapping=false\ntouches_write_lane=false\ntouches_display_geometry=false\ntouches_reader_pagination=false\n")
    f.write("marker=phase40g=x4-home-full-width-library-title-patch-ok\n\n")
    for note in notes: f.write(f"- {note}\n")
    for p in patched: f.write(f"patched={p}\n")
print(out.read_text())
if status != "ACCEPTED": raise SystemExit("Phase 40G patch did not modify files")
