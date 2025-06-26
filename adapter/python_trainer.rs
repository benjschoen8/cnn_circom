use crate::entity::model::Model;

pub struct PythonTrainer {
    pub script_path: String,
    pub trained_model: Option<String>, // Model could be a file path, JSON string, etc.
}

impl PythonTrainer {
    pub fn new(script_path: String) -> Self {
        Self {
            script_path,
            trained_model: None,
        }
    }
}
