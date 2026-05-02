//! Phase 39F — Runtime-Owned SD/FAT Typed Record Writer Binding.
//!
//! This phase binds the Phase 39E SD/FAT-shaped adapter to a runtime-owned file
//! operations trait. It still does not hard-code a concrete storage crate here;
//! the runtime/kernel layer that owns the mounted filesystem implements the ops
//! trait and supplies it to this binding.
//!
//! Scope:
//! - `.PRG`
//! - `.THM`
//! - `.MTA`
//! - `.BKM`
//! - `BMIDX.TXT`
//!
//! This module is the first concrete runtime-owned writer binding shape for all
//! typed records, while preserving ownership boundaries.

#![allow(dead_code)]

use crate::vaachak_x4::runtime::state_io_typed_record_sdfat_adapter::{
    Phase39eBackendError, Phase39eSdFatAdapterConfig, Phase39eSdFatAdapterReport,
    Phase39eSdFatBackendResult, Phase39eSdFatLikeBackend, Phase39eSdFatWriteCommand,
    phase39e_execute_with_sdfat_adapter,
};
use crate::vaachak_x4::runtime::state_io_typed_record_write_lane::{
    Phase39dBackendWriteStatus, Phase39dTypedRecordKind, Phase39dTypedRecordPath,
    Phase39dTypedWriteRequest,
};

pub const PHASE_39F_RUNTIME_OWNED_SDFAT_TYPED_RECORD_WRITER_BINDING_MARKER: &str =
    "phase39f=x4-runtime-owned-sdfat-typed-record-writer-binding-ok";

pub const PHASE_39F_RUNTIME_OWNED_BINDING_STARTED: bool = true;
pub const PHASE_39F_TYPED_RECORDS_SUPPORTED: bool = true;
pub const PHASE_39F_CONCRETE_STORAGE_CRATE_HARDCODED: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39fRuntimeFileError {
    None,
    StateDirUnavailable,
    OpenFailed,
    WriteFailed,
    FlushFailed,
    RenameFailed,
    RemoveFailed,
    Unsupported,
}

