use std::time::Duration;
use crate::core::models::Params;


#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub total_file: usize,
    pub total_records: usize,
}

#[derive(Debug, Default, Clone)]
pub struct AuditSummary {
    pub params: Params,
    pub metrics: Metrics,
    pub elapsed: Duration,
}
