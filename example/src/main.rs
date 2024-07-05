#![allow(warnings)]

extern crate libloading;
use orchestrator_pipeline::prelude::*;
use std::path::Path;

fn main() {
    let cfg_path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\cfg\module-config.yaml");
    let connection_cfg_path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\cfg\connection-config.yaml");
    let module_config = ModuleConfig::new(cfg_path);
    let connection_config = ConnectionConfig::new(connection_cfg_path);
    let connection = Connection::new(&connection_config);
    let modules = load_modules(&module_config);
    let mut orchestrator = Orchestrator::new();
    for module in modules {
        orchestrator.register_module(module)
    }
    orchestrator.trigger_cycle();

}