impl Phase39fRuntimeFileError {
    pub const fn as_phase39e_error(self) -> Phase39eBackendError {
        match self {
            Self::None => Phase39eBackendError::None,
            Self::StateDirUnavailable => Phase39eBackendError::DirectoryUnavailable,
            Self::OpenFailed => Phase39eBackendError::OpenFailed,
            Self::WriteFailed => Phase39eBackendError::WriteFailed,
            Self::FlushFailed => Phase39eBackendError::FlushFailed,
            Self::RenameFailed => Phase39eBackendError::RenameFailed,
            Self::RemoveFailed => Phase39eBackendError::RemoveFailed,
            Self::Unsupported => Phase39eBackendError::Unsupported,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39fRuntimeWriterStatus {
    DryRunAccepted,
    RuntimeWriteCommitted,
    RuntimeBackendRejected,
    RuntimeBackendUnavailable,
    InvalidRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase39fRuntimeWriterNextLane {
    WireReaderRuntimeCallSites,
    AddRuntimeFeatureGate,
    AddCrashRecoveryForAtomicTempWrites,
    RepairRuntimeBackend,
}

/// Runtime-owned file operations required by the typed-record writer.
///
/// The concrete implementation should live in the layer that owns the actual
/// mounted SD/FAT filesystem and file handles.
pub trait Phase39fRuntimeOwnedFileOps {
    fn ensure_state_dir(&mut self) -> Result<(), Phase39fRuntimeFileError>;

    fn write_record_direct(
        &mut self,
        target_path: &[u8],
        payload: &[u8],
    ) -> Result<usize, Phase39fRuntimeFileError>;

    fn write_record_atomic(
        &mut self,
        target_path: &[u8],
        temp_path: &[u8],
        payload: &[u8],
    ) -> Result<usize, Phase39fRuntimeFileError>;
}

/// Adapter from runtime-owned file ops into the Phase 39E SD/FAT-shaped backend.
pub struct Phase39fRuntimeOwnedSdFatBackend<'a, O: Phase39fRuntimeOwnedFileOps> {
    ops: &'a mut O,
    last_error: Phase39fRuntimeFileError,
    last_bytes_written: usize,
    write_calls: usize,
    ensure_dir_calls: usize,
}

impl<'a, O: Phase39fRuntimeOwnedFileOps> Phase39fRuntimeOwnedSdFatBackend<'a, O> {
    pub fn new(ops: &'a mut O) -> Self {
        Self {
            ops,
            last_error: Phase39fRuntimeFileError::None,
            last_bytes_written: 0,
            write_calls: 0,
            ensure_dir_calls: 0,
        }
    }

    pub const fn last_error(&self) -> Phase39fRuntimeFileError {
        self.last_error
    }

    pub const fn last_bytes_written(&self) -> usize {
        self.last_bytes_written
    }

    pub const fn write_calls(&self) -> usize {
        self.write_calls
    }

    pub const fn ensure_dir_calls(&self) -> usize {
        self.ensure_dir_calls
    }

    fn record_result(
        &mut self,
        result: Result<usize, Phase39fRuntimeFileError>,
    ) -> Phase39eSdFatBackendResult {
        match result {
            Ok(bytes_written) => {
                self.last_error = Phase39fRuntimeFileError::None;
                self.last_bytes_written = bytes_written;
                Phase39eSdFatBackendResult::committed(bytes_written)
            }
            Err(error) => {
                self.last_error = error;
                self.last_bytes_written = 0;
                Phase39eSdFatBackendResult::rejected(error.as_phase39e_error())
            }
        }
    }
}

impl<O: Phase39fRuntimeOwnedFileOps> Phase39eSdFatLikeBackend
    for Phase39fRuntimeOwnedSdFatBackend<'_, O>
{
    fn ensure_state_dir(&mut self) -> Result<(), Phase39eBackendError> {
        self.ensure_dir_calls = self.ensure_dir_calls.saturating_add(1);
        match self.ops.ensure_state_dir() {
            Ok(()) => {
                self.last_error = Phase39fRuntimeFileError::None;
                Ok(())
            }
            Err(error) => {
                self.last_error = error;
                Err(error.as_phase39e_error())
            }
        }
    }

    fn write_direct(
        &mut self,
        command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult {
        self.write_calls = self.write_calls.saturating_add(1);
        let result = self
            .ops
            .write_record_direct(command.target_path.as_slice(), command.payload);
        self.record_result(result)
    }

    fn write_atomic(
        &mut self,
        command: Phase39eSdFatWriteCommand<'_>,
    ) -> Phase39eSdFatBackendResult {
        self.write_calls = self.write_calls.saturating_add(1);
        let result = self.ops.write_record_atomic(
            command.target_path.as_slice(),
            command.temp_path.as_slice(),
            command.payload,
        );
        self.record_result(result)
    }
}

/// Recording runtime-owned file ops for validation and smoke tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39fRecordingRuntimeFileOps {
    pub ensure_state_dir_calls: usize,
    pub direct_write_calls: usize,
    pub atomic_write_calls: usize,
    pub last_target_path: Phase39dTypedRecordPath,
    pub last_payload_len: usize,
    pub last_error: Phase39fRuntimeFileError,
    pub accept_state_dir: bool,
    pub accept_writes: bool,
}

impl Phase39fRecordingRuntimeFileOps {
    pub const fn accepting() -> Self {
        Self {
            ensure_state_dir_calls: 0,
            direct_write_calls: 0,
            atomic_write_calls: 0,
            last_target_path: Phase39dTypedRecordPath::empty(),
            last_payload_len: 0,
            last_error: Phase39fRuntimeFileError::None,
            accept_state_dir: true,
            accept_writes: true,
        }
    }

    pub const fn rejecting_writes() -> Self {
        Self {
            ensure_state_dir_calls: 0,
            direct_write_calls: 0,
            atomic_write_calls: 0,
            last_target_path: Phase39dTypedRecordPath::empty(),
            last_payload_len: 0,
            last_error: Phase39fRuntimeFileError::WriteFailed,
            accept_state_dir: true,
            accept_writes: false,
        }
    }

    pub const fn rejecting_state_dir() -> Self {
        Self {
            ensure_state_dir_calls: 0,
            direct_write_calls: 0,
            atomic_write_calls: 0,
            last_target_path: Phase39dTypedRecordPath::empty(),
            last_payload_len: 0,
            last_error: Phase39fRuntimeFileError::StateDirUnavailable,
            accept_state_dir: false,
            accept_writes: true,
        }
    }

    pub fn write_committed(&self) -> bool {
        self.last_error == Phase39fRuntimeFileError::None
            && (self.direct_write_calls + self.atomic_write_calls) == 1
    }
}

impl Phase39fRuntimeOwnedFileOps for Phase39fRecordingRuntimeFileOps {
    fn ensure_state_dir(&mut self) -> Result<(), Phase39fRuntimeFileError> {
        self.ensure_state_dir_calls = self.ensure_state_dir_calls.saturating_add(1);
        if self.accept_state_dir {
            self.last_error = Phase39fRuntimeFileError::None;
            Ok(())
        } else {
            self.last_error = Phase39fRuntimeFileError::StateDirUnavailable;
            Err(Phase39fRuntimeFileError::StateDirUnavailable)
        }
    }

    fn write_record_direct(
        &mut self,
        target_path: &[u8],
        payload: &[u8],
    ) -> Result<usize, Phase39fRuntimeFileError> {
        self.direct_write_calls = self.direct_write_calls.saturating_add(1);
        self.last_payload_len = payload.len();
        self.last_target_path = path_from_slice(target_path);

        if self.accept_writes {
            self.last_error = Phase39fRuntimeFileError::None;
            Ok(payload.len())
        } else {
            self.last_error = Phase39fRuntimeFileError::WriteFailed;
            Err(Phase39fRuntimeFileError::WriteFailed)
        }
    }

    fn write_record_atomic(
        &mut self,
        target_path: &[u8],
        _temp_path: &[u8],
        payload: &[u8],
    ) -> Result<usize, Phase39fRuntimeFileError> {
        self.atomic_write_calls = self.atomic_write_calls.saturating_add(1);
        self.last_payload_len = payload.len();
        self.last_target_path = path_from_slice(target_path);

        if self.accept_writes {
            self.last_error = Phase39fRuntimeFileError::None;
            Ok(payload.len())
        } else {
            self.last_error = Phase39fRuntimeFileError::RenameFailed;
            Err(Phase39fRuntimeFileError::RenameFailed)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase39fRuntimeOwnedWriterReport {
    pub status: Phase39fRuntimeWriterStatus,
    pub kind: Phase39dTypedRecordKind,
    pub backend_status: Phase39dBackendWriteStatus,
    pub runtime_error: Phase39fRuntimeFileError,
    pub payload_len: usize,
    pub bytes_written: usize,
    pub target_path: Phase39dTypedRecordPath,
    pub ensure_dir_calls: usize,
    pub write_calls: usize,
    pub next_lane: Phase39fRuntimeWriterNextLane,
}

impl Phase39fRuntimeOwnedWriterReport {
    pub const fn committed(self) -> bool {
        matches!(
            self.status,
            Phase39fRuntimeWriterStatus::RuntimeWriteCommitted
        )
    }
}

pub fn phase39f_execute_with_runtime_owned_file_ops<O: Phase39fRuntimeOwnedFileOps>(
    ops: &mut O,
    request: Phase39dTypedWriteRequest<'_>,
    config: Phase39eSdFatAdapterConfig,
) -> Phase39fRuntimeOwnedWriterReport {
    let mut backend = Phase39fRuntimeOwnedSdFatBackend::new(ops);
    let adapter_report = phase39e_execute_with_sdfat_adapter(&mut backend, request, config);
    let runtime_error = backend.last_error();
    let ensure_dir_calls = backend.ensure_dir_calls();
    let write_calls = backend.write_calls();

    phase39f_report_from_phase39e(adapter_report, runtime_error, ensure_dir_calls, write_calls)
}

pub const fn phase39f_report_from_phase39e(
    report: Phase39eSdFatAdapterReport,
    runtime_error: Phase39fRuntimeFileError,
    ensure_dir_calls: usize,
    write_calls: usize,
) -> Phase39fRuntimeOwnedWriterReport {
    Phase39fRuntimeOwnedWriterReport {
        status: phase39f_status_from_phase39e(report.backend_status, runtime_error),
        kind: report.kind,
        backend_status: report.backend_status,
        runtime_error,
        payload_len: report.payload_len,
        bytes_written: report.bytes_written,
        target_path: report.target_path,
        ensure_dir_calls,
        write_calls,
        next_lane: phase39f_next_lane_from_status(report.backend_status, runtime_error),
    }
}

pub const fn phase39f_status_from_phase39e(
    backend_status: Phase39dBackendWriteStatus,
    runtime_error: Phase39fRuntimeFileError,
) -> Phase39fRuntimeWriterStatus {
    match backend_status {
        Phase39dBackendWriteStatus::NotCalled => Phase39fRuntimeWriterStatus::DryRunAccepted,
        Phase39dBackendWriteStatus::Committed => Phase39fRuntimeWriterStatus::RuntimeWriteCommitted,
        Phase39dBackendWriteStatus::BackendRejected => {
            if matches!(runtime_error, Phase39fRuntimeFileError::Unsupported) {
                Phase39fRuntimeWriterStatus::RuntimeBackendUnavailable
            } else {
                Phase39fRuntimeWriterStatus::RuntimeBackendRejected
            }
        }
        Phase39dBackendWriteStatus::BackendUnavailable => {
            Phase39fRuntimeWriterStatus::RuntimeBackendUnavailable
        }
    }
}

pub const fn phase39f_next_lane_from_status(
    backend_status: Phase39dBackendWriteStatus,
    runtime_error: Phase39fRuntimeFileError,
) -> Phase39fRuntimeWriterNextLane {
    match backend_status {
        Phase39dBackendWriteStatus::Committed | Phase39dBackendWriteStatus::NotCalled => {
            Phase39fRuntimeWriterNextLane::WireReaderRuntimeCallSites
        }
        Phase39dBackendWriteStatus::BackendRejected => match runtime_error {
            Phase39fRuntimeFileError::None => Phase39fRuntimeWriterNextLane::RepairRuntimeBackend,
            Phase39fRuntimeFileError::Unsupported => {
                Phase39fRuntimeWriterNextLane::AddRuntimeFeatureGate
            }
            Phase39fRuntimeFileError::StateDirUnavailable
            | Phase39fRuntimeFileError::OpenFailed
            | Phase39fRuntimeFileError::WriteFailed
            | Phase39fRuntimeFileError::FlushFailed
            | Phase39fRuntimeFileError::RenameFailed
            | Phase39fRuntimeFileError::RemoveFailed => {
                Phase39fRuntimeWriterNextLane::RepairRuntimeBackend
            }
        },
        Phase39dBackendWriteStatus::BackendUnavailable => {
            Phase39fRuntimeWriterNextLane::AddRuntimeFeatureGate
        }
    }
}

fn path_from_slice(path: &[u8]) -> Phase39dTypedRecordPath {
    let mut bytes = [0u8;
        crate::vaachak_x4::runtime::state_io_typed_record_write_lane::PHASE_39D_MAX_RECORD_PATH_LEN];
    let len = path.len().min(bytes.len());
    bytes[..len].copy_from_slice(&path[..len]);
    // Re-rendering by kind is not available here, so store a best-effort empty
    // path for validation if the raw bytes do not map through the public
    // constructor. Runtime call-sites should use the report target_path from
    // Phase 39E for authoritative path inspection.
    let _ = bytes;
    Phase39dTypedRecordPath::empty()
}

pub fn phase39f_marker() -> &'static str {
    PHASE_39F_RUNTIME_OWNED_SDFAT_TYPED_RECORD_WRITER_BINDING_MARKER
}
