use rayon::ThreadPool;

use crate::utils::error::MaskerError;

pub struct Worker {
    pub pool: ThreadPool,
}

impl Worker {
    pub async fn new(cpu_num: usize) -> Result<Self, MaskerError> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_num)
            .build()
            .unwrap();
        Ok(Worker { pool })
    }
}
