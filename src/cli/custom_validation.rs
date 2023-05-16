use std::{ops::RangeInclusive, path::PathBuf};

pub fn worker_in_range(s: &str) -> Result<u16, String> {
    let worker: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a worker number", s))?;

    let worker_range: RangeInclusive<usize> = 2..=num_cpus::get();

    if worker_range.contains(&worker) {
        Ok(worker as u16)
    } else {
        Err(format!(
            "worker is over your current max cores, consider lower the workerworker not in range {}-{} (max)",
            worker_range.start(),
            worker_range.end()
        ))
    }
}

pub fn dir_exist(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    match path.is_dir() {
        true => Ok(path),
        false => return Err(format!("{} not exist", s)),
    }
}
