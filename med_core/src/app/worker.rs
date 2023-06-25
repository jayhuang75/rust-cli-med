use crate::utils::error::MedError;
use std::sync::Once;
use threadpool::ThreadPool;

#[derive(Debug)]
pub struct Worker {
    pub cpu_num: u16,
    pub pool: ThreadPool,
}

impl Worker {
    /// Returns a Worker instant
    ///
    /// # Arguments
    ///
    /// * `cpu_num` [u16] - cpu number
    ///
    /// # Examples
    ///
    /// ```
    /// use med_core::utils::error::MedError;
    /// use med_core::app::worker::Worker;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), MedError> {
    ///     let worker = Worker::new(4).await?;
    ///     Ok(())
    /// }
    ///
    /// ```
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
