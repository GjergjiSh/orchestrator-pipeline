use crate::shared_data::SharedData;


pub trait Module {
    fn process(&self, shared_data: &SharedData);
}