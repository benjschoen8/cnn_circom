use crate::entity::layer::Layer;

pub struct Relu {
    pub name: String,
}

impl Layer for Relu {
    fn get_name(&self) -> &str {
        &self.name
    }
}
