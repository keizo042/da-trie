#[derive(Default)]
pub struct DoubleArrayTrie {
    base: Vec<i64>,
    check: Vec<i64>,
}

impl DoubleArrayTrie {
    pub fn new() -> DoubleArrayTrie {
        DoubleArrayTrie {
            base: Vec::new(),
            check: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct LikedListTrieNode {
    code: char,
    depth: i64,
    left: i64,
    index: i64,
    base: i64,
    children: Vec<LikedListTrieNode>,
}

impl LikedListTrieNode {
    pub fn new() -> LikedListTrieNode {
        LikedListTrieNode {
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
pub struct LinkedListTrie {
    root: Option<LikedListTrieNode>,
}

impl LinkedListTrie {
    pub fn new() -> LinkedListTrie {
        LinkedListTrie { root: None }
    }
}
