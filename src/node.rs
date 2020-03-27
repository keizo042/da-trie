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

    pub fn clone(&self) -> Node {
        Node {
            code: self.code,
            depth: self.depth,
            left: self.left,
            right: self.right,
        }
    }
}
