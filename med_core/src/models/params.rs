use std::fmt;

use serde::Serialize;

use crate::utils::enums::{AppMode, FileType, Mode, Standard};

#[derive(Debug, Clone, Serialize)]
pub struct Params {
    pub app_mode: AppMode,
    pub file_path: String,
    pub file_type: FileType,
    pub conf_path: String,
    pub output_path: String,
    pub mode: Mode,
    pub worker: u16,
    pub key: Option<String>,
    pub standard: Standard,
    pub debug: bool,
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "app_mode: {}, file_path: {}, file_type: {}, conf_path: {}, output_path: {}, mode: {}, key: {:?}, debug: {}, worker: {}",
            self.app_mode, self.file_path, self.file_type, self.conf_path, self.output_path, self.mode, self.key, self.debug, self.worker
        )
    }
}

impl Default for Params {
    fn default() -> Self {
        let app_mode: AppMode = AppMode::default();
        let file_path: String = String::default();
        let file_type: FileType = FileType::default();
        let conf_path: String = String::default();
        let output_path: String = String::default();
        let mode: Mode = Mode::default();
        let key: String = String::default();
        let debug: bool = false;
        let worker = 2;
        let standard = Standard::default();

        Params {
            app_mode,
            file_path,
            file_type,
            conf_path,
            output_path,
            mode,
            key: Some(key),
            standard,
            debug,
            worker,
        }
    }
}
