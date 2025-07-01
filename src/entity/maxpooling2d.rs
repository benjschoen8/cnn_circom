use crate::entity::layer::Layer;

pub struct MaxPooling2D {
    pub pool_size: (usize, usize), // e.g. (2, 2)
    pub name: String,
}

impl Layer for MaxPooling2D {
    fn get_name(&self) -> &str {
        &self.name
    }
}
