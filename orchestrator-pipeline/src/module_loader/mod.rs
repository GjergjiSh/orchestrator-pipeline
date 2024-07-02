extern crate libloading;

use crate::module::{Module, ModuleConfig};
use std::{collections::HashMap, path::PathBuf};

static mut LOADED_LIBRARIES: Vec<libloading::Library> = Vec::new();

pub fn load_modules(module_config: &ModuleConfig) -> Vec<Box<dyn Module>>  {
    let module_root = module_config.module_root();
    let module_cfgs = module_config.modules();
    let module_count = module_cfgs.len();
    let mut modules =  Vec::with_capacity(module_count);

    for (name, parameters) in &module_cfgs {
        let mut module_path = PathBuf::from(module_root);
        module_path.push(name);
        #[cfg(target_os = "windows")]
        module_path.set_extension("dll");
        #[cfg(target_os = "linux")]
        module_path.set_extension("so");
        #[cfg(target_os = "macos")]
        module_path.set_extension("dylib");
        let module = _load_module(&module_path, parameters);
        modules.push(module);
    }

    modules
}

fn _load_module(
    path: &std::path::Path,
    parameters: &HashMap<String, String>,
) -> Box<dyn Module> {
    if !path.exists() {
        panic!("Module library not found at: {:?}", path);
    }

    let extension = path.extension().unwrap_or_default();
    if !_is_shared_library(extension) {
        panic!("Module library must be a shared library");
    }

    let lib = unsafe {
        libloading::Library::new(path)
            .unwrap_or_else(|e| panic!("Failed to load module library at {:?}: {}", path, e))
    };

    let constructor: libloading::Symbol<
        unsafe extern "C" fn(&HashMap<String, String>) -> Box<dyn Module>,
    > = unsafe {
        lib.get(b"create_module").unwrap_or_else(|e| {
            panic!(
                "A 'create_module' constructor method was not \
                found for the module. Error: {}",
                e
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
fn _is_shared_library(extension: &std::ffi::OsStr) -> bool {
    extension == "dll"
}

#[cfg(target_os = "macos")]
fn _is_shared_library(extension: &std::ffi::OsStr) -> bool {
    extension == "dylib"
}

#[cfg(target_os = "linux")]
fn _is_shared_library(extension: &std::ffi::OsStr) -> bool {
    extension == "so"
}
