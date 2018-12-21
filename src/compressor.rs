use std::collections::HashMap;
use std::hash::Hash;

use bits::{BitString, BitVec};

#[derive(Debug)]
pub struct Compressor<T : Hash + Clone + Eq> {
    map : HashMap<T, BitString>
}

impl<T: Hash + Clone + Eq> Compressor<T>{

    pub fn from(map : HashMap<T, BitString>) -> Compressor<T> {
        Compressor {
            map
        }
    }

    pub fn compress(&self, symbols : &[T]) -> BitVec {
        let mut ret = BitVec::new();
        for sym in symbols {
            let conversion = self.map.get(sym).unwrap();
            ret.push_bits(conversion);
        }
        ret
    }

    pub fn get_map(&self) ->  &HashMap<T, BitString> {
        &self.map
    }

}

impl<T: Hash + Clone + Eq> Compressor<T> {

    pub fn from_option(comp : Compressor<Option<T>>) -> (Option<BitString>, Compressor<T>) {

        let mut none = None;
        let mut map = HashMap::new();
        for (key, value) in comp.get_map() {
            match key {
                Some(x) => {
                    map.insert(x.clone(), *value);
                }
                None => none = Some(*value),
            }
        }
        (none, Compressor::from(map))

    }

    pub fn to_option(delimiter : BitString, comp : &Compressor<T>) -> Compressor<Option<T>> {
        let mut map = HashMap::new();
        map.insert(None, delimiter);
        for (key, value) in comp.get_map() {
            map.insert(Some(key.clone()), *value);
        }
        Compressor::from(map)

    }

}

