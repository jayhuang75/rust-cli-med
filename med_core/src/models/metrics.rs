use crate::utils::error::MaskerError;

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub total_files: usize,
    pub total_records: usize,
    pub failed_records: usize, 
    pub record_failed_reason: Vec<MaskerError>,
}