// Immutable outside, but can mutable interior.
use std::cell::RefCell;
// Reference Counting pointer
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DoublyNode<T> {
    data: T,
    next: Option<Rc<RefCell<DoublyNode<T>>>>,
    prev: Option<Weak<RefCell<DoublyNode<T>>>>,
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<DoublyNode<T>>>>,
    last: Option<Weak<RefCell<DoublyNode<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                // create new front object.
                let new_front = Rc::new(RefCell::new(DoublyNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // tell the first object this is now in front of it
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // put this on the front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DoublyNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // create new back object.
                let new_back = Rc::new(RefCell::new(DoublyNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                // tell the last object this is now in behind it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DoublyNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    // pub fn pop_front
    // pub fn pop_back
}

fn main() {
    let mut dl = DoublyLinkedList::new();
    dl.push_front(6);
    dl.push_back(11);
    dl.push_front(5);
    dl.push_back(15);
    dl.push_front(4);
    println!("dl = {:?}", dl)
}
