use crate::usecase::writer::Writer;

pub struct CircomWriter {
    // Empty CircomWriter structure
}

impl CircomWriter {
    pub fn new() -> Self {
        Self {
            // Empty for now
        }
    }
}

impl Writer for CircomWriter {
    fn get_string(&self) -> &str {
        // Empty placeholder
        ""
    }

    fn get_file(&self) -> String {
        // Empty placeholder
    }
}

