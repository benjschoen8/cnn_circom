pub trait Trainer {
    fn train(&self, data: &str) -> Result<String, String>;
}

