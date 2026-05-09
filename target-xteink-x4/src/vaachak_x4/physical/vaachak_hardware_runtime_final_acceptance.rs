#![allow(dead_code)]

use super::hardware_physical_full_migration_cleanup::VaachakHardwarePhysicalFullMigrationCleanup;
use super::hardware_physical_full_migration_consolidation::VaachakHardwarePhysicalFullMigrationConsolidation;
use super::pulp_hardware_dead_path_quarantine::VaachakPulpHardwareDeadPathQuarantine;
use super::pulp_hardware_dead_path_removal::VaachakPulpHardwareDeadPathRemoval;
use super::pulp_hardware_reference_deprecation_audit::VaachakPulpHardwareReferenceDeprecationAudit;
use super::vendor_pulp_os_scope_reduction::VaachakVendorPulpOsScopeReduction;

/// Final acceptance gate for the Vaachak-owned hardware runtime.
///
/// This checkpoint intentionally does not move new behavior. It consolidates
/// the accepted full native hardware stack, Pulp hardware deprecation state,
/// vendor scope reduction, and required device smoke evidence into one final
/// acceptance surface before upload.
pub struct VaachakHardwareRuntimeFinalAcceptance;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeFinalAcceptanceDomain {
    FullPhysicalMigration,
    FullPhysicalMigrationCleanup,
    PulpHardwareReferenceDeprecationAudit,
    PulpHardwareDeadPathQuarantine,
    PulpHardwareDeadPathRemoval,
    VendorPulpOsScopeReduction,
    DeviceSmokeRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeFinalAcceptanceStatus {
    pub marker: &'static str,
    pub acceptance_owner: &'static str,
    pub full_physical_migration_ok: bool,
    pub full_physical_migration_cleanup_ok: bool,
    pub pulp_reference_deprecation_audit_ok: bool,
    pub pulp_dead_path_quarantine_ok: bool,
    pub pulp_dead_path_removal_ok: bool,
    pub vendor_pulp_os_scope_reduction_ok: bool,
    pub vendor_pulp_os_present: bool,
    pub vendor_pulp_os_removed_by_acceptance: bool,
    pub active_pulp_hardware_fallback_remaining: bool,
    pub unclassified_pulp_hardware_reference_remaining: bool,
    pub device_smoke_required_after_flash: bool,
    pub app_behavior_changed_by_acceptance: bool,
    pub reader_file_browser_ux_changed_by_acceptance: bool,
    pub display_input_storage_spi_behavior_changed_by_acceptance: bool,
    pub accepted_domain_count: usize,
}

impl VaachakHardwareRuntimeFinalAcceptanceStatus {
    pub const fn ok(self) -> bool {
        self.full_physical_migration_ok
            && self.full_physical_migration_cleanup_ok
            && self.pulp_reference_deprecation_audit_ok
            && self.pulp_dead_path_quarantine_ok
            && self.pulp_dead_path_removal_ok
            && self.vendor_pulp_os_scope_reduction_ok
            && self.vendor_pulp_os_present
            && !self.vendor_pulp_os_removed_by_acceptance
            && !self.active_pulp_hardware_fallback_remaining
            && !self.unclassified_pulp_hardware_reference_remaining
            && self.device_smoke_required_after_flash
            && !self.app_behavior_changed_by_acceptance
            && !self.reader_file_browser_ux_changed_by_acceptance
            && !self.display_input_storage_spi_behavior_changed_by_acceptance
            && self.accepted_domain_count
                == VaachakHardwareRuntimeFinalAcceptance::ACCEPTED_DOMAIN_COUNT
    }
}

impl VaachakHardwareRuntimeFinalAcceptance {
    pub const MARKER: &'static str = "vaachak_hardware_runtime_final_acceptance=ok";
    pub const ACCEPTANCE_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const ACCEPTED_DOMAIN_COUNT: usize = 7;
    pub const DEVICE_SMOKE_REQUIRED_AFTER_FLASH: bool = true;
    pub const VENDOR_PULP_OS_REMOVED_BY_ACCEPTANCE: bool = false;
    pub const APP_BEHAVIOR_CHANGED_BY_ACCEPTANCE: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED_BY_ACCEPTANCE: bool = false;
    pub const DISPLAY_INPUT_STORAGE_SPI_BEHAVIOR_CHANGED_BY_ACCEPTANCE: bool = false;

