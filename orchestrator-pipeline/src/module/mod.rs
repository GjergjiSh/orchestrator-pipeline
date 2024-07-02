use crate::shared_data::SharedData;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ModuleConfig {
    name: String,
    parameters: std::collections::HashMap<String, String>,
}

pub trait Module {
    fn process(&self, shared_data: &SharedData);
}
