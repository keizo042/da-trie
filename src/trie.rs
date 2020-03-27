use crate::da;
use crate::node;
use crate::value;

pub struct Trie {
    key: Vec<String>,
    freq: Vec<usize>,
    da: da::DoubleArray,
    value_pool: Vec<value::Value>,
    next_check_pos: isize,
    left: i64,
    right: i64,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            key: Vec::new(),
            freq: Vec::new(),
            da: da::DoubleArray::new(),
            value_pool: Vec::new(),
            next_check_pos: 0,
            left: 0,
            right: 0,
        }
    }

    pub fn build(key: Vec<String>, freq: Vec<usize>) -> Result<Self, ()> {
        let mut d = Self::new();
        d.key = key;
        d.freq = freq;
        unimplemented!();
    }

    fn fetch(&mut self, parent: node::Node) -> Option<Vec<node::Node>> {
        let mut siblings: Vec<node::Node> = Vec::with_capacity(2);
        let mut prev: char = ' ';
        let mut cur: char = ' ';
        for i in self.left..self.right {
            let v = self.key.get(i as usize)?;

            let tmp: &String = self.key.get(i as usize)?;
            if v.len() != parent.depth {
                cur = tmp.chars().nth(parent.depth + 1)?;
            }
            if prev > cur {
                return Option::Some(Vec::with_capacity(2));
            }
            if cur != prev || siblings.is_empty() {
                let n = node::Node::new();
                siblings.push(n);
            }
            prev = cur;
        }

        if !siblings.is_empty() {
            let idx = siblings.len() - 1;
            siblings[idx].right = parent.right;
        }
        Option::Some(siblings)
    }

    fn insert() -> isize {
        unimplemented!();
    }
}
