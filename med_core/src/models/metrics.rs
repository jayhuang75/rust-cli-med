use crate::utils::error::MedError;

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub total_files: usize,
    pub metadata: Metadata,
}

#[derive(Debug, Default, Clone)]
pub struct Metadata {
    pub total_records: usize,
    pub failed_records: usize,
    pub record_failed_reason: Vec<MedError>,
}
