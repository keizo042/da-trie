pub struct DoubleArray {
    base: Vec<isize>,
    check: Vec<isize>,
    used: Vec<bool>,
}

impl DoubleArray {
    pub fn new() -> Self {
        DoubleArray {
            base: Vec::new(),
            check: Vec::new(),
            used: Vec::new(),
        }
    }

    pub fn resize(&mut self, size: usize) {
        self.base.resize(size, 0);
        self.check.resize(size, 0);
        self.used.resize(size, false);
    }
}
