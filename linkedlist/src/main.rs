fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);

    println!("ll = {:?}", ll);
    // Output: ll = LinkedList(Some((1, LinkedList(Some((3, LinkedList(Some((12, LinkedList(None))))))))))

    let mut ll2 = LinkedList::new();
    ll2.sorted_insert(10);
    ll2.sorted_insert(6);
    ll2.sorted_insert(7);
    ll2.sorted_insert(24);
    ll2.sorted_insert(1);
    ll2.sorted_insert(100);
    ll2.sorted_insert(24);

    println!("ll2 = {:?}", ll2)
}

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd> LinkedList<T> {
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

    // challenge: insert sorted.
    // find the place it needs to go, and call push_front
    // Tips: implement std::cmp::PartialOrd for LinkedList
    // impl<T: PartialOrd> LinkedList<T> {}
    // 顺序插入linkedlist
    pub fn sorted_insert(&mut self, data: T) {
        match self.0 {
            Some((ref mut node, ref mut child)) => {
                if data.gt(node) {
                    child.sorted_insert(data);
                } else if data.eq(node) {
                    // not todo
                } else {
                    self.push_front(data);
                }
            }
            None => self.push_front(data),
        }
    }
}
