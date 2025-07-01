use crate::layer::Layer;

pub struct Conv2D {
    pub weight: Vec<Vec<Vec<Vec<f32>>>>, // shape: [out_channels][in_channels][kernel_height][kernel_width]
    pub bias: Vec<f32>,                  // shape: [out_channels]
    pub name: str,
}

impl Conv2D {
    pub fn new() -> Self {
        Self {
            input_name: String::new(),
            input_size: 0,
            weight_size: 0,
            weight: Vec::new(),
            bias: Vec::new(),
            name: String::new(),
        }
    }
}

impl Layer for Conv2D {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_operation_tuples(&self) -> Vec<(String, String, String, String)> {
        if !self.check_valid() {
            panic!(
                "Invalid Conv2D: input channels do not match weight channels in layer {}",
                self.name
            );
        }

        let mut operations = Vec::new();
        let mut op_index = 0;

        for (out_idx, out_channel) in self.weight.iter().enumerate() {
            for (in_idx, in_channel) in out_channel.iter().enumerate() {
                for (kh_idx, kernel_row) in in_channel.iter().enumerate() {
                    for (kw_idx, _) in kernel_row.iter().enumerate() {
                        let input_name = format!("{}_{}", self.input_name, op_index);
                        let weight_name = format!("{}_weight_{}", self.name, op_index);
                        let output_name = format!("{}_output_{}", self.name, op_index);

                        operations.push((
                            "*".to_string(),
                            input_name,
                            weight_name,
                            output_name.clone(),
                        ));

                        operations.push((
                            "+".to_string(),
                            output_name.clone(),
                            format!("{}_bias_{}", self.name, out_idx),
                            output_name,
                        ));

                        op_index += 1;
                    }
                }
            }
        }

        operations
    }
}


