//! Phase 38E — File Explorer EPUB Display Name Active Wiring.
//!
//! This module fixes the class of Files-screen label bugs where an EPUB such as
//! `Alice's Adventures in Wonderland.epub` is rendered as a short initials label
//! such as `Al`.
//!
//! The resolver is intentionally side-effect free:
//! - no SD/FAT calls
//! - no SPI/display/input/power calls
//! - no writes
//! - no allocation
//!
//! Call-site contract:
//! the Files row-label renderer should call
//! [`phase38e_resolve_file_explorer_display_name`] before drawing the row text.

#![allow(dead_code)]

use core::str;

pub const PHASE_38E_FILE_EXPLORER_EPUB_DISPLAY_NAME_ACTIVE_WIRING_MARKER: &str =
    "phase38e=x4-file-explorer-epub-display-name-active-wiring-ok";

pub const PHASE_38E_WRITES_ENABLED: bool = false;
pub const PHASE_38E_SD_FAT_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38E_SPI_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38E_DISPLAY_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38E_INPUT_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38E_POWER_BEHAVIOR_MOVED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase38eDisplayNameSource {
    MetadataTitle,
    LongFileNameStem,
    ShortFileNameStem,
    ExistingNonInitialLabel,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase38eDisplayNameOutcome {
    Rendered(Phase38eDisplayNameSource, usize),
    OutputBufferTooSmall(Phase38eDisplayNameSource),
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Phase38eFileExplorerEntryNameParts<'a> {
    /// Title discovered from metadata/state, for example `.MTA` or EPUB package metadata.
    pub metadata_title: Option<&'a str>,
    /// Long filename when available, for example `Alice's Adventures in Wonderland.epub`.
    pub long_file_name: Option<&'a str>,
    /// FAT 8.3 / short filename, for example `ALICE~1.EPU`.
    pub short_file_name: Option<&'a str>,
    /// Current label already produced by the legacy Files path, for example `Al`.
    pub existing_label: Option<&'a str>,
    /// Extension hint when the caller already has it, for example `epub` or `EPU`.
    pub extension_hint: Option<&'a str>,
}

impl<'a> Phase38eFileExplorerEntryNameParts<'a> {
    pub const fn empty() -> Self {
        Self {
            metadata_title: None,
            long_file_name: None,
            short_file_name: None,
            existing_label: None,
            extension_hint: None,
        }
    }
}

pub fn phase38e_is_epub_extension(extension: &str) -> bool {
    ascii_eq_ignore_case(extension, "epub") || ascii_eq_ignore_case(extension, "epu")
}

pub fn phase38e_is_probably_initials(label: &str) -> bool {
    let trimmed = trim_ascii(label);
    if trimmed.is_empty() || trimmed.len() > 3 {
        return false;
    }

    let bytes = trimmed.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let b = bytes[index];
        if !b.is_ascii_alphabetic() {
            return false;
        }
        index += 1;
    }

    true
}

pub fn phase38e_resolve_file_explorer_display_name<'a>(
    parts: Phase38eFileExplorerEntryNameParts<'a>,
    output: &'a mut [u8],
) -> Phase38eDisplayNameOutcome {
    if let Some(title) = clean_candidate(parts.metadata_title) {
        return copy_clean_name(title, Phase38eDisplayNameSource::MetadataTitle, output);
    }

    let is_epub = parts
        .extension_hint
        .map(phase38e_is_epub_extension)
        .unwrap_or_else(|| {
            parts
                .long_file_name
                .and_then(file_extension)
                .map(phase38e_is_epub_extension)
                .unwrap_or(false)
                || parts
                    .short_file_name
                    .and_then(file_extension)
                    .map(phase38e_is_epub_extension)
                    .unwrap_or(false)
        });

    let existing_is_initials = parts
        .existing_label
        .map(phase38e_is_probably_initials)
        .unwrap_or(false);

    if (is_epub || existing_is_initials)
        && let Some(long_name) = clean_candidate(parts.long_file_name)
    {
        return copy_stem_clean_name(
            long_name,
            Phase38eDisplayNameSource::LongFileNameStem,
            output,
        );
    }

    if let Some(existing) = clean_candidate(parts.existing_label)
        && !phase38e_is_probably_initials(existing)
    {
        return copy_clean_name(
            existing,
            Phase38eDisplayNameSource::ExistingNonInitialLabel,
            output,
        );
    }

    if let Some(short_name) = clean_candidate(parts.short_file_name) {
        return copy_stem_clean_name(
            short_name,
            Phase38eDisplayNameSource::ShortFileNameStem,
            output,
        );
    }

    Phase38eDisplayNameOutcome::Empty
}

pub fn phase38e_outcome_len(outcome: Phase38eDisplayNameOutcome) -> usize {
    match outcome {
        Phase38eDisplayNameOutcome::Rendered(_, len) => len,
        Phase38eDisplayNameOutcome::OutputBufferTooSmall(_) | Phase38eDisplayNameOutcome::Empty => {
            0
        }
    }
}

