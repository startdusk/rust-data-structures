fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);

    println!("ll = {:?}", ll)
    // Output: ll = LinkedList(Some((1, LinkedList(Some((3, LinkedList(Some((12, LinkedList(None))))))))))
}

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            // Some 里面的 ref mut child 等价于 &mut child, 但在Some中只能用 ref mut
            // 相当于递归到最后一个child
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

// challenge: insert sorted.
// find the place it needs to go, and call push_front
// Tips: implement std::cmp::PartialOrd for LinkedList
// impl<T: PartialOrd> LinkedList<T> {}
