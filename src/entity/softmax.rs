use crate::entity::layer::Layer;

pub struct Softmax {
    pub axis: usize,
    pub name: String,
}

impl Layer for Softmax {
    fn get_name(&self) -> &str {
        &self.name
    }
}
