use crate::da;
use crate::node;
use crate::value;
use std::cmp::max;
use std::ops::Neg;

pub struct ResultPair {
    pub prefix_len: usize,
    pub value: value::Value,
}

pub struct Trie {
    key: Vec<String>,
    freq: Vec<usize>,
    da: da::DoubleArray,
    value_pool: Vec<value::Value>,
    next_check_pos: isize,
    left: i64,
    right: i64,
    size: isize,
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

    pub fn common_prefix_search(
        &mut self,
        key: String,
        node_pos: usize,
    ) -> Result<Vec<ResultPair>, ()> {
        let mut b = self.da.base[node_pos];
        let mut result: Vec<ResultPair> = vec![];
        let mut p: isize;
        for i in 0..key.len() {
            p = b;
            let n = self.da.base[p as usize];
            if b == self.da.check[p as usize] && n < 0 {
                let v = self.value_pool[(n.abs() - 1) as usize].clone();
                let pair = ResultPair {
                    prefix_len: i,
                    value: v,
                };
                result.push(pair);
            }

            p = b + key.len() as isize + 1;
            if p >= self.da.check.len() as isize {
                b = self.da.base[p as usize];
            } else {
                return Result::Ok(result);
            }
        }

        p = b;
        let n = self.da.base[p as usize];
        if b == self.da.check[p as usize] && n < 0 {
            let v = self.value_pool.get((n.abs() - 1) as usize).unwrap();
            let vc = v.clone();
            let pair = ResultPair {
                prefix_len: key.len(),
                value: vc,
            };
            result.push(pair);
        }
        Result::Ok(result)
    }

    pub fn extract_match_search(&mut self, key: String, node_pos: usize) -> bool {
        let mut p: usize;
        let mut b = self.da.base[node_pos];
        for i in 0..key.len() {
            let tmp = b + key.chars().nth(i).unwrap() as isize + 1;
            p = tmp as usize;
            if b == self.da.check[p] {
                b = self.da.base[p];
            } else {
                return false;
            }
        }
        p = b as usize;
        let n = self.da.base[p];
        if b == self.da.check[p] && n < 0 {
            return true;
        }
        false
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

    fn insert(&mut self, siblings: Vec<node::Node>) -> isize {
        let mut begin: isize;
        let mut pos: isize = max(siblings[0].code as isize + 1, self.next_check_pos);
        let mut non_zero_num = 0;
        let mut first = false;
        loop {
            pos += 1;

            if self.da.base.len() <= pos as usize {
                self.da.resize((pos + 1) as usize);
            }

            if self.da.check[pos as usize] > 0 {
                non_zero_num += 1;
                continue;
            } else if !first {
                self.next_check_pos = pos;
                first = true;
            }

            begin = pos - siblings[0].code as isize;
            let c = begin + siblings[siblings.len() as usize - 1].code as isize;
            if self.da.base.len() <= c as usize {
                let len = c as usize + 400;
                self.da.resize(len);
            }

            if self.da.used[begin as usize] {
                continue;
            }

            for i in 1..siblings.len() {
                if self.da.check[begin as usize + siblings[i].code as usize] != 0 {
                    continue;
                }
            }

            if non_zero_num as f64 / (pos - self.next_check_pos - 1) as f64 > 0.95 {
                self.next_check_pos = pos;
            }
            self.da.used[begin as usize] = true;
            self.size = max(
                self.size,
                begin + siblings[siblings.len() as usize - 1].code as isize + 1,
            );

            for i in 0..siblings.len() {
                self.da.check[begin as usize + siblings[i].code as usize] = begin as isize;
            }

            for i in 0..siblings.len() {
                let s = siblings[i].clone();
                let new_siblings = self.fetch(s).unwrap_or(Vec::new());
                if new_siblings.is_empty() {
                    let mut v = value::Value::new();
                    v.freq = self.freq[siblings[i].left];
                    let n = Neg::neg(self.value_pool.len() as isize) - 1;
                    self.da.base[begin as usize + siblings[i].code as usize] = n;
                } else {
                    let h = self.insert(new_siblings);
                    self.da.base[begin as usize + siblings[i].code as usize] = h
                }
            }
            return begin;
        }
    }
}

#[cfg(test)]
mod tests {
    fn common_prexi_match() {
        unimplemented!();
    }
    fn exact_match_search() {
        unimplemented!();
    }
}
