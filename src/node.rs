#[derive(Default)]
pub struct Node {
    pub code: char,
    pub depth: usize,
    pub left: usize,
    pub right: usize,
}

impl Node {
    pub fn new() -> Self {
        Node {
            code: ' ',
            depth: 0,
            left: 0,
            right: 0,
        }
    }
}
