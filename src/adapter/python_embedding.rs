pub trait PythonEmbedding {
    fn train(&self, dataset: &str) -> Result<String, String>;
}

