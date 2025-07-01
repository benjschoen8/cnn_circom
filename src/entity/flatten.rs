use crate::entity::layer::Layer;

pub struct Flatten {
    pub name: String,
}

impl Layer for Flatten {
    fn get_name(&self) -> &str {
        &self.name
    }
}

