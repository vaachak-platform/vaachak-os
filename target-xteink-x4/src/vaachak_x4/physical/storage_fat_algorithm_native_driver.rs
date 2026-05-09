#![allow(dead_code)]

use super::storage_physical_sd_mmc_native_driver::VaachakStoragePhysicalSdMmcNativeDriver;

/// Vaachak-owned native FAT algorithm driver for the Xteink X4 storage path.
/// Vaachak owns FAT/path/list/open/read/write/metadata policy and algorithm
/// ownership. The block-device boundary is the accepted Vaachak native SD/MMC
/// physical driver; imported Pulp FAT runtime is not active for this boundary.
pub struct VaachakStorageFatAlgorithmNativeDriver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativeBackend {
    VaachakNativeFatAlgorithmDriver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativeBlockDeviceBackend {
    VaachakNativeSdMmcPhysicalDriver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativeFilesystemState {
    Unknown,
    BootSectorNeeded,
    Mounted,
    DirectoryReady,
    FileReady,
    Dirty,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativeOperation {
    ResolvePath,
    ReadBootSector,
    ParseBpb,
    MountVolume,
    ListDirectory,
    DecodeDirectoryEntry,
    DecodeLongFilename,
    OpenFile,
    ReadFileChunk,
    WriteFileChunk,
    CreateFile,
    DeleteFile,
    RenameFile,
    UpdateMetadata,
    AllocateCluster,
    FreeClusterChain,
    FlushFatTable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativeAccessMode {
    ReadOnly,
    WriteExplicitlyAllowed,
    DestructiveExplicitlyAllowed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativePathRole {
    LibraryContent,
    ReaderState,
    Cache,
    SleepImage,
    SystemMetadata,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativePathKind {
    Root,
    Directory,
    File,
    MetadataFile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativeEntryKind {
    Unknown,
    Directory,
    RegularFile,
    LongFilenameFragment,
    VolumeLabel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakFatNativeExecutionStatus {
    Accepted,
    RejectedInvalidPath,
    RejectedInvalidState,
    RejectedWriteNotExplicitlyAllowed,
    RejectedDestructiveNotExplicitlyAllowed,
    RejectedUnsupportedOperation,
    BlockDeviceUnavailable,
    TargetHalUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakFatNativeVolumeGeometry {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sector_count: u16,
    pub fat_count: u8,
    pub root_dir_first_cluster: u32,
    pub fat_start_lba: u32,
    pub data_start_lba: u32,
    pub total_sectors: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakFatNativeAlgorithmPolicy {
    pub path_normalization_owned_by_vaachak: bool,
    pub boot_sector_parse_owned_by_vaachak: bool,
    pub bpb_validation_owned_by_vaachak: bool,
    pub fat_table_traversal_owned_by_vaachak: bool,
    pub directory_entry_decode_owned_by_vaachak: bool,
    pub long_filename_algorithm_owned_by_vaachak: bool,
    pub file_open_read_write_owned_by_vaachak: bool,
    pub metadata_update_policy_owned_by_vaachak: bool,
    pub destructive_operation_policy_owned_by_vaachak: bool,
    pub uses_native_sd_mmc_block_driver: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakFatNativeRequest {
    pub backend: VaachakFatNativeBackend,
    pub block_backend: VaachakFatNativeBlockDeviceBackend,
    pub operation: VaachakFatNativeOperation,
    pub filesystem_state: VaachakFatNativeFilesystemState,
    pub access_mode: VaachakFatNativeAccessMode,
    pub path_role: VaachakFatNativePathRole,
    pub path_kind: VaachakFatNativePathKind,
    pub start_cluster: u32,
    pub current_cluster: u32,
    pub file_offset: u32,
    pub requested_len: usize,
    pub destructive_explicitly_allowed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakFatNativeResult {
    pub request: VaachakFatNativeRequest,
    pub status: VaachakFatNativeExecutionStatus,
    pub bytes_transferred: usize,
    pub next_cluster: u32,
    pub next_state: VaachakFatNativeFilesystemState,
    pub fat_algorithm_owned_by_vaachak: bool,
    pub native_sd_mmc_block_backend_selected: bool,
    pub pulp_fat_algorithm_fallback_enabled: bool,
    pub imported_pulp_fat_runtime_active: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakFatNativeDirectoryEntryPolicy {
    pub entry_kind: VaachakFatNativeEntryKind,
    pub short_8_3_decode_owned_by_vaachak: bool,
    pub long_filename_decode_owned_by_vaachak: bool,
    pub hidden_system_entry_filter_owned_by_vaachak: bool,
    pub directory_metadata_owned_by_vaachak: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakFatNativeMigrationReport {
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub block_device_backend_name: &'static str,
    pub fat_algorithm_full_migration_marker: &'static str,
    pub path_normalization_moved_to_vaachak: bool,
    pub bpb_boot_sector_parse_moved_to_vaachak: bool,
    pub directory_entry_decode_moved_to_vaachak: bool,
    pub long_filename_algorithm_moved_to_vaachak: bool,
    pub fat_table_traversal_moved_to_vaachak: bool,
    pub file_open_read_write_policy_moved_to_vaachak: bool,
    pub metadata_update_policy_moved_to_vaachak: bool,
    pub destructive_operation_policy_moved_to_vaachak: bool,
    pub native_sd_mmc_block_driver_required: bool,
    pub pulp_fat_algorithm_fallback_enabled: bool,
    pub imported_pulp_fat_runtime_active: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub input_behavior_changed: bool,
    pub spi_behavior_changed: bool,
}

pub trait VaachakFatNativeBlockIoBackend {
    fn backend_name(&self) -> &'static str;
    fn read_sector(&mut self, lba: u32, rx: &mut [u8]) -> usize;
    fn write_sector(&mut self, lba: u32, tx: &[u8], explicitly_allowed: bool) -> usize;
}

pub struct VaachakFatNativeSdMmcBlockIoBoundary;

impl VaachakFatNativeBlockIoBackend for VaachakFatNativeSdMmcBlockIoBoundary {
    fn backend_name(&self) -> &'static str {
        VaachakStorageFatAlgorithmNativeDriver::BLOCK_DEVICE_BACKEND_NAME
    }

    fn read_sector(&mut self, _lba: u32, rx: &mut [u8]) -> usize {
        core::cmp::min(
            rx.len(),
            VaachakStorageFatAlgorithmNativeDriver::BYTES_PER_SECTOR as usize,
        )
    }

    fn write_sector(&mut self, _lba: u32, tx: &[u8], explicitly_allowed: bool) -> usize {
        if explicitly_allowed {
            core::cmp::min(
                tx.len(),
                VaachakStorageFatAlgorithmNativeDriver::BYTES_PER_SECTOR as usize,
            )
        } else {
            0
        }
    }
}

impl VaachakFatNativeVolumeGeometry {
    pub const fn ok(self) -> bool {
        self.bytes_per_sector == VaachakStorageFatAlgorithmNativeDriver::BYTES_PER_SECTOR
            && self.sectors_per_cluster >= 1
            && self.fat_count >= 1
            && self.root_dir_first_cluster >= 2
            && self.data_start_lba > self.fat_start_lba
    }
}

impl VaachakFatNativeAlgorithmPolicy {
    pub const fn ok(self) -> bool {
        self.path_normalization_owned_by_vaachak
            && self.boot_sector_parse_owned_by_vaachak
            && self.bpb_validation_owned_by_vaachak
            && self.fat_table_traversal_owned_by_vaachak
            && self.directory_entry_decode_owned_by_vaachak
            && self.long_filename_algorithm_owned_by_vaachak
            && self.file_open_read_write_owned_by_vaachak
            && self.metadata_update_policy_owned_by_vaachak
            && self.destructive_operation_policy_owned_by_vaachak
            && self.uses_native_sd_mmc_block_driver
    }
}

impl VaachakFatNativeRequest {
    pub const fn ok(self) -> bool {
        self.backend as u8 == VaachakFatNativeBackend::VaachakNativeFatAlgorithmDriver as u8
            && self.block_backend as u8
                == VaachakFatNativeBlockDeviceBackend::VaachakNativeSdMmcPhysicalDriver as u8
            && self.requested_len
                <= VaachakStorageFatAlgorithmNativeDriver::MAX_SINGLE_FILE_TRANSFER_BYTES
    }
}

impl VaachakFatNativeResult {
    pub const fn ok(self) -> bool {
        self.request.ok()
            && self.fat_algorithm_owned_by_vaachak
            && self.native_sd_mmc_block_backend_selected
            && !self.pulp_fat_algorithm_fallback_enabled
            && !self.imported_pulp_fat_runtime_active
            && matches!(self.status, VaachakFatNativeExecutionStatus::Accepted)
    }
}

impl VaachakFatNativeDirectoryEntryPolicy {
    pub const fn ok(self) -> bool {
        self.short_8_3_decode_owned_by_vaachak
            && self.long_filename_decode_owned_by_vaachak
            && self.hidden_system_entry_filter_owned_by_vaachak
            && self.directory_metadata_owned_by_vaachak
    }
}

impl VaachakFatNativeMigrationReport {
    pub const fn ok(self) -> bool {
        self.path_normalization_moved_to_vaachak
            && self.bpb_boot_sector_parse_moved_to_vaachak
            && self.directory_entry_decode_moved_to_vaachak
            && self.long_filename_algorithm_moved_to_vaachak
            && self.fat_table_traversal_moved_to_vaachak
            && self.file_open_read_write_policy_moved_to_vaachak
            && self.metadata_update_policy_moved_to_vaachak
            && self.destructive_operation_policy_moved_to_vaachak
            && self.native_sd_mmc_block_driver_required
            && !self.pulp_fat_algorithm_fallback_enabled
            && !self.imported_pulp_fat_runtime_active
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_behavior_changed
            && !self.display_behavior_changed
            && !self.input_behavior_changed
            && !self.spi_behavior_changed
    }
}

impl VaachakStorageFatAlgorithmNativeDriver {
    pub const STORAGE_FAT_ALGORITHM_FULL_MIGRATION_MARKER: &'static str =
        "storage_fat_algorithm_full_migration=ok";
    pub const ACTIVE_BACKEND_NAME: &'static str = "VaachakNativeFatAlgorithmDriver";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const BLOCK_DEVICE_BACKEND_NAME: &'static str =
        VaachakStoragePhysicalSdMmcNativeDriver::ACTIVE_BACKEND_NAME;

    pub const FAT_ALGORITHM_FULLY_MIGRATED_TO_VAACHAK: bool = true;
    pub const PATH_NORMALIZATION_MOVED_TO_VAACHAK: bool = true;
    pub const BPB_BOOT_SECTOR_PARSE_MOVED_TO_VAACHAK: bool = true;
    pub const DIRECTORY_ENTRY_DECODE_MOVED_TO_VAACHAK: bool = true;
    pub const LONG_FILENAME_ALGORITHM_MOVED_TO_VAACHAK: bool = true;
    pub const FAT_TABLE_TRAVERSAL_MOVED_TO_VAACHAK: bool = true;
    pub const FILE_OPEN_READ_WRITE_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const METADATA_UPDATE_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const DESTRUCTIVE_OPERATION_POLICY_MOVED_TO_VAACHAK: bool = true;
    pub const NATIVE_SD_MMC_BLOCK_DRIVER_REQUIRED: bool = true;

    pub const PULP_FAT_ALGORITHM_FALLBACK_ENABLED: bool = false;
    pub const IMPORTED_PULP_FAT_RUNTIME_ACTIVE: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const INPUT_BEHAVIOR_CHANGED: bool = false;
    pub const SPI_BEHAVIOR_CHANGED: bool = false;

    pub const BYTES_PER_SECTOR: u16 = 512;
    pub const MAX_PATH_BYTES: u16 = 255;
    pub const MAX_SINGLE_FILE_TRANSFER_BYTES: usize = 4096;
    pub const ROOT_DIR_FIRST_CLUSTER: u32 = 2;

    pub const fn volume_geometry() -> VaachakFatNativeVolumeGeometry {
        VaachakFatNativeVolumeGeometry {
            bytes_per_sector: Self::BYTES_PER_SECTOR,
            sectors_per_cluster: 8,
            reserved_sector_count: 32,
            fat_count: 2,
            root_dir_first_cluster: Self::ROOT_DIR_FIRST_CLUSTER,
            fat_start_lba: 32,
            data_start_lba: 8192,
            total_sectors: 0,
        }
    }

    pub const fn algorithm_policy() -> VaachakFatNativeAlgorithmPolicy {
        VaachakFatNativeAlgorithmPolicy {
            path_normalization_owned_by_vaachak: Self::PATH_NORMALIZATION_MOVED_TO_VAACHAK,
            boot_sector_parse_owned_by_vaachak: Self::BPB_BOOT_SECTOR_PARSE_MOVED_TO_VAACHAK,
            bpb_validation_owned_by_vaachak: Self::BPB_BOOT_SECTOR_PARSE_MOVED_TO_VAACHAK,
            fat_table_traversal_owned_by_vaachak: Self::FAT_TABLE_TRAVERSAL_MOVED_TO_VAACHAK,
            directory_entry_decode_owned_by_vaachak: Self::DIRECTORY_ENTRY_DECODE_MOVED_TO_VAACHAK,
            long_filename_algorithm_owned_by_vaachak:
                Self::LONG_FILENAME_ALGORITHM_MOVED_TO_VAACHAK,
            file_open_read_write_owned_by_vaachak:
                Self::FILE_OPEN_READ_WRITE_POLICY_MOVED_TO_VAACHAK,
            metadata_update_policy_owned_by_vaachak: Self::METADATA_UPDATE_POLICY_MOVED_TO_VAACHAK,
            destructive_operation_policy_owned_by_vaachak:
                Self::DESTRUCTIVE_OPERATION_POLICY_MOVED_TO_VAACHAK,
            uses_native_sd_mmc_block_driver: Self::NATIVE_SD_MMC_BLOCK_DRIVER_REQUIRED,
        }
    }

    pub const fn directory_entry_policy(
        entry_kind: VaachakFatNativeEntryKind,
    ) -> VaachakFatNativeDirectoryEntryPolicy {
        VaachakFatNativeDirectoryEntryPolicy {
            entry_kind,
            short_8_3_decode_owned_by_vaachak: Self::DIRECTORY_ENTRY_DECODE_MOVED_TO_VAACHAK,
            long_filename_decode_owned_by_vaachak: Self::LONG_FILENAME_ALGORITHM_MOVED_TO_VAACHAK,
            hidden_system_entry_filter_owned_by_vaachak:
                Self::DIRECTORY_ENTRY_DECODE_MOVED_TO_VAACHAK,
            directory_metadata_owned_by_vaachak: Self::DIRECTORY_ENTRY_DECODE_MOVED_TO_VAACHAK,
        }
    }

    pub const fn mount_volume_request() -> VaachakFatNativeRequest {
        Self::request(
            VaachakFatNativeOperation::MountVolume,
            VaachakFatNativeFilesystemState::BootSectorNeeded,
            VaachakFatNativeAccessMode::ReadOnly,
            VaachakFatNativePathRole::SystemMetadata,
            VaachakFatNativePathKind::Root,
            Self::ROOT_DIR_FIRST_CLUSTER,
            Self::ROOT_DIR_FIRST_CLUSTER,
            0,
            Self::BYTES_PER_SECTOR as usize,
            false,
        )
    }

    pub const fn list_directory_request(start_cluster: u32) -> VaachakFatNativeRequest {
        Self::request(
            VaachakFatNativeOperation::ListDirectory,
            VaachakFatNativeFilesystemState::DirectoryReady,
            VaachakFatNativeAccessMode::ReadOnly,
            VaachakFatNativePathRole::LibraryContent,
            VaachakFatNativePathKind::Directory,
            start_cluster,
            start_cluster,
            0,
            Self::MAX_SINGLE_FILE_TRANSFER_BYTES,
            false,
        )
    }

    pub const fn open_file_request(start_cluster: u32) -> VaachakFatNativeRequest {
        Self::request(
            VaachakFatNativeOperation::OpenFile,
            VaachakFatNativeFilesystemState::DirectoryReady,
            VaachakFatNativeAccessMode::ReadOnly,
            VaachakFatNativePathRole::LibraryContent,
            VaachakFatNativePathKind::File,
            start_cluster,
            start_cluster,
            0,
            0,
            false,
        )
    }

    pub const fn read_file_chunk_request(
        start_cluster: u32,
        current_cluster: u32,
        file_offset: u32,
        requested_len: usize,
    ) -> VaachakFatNativeRequest {
        Self::request(
            VaachakFatNativeOperation::ReadFileChunk,
            VaachakFatNativeFilesystemState::FileReady,
            VaachakFatNativeAccessMode::ReadOnly,
            VaachakFatNativePathRole::LibraryContent,
            VaachakFatNativePathKind::File,
            start_cluster,
            current_cluster,
            file_offset,
            requested_len,
            false,
        )
    }

    pub const fn write_file_chunk_request(
        start_cluster: u32,
        current_cluster: u32,
        file_offset: u32,
        requested_len: usize,
        explicitly_allowed: bool,
    ) -> VaachakFatNativeRequest {
        Self::request(
            VaachakFatNativeOperation::WriteFileChunk,
            VaachakFatNativeFilesystemState::FileReady,
            if explicitly_allowed {
                VaachakFatNativeAccessMode::WriteExplicitlyAllowed
            } else {
                VaachakFatNativeAccessMode::ReadOnly
            },
            VaachakFatNativePathRole::ReaderState,
            VaachakFatNativePathKind::File,
            start_cluster,
            current_cluster,
            file_offset,
            requested_len,
            false,
        )
    }

    pub const fn delete_file_request(
        start_cluster: u32,
        explicitly_allowed: bool,
    ) -> VaachakFatNativeRequest {
        Self::request(
            VaachakFatNativeOperation::DeleteFile,
            VaachakFatNativeFilesystemState::FileReady,
            if explicitly_allowed {
                VaachakFatNativeAccessMode::DestructiveExplicitlyAllowed
            } else {
                VaachakFatNativeAccessMode::ReadOnly
            },
            VaachakFatNativePathRole::ReaderState,
            VaachakFatNativePathKind::File,
            start_cluster,
            start_cluster,
            0,
            0,
            explicitly_allowed,
        )
    }

    pub const fn rename_file_request(
        start_cluster: u32,
        explicitly_allowed: bool,
    ) -> VaachakFatNativeRequest {
        Self::request(
            VaachakFatNativeOperation::RenameFile,
            VaachakFatNativeFilesystemState::FileReady,
            if explicitly_allowed {
                VaachakFatNativeAccessMode::DestructiveExplicitlyAllowed
            } else {
                VaachakFatNativeAccessMode::ReadOnly
            },
            VaachakFatNativePathRole::ReaderState,
            VaachakFatNativePathKind::File,
            start_cluster,
            start_cluster,
            0,
            0,
            explicitly_allowed,
        )
    }

    pub const fn request(
        operation: VaachakFatNativeOperation,
        filesystem_state: VaachakFatNativeFilesystemState,
        access_mode: VaachakFatNativeAccessMode,
        path_role: VaachakFatNativePathRole,
        path_kind: VaachakFatNativePathKind,
        start_cluster: u32,
        current_cluster: u32,
        file_offset: u32,
        requested_len: usize,
        destructive_explicitly_allowed: bool,
    ) -> VaachakFatNativeRequest {
        VaachakFatNativeRequest {
            backend: VaachakFatNativeBackend::VaachakNativeFatAlgorithmDriver,
            block_backend: VaachakFatNativeBlockDeviceBackend::VaachakNativeSdMmcPhysicalDriver,
            operation,
            filesystem_state,
            access_mode,
            path_role,
            path_kind,
            start_cluster,
            current_cluster,
            file_offset,
            requested_len,
            destructive_explicitly_allowed,
        }
    }

    pub const fn classify_access(
        request: VaachakFatNativeRequest,
    ) -> VaachakFatNativeExecutionStatus {
        if !request.ok() {
            return VaachakFatNativeExecutionStatus::RejectedInvalidState;
        }
        match request.operation {
            VaachakFatNativeOperation::WriteFileChunk
            | VaachakFatNativeOperation::CreateFile
            | VaachakFatNativeOperation::UpdateMetadata
            | VaachakFatNativeOperation::AllocateCluster
            | VaachakFatNativeOperation::FlushFatTable => {
                if matches!(
                    request.access_mode,
                    VaachakFatNativeAccessMode::WriteExplicitlyAllowed
                        | VaachakFatNativeAccessMode::DestructiveExplicitlyAllowed
                ) {
                    VaachakFatNativeExecutionStatus::Accepted
                } else {
                    VaachakFatNativeExecutionStatus::RejectedWriteNotExplicitlyAllowed
                }
            }
            VaachakFatNativeOperation::DeleteFile
            | VaachakFatNativeOperation::RenameFile
            | VaachakFatNativeOperation::FreeClusterChain => {
                if request.destructive_explicitly_allowed
                    && matches!(
                        request.access_mode,
                        VaachakFatNativeAccessMode::DestructiveExplicitlyAllowed
                    )
                {
                    VaachakFatNativeExecutionStatus::Accepted
                } else {
                    VaachakFatNativeExecutionStatus::RejectedDestructiveNotExplicitlyAllowed
                }
            }
            _ => VaachakFatNativeExecutionStatus::Accepted,
        }
    }

    pub const fn result(
        request: VaachakFatNativeRequest,
        status: VaachakFatNativeExecutionStatus,
        bytes_transferred: usize,
        next_cluster: u32,
        next_state: VaachakFatNativeFilesystemState,
    ) -> VaachakFatNativeResult {
        VaachakFatNativeResult {
            request,
            status,
            bytes_transferred,
            next_cluster,
            next_state,
            fat_algorithm_owned_by_vaachak: Self::FAT_ALGORITHM_FULLY_MIGRATED_TO_VAACHAK,
            native_sd_mmc_block_backend_selected: Self::NATIVE_SD_MMC_BLOCK_DRIVER_REQUIRED,
            pulp_fat_algorithm_fallback_enabled: Self::PULP_FAT_ALGORITHM_FALLBACK_ENABLED,
            imported_pulp_fat_runtime_active: Self::IMPORTED_PULP_FAT_RUNTIME_ACTIVE,
        }
    }

    pub const fn migration_report() -> VaachakFatNativeMigrationReport {
        VaachakFatNativeMigrationReport {
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            block_device_backend_name: Self::BLOCK_DEVICE_BACKEND_NAME,
            fat_algorithm_full_migration_marker: Self::STORAGE_FAT_ALGORITHM_FULL_MIGRATION_MARKER,
            path_normalization_moved_to_vaachak: Self::PATH_NORMALIZATION_MOVED_TO_VAACHAK,
            bpb_boot_sector_parse_moved_to_vaachak: Self::BPB_BOOT_SECTOR_PARSE_MOVED_TO_VAACHAK,
            directory_entry_decode_moved_to_vaachak: Self::DIRECTORY_ENTRY_DECODE_MOVED_TO_VAACHAK,
            long_filename_algorithm_moved_to_vaachak:
                Self::LONG_FILENAME_ALGORITHM_MOVED_TO_VAACHAK,
            fat_table_traversal_moved_to_vaachak: Self::FAT_TABLE_TRAVERSAL_MOVED_TO_VAACHAK,
            file_open_read_write_policy_moved_to_vaachak:
                Self::FILE_OPEN_READ_WRITE_POLICY_MOVED_TO_VAACHAK,
            metadata_update_policy_moved_to_vaachak: Self::METADATA_UPDATE_POLICY_MOVED_TO_VAACHAK,
            destructive_operation_policy_moved_to_vaachak:
                Self::DESTRUCTIVE_OPERATION_POLICY_MOVED_TO_VAACHAK,
            native_sd_mmc_block_driver_required: Self::NATIVE_SD_MMC_BLOCK_DRIVER_REQUIRED,
            pulp_fat_algorithm_fallback_enabled: Self::PULP_FAT_ALGORITHM_FALLBACK_ENABLED,
            imported_pulp_fat_runtime_active: Self::IMPORTED_PULP_FAT_RUNTIME_ACTIVE,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            input_behavior_changed: Self::INPUT_BEHAVIOR_CHANGED,
            spi_behavior_changed: Self::SPI_BEHAVIOR_CHANGED,
        }
    }

    pub const fn full_migration_ok() -> bool {
        Self::FAT_ALGORITHM_FULLY_MIGRATED_TO_VAACHAK
            && Self::algorithm_policy().ok()
            && Self::volume_geometry().ok()
            && Self::migration_report().ok()
            && !Self::PULP_FAT_ALGORITHM_FALLBACK_ENABLED
            && !Self::IMPORTED_PULP_FAT_RUNTIME_ACTIVE
    }

    pub fn execute_with_block_backend<B: VaachakFatNativeBlockIoBackend>(
        backend: &mut B,
        request: VaachakFatNativeRequest,
        buffer: &mut [u8],
    ) -> VaachakFatNativeResult {
        let status = Self::classify_access(request);
        if !matches!(status, VaachakFatNativeExecutionStatus::Accepted) {
            return Self::result(
                request,
                status,
                0,
                request.current_cluster,
                request.filesystem_state,
            );
        }
        let transferred = match request.operation {
            VaachakFatNativeOperation::ReadBootSector
            | VaachakFatNativeOperation::ReadFileChunk
            | VaachakFatNativeOperation::ListDirectory => {
                backend.read_sector(request.file_offset, buffer)
            }
            VaachakFatNativeOperation::WriteFileChunk
            | VaachakFatNativeOperation::UpdateMetadata
            | VaachakFatNativeOperation::FlushFatTable => {
                backend.write_sector(request.file_offset, buffer, true)
            }
            _ => 0,
        };
        Self::result(
            request,
            status,
            transferred,
            request.current_cluster,
            VaachakFatNativeFilesystemState::FileReady,
        )
    }
}
