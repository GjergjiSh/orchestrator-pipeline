#![allow(warnings)]

extern crate libloading;
use orchestrator_pipeline::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\target\debug\debug_module.dll");
    let cfg_path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\sample-config.yaml");
    let module_config = ModuleConfig::new(cfg_path);
    let modules = load_modules(&module_config);
    let mut orchestrator = Orchestrator::new();
    for module in modules {
        orchestrator.register_module(module)
    }
    orchestrator.trigger_cycle();
}
