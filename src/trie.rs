#[derive(Default)]
pub struct DoubleArray {
    base: Vec<i64>,
    check: Vec<i64>,
}

impl DoubleArray {
    pub fn new() -> DoubleArray {
        DoubleArray {
            base: Vec::new(),
            check: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct LikedListNode {
    code: char,
    depth: i64,
    left: i64,
    index: i64,
    base: i64,
    children: Vec<LikedListNode>,
}

impl LikedListNode {
    pub fn new() -> LikedListNode {
        LikedListNode {
            children: Vec::new(),
            code: ' ',
            depth: 0,
            left: 0,
            index: 0,
            base: 0,
        }
    }
}

#[derive(Default)]
pub struct LinkedList {
    root: Option<LikedListNode>,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { root: None }
    }
}
