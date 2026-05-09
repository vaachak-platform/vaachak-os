#![allow(dead_code)]

use crate::vaachak_x4::physical::storage_fat_algorithm_native_driver::{
    VaachakFatNativeEntryKind, VaachakFatNativeExecutionStatus,
    VaachakStorageFatAlgorithmNativeDriver,
};

pub struct VaachakStorageFatAlgorithmNativeDriverSmoke;

impl VaachakStorageFatAlgorithmNativeDriverSmoke {
    pub const MARKER: &'static str = "storage_fat_algorithm_full_migration=ok";

    pub fn run() -> bool {
        let mount = VaachakStorageFatAlgorithmNativeDriver::mount_volume_request();
        let list = VaachakStorageFatAlgorithmNativeDriver::list_directory_request(2);
        let open = VaachakStorageFatAlgorithmNativeDriver::open_file_request(2);
        let read = VaachakStorageFatAlgorithmNativeDriver::read_file_chunk_request(2, 2, 0, 512);
        let write_rejected =
            VaachakStorageFatAlgorithmNativeDriver::write_file_chunk_request(2, 2, 0, 512, false);
        let write_allowed =
            VaachakStorageFatAlgorithmNativeDriver::write_file_chunk_request(2, 2, 0, 512, true);
        let delete_rejected = VaachakStorageFatAlgorithmNativeDriver::delete_file_request(2, false);
        let delete_allowed = VaachakStorageFatAlgorithmNativeDriver::delete_file_request(2, true);
        let dir_policy = VaachakStorageFatAlgorithmNativeDriver::directory_entry_policy(
            VaachakFatNativeEntryKind::LongFilenameFragment,
        );
        VaachakStorageFatAlgorithmNativeDriver::full_migration_ok()
            && mount.ok()
            && list.ok()
            && open.ok()
            && read.ok()
            && dir_policy.ok()
            && matches!(
                VaachakStorageFatAlgorithmNativeDriver::classify_access(write_rejected),
                VaachakFatNativeExecutionStatus::RejectedWriteNotExplicitlyAllowed
            )
            && matches!(
                VaachakStorageFatAlgorithmNativeDriver::classify_access(write_allowed),
                VaachakFatNativeExecutionStatus::Accepted
            )
            && matches!(
                VaachakStorageFatAlgorithmNativeDriver::classify_access(delete_rejected),
                VaachakFatNativeExecutionStatus::RejectedDestructiveNotExplicitlyAllowed
            )
            && matches!(
                VaachakStorageFatAlgorithmNativeDriver::classify_access(delete_allowed),
                VaachakFatNativeExecutionStatus::Accepted
            )
    }
}

pub fn storage_fat_algorithm_native_driver_smoke_ok() -> bool {
    VaachakStorageFatAlgorithmNativeDriverSmoke::run()
}
