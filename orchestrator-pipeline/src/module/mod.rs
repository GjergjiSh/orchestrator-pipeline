pub mod module_loader;

use crate::shared_data::specification::SharedData;

pub trait Module {
    fn process(&self, shared_data: &SharedData);
}
