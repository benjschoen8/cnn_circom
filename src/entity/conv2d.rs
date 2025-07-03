use crate::layer::Layer;
use std::collections::VecDeque;

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
    pub input_signal_list: Vec<String>,
    pub signal_list: Vec<String>,
    pub operation_tuples: Vec<(String, String, String, String)>,
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
            input_signal_list: Vec::new(),
            signal_list: Vec::new(),
            operation_tuples: Vec::new(),
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
    fn calculate_signals(&mut self) {
        let (in_channels, in_height, in_width) = self.input_size;
        let (out_channels, _weight_in_channels, kernel_height, kernel_width) = self.weight_size;

        let out_height = (in_height + 2 * self.padding - kernel_height) / self.stride + 1;
        let out_width = (in_width + 2 * self.padding - kernel_width) / self.stride + 1;

        for out_c in 0..out_channels {
            for out_y in 0..out_height {
                for out_x in 0..out_width {
                    let mut sum_list: VecDeque<String> = VecDeque::new();

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

                                self.input_signal_list.push(weight.clone());
                                self.signal_list.push(prod_name.clone());

                                // Add multiplication operation
                                self.operation_tuples.push((
                                    "*".to_string(),
                                    input_name,
                                    weight_name,
                                    prod_name.clone(),
                                ));

                                // Add to sum queue (push to front)
                                sum_list.push_front(prod_name);
                            }
                        }
                    }

                    // Sum all products two at a time
                    let mut sum_counter = 0;
                    while sum_list.len() > 1 {
                        let a = sum_list.pop_front().unwrap();
                        let b = sum_list.pop_front().unwrap();
                        let sum_target = format!("{}_sum_tmp_{}_{}_{}_{}", self.name, out_c, out_y, out_x, sum_counter);

                        self.operation_tuples.push((
                            "+".to_string(),
                            a,
                            b,
                            sum_target.clone(),
                        ));

                        sum_list.push_back(sum_target.clone());
                        self.signal_list.push(sum_target);
                        sum_counter += 1;
                    }

                    // Add bias at the end
                    let output_name = format!("{}_output_{}_{}_{}", self.name, out_c, out_y, out_x);
                    let last_sum = sum_list.pop_front().unwrap();
                    let bias_name = format!("{}_bias_{}_{}_{}_{}", self.name, out_c, out_y, out_x, in_c);

                    self.operation_tuples.push((
                        "+".to_string(),
                        last_sum,
                        bias_name.clone(),
                        output_name.clone(),
                    ));

                    self.input_signal_list.push(bias_name);
                    self.signal_list.push_back(output_name);
                }
            }
        }
    }

    fn get_operation_tuples(&self) -> Vec<(String, String, String, String)> {
        self.operation_tuples.clone()
    }

    fn get_input_signal_list(&self) -> Vec<String> {
        self.input_signal_list.clone()
    }

    fn get_signal_list(&self) -> Vec<String> {
        self.signal_list.clone()
    }
}
