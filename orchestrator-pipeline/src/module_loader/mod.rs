extern crate libloading;

use libloading::Library;

use crate::module::Module;

static mut LOADED_LIBRARIES: Vec<Library> = Vec::new();

//TODO: Error handling
pub fn load_module(path: &std::path::Path) -> Box<dyn Module> {
    let lib = unsafe { Library::new(path).unwrap() };
    let constructor: libloading::Symbol<fn() -> Box<dyn Module>> = unsafe { lib.get(b"create_module").unwrap() };
    let module = constructor();

    unsafe {
        LOADED_LIBRARIES.push(lib);
    }

    module
}