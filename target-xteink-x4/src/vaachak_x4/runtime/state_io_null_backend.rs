//! Phase 36S — State I/O Null Backend Overlay.
//!
//! This module provides the first implementation-shaped backend for the typed
//! state I/O lane. It implements the Phase 36R scaffold trait with a null
//! backend that validates request shapes and returns side-effect-free responses.
//! It deliberately performs no storage, display, input, power, or SPI work.

#![allow(dead_code)]

use super::state_io_real_backend_scaffold::{
    StateBackendOperation, StateBackendProbe, StateBackendRequest, StateBackendResponse,
    StateIoBackendScaffold, StateRecordKind, phase36r_is_accepted, phase36r_marker,
};

/// Phase 36S boot/build marker.
pub const PHASE_36S_STATE_IO_NULL_BACKEND_MARKER: &str = "phase36s=x4-state-io-null-backend-ok";

/// Prior scaffold marker required before this null backend is accepted.
pub const REQUIRED_PHASE_36R_MARKER: &str = "phase36r=x4-state-io-real-backend-scaffold-ok";

/// Side-effect-free backend name used by the Phase 36S probe.
pub const STATE_IO_NULL_BACKEND_NAME: &str = "vaachak-x4-state-io-null-backend";

/// Records covered by the null backend implementation shape.
pub const STATE_IO_NULL_BACKEND_RECORDS: [StateRecordKind; 5] = [
    StateRecordKind::Progress,
    StateRecordKind::Theme,
    StateRecordKind::Metadata,
    StateRecordKind::Bookmark,
    StateRecordKind::BookmarkIndex,
];

/// Operations accepted by the null backend implementation shape.
pub const STATE_IO_NULL_BACKEND_OPERATIONS: [StateBackendOperation; 5] = [
    StateBackendOperation::Probe,
    StateBackendOperation::Load,
    StateBackendOperation::Store,
    StateBackendOperation::Remove,
    StateBackendOperation::RebuildIndex,
];

/// Guardrails that keep the null backend safe for the current phase.
pub const STATE_IO_NULL_BACKEND_GUARDRAILS: [&str; 10] = [
    "phase36r-scaffold-accepted",
    "null-backend-only",
    "request-shape-validation-only",
    "side-effects-disabled",
    "no-real-storage-calls",
    "no-display-runtime-calls",
    "no-input-runtime-calls",
    "no-power-runtime-calls",
    "no-spi-runtime-calls",
    "existing-pulp-runtime-remains-authoritative",
];

/// Minimum record kind count expected for this phase.
pub const REQUIRED_NULL_BACKEND_RECORD_COUNT: usize = 5;

/// Minimum operation count expected for this phase.
pub const REQUIRED_NULL_BACKEND_OPERATION_COUNT: usize = 5;

/// Minimum guardrail count expected for this phase.
pub const REQUIRED_NULL_BACKEND_GUARDRAIL_COUNT: usize = 10;

/// Errors produced by the null backend implementation shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NullStateIoBackendError {
    PriorScaffoldNotAccepted,
    InvalidRequestShape,
    InvalidOperationForMethod,
}

impl NullStateIoBackendError {
    pub const fn label(self) -> &'static str {
        match self {
            Self::PriorScaffoldNotAccepted => "prior-scaffold-not-accepted",
            Self::InvalidRequestShape => "invalid-request-shape",
            Self::InvalidOperationForMethod => "invalid-operation-for-method",
        }
    }
}

/// Side-effect-free null backend for exercising the Phase 36R trait shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NullStateIoBackend {
    pub scaffold_accepted: bool,
    pub side_effects_enabled: bool,
}

impl NullStateIoBackend {
    pub const fn new() -> Self {
        Self {
            scaffold_accepted: phase36r_is_accepted(),
            side_effects_enabled: false,
        }
    }

    pub const fn is_ready(self) -> bool {
        self.scaffold_accepted && !self.side_effects_enabled
    }

    pub const fn status(self) -> StateIoNullBackendStatus {
        StateIoNullBackendStatus::from_backend(self)
    }

    fn ensure_ready(self) -> Result<(), NullStateIoBackendError> {
        if self.is_ready() {
            Ok(())
        } else {
            Err(NullStateIoBackendError::PriorScaffoldNotAccepted)
        }
    }

    fn validate_request(
        self,
        request: StateBackendRequest<'_>,
        expected_operation: StateBackendOperation,
    ) -> Result<(), NullStateIoBackendError> {
        self.ensure_ready()?;

        if request.operation != expected_operation {
            return Err(NullStateIoBackendError::InvalidOperationForMethod);
        }

        if !request.is_shape_valid() {
            return Err(NullStateIoBackendError::InvalidRequestShape);
        }

        Ok(())
    }
}

impl Default for NullStateIoBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl StateIoBackendScaffold for NullStateIoBackend {
    type Error = NullStateIoBackendError;

