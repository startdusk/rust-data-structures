use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_v(max)
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    module: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            // 下面的数字乱填的，没有实际意义，只是为了让数字更随机
            mul: 56394237,
            inc: 34642349,
            module: 2325454456,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.module;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rands_pringout() {
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("--{}", r.next_v(100));
        }
    }
}
