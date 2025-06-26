pub trait PythonTrainingAdapter {
    fn train(&self, dataset: &str);
    fn get_model(&self) -> Model;
}

