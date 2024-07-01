#[derive(Debug)]
pub struct SharedData {
    foo: i32
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            foo: 0
        }
    }
}