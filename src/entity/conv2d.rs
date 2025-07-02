use crate::layer::Layer;

pub struct Conv2D {
    pub input_name: String,
    pub input_size: (usize, usize, usize),         // (channels, height, width)
    pub weight_size: (usize, usize, usize, usize), // (out_channels, in_channels, kernel_height, kernel_width)
    pub weight: Vec<Vec<Vec<Vec<f32>>>>,           // [out_channels][in_channels][kernel_height][kernel_width]
    pub bias: Vec<f32>,                            // [out_channels]
    pub padding: usize,
    pub stride: usize,
    pub name: String,
    pub to_integer_bias: usize,
}

impl Conv2D {
    pub fn new() -> Self {
        Self {
            input_name: String::new(),
            input_size: (0, 0, 0),
            weight_size: (0, 0, 0, 0),
            weight: Vec::new(),
            bias: Vec::new(),
            padding: 0,
            stride: 1,
            name: String::new(),
            to_integer_bias: 0,
        }
    }

    pub fn get_input_name(&self) -> &str {
        &self.input_name
    }

    pub fn get_input_size(&self) -> (usize, usize, usize) {
        self.input_size
    }

    pub fn get_weight_size(&self) -> (usize, usize, usize, usize) {
        self.weight_size
    }
}

impl Layer for Conv2D {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_operation_tuples(&self) -> Vec<(String, String, String, String)> {
        let mut operations = Vec::new();

        let (in_channels, in_height, in_width) = self.input_size;
        let (out_channels, weight_in_channels, kernel_height, kernel_width) = self.weight_size;

        if in_channels != weight_in_channels {
            panic!("Input channels and weight channels do not match in layer {}", self.name);
        }

        let out_height = (in_height + 2 * self.padding - kernel_height) / self.stride + 1;
        let out_width = (in_width + 2 * self.padding - kernel_width) / self.stride + 1;

        for out_c in 0..out_channels {
            for out_y in 0..out_height {
                for out_x in 0..out_width {
                    let mut sum_name = String::new();
                    let mut is_first = true;

                    for in_c in 0..in_channels {
                        for k_y in 0..kernel_height {
                            for k_x in 0..kernel_width {
                                let in_y = out_y * self.stride + k_y as isize - self.padding as isize;
                                let in_x = out_x * self.stride + k_x as isize - self.padding as isize;

                                // Skip if out of bounds (zero padding)
                                if in_y < 0 || in_y >= in_height as isize || in_x < 0 || in_x >= in_width as isize {
                                    continue;
                                }

                                let input_name = format!("{}_{}_{}_{}", self.input_name, in_c, in_y, in_x);
                                let weight_name = format!("{}_weight_{}_{}_{}_{}", self.name, out_c, in_c, k_y, k_x);
                                let prod_name = format!("{}_prod_tmp_{}_{}_{}_{}_{}", self.name, out_c, out_y, out_x, in_c, k_y * kernel_width + k_x);

                                operations.push((
                                    "*".to_string(),
                                    input_name,
                                    weight_name,
                                    prod_name.clone(),
                                ));

                                if is_first {
                                    // First addition initializes the sum
                                    sum_name = format!("{}_sum_tmp_{}_{}_{}", self.name, out_c, out_y, out_x);
                                    operations.push((
                                        "+".to_string(),
                                        prod_name.clone(),
                                        "0".to_string(),
                                        sum_name.clone(),
                                    ));
                                    is_first = false;
                                } else {
                                    let new_sum_name = format!("{}_sum_tmp_{}_{}_{}", self.name, out_c, out_y, out_x);
                                    operations.push((
                                        "+".to_string(),
                                        prod_name.clone(),
                                        sum_name.clone(),
                                        new_sum_name.clone(),
                                    ));
                                    sum_name = new_sum_name;
                                }
                            }
                        }
                    }

                    // Add bias at the end
                    let output_name = format!("{}_output_{}_{}_{}", self.name, out_c, out_y, out_x);
                    operations.push((
                        "+".to_string(),
                        sum_name.clone(),
                        format!("{}_bias_{}", self.name, out_c),
                        output_name,
                    ));
                }
            }
        }

        operations
    }
}
