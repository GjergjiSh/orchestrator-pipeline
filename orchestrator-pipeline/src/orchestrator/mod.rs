use crate::module::Module;
use crate::shared_data::specification::{Motors, SharedData};
use protobuf::MessageField;

pub struct Orchestrator {
    modules: Vec<Box<dyn Module>>,
    shared_data: SharedData,
}

impl Orchestrator {
    pub fn new() -> Self {
        let mut motors = Motors::new();
        motors.left = 0;
        motors.right = 0;
        let mut shared_data = SharedData::new();
        shared_data.motors = MessageField::some(motors);


        Orchestrator {
            modules: Vec::new(),
            shared_data: shared_data,
        }
    }

    pub fn register_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    pub fn trigger_cycle(&self) -> &SharedData {
        for module in &self.modules {
            module.process(&self.shared_data)
        }

        &self.shared_data
    }

    pub fn shared_data(&self) -> &SharedData {
        &self.shared_data
    }

    pub fn mut_shared_data(&mut self) -> &mut SharedData {
        &mut self.shared_data
    }

}