pub fn phase38e_outcome_as_str(outcome: Phase38eDisplayNameOutcome, output: &[u8]) -> Option<&str> {
    let len = phase38e_outcome_len(outcome);
    if len == 0 || len > output.len() {
        return None;
    }
    str::from_utf8(&output[..len]).ok()
}

fn clean_candidate(candidate: Option<&str>) -> Option<&str> {
    let value = trim_ascii(candidate?);
    if value.is_empty() { None } else { Some(value) }
}

fn copy_stem_clean_name(
    name: &str,
    source: Phase38eDisplayNameSource,
    output: &mut [u8],
) -> Phase38eDisplayNameOutcome {
    let stem = file_stem(name).unwrap_or(name);
    copy_clean_name(stem, source, output)
}

fn copy_clean_name(
    input: &str,
    source: Phase38eDisplayNameSource,
    output: &mut [u8],
) -> Phase38eDisplayNameOutcome {
    let mut written = 0usize;
    let bytes = input.as_bytes();
    let mut index = 0usize;
    let mut last_was_space = true;

    while index < bytes.len() {
        let normalized = match bytes[index] {
            b'_' | b'-' => b' ',
            b => b,
        };

        if normalized == b' ' {
            if !last_was_space {
                if written >= output.len() {
                    return Phase38eDisplayNameOutcome::OutputBufferTooSmall(source);
                }
                output[written] = b' ';
                written += 1;
            }
            last_was_space = true;
        } else {
            if written >= output.len() {
                return Phase38eDisplayNameOutcome::OutputBufferTooSmall(source);
            }
            output[written] = normalized;
            written += 1;
            last_was_space = false;
        }

        index += 1;
    }

    while written > 0 && output[written - 1] == b' ' {
        written -= 1;
    }

    if written == 0 {
        Phase38eDisplayNameOutcome::Empty
    } else {
        Phase38eDisplayNameOutcome::Rendered(source, written)
    }
}

fn file_stem(name: &str) -> Option<&str> {
    let bytes = name.as_bytes();
    let mut dot = None;
    let mut index = bytes.len();
    while index > 0 {
        index -= 1;
        if bytes[index] == b'.' {
            dot = Some(index);
            break;
        }
        if bytes[index] == b'/' || bytes[index] == b'\\' {
            break;
        }
    }

    match dot {
        Some(0) | None => Some(name),
        Some(pos) => Some(&name[..pos]),
    }
}

fn file_extension(name: &str) -> Option<&str> {
    let bytes = name.as_bytes();
    let mut index = bytes.len();
    while index > 0 {
        index -= 1;
        if bytes[index] == b'.' {
            if index + 1 < bytes.len() {
                return Some(&name[index + 1..]);
            }
            return None;
        }
        if bytes[index] == b'/' || bytes[index] == b'\\' {
            return None;
        }
    }
    None
}

fn ascii_eq_ignore_case(left: &str, right: &str) -> bool {
    let l = left.as_bytes();
    let r = right.as_bytes();
    if l.len() != r.len() {
        return false;
    }

    let mut index = 0usize;
    while index < l.len() {
        if !l[index].eq_ignore_ascii_case(&r[index]) {
            return false;
        }
        index += 1;
    }

    true
}

fn trim_ascii(input: &str) -> &str {
    let bytes = input.as_bytes();
    let mut start = 0usize;
    let mut end = bytes.len();

    while start < end && bytes[start].is_ascii_whitespace() {
        start += 1;
    }
    while end > start && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &input[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alice_epub_does_not_render_as_initials() {
        let mut output = [0u8; 64];
        let outcome = phase38e_resolve_file_explorer_display_name(
            Phase38eFileExplorerEntryNameParts {
                metadata_title: None,
                long_file_name: Some("Alice's Adventures in Wonderland.epub"),
                short_file_name: Some("ALICE~1.EPU"),
                existing_label: Some("Al"),
                extension_hint: Some("epub"),
            },
            &mut output,
        );

        assert_eq!(
            phase38e_outcome_as_str(outcome, &output),
            Some("Alice's Adventures in Wonderland")
        );
    }

    #[test]
    fn metadata_title_wins() {
        let mut output = [0u8; 64];
        let outcome = phase38e_resolve_file_explorer_display_name(
            Phase38eFileExplorerEntryNameParts {
                metadata_title: Some("Alice in Wonderland"),
                long_file_name: Some("ALICE~1.EPU"),
                short_file_name: Some("ALICE~1.EPU"),
                existing_label: Some("Al"),
                extension_hint: Some("epub"),
            },
            &mut output,
        );

        assert_eq!(
            phase38e_outcome_as_str(outcome, &output),
            Some("Alice in Wonderland")
        );
    }
}
