use serde_json::{json, Value};

pub struct Input {
    input_string: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input_string: String::new(),
        }
    }

    pub fn set_string(&mut self, value: String) {
        self.input_string = value;
    }

    pub fn get_string(&self) -> &str {
        &self.input_string
    }

    pub fn get_json(&self) -> Value {
        json!({
            "input": self.input_string
        })
    }
}

