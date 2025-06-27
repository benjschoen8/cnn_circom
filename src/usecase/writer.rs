pub trait Writer {
    fn get_string(&self) -> &str;
    fn get_file(&self) -> String;
}