    pub const ACCEPTED_DOMAINS: [VaachakHardwareRuntimeFinalAcceptanceDomain; 7] = [
        VaachakHardwareRuntimeFinalAcceptanceDomain::FullPhysicalMigration,
        VaachakHardwareRuntimeFinalAcceptanceDomain::FullPhysicalMigrationCleanup,
        VaachakHardwareRuntimeFinalAcceptanceDomain::PulpHardwareReferenceDeprecationAudit,
        VaachakHardwareRuntimeFinalAcceptanceDomain::PulpHardwareDeadPathQuarantine,
        VaachakHardwareRuntimeFinalAcceptanceDomain::PulpHardwareDeadPathRemoval,
        VaachakHardwareRuntimeFinalAcceptanceDomain::VendorPulpOsScopeReduction,
        VaachakHardwareRuntimeFinalAcceptanceDomain::DeviceSmokeRequired,
    ];

    pub fn status() -> VaachakHardwareRuntimeFinalAcceptanceStatus {
        VaachakHardwareRuntimeFinalAcceptanceStatus {
            marker: Self::MARKER,
            acceptance_owner: Self::ACCEPTANCE_OWNER,
            full_physical_migration_ok:
                VaachakHardwarePhysicalFullMigrationConsolidation::consolidation_ok(),
            full_physical_migration_cleanup_ok:
                VaachakHardwarePhysicalFullMigrationCleanup::cleanup_ok(),
            pulp_reference_deprecation_audit_ok:
                VaachakPulpHardwareReferenceDeprecationAudit::audit_ok(),
            pulp_dead_path_quarantine_ok: VaachakPulpHardwareDeadPathQuarantine::quarantine_ok(),
            pulp_dead_path_removal_ok: VaachakPulpHardwareDeadPathRemoval::removal_ok(),
            vendor_pulp_os_scope_reduction_ok:
                VaachakVendorPulpOsScopeReduction::scope_reduction_ok(),
            vendor_pulp_os_present: VaachakVendorPulpOsScopeReduction::VENDOR_PULP_OS_PRESENT,
            vendor_pulp_os_removed_by_acceptance: Self::VENDOR_PULP_OS_REMOVED_BY_ACCEPTANCE,
            active_pulp_hardware_fallback_remaining:
                VaachakVendorPulpOsScopeReduction::ACTIVE_PULP_HARDWARE_FALLBACK_REMAINING,
            unclassified_pulp_hardware_reference_remaining:
                VaachakVendorPulpOsScopeReduction::UNCLASSIFIED_VENDOR_PULP_HARDWARE_SURFACE_REMAINING,
            device_smoke_required_after_flash: Self::DEVICE_SMOKE_REQUIRED_AFTER_FLASH,
            app_behavior_changed_by_acceptance: Self::APP_BEHAVIOR_CHANGED_BY_ACCEPTANCE,
            reader_file_browser_ux_changed_by_acceptance:
                Self::READER_FILE_BROWSER_UX_CHANGED_BY_ACCEPTANCE,
            display_input_storage_spi_behavior_changed_by_acceptance:
                Self::DISPLAY_INPUT_STORAGE_SPI_BEHAVIOR_CHANGED_BY_ACCEPTANCE,
            accepted_domain_count: Self::ACCEPTED_DOMAINS.len(),
        }
    }

    pub fn final_acceptance_ok() -> bool {
        Self::status().ok()
    }
}
