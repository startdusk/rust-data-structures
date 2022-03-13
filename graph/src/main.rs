use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

// edgelist
pub struct EdgeListGraph<E, ID> {
    // Data on the edges at E don't care too much about the nodes.
    // simple, but can be slow at traversal
    v: Vec<(E, ID, ID)>,
}

// Pointer based
// good for directed graphs as edges go one way,
// Using Weak pointer means the edge will fail safely if node has been removed
// can stick Edge data if needed
pub struct RccGraph<T, E> {
    nodes: Vec<Rcc<RccNode<T, E>>>,
}

pub struct RccNode<T, E> {
    data: T,
    edges: Vec<(E, Weak<RefCell<RccNode<T, E>>>)>,
}

// Map based
// Maps point from key to value normally quickly eg HashMap
pub struct MapGraph<T, E, ID: Hash> {
    mp: HashMap<ID, T>,
    edges: Vec<(E, ID, ID)>,
}

// Mappointer based
pub struct MapPGraph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

fn main() {
    println!("Hello, world!");
}
