use crate::snapshots::snapshot_region::SnapshotRegion;
use std::time::SystemTime;

pub struct Snapshot {
    name: String,
    creation_time: SystemTime,
    snapshot_regions: Vec<SnapshotRegion>,
}

/// Represents a snapshot of memory in another process. By design, a snapshot is entirely immutable to avoid resource contention.
impl Snapshot {
    pub fn new(name: String, mut snapshot_regions: Vec<SnapshotRegion>) -> Self {
        // Remove empty regions and sort them ascending
        snapshot_regions.retain(|region| region.get_region_size() > 0);
        snapshot_regions.sort_by_key(|region| region.get_base_address());

        Self {
            name,
            creation_time: SystemTime::now(),
            snapshot_regions,
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    pub fn get_creation_time(&self) -> &SystemTime {
        return &self.creation_time;
    }

    pub fn get_snapshot_regions(&self) -> &Vec<SnapshotRegion> {
        return &self.snapshot_regions;
    }

    pub fn get_region_count(&self) -> u64 {
        return self.snapshot_regions.len() as u64;
    }
    
    pub fn get_byte_count(&self) -> u64 {
        return self.snapshot_regions.iter().map(|region| region.get_region_size()).sum();
    }
}
