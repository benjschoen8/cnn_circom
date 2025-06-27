use crate::usecase::model::Model;

pub trait Trainer {
    fn train(&mut self);
    fn get_model(&self) -> String;
}

