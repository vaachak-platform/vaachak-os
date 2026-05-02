//! Phase 38D file-explorer display-name binding.
//!
//! This module is intentionally side-effect free.  It normalizes the label that
//! the Files screen should render for a discovered book/file entry, preferring a
//! metadata title over a long file name over a FAT 8.3 name.  It performs no
//! SD/FAT/SPI/display/input/power operations and enables no writes.

#![allow(dead_code)]

pub const PHASE_38D_FILE_EXPLORER_DISPLAY_NAME_BINDING_MARKER: &str =
    "phase38d=x4-file-explorer-display-name-binding-ok";

pub const PHASE_38D_WRITES_ENABLED: bool = false;
pub const PHASE_38D_SD_FAT_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38D_SPI_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38D_DISPLAY_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38D_INPUT_BEHAVIOR_MOVED: bool = false;
pub const PHASE_38D_POWER_BEHAVIOR_MOVED: bool = false;

pub const PHASE_38D_REFERENCE_TITLE: &str = "Alice's Adventures in Wonderland";
pub const PHASE_38D_REFERENCE_SHORT_NAME: &str = "ALICE~1.TXT";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38dDisplayNameSource {
    MetadataTitle,
    LongFileName,
    Fat83Name,
    Fallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38dDisplayNameOutcome {
    Rendered,
    RenderedTruncated,
    MissingAllNames,
    OutputBufferTooSmall,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38dDisplayNameResult {
    pub len: usize,
    pub source: Phase38dDisplayNameSource,
    pub outcome: Phase38dDisplayNameOutcome,
}

impl Phase38dDisplayNameResult {
    pub const fn empty(source: Phase38dDisplayNameSource) -> Self {
        Self {
            len: 0,
            source,
            outcome: Phase38dDisplayNameOutcome::MissingAllNames,
        }
    }

    pub const fn output_buffer_too_small(source: Phase38dDisplayNameSource) -> Self {
        Self {
            len: 0,
            source,
            outcome: Phase38dDisplayNameOutcome::OutputBufferTooSmall,
        }
    }

    pub const fn rendered(len: usize, source: Phase38dDisplayNameSource, truncated: bool) -> Self {
        Self {
            len,
            source,
            outcome: if truncated {
                Phase38dDisplayNameOutcome::RenderedTruncated
            } else {
                Phase38dDisplayNameOutcome::Rendered
            },
        }
    }

    pub fn as_slice<'a>(&self, output: &'a [u8]) -> &'a [u8] {
        if self.len <= output.len() {
            &output[..self.len]
        } else {
            &output[..0]
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38dDisplayNameCandidate<'a> {
    pub raw: &'a [u8],
    pub source: Phase38dDisplayNameSource,
}

pub fn phase38d_choose_display_name_candidate<'a>(
    metadata_title: Option<&'a [u8]>,
    long_file_name: Option<&'a [u8]>,
    fat83_name: Option<&'a [u8]>,
) -> Phase38dDisplayNameCandidate<'a> {
    if let Some(title) = non_blank(metadata_title) {
        return Phase38dDisplayNameCandidate {
            raw: title,
            source: Phase38dDisplayNameSource::MetadataTitle,
        };
    }

    if let Some(name) = non_blank(long_file_name) {
        return Phase38dDisplayNameCandidate {
            raw: name,
            source: Phase38dDisplayNameSource::LongFileName,
        };
    }

    if let Some(name) = non_blank(fat83_name) {
        return Phase38dDisplayNameCandidate {
            raw: name,
            source: Phase38dDisplayNameSource::Fat83Name,
        };
    }

    Phase38dDisplayNameCandidate {
        raw: b"",
        source: Phase38dDisplayNameSource::Fallback,
    }
}

pub fn phase38d_render_file_explorer_display_name(
    metadata_title: Option<&[u8]>,
    long_file_name: Option<&[u8]>,
    fat83_name: Option<&[u8]>,
    output: &mut [u8],
) -> Phase38dDisplayNameResult {
    let candidate =
        phase38d_choose_display_name_candidate(metadata_title, long_file_name, fat83_name);
    if candidate.raw.is_empty() {
        return Phase38dDisplayNameResult::empty(candidate.source);
    }

    if output.is_empty() {
        return Phase38dDisplayNameResult::output_buffer_too_small(candidate.source);
    }

    render_candidate(candidate, output)
}

