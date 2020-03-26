#[derive(Default)]
pub struct DoubleArray {
    pub base: Vec<i64>,
    pub check: Vec<i64>,
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
    pub code: char,
    pub depth: i64,
    pub left: i64,
    pub right: i64,
    pub index: i64,
    pub base: i64,
    pub subkey: Vec<char>,
}

impl LikedListNode {
    pub fn new() -> LikedListNode {
        LikedListNode {
            code: ' ',
            depth: 0,
            left: 0,
            right: 0,
            index: 0,
            base: 0,
            subkey: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct LinkedList {
    pub head: Option<LikedListNode>,
    pub tails: Vec<LikedListNode>,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            tails: Vec::new(),
        }
    }
}
