#[derive(Debug)]
pub struct Motors {
    left: i32,
    right: i32,
}

#[derive(Debug)]
pub struct SharedData {
    motors: Motors
}

impl Motors {
    pub fn new() -> Self {
        Motors {
            left: 0,
            right: 0
        }
    }
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            motors: Motors::new()
        }
    }
}