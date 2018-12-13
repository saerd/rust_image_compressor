use compressor::Compressor;
use trie::Trie;
use bits::{BitVec, BitVecIter};

use std::hash::Hash;

#[derive(Debug)]
pub struct Decoder<T : Hash + Clone + Eq> {
    trie : Trie<T>
}

impl<T : Hash + Clone + Eq> Decoder<T> {

    pub fn new(map : &Compressor<T>) -> Decoder<T> {
        let mut trie = Trie::new();
        for (key, val) in map.get_map().iter() {
            trie.insert(&val.nums(), key.clone());
        }
        Decoder {
            trie
        }
    }

    pub fn decode(&self, bits : &BitVec) -> Vec<T> {
        let mut ret = Vec::new();
        let mut each_bit = bits.iter();
        while let Some(add) = self.next_symbol(&mut each_bit) {
            ret.push(add);
        }

        ret
    }
    
    fn next_symbol(&self, bits : &mut BitVecIter) -> Option<T> {
        let mut trie = &self.trie;
        while let Some(i) = bits.next() {
            match trie.get_node(i) {
                Some(n) => {
                    trie = n;
                    match trie.value {
                        Some(ref x) => return Some(x.clone()),
                        None => (),
                    }
                }
                None => return trie.value.clone()
            }

        }
            /*
            Some(n) => s = n,
            None => break,
            */
  //      }
        None
    }
}