    fn probe(&mut self) -> Result<StateBackendProbe, Self::Error> {
        self.ensure_ready()?;
        Ok(StateBackendProbe {
            backend_name: STATE_IO_NULL_BACKEND_NAME,
            marker: PHASE_36S_STATE_IO_NULL_BACKEND_MARKER,
            ready: self.is_ready(),
            side_effects_enabled: self.side_effects_enabled,
            record_count: STATE_IO_NULL_BACKEND_RECORDS.len(),
            operation_count: STATE_IO_NULL_BACKEND_OPERATIONS.len(),
        })
    }

    fn load<'a>(
        &mut self,
        request: StateBackendRequest<'a>,
    ) -> Result<StateBackendResponse<'a>, Self::Error> {
        self.validate_request(request, StateBackendOperation::Load)?;
        Ok(StateBackendResponse::dry_shape(
            request.operation,
            request.record_kind,
            None,
        ))
    }

    fn store<'a>(
        &mut self,
        request: StateBackendRequest<'a>,
    ) -> Result<StateBackendResponse<'a>, Self::Error> {
        self.validate_request(request, StateBackendOperation::Store)?;
        Ok(StateBackendResponse::dry_shape(
            request.operation,
            request.record_kind,
            request.payload,
        ))
    }

    fn remove<'a>(
        &mut self,
        request: StateBackendRequest<'a>,
    ) -> Result<StateBackendResponse<'a>, Self::Error> {
        self.validate_request(request, StateBackendOperation::Remove)?;
        Ok(StateBackendResponse::dry_shape(
            request.operation,
            request.record_kind,
            None,
        ))
    }
}

/// Side-effect-free status surface for the Phase 36S null backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoNullBackendStatus {
    pub marker: &'static str,
    pub required_scaffold_marker: &'static str,
    pub observed_scaffold_marker: &'static str,
    pub backend_name: &'static str,
    pub scaffold_accepted: bool,
    pub null_backend_available: bool,
    pub backend_default_enabled: bool,
    pub side_effects_enabled: bool,
    pub record_count: usize,
    pub operation_count: usize,
    pub guardrail_count: usize,
    pub required_record_count: usize,
    pub required_operation_count: usize,
    pub required_guardrail_count: usize,
    pub storage_behavior_moved: bool,
    pub display_behavior_moved: bool,
    pub input_behavior_moved: bool,
    pub power_behavior_moved: bool,
    pub spi_behavior_moved: bool,
    pub next_phase: &'static str,
}

impl StateIoNullBackendStatus {
    pub const fn from_backend(backend: NullStateIoBackend) -> Self {
        Self {
            marker: PHASE_36S_STATE_IO_NULL_BACKEND_MARKER,
            required_scaffold_marker: REQUIRED_PHASE_36R_MARKER,
            observed_scaffold_marker: phase36r_marker(),
            backend_name: STATE_IO_NULL_BACKEND_NAME,
            scaffold_accepted: backend.scaffold_accepted,
            null_backend_available: true,
            backend_default_enabled: false,
            side_effects_enabled: backend.side_effects_enabled,
            record_count: STATE_IO_NULL_BACKEND_RECORDS.len(),
            operation_count: STATE_IO_NULL_BACKEND_OPERATIONS.len(),
            guardrail_count: STATE_IO_NULL_BACKEND_GUARDRAILS.len(),
            required_record_count: REQUIRED_NULL_BACKEND_RECORD_COUNT,
            required_operation_count: REQUIRED_NULL_BACKEND_OPERATION_COUNT,
            required_guardrail_count: REQUIRED_NULL_BACKEND_GUARDRAIL_COUNT,
            storage_behavior_moved: false,
            display_behavior_moved: false,
            input_behavior_moved: false,
            power_behavior_moved: false,
            spi_behavior_moved: false,
            next_phase: "phase36t-state-io-null-backend-acceptance",
        }
    }

    pub const fn current() -> Self {
        Self::from_backend(NullStateIoBackend::new())
    }

    pub const fn is_accepted(self) -> bool {
        self.scaffold_accepted
            && self.null_backend_available
            && !self.backend_default_enabled
            && !self.side_effects_enabled
            && self.record_count >= self.required_record_count
            && self.operation_count >= self.required_operation_count
            && self.guardrail_count >= self.required_guardrail_count
            && !self.storage_behavior_moved
            && !self.display_behavior_moved
            && !self.input_behavior_moved
            && !self.power_behavior_moved
            && !self.spi_behavior_moved
    }
}

/// Compile-time Phase 36S status.
pub const STATE_IO_NULL_BACKEND_STATUS: StateIoNullBackendStatus =
    StateIoNullBackendStatus::current();

/// Return the accepted Phase 36S marker for boot/runtime status reporting.
pub const fn phase36s_marker() -> &'static str {
    PHASE_36S_STATE_IO_NULL_BACKEND_MARKER
}

/// Return the Phase 36S null-backend status report.
pub const fn phase36s_null_backend_status() -> StateIoNullBackendStatus {
    STATE_IO_NULL_BACKEND_STATUS
}

/// Return whether the Phase 36S null backend is accepted.
pub const fn phase36s_is_accepted() -> bool {
    STATE_IO_NULL_BACKEND_STATUS.is_accepted()
}

/// Return a new null backend instance for compile-only probes.
pub const fn phase36s_null_backend() -> NullStateIoBackend {
    NullStateIoBackend::new()
}
