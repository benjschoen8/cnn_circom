use crate::entity::layer::Layer;

pub struct Input {
    pub shape: Vec<usize>, // e.g. [1, 28, 28] for MNIST
    pub name: String,
}

impl Layer for Input {
    fn get_name(&self) -> &str {
        &self.name
    }
}
