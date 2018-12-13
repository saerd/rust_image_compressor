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

    /*
    pub fn from_map(map : &HashMap<T, Vec<u32>>) -> Compressor<T> {
        let mut ret = HashMap::new();
        {
            let consume = map.iter().map(|(key, val)| ret.insert(key.clone(), BitString::from_nums(&val)));
            for _ in consume {}
        }
        Self::from(ret)
    }
    */


}

