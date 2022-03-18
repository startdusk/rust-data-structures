use std::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
};

use hasher::hash;

mod hasher;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> BucketList<K, V> {
    pub fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }

    // usize returned how big chosen bucket is
    // tell caller if its too full
    pub fn push(&mut self, k: K, v: V) -> usize {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        self.buckets[h].push((k, v));
        self.len += 1;
        self.buckets[h].len()
    }

    pub fn get<KB>(&mut self, k: &KB) -> Option<&mut V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &mut self.buckets[h] {
            if k == (ik as &K).borrow() {
                return Some(iv);
            }
        }
        None
    }
    pub fn get_mut<KB>(&self, k: &KB) -> Option<&V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &self.buckets[h] {
            if k == ik.borrow() {
                return Some(iv);
            }
        }
        None
    }

    pub fn bucket(&mut self, n: usize) -> Option<Vec<(K, V)>> {
        if n >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[n]);
        self.len -= res.len();
        Some(res)
    }

    pub fn set_buckets(&mut self, n: usize) {
        for _ in self.buckets.len()..n {
            self.buckets.push(Vec::new());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
