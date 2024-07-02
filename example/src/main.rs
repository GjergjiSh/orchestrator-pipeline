#![allow(warnings)]

extern crate libloading;
use orchestrator_pipeline::prelude::*;

fn main() {
    let path = std::path::Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\target\debug\debug_module.dll");
    let module = load_module(path);
    let mut orchestrator = Orchestrator::new();
    orchestrator.register_module(module);
    orchestrator.trigger_cycle();
}
