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
        false => Err(format!("{} not exist", s)),
    }
}

//************************************************************************************************
// Unit Test
//////////////////////////////// */
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{dir_exist, worker_in_range};

    #[test]
    fn test_worker_in_range() {
        match worker_in_range(&num_cpus::get().to_string()) {
            Ok(num) => {
                assert_eq!(num, num_cpus::get() as u16);
            }
            Err(e) => {
                assert_eq!(e,format!("worker is over your current max cores, consider lower the workerworker not in range 2-{:?} (max)", num_cpus::get()));
            }
        }
    }

    #[test]
    fn test_dir_exist() {
        match dir_exist("./") {
            Ok(path) => {
                assert_eq!(path, PathBuf::from("./"));
            }
            Err(e) => {
                assert_eq!(e, "");
            }
        }
        match dir_exist("./test") {
            Ok(path) => {
                assert_eq!(path, PathBuf::from("./"));
            }
            Err(e) => {
                assert_eq!(e, "./test not exist");
            }
        }
    }
}
