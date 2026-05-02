//! Phase 38F — File Explorer row-label call-site wiring helper.
//!
//! This module is intentionally small and side-effect free. It exists to make
//! the Files screen label policy explicit while the active imported/runtime UI
//! call site is being moved away from legacy two-character initials.

#![allow(dead_code)]

pub const PHASE_38F_FILE_EXPLORER_ROW_LABEL_CALLSITE_WIRING_MARKER: &str =
    "phase38f=x4-file-explorer-row-label-callsite-wiring-ok";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase38fRowLabelSource {
    MetadataTitle,
    LongFileName,
    ShortFileName,
    ExistingNonInitialsLabel,
    ExistingInitialsFallback,
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38fRowLabelOutcome {
    pub source: Phase38fRowLabelSource,
    pub len: usize,
}

impl Phase38fRowLabelOutcome {
    pub const fn empty() -> Self {
        Self {
            source: Phase38fRowLabelSource::Empty,
            len: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase38fRowLabelParts<'a> {
    pub metadata_title: Option<&'a str>,
    pub long_file_name: Option<&'a str>,
    pub short_file_name: Option<&'a str>,
    pub existing_label: Option<&'a str>,
}

pub fn phase38f_resolve_files_row_label(
    parts: Phase38fRowLabelParts<'_>,
    output: &mut [u8],
) -> Phase38fRowLabelOutcome {
    if let Some(title) = clean_candidate(parts.metadata_title) {
        return copy_stemless(title, output, Phase38fRowLabelSource::MetadataTitle);
    }

    if let Some(long_name) = clean_candidate(parts.long_file_name) {
        return copy_stemless(long_name, output, Phase38fRowLabelSource::LongFileName);
    }

    if let Some(existing) = clean_candidate(parts.existing_label)
        && !phase38f_is_probably_initials(existing)
    {
        return copy_stemless(
            existing,
            output,
            Phase38fRowLabelSource::ExistingNonInitialsLabel,
        );
    }

    if let Some(short_name) = clean_candidate(parts.short_file_name) {
        return copy_stemless(short_name, output, Phase38fRowLabelSource::ShortFileName);
    }

    if let Some(existing) = clean_candidate(parts.existing_label) {
        return copy_stemless(
            existing,
            output,
            Phase38fRowLabelSource::ExistingInitialsFallback,
        );
    }

    Phase38fRowLabelOutcome::empty()
}

pub fn phase38f_row_label_as_str(outcome: Phase38fRowLabelOutcome, output: &[u8]) -> Option<&str> {
    if outcome.len == 0 || outcome.len > output.len() {
        return None;
    }
    core::str::from_utf8(&output[..outcome.len]).ok()
}

pub fn phase38f_is_probably_initials(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.len() > 3 || trimmed.is_empty() {
        return false;
    }

    let mut alpha_count = 0usize;
    for byte in trimmed.as_bytes() {
        if byte.is_ascii_alphabetic() {
            alpha_count += 1;
        } else if *byte != b'.' && *byte != b' ' {
            return false;
        }
    }

    alpha_count <= 2
}

fn clean_candidate(candidate: Option<&str>) -> Option<&str> {
    let value = candidate?.trim();
    if value.is_empty() { None } else { Some(value) }
}

fn copy_stemless(
    value: &str,
    output: &mut [u8],
    source: Phase38fRowLabelSource,
) -> Phase38fRowLabelOutcome {
    let stem = strip_known_extension(value.trim());
    let bytes = stem.as_bytes();
    let len = core::cmp::min(bytes.len(), output.len());
    if len > 0 {
        output[..len].copy_from_slice(&bytes[..len]);
    }
    Phase38fRowLabelOutcome { source, len }
}

fn strip_known_extension(value: &str) -> &str {
    let lower = value.as_bytes();
    if lower.len() > 5 && ends_with_ascii_ignore_case(lower, b".epub") {
        return &value[..value.len() - 5];
    }
    if lower.len() > 4 && ends_with_ascii_ignore_case(lower, b".txt") {
        return &value[..value.len() - 4];
    }
    if lower.len() > 4 && ends_with_ascii_ignore_case(lower, b".xth") {
        return &value[..value.len() - 4];
    }
    if lower.len() > 4 && ends_with_ascii_ignore_case(lower, b".xtc") {
        return &value[..value.len() - 4];
    }
    value
}

fn ends_with_ascii_ignore_case(left: &[u8], suffix: &[u8]) -> bool {
    if suffix.len() > left.len() {
        return false;
    }
    let offset = left.len() - suffix.len();
    let mut index = 0usize;
    while index < suffix.len() {
        if !left[offset + index].eq_ignore_ascii_case(&suffix[index]) {
            return false;
        }
        index += 1;
    }
    true
}
