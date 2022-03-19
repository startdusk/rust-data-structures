use crate::gen::GenData;

// This could be implemented by Vec type object,
// or tree or hashmap, depending onhow full you
// expect it to be
pub trait EcsStore<T> {
    fn add(&mut self, f: GenData, t: T);
    fn get(&self, g: GenData) -> Option<&T>;
    fn get_mut(&mut self, g: GenData) -> Option<&mut T>;
    fn drop(&mut self, g: GenData);
    // Optional but helpful could be another trait even
    //
    fn for_each<F: FnMut(GenData, &T)>(&self, f: F);
    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, f: F);
}

pub struct VecStore<T> {
    items: Vec<Option<(u64, T)>>,
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        VecStore { items: Vec::new() }
    }
}

impl<T> EcsStore<T> for VecStore<T> {
    fn add(&mut self, g: GenData, t: T) {
        while g.pos >= self.items.len() {
            self.items.push(None);
        }
        self.items[g.pos] = Some((g.gen, t));
    }

    fn get(&self, g: GenData) -> Option<&T> {
        // get returns option, vec holding options
        if let Some(Some((ig, d))) = self.items.get(g.pos) {
            if *ig == g.gen {
                return Some(d);
            }
        }
        None
    }

    fn get_mut(&mut self, g: GenData) -> Option<&mut T> {
        // get returns option, vec holding options
        if let Some(Some((ig, d))) = self.items.get_mut(g.pos) {
            if *ig == g.gen {
                return Some(d);
            }
        }
        None
    }

    fn drop(&mut self, g: GenData) {
        if let Some(Some((ig, _))) = self.items.get(g.pos) {
            if *ig == g.gen {
                self.items[g.pos] = None
            }
        }
    }

    fn for_each<F: FnMut(GenData, &T)>(&self, mut f: F) {
        for (n, x) in self.items.iter().enumerate() {
            if let Some((g, d)) = x {
                f(GenData { gen: *g, pos: n }, d);
            }
        }
    }

    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut f: F) {
        for (n, x) in self.items.iter_mut().enumerate() {
            if let Some((g, d)) = x {
                f(GenData { gen: *g, pos: n }, d);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen::GenManager;

    #[test]
    fn test_store_can_drop() {
        let mut gm = GenManager::new();
        let mut vs = VecStore::new();

        vs.add(gm.next(), 5);
        vs.add(gm.next(), 3);
        vs.add(gm.next(), 2);

        let g4 = gm.next();
        vs.add(g4, 5);

        vs.for_each_mut(|_, d| *d += 2);
        assert_eq!(vs.get(g4), Some(&7));

        vs.drop(g4);
        assert_eq!(vs.get(g4), None);
    }
}
