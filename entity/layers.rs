use crate::entity::layer::Layer;

pub struct Layers {
    pub layers: Vec<Box<dyn Layer>>,
}

impl Layers {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }
}

