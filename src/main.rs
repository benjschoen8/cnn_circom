mod framework;
mod adapter;
mod usecase;

use crate::framework::pytorch::Pytorch;
use crate::adapter::python_embedding::PythonEmbedding;
use crate::adapter::python_trainer::PythonTrainer;
use crate::usecase::trainer::Trainer;

fn main() {
    let trainer = PythonTrainer::new(Box::new(Pytorch::new()));
    let dataset_path = "framework/python/data";

    match trainer.train(dataset_path) {
        Ok(string) => println!("{}", string),
        Err(e) => eprintln!("error!\n{}", e),
    }
}
