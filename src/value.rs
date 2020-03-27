#[derive(Default)]
pub struct Value {
    pub freq: usize,
}

impl Value {
    pub fn new() -> Value {
        Value { freq: 0 }
    }

    pub fn clone(&self) -> Value {
        Value { freq: self.freq }
    }
}