pub fn phase38d_render_reference_alice_name(output: &mut [u8]) -> Phase38dDisplayNameResult {
    phase38d_render_file_explorer_display_name(
        Some(PHASE_38D_REFERENCE_TITLE.as_bytes()),
        None,
        Some(PHASE_38D_REFERENCE_SHORT_NAME.as_bytes()),
        output,
    )
}

fn render_candidate(
    candidate: Phase38dDisplayNameCandidate<'_>,
    output: &mut [u8],
) -> Phase38dDisplayNameResult {
    let base = basename(candidate.raw);
    let stem = if candidate.source == Phase38dDisplayNameSource::MetadataTitle {
        base
    } else {
        strip_extension(base)
    };

    let mut len = 0usize;
    let mut previous_space = false;
    let mut truncated = false;
    let uppercase_only =
        candidate.source == Phase38dDisplayNameSource::Fat83Name && is_ascii_uppercaseish(stem);

    for &input in trim_ascii(stem) {
        let normalized = normalize_byte(input, uppercase_only, len == 0 || previous_space);
        if normalized == 0 {
            continue;
        }

        if len >= output.len() {
            truncated = true;
            break;
        }

        if normalized == b' ' {
            if len == 0 || previous_space {
                continue;
            }
            previous_space = true;
        } else {
            previous_space = false;
        }

        output[len] = normalized;
        len += 1;
    }

    while len > 0 && output[len - 1] == b' ' {
        len -= 1;
    }

    if len == 0 {
        return Phase38dDisplayNameResult::empty(candidate.source);
    }

    Phase38dDisplayNameResult::rendered(len, candidate.source, truncated)
}

fn non_blank(value: Option<&[u8]>) -> Option<&[u8]> {
    let value = value?;
    if trim_ascii(value).is_empty() {
        None
    } else {
        Some(value)
    }
}

fn basename(mut value: &[u8]) -> &[u8] {
    let mut index = 0usize;
    while index < value.len() {
        let byte = value[index];
        if byte == b'/' || byte == b'\\' {
            value = &value[index + 1..];
            index = 0;
        } else {
            index += 1;
        }
    }
    value
}

fn strip_extension(value: &[u8]) -> &[u8] {
    let mut index = value.len();
    while index > 0 {
        index -= 1;
        if value[index] == b'.' {
            return &value[..index];
        }
    }
    value
}

fn trim_ascii(value: &[u8]) -> &[u8] {
    let mut start = 0usize;
    let mut end = value.len();

    while start < end && value[start].is_ascii_whitespace() {
        start += 1;
    }

    while end > start && value[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &value[start..end]
}

fn normalize_byte(input: u8, uppercase_only: bool, should_capitalize: bool) -> u8 {
    match input {
        b'_' | b'-' | b'~' => b' ',
        b'.' => b' ',
        b'\'' | b'&' | b'(' | b')' | b',' | b':' | b';' | b'!' | b'?' => input,
        b'0'..=b'9' | b' ' => input,
        b'A'..=b'Z' if uppercase_only && !should_capitalize => input + 32,
        b'a'..=b'z' if uppercase_only && should_capitalize => input - 32,
        b'A'..=b'Z' | b'a'..=b'z' => input,
        0x80..=0xff => input,
        _ => b' ',
    }
}

fn is_ascii_uppercaseish(value: &[u8]) -> bool {
    let mut saw_letter = false;
    for &byte in value {
        match byte {
            b'A'..=b'Z' => saw_letter = true,
            b'a'..=b'z' => return false,
            _ => {}
        }
    }
    saw_letter
}

pub fn phase38d_alice_reference_matches() -> bool {
    let mut output = [0u8; 64];
    let rendered = phase38d_render_reference_alice_name(&mut output);
    rendered.outcome == Phase38dDisplayNameOutcome::Rendered
        && rendered.source == Phase38dDisplayNameSource::MetadataTitle
        && rendered.as_slice(&output) == PHASE_38D_REFERENCE_TITLE.as_bytes()
}

pub fn phase38d_keeps_writes_disabled() -> bool {
    !PHASE_38D_WRITES_ENABLED
        && !PHASE_38D_SD_FAT_BEHAVIOR_MOVED
        && !PHASE_38D_SPI_BEHAVIOR_MOVED
        && !PHASE_38D_DISPLAY_BEHAVIOR_MOVED
        && !PHASE_38D_INPUT_BEHAVIOR_MOVED
        && !PHASE_38D_POWER_BEHAVIOR_MOVED
}
