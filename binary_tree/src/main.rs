use std::fmt::Debug;

#[derive(Debug)]
pub struct BinaryTree<T>(Option<Box<BinaryData<T>>>);

#[derive(Debug)]
pub struct BinaryData<T> {
    data: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree(None)
    }
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut db) => {
                if data < db.data {
                    db.left.add_sorted(data);
                } else {
                    db.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinaryData {
                    data,
                    left: BinaryTree::new(),
                    right: BinaryTree::new(),
                }))
            }
        }
    }
}

impl<T: Debug> BinaryTree<T> {
    pub fn print_lfirst(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('-');
            }
            println!("{}{:?}", spc, bd.data);
            bd.right.print_lfirst(dp + 1);
        }
    }
}

fn main() {
    let mut t = BinaryTree::new();

    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(6);
    t.add_sorted(10);
    t.add_sorted(1);
    t.add_sorted(94);
    t.add_sorted(54);
    t.add_sorted(3);
    t.print_lfirst(0);
}
