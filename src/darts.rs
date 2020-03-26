use crate::c;
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

    pub fn build(
        &mut self,
        keywords: Vec<String>,
    ) -> Result<(&trie::DoubleArray, &trie::LinkedList), ()> {
        if keywords.is_empty() {
            return Result::Err(());
        }
        self.resize(c::RESIZE_DELTA);
        self.init()?;
        self.key.sort();
        let mut s = self.fetch()?;
        let sb = &mut s;
        self.insert(sb)?;
        Result::Ok((&self.dat, &self.llt))
    }

    fn init(&mut self) -> Result<(), ()> {
        unimplemented!();
    }

    fn resize(&mut self, size: usize) {
        self.dat
            .base
            .append(&mut Vec::with_capacity(size - self.dat.base.len()));
        self.dat
            .check
            .append(&mut Vec::with_capacity(size - self.dat.check.len()));
        self.used
            .append(&mut Vec::with_capacity(size - self.used.len()));
    }

    fn fetch(&mut self) -> Result<Vec<trie::LinkedList>, ()> {
        Result::Ok(Vec::new())
    }

    fn insert(&mut self, _parent: &mut Vec<trie::LinkedList>) -> Result<(), ()> {
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
