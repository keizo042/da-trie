use std::cmp::Ordering;
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

pub struct Darts {
    dat: DoubleArrayTrie,
    llt: LinkedListTrie,
    used: Vec<bool>,
    next_check_pos: i64,
    key: DartsKeySlice,
    output: HashMap<i64, String>,
}

impl Darts {
    pub fn new() -> Darts {
        Darts{
            dat: DoubleArrayTrie::new(),
            llt: LinkedListTrie::new(),
            key: DartsKeySlice::new(),
            next_check_pos: 0,
            output: HashMap::new(),
            used: Vec::new(),
        }
    }

    pub fn build(keywords: Vec<Vec<u8>>)-> Option<()>{
    }
}

pub struct DoubleArrayTrie {
    base: Vec<i64>,
    check: Vec<i64>,
}

impl DoubleArrayTrie {
    pub fn new() -> DoubleArrayTrie{
        DoubleArrayTrie{
            base: Vec::new(),
            check: Vec::new(),
        }
    }
}

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
        LikedListTrieNode{
            children: Vec::new(),
            code: ' ',
            depth:0,
            left: 0,
            index: 0,
            base:0,
        }
    }
}


pub struct LinkedListTrie {
    root: Option<LikedListTrieNode>,
}

impl LinkedListTrie {
    pub fn new() -> LinkedListTrie {
        LinkedListTrie{
            root: None,
        }
    }
}

struct DartsKey(String);

impl DartsKey {
    pub fn len(&self) -> usize {
        return self.0.len();
    }
}

struct DartsKeySlice(Vec<DartsKey>);

impl DartsKeySlice {
    pub fn new() -> DartsKeySlice {
        return DartsKeySlice(Vec::new());
    }

    pub fn sort(&mut self) -> () {
        self.0.sort_by(DartsKeySlice::f);
    }

    fn f(_a: &DartsKey, _b: &DartsKey) -> Ordering {
        return Ordering::Less;
    }
}
