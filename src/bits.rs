use self::BitResult::{All, Cut};
use std::u32::MAX;

#[derive(Debug)]
pub struct BitVec {
    bits : Vec<u32>,
    offset : u32,

}

#[derive(Debug)]
pub struct BitString {
    bits : u32,
    size : u32,
}

pub enum BitResult {
    All(u32),
    Cut(u32, u32)
}

impl BitString {
    pub fn new(bit_string : &str, size : u32) -> BitString {
        BitString {
            bits : u32::from_str_radix(&bit_string, 2).unwrap(),
            size
        }
    }

    pub fn from(num : u32, size : u32) -> BitString {
        BitString {
            bits : num,
            size
        }
    }

    pub fn leading_zeros(&self) -> u32 {
        32 - self.size
    }

    pub fn offset(&self, off : u32) -> BitResult {
        let shift : i32 = off as i32 - self.leading_zeros() as i32;
        match shift {
            0 => All(self.bits),
            x if x > 0 => Cut(self.bits >> shift, (MAX >> (32 - shift)) & self.bits),
            _ => All(self.bits << -shift)
        }
    }

}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {
            bits : Vec::new(),
            offset : 0
        }
    }

    pub fn push_bits(&mut self, bits : BitString) {
        let mut add_extra = None;
        if let Some(x) = self.bits.last_mut() {
            let method = bits.offset(self.offset);
            self.offset = (self.offset + bits.size) % 32;
            match method {
                All(mask) => {
                    *x |= mask;
                    return;
                }
                Cut(mask, extra) => {
                    *x |= mask;
                    add_extra = Some(BitString::from(extra, self.offset));
                }
            }
        }
        if let Some(extra) = add_extra {
            if let All(add) = extra.offset(0) {
                self.bits.push(add);
            }
            return;
        }
        if let All(add) = bits.offset(0) {
            self.bits.push(add);
            self.offset = bits.size;

        }
    }

    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.bits.iter()
    }

}
