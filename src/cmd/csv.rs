use csv::StringRecord;

use crate::utils::error::MaskerError;

#[derive(Debug)]
pub struct ProcessFile {
    pub headers: StringRecord,
    pub data: Vec<StringRecord>,
}

