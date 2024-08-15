use crate::scanners::comparers::scalar::scanner_scalar_iterative::ScannerScalarIterative;
use crate::scanners::comparers::scalar::scanner_scalar_single_element::ScannerScalarSingleElement;
use crate::scanners::comparers::snapshot_scanner::Scanner;
use crate::scanners::constraints::scan_constraint::ScanConstraint;
use crate::snapshots::snapshot_sub_region::SnapshotSubRegion;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_architecture::vectors::vectors;
use squalr_engine_common::dynamic_struct::field_value::FieldValue;
use std::sync::{Arc, Once, RwLock};
use tokio::task::JoinHandle;

pub struct ScanDispatcher {
}

impl ScanDispatcher {
    // Stateless
    fn new() -> Self { Self { } }
    
    pub fn get_instance() -> Arc<RwLock<ScanDispatcher>> {
        static mut INSTANCE: Option<Arc<RwLock<ScanDispatcher>>> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = Arc::new(RwLock::new(ScanDispatcher::new()));
                INSTANCE = Some(instance);
            });

            return INSTANCE.as_ref().unwrap().clone();
        }
    }

    pub fn dispatch_scan(&self, snapshot_region: Arc<RwLock<SnapshotRegion>>, constraints: &ScanConstraint) -> Vec<Arc<RwLock<SnapshotSubRegion>>> {
        let has_snapshot_region = snapshot_region.read().unwrap().get_snapshot_sub_regions().is_empty();
        let has_valid_size = snapshot_region.read().unwrap().get_region_size() > 0;

        if has_snapshot_region && has_valid_size {
            let mut sub_regions = Vec::new();
            let sub_region = Arc::new(RwLock::new(SnapshotSubRegion::new(snapshot_region.clone())));
                
            sub_regions.push(sub_region);
            
            snapshot_region.write().unwrap().set_snapshot_sub_regions(sub_regions);
        }

        let snapshot_region = snapshot_region.read().unwrap();
        let snapshot_sub_regions = snapshot_region.get_snapshot_sub_regions();
        let mut results = Vec::new();
    
        for snapshot_sub_region in snapshot_sub_regions {
            let constraints = constraints.clone();
            let snapshot_sub_region = snapshot_sub_region.clone();
            let scanner_instance = self.acquire_scanner_instance(&snapshot_sub_region, &constraints);
    
            let scanner = scanner_instance.read().unwrap();
            let result_sub_regions = scanner.scan_region(&snapshot_sub_region, Arc::new(constraints));
            
            for result_sub_region in result_sub_regions {
                results.push(result_sub_region);
            }
        }
    
        return results;
    }

    pub async fn dispatch_scan_parallel(&self, snapshot_region: Arc<RwLock<SnapshotRegion>>, constraints: &ScanConstraint) -> Vec<Arc<RwLock<SnapshotSubRegion>>> {
        let mut snapshot_region_mut = snapshot_region.write().unwrap();
        let snapshot_sub_regions = snapshot_region_mut.get_snapshot_sub_regions_create_if_none(snapshot_region.clone());
        drop(snapshot_region_mut);

        let mut handles = Vec::new();

        for snapshot_sub_region in snapshot_sub_regions {
            let constraints = constraints.clone();
            let snapshot_sub_region = snapshot_sub_region.clone();
            let scanner_instance = self.acquire_scanner_instance(&snapshot_sub_region, &constraints);

            let handle: JoinHandle<Arc<RwLock<SnapshotSubRegion>>> = tokio::spawn(async move {
                let scanner = scanner_instance.read().unwrap();
                scanner.scan_region(&snapshot_sub_region, Arc::new(constraints));
                return snapshot_sub_region;
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }

        return results;
    }

    fn acquire_scanner_instance(&self, snapshot_sub_region: &Arc<RwLock<SnapshotSubRegion>>, constraints: &ScanConstraint) -> Arc<RwLock<dyn Scanner>> {
        let alignment = constraints.get_byte_alignment();
        let data_type_size = constraints.get_element_type().size_in_bytes();

        if snapshot_sub_region.read().unwrap().get_element_count(data_type_size, alignment) == 1 {
            // Single element scanner
            return ScannerScalarSingleElement::get_instance();
        } else if vectors::has_vector_support() && snapshot_sub_region.read().unwrap().parent_region.read().unwrap().get_region_size() >= vectors::get_hardware_vector_size() as u64 {
            match constraints.get_element_type() {
                FieldValue::Bytes(_) => {
                    // Vector array of bytes scanner
                    // return ScannerVectorArrayOfBytes::get_instance();
                }
                _ => {
                    /*
                    if alignment_size == element_size as i32 {
                        // Fast vector scanner
                        // return ScannerVectorFast::get_instance();
                    } else if alignment_size > element_size as i32 {
                        // Sparse vector scanner
                        // return ScannerVectorSparse::get_instance();
                    } else {
                        // Staggered vector scanner
                        // return ScannerVectorStaggered::get_instance();
                    } */
                }
            }
        } else {
            // Iterative scanner
            return ScannerScalarIterative::get_instance();
        }

        return ScannerScalarIterative::get_instance();
    }
}
