use crate::trie;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Default)]
pub struct Darts {
    dat: trie::DoubleArray,
    llt: trie::LinkedList,
    used: Vec<bool>,
    next_check_pos: i64,
    key: KeySlice,
    output: HashMap<i64, String>,
}

impl Darts {
    pub fn new() -> Darts {
        Darts {
            dat: trie::DoubleArray::new(),
            llt: trie::LinkedList::new(),
            key: KeySlice::new(),
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
struct Key(String);

impl Key {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default)]
struct KeySlice(Vec<Key>);

impl KeySlice {
    pub fn new() -> KeySlice {
        KeySlice(Vec::new())
    }

    pub fn sort(&mut self) {
        self.0.sort_by(KeySlice::f)
    }

    fn f(_a: &Key, _b: &Key) -> Ordering {
        Ordering::Less
    }
}
