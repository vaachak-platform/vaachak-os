//! Phase 39E — Typed Record SD/FAT Adapter Binding Bundle.
//!
//! This phase bridges the Phase 39D typed-record write lane to a real
//! SD/FAT-shaped adapter without hard-coding a concrete filesystem crate here.
//!
//! Scope:
//! - `.PRG`
//! - `.THM`
//! - `.MTA`
//! - `.BKM`
//! - `BMIDX.TXT`
//!
//! The caller supplies an implementation of [`Phase39eSdFatLikeBackend`].
//! This module maps Phase 39D typed writes into that backend with:
//!
//! - `STATE/` directory preflight
//! - target path rendering
//! - payload-size guard
//! - write mode selection
//! - optional atomic temporary-write plan
//! - backend status/error mapping
//!
//! No display, input, or power behavior is touched.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_typed_record_write_lane::{
    PHASE_39D_MAX_RECORD_PAYLOAD_LEN, Phase39dBackendWriteResult, Phase39dBackendWriteStatus,
    Phase39dTypedBackendKind, Phase39dTypedRecordKind, Phase39dTypedRecordPath,
    Phase39dTypedWriteBackend, Phase39dTypedWriteIntent, Phase39dTypedWriteMode,
    Phase39dTypedWriteRequest,
};

pub const PHASE_39E_TYPED_RECORD_SDFAT_ADAPTER_BINDING_MARKER: &str =
    "phase39e=x4-typed-record-sdfat-adapter-binding-bundle-ok";

