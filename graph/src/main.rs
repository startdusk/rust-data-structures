use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct GraphErr {
    msg: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr { msg: s.to_string() }
    }
}

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

// 实现graph有多种结构方式如下:
// // edgelist
// pub struct EdgeListGraph<E, ID> {
//     // Data on the edges at E don't care too much about the nodes.
//     // simple, but can be slow at traversal
//     v: Vec<(E, ID, ID)>,
// }

// // Pointer based
// // good for directed graphs as edges go one way,
// // Using Weak pointer means the edge will fail safely if node has been removed
// // can stick Edge data if needed
// pub struct RccGraph<T, E> {
//     nodes: Vec<Rcc<RccNode<T, E>>>,
// }

// pub struct RccNode<T, E> {
//     data: T,
//     edges: Vec<(E, Weak<RefCell<RccNode<T, E>>>)>,
// }

// // Map based
// // Maps point from key to value normally quickly eg HashMap
// pub struct MapGraph<T, E, ID: Hash> {
//     mp: HashMap<ID, T>,
//     edges: Vec<(E, ID, ID)>,
// }

// // Mappointer based
// pub struct MapPGraph<T, E, ID: Hash + Eq> {
//     data: HashMap<ID, (T, Vec<ID>)>,
//     edges: HashMap<ID, (E, ID, ID)>,
// }

// Mappointer based
#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, dt: T) {
        // node has no edges yet
        self.data.insert(id, (dt, Vec::new()));
    }

    pub fn add_edge(
        &mut self,
        edge_id: ID,
        from: ID,
        to: ID,
        edge_data: E,
    ) -> Result<(), GraphErr> {
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new("'from' not in nodes"));
        }

        if let Some(ref mut data) = self.data.get_mut(&to) {
            self.edges
                .insert(edge_id.clone(), (edge_data, from.clone(), to));
            data.1.push(edge_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }
        self.data.get_mut(&from).unwrap().1.push(edge_id);
        Ok(())
    }
}

fn main() -> Result<(), GraphErr> {
    let mut g = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'] {
        g.add_node(x, ());
    }
    g.add_edge('a', 'H', 'D', 7)?;
    g.add_edge('b', 'D', 'C', 8)?;
    g.add_edge('c', 'J', 'A', 9)?;
    g.add_edge('d', 'C', 'D', 10)?;
    g.add_edge('e', 'H', 'E', 11)?;
    g.add_edge('f', 'F', 'G', 12)?;
    g.add_edge('g', 'A', 'B', 13)?;
    g.add_edge('h', 'C', 'I', 14)?;
    g.add_edge('i', 'J', 'E', 15)?;
    g.add_edge('j', 'B', 'A', 16)?;
    println!("graph = {:?}", g);

    Ok(())
}
