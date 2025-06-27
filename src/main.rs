mod framework;
mod adapter;

use framework::pytorch::Pytorch;
use crate::adapter::python_embedding::PythonEmbedding;

fn main() {
    let trainer = Pytorch::new();
    let dataset_path = "framework/python/data";

    match trainer.train(dataset_path) {
        Ok(json) => println!("Training completed successfully:\n{}", json),
        Err(e) => eprintln!("Training failed: {}", e),
    }
}
