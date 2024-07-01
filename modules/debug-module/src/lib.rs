#![allow(warnings)]

use orchestrator_pipeline::prelude::*;

pub struct DebugModule {}

impl DebugModule {
    pub fn new() -> Self {
        DebugModule {}
    }
}

impl Module for DebugModule {
    fn process(&self, shared_data: &SharedData) {
        dbg!(shared_data);
    }
}