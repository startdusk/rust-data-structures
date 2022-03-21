use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};
use std::hash::Hash;
use std::rc::Rc;

use rand::prelude::SliceRandom;

#[derive(Debug)]
#[warn(dead_code)]
pub struct GraphErr(String);

pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

impl<ID: Eq> Route<ID> {
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }

    pub fn contains(&self, id: &ID) -> bool {
        if self.pos == *id {
            return true;
        }
        match self.path {
            Some(ref p) => p.contains(id),
            None => false,
        }
    }
}

impl<ID: fmt::Debug> fmt::Display for Route<ID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref p) = self.path {
            write!(f, "{}-{}-", p, self.len)?;
        }
        write!(f, "{:?}", self.pos)
    }
}

impl<T, E: Weighted, ID: Clone + Hash + Eq + Debug> Graph<T, E, ID> {
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        self.shortest_path_r(Route::start_rc(from), to)
    }
    pub fn shortest_path_r(&self, from: Rc<Route<ID>>, to: ID) -> Option<Rc<Route<ID>>> {
        let mut toset = HashSet::new();
        toset.insert(to);
        self.closest(from, &toset)
    }
    pub fn closest(&self, from: Rc<Route<ID>>, to: &HashSet<ID>) -> Option<Rc<Route<ID>>> {
        let mut visited = HashSet::new();
        let mut routes = Vec::new();
        routes.push(from);
        loop {
            let c_route = routes.pop()?;
            if to.contains(&c_route.pos) {
                return Some(c_route);
            }
            if visited.contains(&c_route.pos) {
                // no point in searching from the same place twice
                continue;
            }
            visited.insert(c_route.pos.clone());

            let exits = self.data.get(&c_route.pos)?;
            for eid in &exits.1 {
                let edge = self.edges.get(eid)?;
                let npos = if edge.1 == c_route.pos {
                    // opposite side of edge to current pos
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let nlen = c_route.len + edge.0.weight();
                let nroute = Rc::new(Route {
                    pos: npos,
                    len: nlen,
                    path: Some(c_route.clone()), // RC increase
                });
                if routes.len() == 0 {
                    routes.push(nroute);
                    continue;
                }
                // insert into the list sorted
                let mut iafter = routes.len() - 1;
                loop {
                    if routes[iafter].len > nlen {
                        // lowest element last
                        routes.insert(iafter + 1, nroute);
                        break;
                    }
                    if iafter == 0 {
                        // reached end
                        routes.insert(0, nroute);
                        break;
                    }
                    iafter -= 1;
                }
            }
        }
    }
    // greedy 贪心算法
    pub fn greedy_salesman(&self, start: ID) -> Option<Rc<Route<ID>>> {
        let mut to_visit: HashSet<ID> = self.data.keys().map(|k| k.clone()).collect();
        to_visit.remove(&start);
        let mut route = Route::start_rc(start.clone());
        while to_visit.len() > 0 {
            route = self.closest(route, &to_visit)?;
            to_visit.remove(&route.pos);
        }
        self.shortest_path_r(route, start) // returns option like saleman
    }

    pub fn complete_path(&self, path: &[ID]) -> Option<Rc<Route<ID>>> {
        if path.len() < 2 {
            return None;
        }

        let mut route = Route::start_rc(path[0].clone());
        for pos in &path[1..path.len() - 1] {
            if !route.contains(pos) {
                route = self.shortest_path_r(route, pos.clone())?;
            }
        }
        self.shortest_path_r(route, path[path.len() - 1].clone())
    }
}

impl<T, E: Weighted, ID: Clone + Hash + Eq + Debug> Graph<T, E, ID> {
    pub fn iter_salesman(&self, start: ID) -> Option<Rc<Route<ID>>> {
        let mut bpath: Vec<ID> = self.data.keys().map(|k| k.clone()).collect();
        bpath.shuffle(&mut rand::thread_rng());
        // move start to front
        for n in 0..bpath.len() {
            if bpath[n] == start {
                bpath.swap(0, n);
                break;
            };
        }
        bpath.push(start); // start and finish

        let mut broute = self.complete_path(&bpath)?;
        let mut no_imp = 0;
        loop {
            let mut p2 = bpath.clone();
            let sa = (rand::random::<usize>() % (p2.len() - 2)) + 1; // not the ends
            let sb = (rand::random::<usize>() % (p2.len() - 2)) + 1; // not the ends
            p2.swap(sa, sb);
            let r2 = self.complete_path(&p2)?;
            if r2.len < broute.len {
                println!("Improvement on {} = \n{}", broute, r2);
                bpath = p2;
                broute = r2;
                no_imp = 0;
            }
            no_imp += 1;
            if no_imp >= 50 {
                return Some(broute);
            }
        }
    }
}

#[derive(Debug)]
pub struct Route<ID> {
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr(s.to_string())
    }
}

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

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
    // node 不能大于 edge
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, ());
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;
    println!("graph = {:?}", g);

    println!("shortest path A-D = {}", g.shortest_path('A', 'D').unwrap());
    println!("shortest path H-B = {}", g.shortest_path('H', 'B').unwrap());
    println!("greedy_salesman A = {}", g.greedy_salesman('A').unwrap());
    println!("iter_salesman A = {}", g.iter_salesman('A').unwrap());

    Ok(())
}
