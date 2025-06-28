use crate::adapter::python_embedding::PythonEmbedding;
use crate::usecase::trainer::Trainer;

pub struct PythonTrainer {
    pub trainer: Box<dyn PythonEmbedding>,
}

impl PythonTrainer {
    pub fn new(trainer: Box<dyn PythonEmbedding>) -> Self {
        Self { trainer }
    }
}

impl Trainer for PythonTrainer {
    fn train(&self, data_path: &str) -> Result<String, String> {
        self.trainer.train(data_path)
    }
}
