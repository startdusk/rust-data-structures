#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GenData {
    pub pos: usize,
    pub gen: u64,
}

pub struct EntityActive {
    active: bool,
    gen: u64,
}

// where we get new GenerationIDs from
pub struct GenManager {
    items: Vec<EntityActive>,
    drops: Vec<usize>, // listt of all dropped entities
}

impl GenManager {
    pub fn new() -> Self {
        GenManager {
            items: Vec::new(),
            drops: Vec::new(),
        }
    }

    pub fn next(&mut self) -> GenData {
        if let Some(loc) = self.drops.pop() {
            // most recent drop
            let ea = &mut self.items[loc];
            ea.active = true;
            ea.gen += 1;
            return GenData {
                pos: loc,
                gen: ea.gen,
            };
        }
        // if nothing left in drops, add on the end
        self.items.push(EntityActive {
            active: true,
            gen: 0,
        });
        GenData {
            gen: 0,
            pos: self.items.len() - 1,
        }
    }

    pub fn drop(&mut self, g: GenData) {
        if let Some(ea) = self.items.get_mut(g.pos) {
            if ea.active && ea.gen == g.gen {
                // don't drop newer items than given
                ea.active = false;
                self.drops.push(g.pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_items_drop() {
        let mut gm = GenManager::new();
        let g = gm.next(); // 第一次生成gen为0，pos也为0
        assert_eq!(g, GenData { gen: 0, pos: 0 });

        let g2 = gm.next(); // 第二次生成gen为0，pos为1
        assert_eq!(g2, GenData { gen: 0, pos: 1 });
        gm.next();
        gm.next();
        gm.drop(g2); // 把第二次生成的放回，active置为false
        let g3 = gm.next(); // 再生成就是直接上一次放回的g2，此时gen+1，pos为1
        assert_eq!(g3, GenData { gen: 1, pos: 1 })
    }
}
