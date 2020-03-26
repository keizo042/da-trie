use crate::trie;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Default)]
pub struct Darts {
    dat: trie::DoubleArrayTrie,
    llt: trie::LinkedListTrie,
    used: Vec<bool>,
    next_check_pos: i64,
    key: DartsKeySlice,
    output: HashMap<i64, String>,
}

impl Darts {
    pub fn new() -> Darts {
        Darts {
            dat: trie::DoubleArrayTrie::new(),
            llt: trie::LinkedListTrie::new(),
            key: DartsKeySlice::new(),
            next_check_pos: 0,
            output: HashMap::new(),
            used: Vec::new(),
        }
    }

    pub fn build(_keywords: Vec<Vec<u8>>) -> Option<()> {
        unimplemented!();
    }
}

#[derive(Default)]
struct DartsKey(String);

impl DartsKey {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default)]
struct DartsKeySlice(Vec<DartsKey>);

impl DartsKeySlice {
    pub fn new() -> DartsKeySlice {
        DartsKeySlice(Vec::new())
    }

    pub fn sort(&mut self) {
        self.0.sort_by(DartsKeySlice::f)
    }

    fn f(_a: &DartsKey, _b: &DartsKey) -> Ordering {
        Ordering::Less
    }
}
