#![allow(dead_code)]

use crate::vaachak_x4::physical::pulp_hardware_reference_deprecation_audit::{
    VaachakPulpHardwareReferenceDeprecationAudit, VaachakPulpReferenceClassification,
};

pub struct VaachakPulpHardwareReferenceDeprecationAuditSmoke;

impl VaachakPulpHardwareReferenceDeprecationAuditSmoke {
    pub const MARKER: &'static str = "pulp_hardware_reference_deprecation_audit=ok";

    pub fn smoke_ok() -> bool {
        let report = VaachakPulpHardwareReferenceDeprecationAudit::report();
        let entries = VaachakPulpHardwareReferenceDeprecationAudit::AUDIT_ENTRIES;
        report.ok()
            && VaachakPulpHardwareReferenceDeprecationAudit::audit_ok()
            && report.marker == Self::MARKER
            && entries.iter().any(|entry| {
                matches!(
                    entry.classification,
                    VaachakPulpReferenceClassification::StillRequiredRuntimeDependency
                )
            })
            && entries.iter().any(|entry| {
                matches!(
                    entry.classification,
                    VaachakPulpReferenceClassification::CompatibilityImportBoundary
                )
            })
            && entries.iter().any(|entry| {
                matches!(
                    entry.classification,
                    VaachakPulpReferenceClassification::DeadLegacyHardwarePath
                )
            })
            && entries.iter().any(|entry| {
                matches!(
                    entry.classification,
                    VaachakPulpReferenceClassification::DocumentationOnlyReference
                )
            })
            && entries.iter().any(|entry| {
                matches!(
                    entry.classification,
                    VaachakPulpReferenceClassification::SafeToRemoveOverlayScaffoldArtifact
                )
            })
    }
}
