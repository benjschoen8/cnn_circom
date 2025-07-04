use crate::entity::layer::Layer;
use std::collections::VecDeque;


pub struct Dense {
    pub input_name: String,
    pub input_size: usize,                 // Input feature size
    pub weight_size: (usize, usize),       // (output_size, input_size)
    pub weight: Vec<Vec<f32>>,             // [output_size][input_size]
    pub bias: Vec<f32>,                    // [output_size]
    pub name: String,
    pub to_integer_bias: usize,
    pub input_signal_list: Vec<String>,
    pub signal_list: Vec<String>,
    pub operation_tuples: Vec<(String, String, String, String)>,
}

impl Dense {
    pub fn new() -> Self {
        Self {
            input_name: String::new(),
            input_size: 0,
            weight_size: (0, 0),
            weight: Vec::new(),
            bias: Vec::new(),
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

    pub fn get_input_size(&self) -> usize {
        self.input_size
    }

    pub fn get_weight_size(&self) -> (usize, usize) {
        self.weight_size
    }

    pub fn calculate_test(&self, input_values: &Vec<f32>) -> Vec<f32> {
        let (output_size, input_size) = self.weight_size;

        assert_eq!(
            input_values.len(),
            input_size,
            "Input size does not match Dense layer input size"
        );

        let mut outputs = vec![0.0; output_size];

        for out_i in 0..output_size {
            let mut sum = 0.0;
            for in_i in 0..input_size {
                sum += input_values[in_i] * self.weight[out_i][in_i];
            }
            sum += self.bias[out_i];
            outputs[out_i] = sum;
        }

        outputs
    }
}

impl Layer for Dense {
    fn calculate_signals(&mut self) {
        let (output_size, input_size) = self.weight_size;

        for out_i in 0..output_size {
            let mut sum_list: VecDeque<String> = VecDeque::new();

            for in_i in 0..input_size {
                let input_signal = format!("{}_{}", self.input_name, in_i);
                let weight_signal = format!("{}_weight_{}_{}", self.name, out_i, in_i);
                let prod_name = format!("{}_prod_tmp_{}_{}", self.name, out_i, in_i);

                self.input_signal_list.push(weight_signal.clone());
                self.signal_list.push(prod_name.clone());

                // Add multiplication operation
                self.operation_tuples.push((
                    "*".to_string(),
                    input_signal,
                    weight_signal,
                    prod_name.clone(),
                ));

                sum_list.push_front(prod_name);
            }

            // Sum all products two at a time
            let mut sum_counter = 0;
            while sum_list.len() > 1 {
                let a = sum_list.pop_front().unwrap();
                let b = sum_list.pop_front().unwrap();
                let sum_target = format!("{}_sum_tmp_{}_{}", self.name, out_i, sum_counter);

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

            // Add bias
            let output_name = format!("{}_output_{}", self.name, out_i);
            let last_sum = sum_list.pop_front().unwrap();
            let bias_name = format!("{}_bias_{}", self.name, out_i);

            self.operation_tuples.push((
                "+".to_string(),
                last_sum,
                bias_name.clone(),
                output_name.clone(),
            ));

            self.input_signal_list.push(bias_name);
            self.signal_list.push(output_name);
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
