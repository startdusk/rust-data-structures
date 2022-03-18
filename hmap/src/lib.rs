mod hasher;

use hasher::hash;
use std::{borrow::Borrow, hash::Hash};

const BSIZE: usize = 8;
const BGROW: usize = 8;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> BucketList<K, V> {
    fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }

    // usize returned how big chosen bucket is
    // tell caller if its too full
    fn push(&mut self, k: K, v: V) -> usize {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        self.buckets[h].push((k, v));
        self.len += 1;
        self.buckets[h].len()
    }

    fn get<KB>(&self, k: &KB) -> Option<&V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &self.buckets[h] {
            if k == (ik as &K).borrow() {
                return Some(iv);
            }
        }
        None
    }
    fn get_mut<KB>(&mut self, k: &KB) -> Option<&mut V>
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

    fn bucket(&mut self, n: usize) -> Option<Vec<(K, V)>> {
        if n >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[n]);
        self.len -= res.len();
        Some(res)
    }

    fn set_buckets(&mut self, n: usize) {
        for _ in self.buckets.len()..n {
            self.buckets.push(Vec::new());
        }
    }
}

#[derive(Debug)]
pub struct HMap<K, V> {
    n_moved: usize,         // 已经移动的元素数量
    main: BucketList<K, V>, // 主要放置的数据
    grow: BucketList<K, V>, // 将要移动到的数据的地方
}

impl<K: Hash + Eq, V> HMap<K, V> {
    pub fn new() -> Self {
        HMap {
            n_moved: 0,
            main: BucketList::new(),
            grow: BucketList::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(iv) = self.main.get_mut(&k) {
            *iv = v;
            return;
        }
        if let Some(iv) = self.grow.get_mut(&k) {
            *iv = v;
            return;
        }
        if self.n_moved > 0 {
            // we have started move to bigger bucket list
            self.grow.push(k, v);
            self.move_bucket();
            return;
        }
        if self.main.push(k, v) > BSIZE / 2 {
            // grow buckets
            self.move_bucket();
        }
    }

    pub fn get<KR>(&self, kr: &KR) -> Option<&V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        self.main.get(kr).or_else(|| self.grow.get(kr))
    }
    pub fn get_mut<KR>(&mut self, kr: &KR) -> Option<&mut V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        self.main.get_mut(kr).or_else(|| self.grow.get_mut(kr))
        // 等价于下面的写法
        // if let Some(b) = self.main.get_mut(kr) {
        //     return Some(b);
        // }

        // self.grow.get_mut(kr)
    }

    pub fn len(&self) -> usize {
        self.main.len + self.grow.len
    }

    pub fn move_bucket(&mut self) {
        if self.n_moved == 0 {
            self.grow.set_buckets(self.main.buckets.len() * 2); // 数据过多 直接增长两倍的体积，然后搬运数据过去
        }
        if let Some(b) = self.main.bucket(self.n_moved) {
            for (k, v) in b {
                self.grow.push(k, v);
            }
            self.n_moved += 1;
            return;
        }

        // if all data out of main into grow, then grow is main
        std::mem::swap(&mut self.main, &mut self.grow);
        self.n_moved = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_right_values() {
        let cases: Vec<(String, i32)> = vec![
            ("james".to_string(), 18),
            ("dave".to_string(), 45),
            ("andy".to_string(), 23),
            ("pete".to_string(), 14),
            ("steve".to_string(), 90),
            ("jane".to_string(), 105),
            ("grader".to_string(), 23),
            ("irene".to_string(), 65),
            ("sam".to_string(), 66),
            ("andrex".to_string(), 77),
            ("andrew".to_string(), 89),
            ("geralt".to_string(), 99),
        ];
        let mut hm = HMap::new();
        for c in &cases {
            let c = c.clone();
            hm.insert(c.0, c.1);
        }

        for c in cases {
            assert_eq!(hm.get(&c.0), Some(&c.1));
        }

        // 常用做法，查看数据内容
        // println!("hm = {:?}", hm);
        // panic!()
    }

    #[test]
    fn test_lots_of_numbers() {
        let mut hm = HMap::new();
        for x in 0..10000 {
            // 每个值 + 250
            hm.insert(x, x + 250);
        }
        assert_eq!(hm.len(), 10000);
        assert_eq!(hm.get(&500), Some(&750));

        for (n, x) in hm.main.buckets.iter().enumerate() {
            let msg = format!("main bucket too big {}:{}", n, x.len());
            assert!(x.len() < 12, "{}", msg);
        }
        for (n, x) in hm.grow.buckets.iter().enumerate() {
            let msg = format!("grow bucket too big {}:{}", n, x.len());
            assert!(x.len() < 12, "{}", msg);
        }
    }
}