pub const PHASE_39E_TYPED_RECORDS_SUPPORTED: bool = true;
pub const PHASE_39E_REAL_BACKEND_BINDING_STARTED: bool = true;
pub const PHASE_39E_MAX_RECORD_PAYLOAD_LEN: usize = PHASE_39D_MAX_RECORD_PAYLOAD_LEN;
pub const PHASE_39E_MAX_TEMP_PATH_LEN: usize = 40;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eSdFatWriteMode {
    DirectOverwrite,
    AtomicTempThenReplace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eDirectoryPolicy {
    AssumeStateDirExists,
    EnsureStateDir,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eBackendError {
    None,
    DirectoryUnavailable,
    OpenFailed,
    WriteFailed,
    FlushFailed,
    RenameFailed,
    RemoveFailed,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eAdapterDecision {
    DryRunAccepted,
    DirectWriteCommitted,
    AtomicWriteCommitted,
    RejectedInvalidRequest,
    RejectedBackendUnavailable,
    RejectedDirectoryUnavailable,
    RejectedBackendError,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39eAdapterNextLane {
    WireReaderRuntimeCallSites,
    AddThemeMetadataBookmarkRuntimeWrites,
    KeepAdapterBehindFeatureGate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39eTempRecordPath {
    bytes: [u8; PHASE_39E_MAX_TEMP_PATH_LEN],
    len: usize,
}

impl Phase39eTempRecordPath {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; PHASE_39E_MAX_TEMP_PATH_LEN],
            len: 0,
        }
    }

    pub fn from_target(target: Phase39dTypedRecordPath) -> Option<Self> {
        let mut bytes = [0u8; PHASE_39E_MAX_TEMP_PATH_LEN];
        let target_slice = target.as_slice();
        let mut pos = copy_into(target_slice, &mut bytes, 0)?;
        pos = copy_into(b".TMP", &mut bytes, pos)?;
        Some(Self { bytes, len: pos })
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39eSdFatAdapterConfig {
    pub write_mode: Phase39eSdFatWriteMode,
    pub directory_policy: Phase39eDirectoryPolicy,
}

impl Phase39eSdFatAdapterConfig {
    pub const fn direct() -> Self {
        Self {
            write_mode: Phase39eSdFatWriteMode::DirectOverwrite,
            directory_policy: Phase39eDirectoryPolicy::EnsureStateDir,
        }
    }

    pub const fn atomic() -> Self {
        Self {
            write_mode: Phase39eSdFatWriteMode::AtomicTempThenReplace,
            directory_policy: Phase39eDirectoryPolicy::EnsureStateDir,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39eSdFatWriteCommand<'a> {
    pub kind: Phase39dTypedRecordKind,
    pub intent: Phase39dTypedWriteIntent,
    pub target_path: Phase39dTypedRecordPath,
    pub temp_path: Phase39eTempRecordPath,
    pub payload: &'a [u8],
    pub config: Phase39eSdFatAdapterConfig,
}

impl<'a> Phase39eSdFatWriteCommand<'a> {
    pub const fn target_len(self) -> usize {
        self.target_path.len()
    }

    pub const fn temp_len(self) -> usize {
        self.temp_path.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39eSdFatBackendResult {
    pub status: Phase39dBackendWriteStatus,
    pub error: Phase39eBackendError,
    pub bytes_written: usize,
}

impl Phase39eSdFatBackendResult {
    pub const fn committed(bytes_written: usize) -> Self {
        Self {
            status: Phase39dBackendWriteStatus::Committed,
            error: Phase39eBackendError::None,
            bytes_written,
        }
    }

    pub const fn rejected(error: Phase39eBackendError) -> Self {
        Self {
            status: Phase39dBackendWriteStatus::BackendRejected,
            error,
            bytes_written: 0,
        }
    }

    pub const fn unavailable() -> Self {
        Self {
            status: Phase39dBackendWriteStatus::BackendUnavailable,
            error: Phase39eBackendError::Unsupported,
            bytes_written: 0,
        }
    }
}

/// Minimal backend shape for a real SD/FAT writer.
///
/// Implement this trait in the layer that owns the concrete filesystem handle.
/// This keeps runtime policy decoupled from one storage crate while allowing
/// real persistence to be bound in one place.
pub trait Phase39eSdFatLikeBackend {
    fn ensure_state_dir(&mut self) -> Result<(), Phase39eBackendError>;

    fn write_direct(
        &mut self,
        command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult;

    fn write_atomic(
        &mut self,
        command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Phase39eUnavailableSdFatBackend;

impl Phase39eSdFatLikeBackend for Phase39eUnavailableSdFatBackend {
    fn ensure_state_dir(&mut self) -> Result<(), Phase39eBackendError> {
        Err(Phase39eBackendError::DirectoryUnavailable)
    }

    fn write_direct(
        &mut self,
        _command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult {
        Phase39eSdFatBackendResult::unavailable()
    }

    fn write_atomic(
        &mut self,
        _command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult {
        Phase39eSdFatBackendResult::unavailable()
    }
}

/// Recording backend for validation and smoke checks.
///
/// It records commands in memory and can accept or reject writes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39eRecordingSdFatBackend {
    pub calls: usize,
    pub ensure_dir_calls: usize,
    pub last_kind: Phase39dTypedRecordKind,
    pub last_target_path: Phase39dTypedRecordPath,
    pub last_temp_path: Phase39eTempRecordPath,
    pub last_payload_len: usize,
    pub last_error: Phase39eBackendError,
    pub accept_writes: bool,
    pub accept_state_dir: bool,
}

impl Phase39eRecordingSdFatBackend {
    pub const fn accepting() -> Self {
        Self {
            calls: 0,
            ensure_dir_calls: 0,
            last_kind: Phase39dTypedRecordKind::Progress,
            last_target_path: Phase39dTypedRecordPath::empty(),
            last_temp_path: Phase39eTempRecordPath::empty(),
            last_payload_len: 0,
            last_error: Phase39eBackendError::None,
            accept_writes: true,
            accept_state_dir: true,
        }
    }

    pub const fn rejecting() -> Self {
        Self {
            calls: 0,
            ensure_dir_calls: 0,
            last_kind: Phase39dTypedRecordKind::Progress,
            last_target_path: Phase39dTypedRecordPath::empty(),
            last_temp_path: Phase39eTempRecordPath::empty(),
            last_payload_len: 0,
            last_error: Phase39eBackendError::WriteFailed,
            accept_writes: false,
            accept_state_dir: true,
        }
    }

    pub fn wrote_once(self) -> bool {
        self.calls == 1 && self.last_error == Phase39eBackendError::None
    }
}

impl Phase39eSdFatLikeBackend for Phase39eRecordingSdFatBackend {
    fn ensure_state_dir(&mut self) -> Result<(), Phase39eBackendError> {
        self.ensure_dir_calls = self.ensure_dir_calls.saturating_add(1);
        if self.accept_state_dir {
            Ok(())
        } else {
            self.last_error = Phase39eBackendError::DirectoryUnavailable;
            Err(Phase39eBackendError::DirectoryUnavailable)
        }
    }

    fn write_direct(
        &mut self,
        command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult {
        self.record_command(command);

        if self.accept_writes {
            Phase39eSdFatBackendResult::committed(command.payload.len())
        } else {
            self.last_error = Phase39eBackendError::WriteFailed;
            Phase39eSdFatBackendResult::rejected(Phase39eBackendError::WriteFailed)
        }
    }

    fn write_atomic(
        &mut self,
        command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult {
        self.record_command(command);

        if self.accept_writes {
            Phase39eSdFatBackendResult::committed(command.payload.len())
        } else {
            self.last_error = Phase39eBackendError::RenameFailed;
            Phase39eSdFatBackendResult::rejected(Phase39eBackendError::RenameFailed)
        }
    }
}

impl Phase39eRecordingSdFatBackend {
    fn record_command(&mut self, command: Phase39eSdFatWriteCommand<'_>) {
        self.calls = self.calls.saturating_add(1);
        self.last_kind = command.kind;
        self.last_target_path = command.target_path;
        self.last_temp_path = command.temp_path;
        self.last_payload_len = command.payload.len();
        self.last_error = Phase39eBackendError::None;
    }
}

pub struct Phase39eTypedRecordSdFatAdapter<'a, B: Phase39eSdFatLikeBackend> {
    backend: &'a mut B,
    config: Phase39eSdFatAdapterConfig,
}

impl<'a, B: Phase39eSdFatLikeBackend> Phase39eTypedRecordSdFatAdapter<'a, B> {
    pub fn new(backend: &'a mut B, config: Phase39eSdFatAdapterConfig) -> Self {
        Self { backend, config }
    }
}

impl<B: Phase39eSdFatLikeBackend> Phase39dTypedWriteBackend
    for Phase39eTypedRecordSdFatAdapter<'_, B>
{
    fn write_typed_record(
        &mut self,
        path: Phase39dTypedRecordPath,
        request: &Phase39dTypedWriteRequest<'_>,
    ) -> Phase39dBackendWriteResult {
        let temp_path = match Phase39eTempRecordPath::from_target(path) {
            Some(path) => path,
            None => {
                return Phase39dBackendWriteResult {
                    status: Phase39dBackendWriteStatus::BackendRejected,
                    bytes_written: 0,
                };
            }
        };

        if self.config.directory_policy == Phase39eDirectoryPolicy::EnsureStateDir
            && self.backend.ensure_state_dir().is_err()
        {
            return Phase39dBackendWriteResult {
                status: Phase39dBackendWriteStatus::BackendRejected,
                bytes_written: 0,
            };
        }

        let command = Phase39eSdFatWriteCommand {
            kind: request.kind,
            intent: request.intent,
            target_path: path,
            temp_path,
            payload: request.payload,
            config: self.config,
        };

        let result = match self.config.write_mode {
            Phase39eSdFatWriteMode::DirectOverwrite => self.backend.write_direct(command),
            Phase39eSdFatWriteMode::AtomicTempThenReplace => self.backend.write_atomic(command),
        };

        Phase39dBackendWriteResult {
            status: result.status,
            bytes_written: result.bytes_written,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39eSdFatAdapterReport {
    pub decision: Phase39eAdapterDecision,
    pub backend_status: Phase39dBackendWriteStatus,
    pub backend_error: Phase39eBackendError,
    pub kind: Phase39dTypedRecordKind,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub target_path: Phase39dTypedRecordPath,
    pub temp_path: Phase39eTempRecordPath,
    pub next_lane: Phase39eAdapterNextLane,
}

impl Phase39eSdFatAdapterReport {
    pub const fn committed(self) -> bool {
        matches!(
            self.decision,
            Phase39eAdapterDecision::DirectWriteCommitted
                | Phase39eAdapterDecision::AtomicWriteCommitted
        )
    }
}

pub fn phase39e_execute_with_sdfat_adapter<B: Phase39eSdFatLikeBackend>(
    backend: &mut B,
    request: Phase39dTypedWriteRequest<'_>,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39eSdFatAdapterReport {
    let target_path = request
        .path()
        .unwrap_or_else(Phase39dTypedRecordPath::empty);
    let temp_path = Phase39eTempRecordPath::from_target(target_path)
        .unwrap_or_else(Phase39eTempRecordPath::empty);

    if matches!(request.mode, Phase39dTypedWriteMode::DryRun) {
        return Phase39eSdFatAdapterReport {
            decision: Phase39eAdapterDecision::DryRunAccepted,
            backend_status: Phase39dBackendWriteStatus::NotCalled,
            backend_error: Phase39eBackendError::None,
            kind: request.kind,
            payload_len: request.payload.len(),
            bytes_written: 0,
            target_path,
            temp_path,
            next_lane: Phase39eAdapterNextLane::WireReaderRuntimeCallSites,
        };
    }

    let mut adapter = Phase39eTypedRecordSdFatAdapter::new(backend, config);
    let report = crate::vaachak_x4::runtime::state_io_typed_record_write_lane::phase39d_execute_typed_record_write(
        Some(&mut adapter),
        request,
        Phase39dTypedBackendKind::Recording,
    );

    phase39e_report_from_phase39d(report, config)
}

pub const fn phase39e_report_from_phase39d(
    report: crate::vaachak_x4::runtime::state_io_typed_record_write_lane::Phase39dTypedWriteReport,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39eSdFatAdapterReport {
    let decision = phase39e_decision_from_backend_status(report.backend_status, config.write_mode);

    Phase39eSdFatAdapterReport {
        decision,
        backend_status: report.backend_status,
        backend_error: phase39e_error_from_backend_status(report.backend_status),
        kind: report.kind,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        target_path: report.path,
        temp_path: Phase39eTempRecordPath::empty(),
        next_lane: Phase39eAdapterNextLane::WireReaderRuntimeCallSites,
    }
}

pub const fn phase39e_decision_from_backend_status(
    status: Phase39dBackendWriteStatus,
    mode: Phase39eSdFatWriteMode,
) -> Phase39eAdapterDecision {
    match status {
        Phase39dBackendWriteStatus::NotCalled => Phase39eAdapterDecision::DryRunAccepted,
        Phase39dBackendWriteStatus::Committed => match mode {
            Phase39eSdFatWriteMode::DirectOverwrite => {
                Phase39eAdapterDecision::DirectWriteCommitted
            }
            Phase39eSdFatWriteMode::AtomicTempThenReplace => {
                Phase39eAdapterDecision::AtomicWriteCommitted
            }
        },
        Phase39dBackendWriteStatus::BackendRejected => {
            Phase39eAdapterDecision::RejectedBackendError
        }
        Phase39dBackendWriteStatus::BackendUnavailable => {
            Phase39eAdapterDecision::RejectedBackendUnavailable
        }
    }
}

pub const fn phase39e_error_from_backend_status(
    status: Phase39dBackendWriteStatus,
) -> Phase39eBackendError {
    match status {
        Phase39dBackendWriteStatus::NotCalled | Phase39dBackendWriteStatus::Committed => {
            Phase39eBackendError::None
        }
        Phase39dBackendWriteStatus::BackendRejected => Phase39eBackendError::WriteFailed,
        Phase39dBackendWriteStatus::BackendUnavailable => Phase39eBackendError::Unsupported,
    }
}

fn copy_into(src: &[u8], dst: &mut [u8], start: usize) -> Option<usize> {
    let end = start.checked_add(src.len())?;
    if end > dst.len() {
        return None;
    }
    dst[start..end].copy_from_slice(src);
    Some(end)
}

pub fn phase39e_marker() -> &'static str {
    PHASE_39E_TYPED_RECORD_SDFAT_ADAPTER_BINDING_MARKER
}
