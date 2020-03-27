use crate::da;
use crate::node;
use crate::value;
use std::cmp::max;

pub struct Trie {
    key: Vec<String>,
    freq: Vec<usize>,
    da: da::DoubleArray,
    value_pool: Vec<value::Value>,
    next_check_pos: usize,
    left: i64,
    right: i64,
    size: usize,
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
            size: 0,
        }
    }

    pub fn build(key: Vec<String>, freq: Vec<usize>) -> Result<da::DoubleArray, ()> {
        let default_da_len = 512;
        let mut d = Trie::new();
        d.key = key;
        d.freq = freq;
        d.da.resize(default_da_len);
        d.da.base[0] = 1;
        d.next_check_pos = 0;
        let mut root = node::Node::new();
        root.depth = 0;
        root.right = 0;
        root.left = 0;
        let sibilings = d.fetch(root);
        if sibilings.is_none() {
            return Result::Err(());
        }
        d.insert(sibilings.unwrap());
        Result::Ok(d.da)
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

    fn insert(&mut self, siblings: Vec<node::Node>) -> usize {
        let mut begin: usize = 0;
        let mut pos: usize = max(siblings[0].code as usize + 1, self.next_check_pos as usize);
        let mut non_zero_num = 0;
        let mut first = false;
        loop {
            pos += 1;

            if self.da.base.len() <= pos as usize {
                self.da.resize((pos + 1) as usize);
            }

            if self.da.check[pos] > 0 {
                non_zero_num += 1;
                continue;
            } else if !first {
                self.next_check_pos = pos;
                first = true;
            }

            begin = pos - siblings[0].code as usize;
            let c = begin + siblings[siblings.len() - 1].code as usize;
            if self.da.base.len() <= c {
                let len = c + 400;
                self.da.resize(len);
            }

            if self.da.used[begin] {
                continue;
            }

            for i in 1..siblings.len() {
                if self.da.check[begin + siblings[i].code as usize] != 0 {
                    continue;
                }
            }

            if non_zero_num as f64 / (pos - self.next_check_pos - 1) as f64 > 0.95 {
                self.next_check_pos = pos;
            }
            self.da.used[begin] = true;
            self.size = max(
                self.size,
                begin + siblings[siblings.len() - 1].code as usize + 1,
            );

            for i in 0..siblings.len() {
                self.da.check[begin + siblings[i].code as usize] = begin;
            }

            for i in 0..siblings.len() {
                let new_siblings = self.fetch(siblings[i]).unwrap_or(vec![]);
                if new_siblings.is_empty() {
                    let v: value::Value;
                    v.freq = self.freq[siblings[i].left];
                    self.da.base[begin + siblings[i].code as usize] = -self.value_pool.len() - 1;
                } else {
                    let h = self.insert(new_siblings);
                    self.da.base[begin + siblings[i].code as usize] = h
                }
            }
            return begin;
        }
    }
}
