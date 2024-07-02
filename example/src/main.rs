#![allow(warnings)]

extern crate libloading;
use orchestrator_pipeline::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\target\debug\debug_module.dll");
    let cfg_path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\sample-config.yaml");
    let module_config = ModuleConfig::new(cfg_path);
    let modules = module_config.modules();
    let dbg_module_cfg = modules.get("debug_module").unwrap();
    let module = load_module(path, dbg_module_cfg);
    let mut orchestrator = Orchestrator::new();
    orchestrator.register_module(module);
    orchestrator.trigger_cycle();
}
