#![allow(warnings)]

use orchestrator_pipeline::prelude::*;
use std::collections::HashMap;

pub struct DebugModule {
    debug_bool: bool,
    debug_u8: u8,
    debug_u16: u16,
    debug_u32: u32,
    debug_u64: u64,
    debug_i8: i8,
    debug_i16: i16,
    debug_i32: i32,
    debug_i64: i64,
    debug_f32: f32,
    debug_f64: f64,
    debug_string: String,
}

impl DebugModule {
    fn new(parameters: &HashMap<String, String>) -> Self {
        DebugModule {
            debug_bool: parameters.get("debug_bool")
                .expect("Missing debug_bool")
                .parse()
                .expect("Invalid value for debug_bool"),
            debug_u8: parameters.get("debug_u8")
                .expect("Missing debug_u8")
                .parse()
                .expect("Invalid value for debug_u8"),
            debug_u16: parameters.get("debug_u16")
                .expect("Missing debug_u16")
                .parse()
                .expect("Invalid value for debug_u16"),
            debug_u32: parameters.get("debug_u32")
                .expect("Missing debug_u32")
                .parse()
                .expect("Invalid value for debug_u32"),
            debug_u64: parameters.get("debug_u64")
                .expect("Missing debug_u64")
                .parse()
                .expect("Invalid value for debug_u64"),
            debug_i8: parameters.get("debug_i8")
                .expect("Missing debug_i8")
                .parse()
                .expect("Invalid value for debug_i8"),
            debug_i16: parameters.get("debug_i16")
                .expect("Missing debug_i16")
                .parse()
                .expect("Invalid value for debug_i16"),
            debug_i32: parameters.get("debug_i32")
                .expect("Missing debug_i32")
                .parse()
                .expect("Invalid value for debug_i32"),
            debug_i64: parameters.get("debug_i64")
                .expect("Missing debug_i64")
                .parse()
                .expect("Invalid value for debug_i64"),
            debug_f32: parameters.get("debug_f32")
                .expect("Missing debug_f32")
                .parse()
                .expect("Invalid value for debug_f32"),
            debug_f64: parameters.get("debug_f64")
                .expect("Missing debug_f64")
                .parse()
                .expect("Invalid value for debug_f64"),
            debug_string: parameters.get("debug_string")
                .expect("Missing debug_string")
                .clone(),
        }
    }
}

impl Module for DebugModule {
    fn process(&self, shared_data: &SharedData) {
        dbg!(shared_data);
    }
}

#[no_mangle]
pub extern "C" fn create_module(parameters: &HashMap<String, String>) -> Box<dyn Module> {
    Box::new(DebugModule::new(parameters))
}
