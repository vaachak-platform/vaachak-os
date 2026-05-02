//! Phase 36W — State I/O real backend adapter contract.
//!
//! This module describes the adapter seam that a later SD-backed state I/O
//! implementation will use. It is intentionally side-effect free: no device,
//! filesystem, SPI, display, input, or power operation is performed here.

#![allow(clippy::struct_excessive_bools)]

/// Phase marker emitted by the overlay installer/check scripts.
pub const PHASE_36W_STATE_IO_REAL_BACKEND_ADAPTER_CONTRACT_MARKER: &str =
    "phase36w=x4-state-io-real-backend-adapter-contract-ok";

/// The first backend lane that this contract is preparing.
pub const PHASE_36W_BACKEND_LANE: &str = "typed-state-real-backend-adapter-contract";

/// Logical typed-state record kinds managed by the future backend adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoAdapterRecordKind {
    Progress,
    Theme,
    Metadata,
    Bookmark,
    BookmarkIndex,
}

impl StateIoAdapterRecordKind {
    /// Returns the 8.3-safe extension or fixed filename suffix associated with the record kind.
    pub const fn canonical_suffix(self) -> &'static str {
        match self {
            Self::Progress => ".PRG",
            Self::Theme => ".THM",
            Self::Metadata => ".MTA",
            Self::Bookmark => ".BKM",
            Self::BookmarkIndex => "BMIDX.TXT",
        }
    }

    /// Returns whether the record is per-book rather than global/index scoped.
    pub const fn is_per_book(self) -> bool {
        !matches!(self, Self::BookmarkIndex)
    }
}

/// Operation classes that the future adapter may support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoAdapterOperation {
    Read,
    WriteShadow,
    CommitShadow,
    DeleteShadow,
    Probe,
}

/// Safety posture for a future backend adapter operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateIoAdapterSafetyClass {
    ReadOnlyProbe,
    ShadowWriteOnly,
    AtomicCommitRequired,
    RecoveryOnly,
}

/// A single operation permission in the adapter contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoAdapterPermission {
    pub record_kind: StateIoAdapterRecordKind,
    pub operation: StateIoAdapterOperation,
    pub safety_class: StateIoAdapterSafetyClass,
    pub requires_shadow_path: bool,
    pub requires_commit_gate: bool,
}

/// Static adapter contract summary. This is metadata only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateIoRealBackendAdapterContract {
    pub phase_marker: &'static str,
    pub lane: &'static str,
    pub side_effect_free: bool,
    pub hardware_behavior_moved: bool,
    pub real_backend_calls_enabled: bool,
    pub permissions: &'static [StateIoAdapterPermission],
}

const PHASE_36W_PERMISSIONS: &[StateIoAdapterPermission] = &[
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Progress,
        operation: StateIoAdapterOperation::Probe,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Progress,
        operation: StateIoAdapterOperation::Read,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Progress,
        operation: StateIoAdapterOperation::WriteShadow,
        safety_class: StateIoAdapterSafetyClass::ShadowWriteOnly,
        requires_shadow_path: true,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Progress,
        operation: StateIoAdapterOperation::CommitShadow,
        safety_class: StateIoAdapterSafetyClass::AtomicCommitRequired,
        requires_shadow_path: true,
        requires_commit_gate: true,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Theme,
        operation: StateIoAdapterOperation::Probe,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Theme,
        operation: StateIoAdapterOperation::Read,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Metadata,
        operation: StateIoAdapterOperation::Probe,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Metadata,
        operation: StateIoAdapterOperation::Read,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::Bookmark,
        operation: StateIoAdapterOperation::Probe,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
    StateIoAdapterPermission {
        record_kind: StateIoAdapterRecordKind::BookmarkIndex,
        operation: StateIoAdapterOperation::Probe,
        safety_class: StateIoAdapterSafetyClass::ReadOnlyProbe,
        requires_shadow_path: false,
        requires_commit_gate: false,
    },
];

/// Static Phase 36W adapter contract.
pub const PHASE_36W_STATE_IO_REAL_BACKEND_ADAPTER_CONTRACT: StateIoRealBackendAdapterContract =
    StateIoRealBackendAdapterContract {
        phase_marker: PHASE_36W_STATE_IO_REAL_BACKEND_ADAPTER_CONTRACT_MARKER,
        lane: PHASE_36W_BACKEND_LANE,
        side_effect_free: true,
        hardware_behavior_moved: false,
        real_backend_calls_enabled: false,
        permissions: PHASE_36W_PERMISSIONS,
    };

/// Returns the Phase 36W contract.
pub const fn phase36w_state_io_real_backend_adapter_contract()
-> &'static StateIoRealBackendAdapterContract {
    &PHASE_36W_STATE_IO_REAL_BACKEND_ADAPTER_CONTRACT
}

/// Reports whether Phase 36W remains a metadata-only adapter contract.
pub const fn phase36w_is_side_effect_free() -> bool {
    PHASE_36W_STATE_IO_REAL_BACKEND_ADAPTER_CONTRACT.side_effect_free
        && !PHASE_36W_STATE_IO_REAL_BACKEND_ADAPTER_CONTRACT.hardware_behavior_moved
        && !PHASE_36W_STATE_IO_REAL_BACKEND_ADAPTER_CONTRACT.real_backend_calls_enabled
}

/// Finds whether a record/operation pair is declared in the adapter contract.
pub fn phase36w_allows_operation(
    record_kind: StateIoAdapterRecordKind,
    operation: StateIoAdapterOperation,
) -> bool {
    PHASE_36W_PERMISSIONS.iter().any(|permission| {
        permission.record_kind == record_kind && permission.operation == operation
    })
}

/// Returns the permission metadata for a record/operation pair.
pub fn phase36w_permission_for(
    record_kind: StateIoAdapterRecordKind,
    operation: StateIoAdapterOperation,
) -> Option<StateIoAdapterPermission> {
    PHASE_36W_PERMISSIONS.iter().copied().find(|permission| {
        permission.record_kind == record_kind && permission.operation == operation
    })
}
