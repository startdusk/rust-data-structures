use std::collections::BTreeMap;

#[derive(Debug)]
pub enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>), // 树
    Leaf(char),                         // 叶子节点
}

impl HuffNode {
    pub fn print_lfirst(&self, depth: i32, dir: char) {
        match self {
            HuffNode::Tree(l, r) => {
                l.print_lfirst(depth + 1, '/');
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}*", spc, dir);
                r.print_lfirst(depth + 1, '\\')
            }
            HuffNode::Leaf(c) => {
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}{}", spc, dir, c);
            }
        }
    }

    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        // could return vec of bool but chars print nicer
        // once you have this converting it to a byte stream is pretty straight forward

        match self {
            HuffNode::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            }
            HuffNode::Leaf(nc) => {
                if c == *nc {
                    Some(Vec::new())
                } else {
                    None
                }
            }
        }
    }

    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut res = Vec::new();
        for c in s.chars() {
            let v = self.encode_char(c)?;
            res.extend(v.into_iter());
        }
        Some(res)
    }
}

pub struct HScore {
    h: HuffNode,
    s: i32,
}

pub fn build_tree(s: &str) -> HuffNode {
    let mut map = BTreeMap::new();
    for c in s.chars() {
        // if map has already add 1 else put 1
        let n = *map.get(&c).unwrap_or(&0);
        map.insert(c, n + 1);
    }

    let mut t_list: Vec<HScore> = map
        .into_iter()
        .map(|(k, s)| HScore {
            h: HuffNode::Leaf(k),
            s,
        })
        .collect();

    while t_list.len() > 1 {
        let last = t_list.len() - 1;
        for i in 0..last - 1 {
            if t_list[i].s < t_list[last - 1].s {
                t_list.swap(i, last - 1);
            }
            if t_list[last - 1].s < t_list[last].s {
                t_list.swap(last - 1, last);
            }
        }

        let a_node = t_list.pop().unwrap(); // len >= 2
        let b_node = t_list.pop().unwrap();
        let new_node = HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h));
        t_list.push(HScore {
            h: new_node,
            s: a_node.s + b_node.s,
        });
    }

    t_list.pop().unwrap().h
}
fn main() {
    let s = "at an apple app";
    println!("s = {}", s);
    let t = build_tree(s);
    t.print_lfirst(0, '<');
    // 打印结果如下
    // ../a
    // ./*
    // .../
    // ..\*
    // ...../n
    // ..../*
    // .....\l
    // ...\*
    // ...../t
    // ....\*
    // .....\e
    // <*
    // .\p
    // < 表示霍夫曼树树根
    // '\' 表示右边子树, '/' 表示左子树
    // p 出现了4次, 且频繁出现, 证明它被频繁使用, 所以位置比较靠近树根
    // 其他出现了一次的字符被认为是不常出现的, 所以离树根比较远

    println!("encode char 'n' = {:?}", t.encode_char('n'));
    println!("encode str '{}' = {:?}", s, t.encode_str(s));
}

// challenge: Write the decoder.
// it you will want to convert the input char vec into an iterator.
