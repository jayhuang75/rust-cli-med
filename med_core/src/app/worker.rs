use crate::utils::error::MedError;
use std::sync::Once;
use threadpool::ThreadPool;

#[derive(Debug)]
pub struct Worker {
    pub cpu_num: u16,
    pub pool: ThreadPool,
}

impl Worker {
    pub async fn new(cpu_num: u16) -> Result<Self, MedError> {
        let pool = ThreadPool::new(cpu_num as usize);
        static START: Once = Once::new();

        START.call_once(|| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(cpu_num as usize)
                .build_global()
                .unwrap();
        });

        Ok(Worker { cpu_num, pool })
    }
}
