#![allow(warnings)]

use orchestrator_pipeline::prelude::*;
use debug_module::DebugModule;


fn main() {
    let mut orchestrator = Orchestrator::new();
    orchestrator.register_module(Box::new(DebugModule::new()));
    orchestrator.trigger_cycle();
}
