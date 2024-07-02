extern crate libloading;

use crate::module::Module;
use std::collections::HashMap;

static mut LOADED_LIBRARIES: Vec<libloading::Library> = Vec::new();

pub fn load_module(path: &std::path::Path, parameters: &HashMap<String, String>) -> Box<dyn Module> {
    if !path.exists() {
        panic!("Module library not found at: {:?}", path);
    }

    let extension = path.extension().unwrap_or_default();
    if !is_shared_library(extension) {
        panic!("Module library must be a shared library");
    }

    let lib = unsafe {
        libloading::Library::new(path)
            .unwrap_or_else(|e| panic!("Failed to load module library at {:?}: {}", path, e))
    };

    let constructor: libloading::Symbol<unsafe extern "C" fn(&HashMap<String, String>) -> Box<dyn Module>> = unsafe {
        lib.get(b"create_module").unwrap_or_else(|e| {
            panic!(
                "A 'create_module' constructor method was not \
                found for the module. Error: {}", e
            )
        })
    };

    let module = unsafe { constructor(parameters) };

    unsafe {
        LOADED_LIBRARIES.push(lib);
    }

    module
}

#[cfg(target_os = "windows")]
fn is_shared_library(extension: &std::ffi::OsStr) -> bool {
    extension == "dll"
}

#[cfg(target_os = "macos")]
fn is_shared_library(extension: &std::ffi::OsStr) -> bool {
    extension == "dylib"
}

#[cfg(target_os = "linux")]
fn is_shared_library(extension: &std::ffi::OsStr) -> bool {
    extension == "so"
}
