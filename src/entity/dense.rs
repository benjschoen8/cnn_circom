use crate::entity::layer::Layer;

pub struct Dense {
    pub weight: Vec<Vec<f32>>, // [output_size][input_size]
    pub bias: Vec<f32>,        // [output_size]
    pub name: String,
}

impl Layer for Dense {
    fn get_name(&self) -> &str {
        &self.name
    }
}

