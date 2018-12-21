use self::BitResult::{All, Cut};
use std::u32::MAX;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct BitVec {
    bits : Vec<u32>,
    offset : u32,

}

#[derive(Debug, Copy, Clone)]
pub struct BitString {
    bits : u32,
    size : u32,
}

pub enum BitResult {
    All(u32),
    Cut(u32, u32)
}

impl BitString {
    pub fn new(bit_string : &str) -> BitString {
        BitString {
            bits : u32::from_str_radix(&bit_string, 2).unwrap(),
            size : bit_string.len() as u32
        }
    }

    pub fn from(num : u32, size : u32) -> BitString {
        BitString {
            bits : num,
            size
        }
    }

    pub fn nums(&self) -> Vec<u32> {
        let mut vec = Vec::new();
        for i in (0..self.size).rev() {
            match self.bits & (1u32 << i) {
                0 => vec.push(0),
                _ => vec.push(1),
            }
        }
        vec
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

impl Display for BitVec {
    fn fmt(&self, fd : &mut fmt::Formatter) -> fmt::Result {
        write!(fd, "{}\n", self.offset);
        for i in &self.bits {
            write!(fd, "{:#034b}\n", i);
        }
        Ok(())
    }
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {
            bits : Vec::new(),
            offset : 0
        }
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn push_bitvec(&mut self, bits : &BitVec) {
        let bv = BitVec::new();
        for (i, &b) in bits.bits.iter().enumerate() {
            if(i == bits.bits.len() - 1 && bits.offset != 0) {
                self.push_bits(&BitString::from(b >> (32 - bits.offset), bits.offset));
            }
            else {
                self.push_bits(&BitString::from(b, 32));
            };
        }
    }

    pub fn push_bits(&mut self, bits : &BitString) {
        let mut add_extra = None;
        if let Some(x) = self.bits.last_mut() {
            let new_pos = self.offset;
            self.offset = (new_pos + bits.size) % 32;

            if new_pos == 0 {
                add_extra = Some(*bits);
            }
            else {

                let method = bits.offset(new_pos);
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

    pub fn iter(&self) -> BitVecIter {
        BitVecIter {
            bits : &self.bits,
            offset : self.offset,
            curr : 0,
            curr_index: 0
        }

    }
}

pub struct BitVecIter<'a> {
    bits : &'a Vec<u32>,
    offset : u32,
    curr : u32,
    curr_index : u32,
}

impl<'a> Iterator for BitVecIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.bits.get(self.curr_index as usize) {
            None => return None,
            Some(x) => {
                if self.curr_index == (self.bits.len() - 1) as u32 && self.curr >= self.offset {
                    return None;
                }
                match x & (1u32 << (31 - self.curr)) {
                    0 => Some(0),
                    _ => Some(1),
                }
            }
        };
        self.curr = (self.curr + 1) % 32;
        match self.curr {
            0 => self.curr_index += 1,
            _ => ()
                
        }
        ret
    }
}
