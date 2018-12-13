use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie<T>{
    pub value : Option<T>,
    nodes : HashMap<u32, Trie<T>>,
}

impl<T> Trie<T> {

    pub fn new() -> Trie<T> {
        Trie {
            value : None,
            nodes : HashMap::new(),
        }
    }

    pub fn insert(&mut self, mapping : &[u32], value : T) {
        match mapping {
            [] => self.value = Some(value),
            cons => {
                let mut s = self.nodes.entry(cons[0]).or_insert(Trie::new());
                s.insert(&mapping[1..], value);
            }
        }
    }

    pub fn get_mapping(&self, mapping : &[u32]) -> Option<&T> {
        match mapping {
            [] => match self.value {
                Some(ref x) => Some(x),
                _ => None
            }
            cons => {
                match self.nodes.get(&cons[0]) {
                    Some(node) => node.get_mapping(&mapping[1..]),
                    _ => None,
                }
            }

        }
    }

    pub fn get_node(&self, mapping : u32) -> Option<&Trie<T>> {
        self.nodes.get(&mapping)
    }
}

