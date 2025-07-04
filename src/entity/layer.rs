pub trait Layer {
    fn calculate_signals(&mut self);
    fn get_operation_tuples(&self) -> Vec<(String, String, String, String)>;
    fn get_input_signal_list(&self) -> Vec<String>;
    fn get_signal_list(&self) -> Vec<String>;
}
