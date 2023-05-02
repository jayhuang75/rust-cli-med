pub enum FileType {
    CSV,
    JSON
}

impl Default for FileType {
    fn default() -> Self { FileType::CSV }
}

pub enum Action {
    MASK,
    ENCRYPT,
    DECRYPT,
}

impl Default for Action {
    fn default() -> Self { Action::MASK }
}

pub struct App{
    pub file_path: String,
    pub file_type: FileType,
    pub conf_path: String,
    pub action: Action
}

impl App {
    
}