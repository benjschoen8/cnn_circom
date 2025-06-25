pub struct Model {
    model_string: String,
}

impl Model {
    pub fn new() -> Self {
        Self {
            model_string: String::new(),
        }
    }

    pub fn set_string(&mut self, value: String) {
        self.model_string = value;
    }

    pub fn get_string(&self) -> &str {
        &self.model_string
    }
}

