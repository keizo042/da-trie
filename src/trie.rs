use crate::da;

pub struct Trie {
    key: Vec<String>,
    freq: Vec<usize>,
    da: da::DoubleArray,
    valuePool: Vec<value::Value>,
}

impl Trie {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn build(key: Vec<String>, freq: Vec<usize>) -> Result<Self, ()> {
        let mut d = Self::new();
        d.key = key;
        d.freq = freq;
        unimplemented!();
    }
}
