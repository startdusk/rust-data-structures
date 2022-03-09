/// 二叉平衡树
use std::fmt::Debug;

#[derive(Debug)]
pub struct BinaryBalanceTree<T>(Option<Box<BinaryData<T>>>);

#[derive(Debug)]
pub struct BinaryData<T> {
    data: T,
    h: i8, // 记录树高
    left: BinaryBalanceTree<T>,
    right: BinaryBalanceTree<T>,
}

impl<T> BinaryData<T> {
    // 左旋
    pub fn rot_left(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), // No right node how can we rotate?
        };
        // move left of right node to right of start node
        self.right = BinaryBalanceTree(res.left.0.take());
        self.right.set_height();
        // set the results left node to the start node
        res.left = BinaryBalanceTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
    // 右旋
    pub fn rot_right(mut self) -> Box<Self> {
        // result is the left node
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };
        // move right of left node to left of start node
        self.left = BinaryBalanceTree(res.right.0.take());
        self.left.set_height();
        // set the results left node to the start node
        res.right = BinaryBalanceTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

impl<T> BinaryBalanceTree<T> {
    pub fn new() -> Self {
        BinaryBalanceTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }
    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}

impl<T: PartialOrd> BinaryBalanceTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                let res: i32;
                if data < bd.data {
                    bd.left.add_sorted(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        res = 1
                    } else {
                        res = 0
                    };
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        res = -1
                    } else {
                        res = 0
                    };
                }
                res
            }
            None => {
                self.0 = Some(Box::new(BinaryData {
                    data,
                    h: 0, // 创建一颗新的树，显然它的树高为0
                    left: BinaryBalanceTree::new(),
                    right: BinaryBalanceTree::new(),
                }));
                0
            }
        };

        // 添加左旋右旋后，树会非常对称(非常平衡)
        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
    }
}

impl<T: Debug> BinaryBalanceTree<T> {
    pub fn print_lfirst(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('-');
            }
            println!("{}:{}{:?}", bd.h, spc, bd.data);
            bd.right.print_lfirst(dp + 1);
        }
    }
}

fn main() {
    let mut t = BinaryBalanceTree::new();

    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(6);
    t.add_sorted(10);
    t.add_sorted(1);
    t.add_sorted(94);
    t.add_sorted(54);
    t.add_sorted(3);
    for i in 0..100000 {
        t.add_sorted(i);
    }
    t.print_lfirst(0);
}
