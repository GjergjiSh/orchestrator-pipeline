use crate::module::Module;
use crate::shared_data::specification::SharedData;

pub struct Orchestrator {
    modules: Vec<Box<dyn Module>>,
    shared_data: SharedData,
}

impl Orchestrator {
    pub fn new() -> Self {
        Orchestrator {
            modules: Vec::new(),
            shared_data: SharedData::new(),
        }
    }

    pub fn register_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    pub fn trigger_cycle(&self) {
        for module in &self.modules {
            module.process(&self.shared_data)
        }
    }
}
