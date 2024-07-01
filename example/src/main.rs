#![allow(warnings)]

extern crate libloading;
use orchestrator_pipeline::prelude::*;

fn main() {
    let path = std::path::Path::new(r"C:\Users\Gjergji\Repos\orchestartor-pipeline\target\debug\debug_module.dll");
    let module = load_module(path);
    let mut orchestrator = Orchestrator::new();
    orchestrator.register_module(module);
    orchestrator.trigger_cycle();
}
