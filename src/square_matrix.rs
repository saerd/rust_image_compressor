use self::Dir::*;
use std::fmt::{Display, Debug};

use std::cmp::Ordering;

use self::SubSquare::SSquare;

use point_store::PStore::{self, PointStore};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_testing() {
        let s = SquareMatrix::new(4);
        assert_eq!(s.index(1, 1), 5);
        assert_eq!(s.index(1, 2), 6);
        assert_eq!(s.index(2, 1), 9);
        assert_eq!(s.index(3, 2), 14);
    }
}

#[derive(Debug)]
pub struct SquareMatrix<T> {
    pub matrix : Vec<T>,
    size : usize,
}

pub struct MatrixIter<'a, T : 'a > {
    iter : std::slice::Iter<'a, T>,
    size : usize,
    cx : usize,
    cy : usize,
}

impl<'a, T : 'a> MatrixIter<'a, T> {
    fn new(m : &SquareMatrix<T>) -> MatrixIter<T> {
        MatrixIter {
            iter : m.matrix.iter(),
            size : m.len(),
            cx : 0,
            cy : 0,
        }
        
    } 
}

impl<'a, T : 'a> Iterator for MatrixIter<'a, T> {
    type Item = PStore<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => { 
                let ret = PointStore(x, self.cx, self.cy);
                self.cx = (self.cx + 1) % self.size;
                if self.cx == 0 {
                    self.cy += 1;
                }

                Some(ret)
            }
            None => None
        }

    }
    

}

impl<T : Display> Display for SquareMatrix<T> {

    fn fmt(&self, fmt : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(fmt, "{: >3} ", self.get(j, i).unwrap());
            }
            write!(fmt, "\n");
        }

        Ok(())
    }


}

#[derive(Debug)]
pub enum SubSquare<T> {
    SSquare(SquareMatrix<T>, usize, usize)
}

impl<T : PartialOrd> PartialOrd for SubSquare<T> {
    fn partial_cmp(&self, cmp : &SubSquare<T>) -> Option<Ordering> {
        let SSquare(_, x1, y1) = self;
        let SSquare(_, x2, y2) = cmp;

        if (y2 > y1) {
            Some(Ordering::Less)
        }
        else if y2 == y1 {
            x1.partial_cmp(x2)
        }
        else {
            Some(Ordering::Greater)
        }

    }

}

impl<T : Ord> Ord for SubSquare<T> {
    fn cmp(&self, cmp : &SubSquare<T>) -> Ordering {
        self.partial_cmp(cmp).unwrap()
    }
}

impl<T : Eq> Eq for SubSquare<T> {}

impl<T : PartialEq> PartialEq for SubSquare<T> {

    fn eq(&self, cmp : &SubSquare<T>) -> bool {
        true

    }

}

impl SquareMatrix<u32> {
    pub fn new(size : usize) -> SquareMatrix<u32> {
        SquareMatrix {
            matrix : vec![0; size * size],
            size
        }
    }
}

impl<T : Copy> SquareMatrix<T> {

    pub fn new_with(size : usize, of : T) -> SquareMatrix<T> {
        SquareMatrix {
            matrix : vec![of; size * size],
            size
        }
    }


    pub fn sub(&self, x : usize, y : usize, size : usize) -> Option<SubSquare<T>> {

        if(x + size > self.size || y + size > self.size) {
            None
        }
        else {
            let mut ret = Vec::new();
            for cy in y..(y + size) {
                let from = self.index(x, cy);
                ret.extend(&self.matrix[from..(from + size)]);
            }
            Some(SSquare(SquareMatrix::from(ret, size), x, y))
        }
    }

    pub fn copy_sub(&mut self, cpy : &SubSquare<T>) {

        let SSquare(m, u, v) = cpy;
        if(u + m.len() > self.len() || v + m.len() > self.len() ) {
            return;
        }

        for PointStore(c, x, y) in m.iter_enum() {
            self.set(x + u, y + v, *c); // CHECK THIS FUNCTION
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.matrix.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.matrix.iter_mut()
    }

    pub fn iter_enum(&self) -> MatrixIter<T> {
        MatrixIter::new(&self)
    }

    pub fn diagonal_unwrap(&self) -> Vec<T> {
        let len = self.size;
        let mut ret = Vec::with_capacity(len * len);

        let mut dir = Up;

        for i in 0..len {
            let add = self.diagonal_strip(i, 0, &dir);
            ret.extend(add);
            dir.switch();
        }

        for i in 1..len {
            let add = self.diagonal_strip(len - 1, i, &dir);
            ret.extend(add);
            dir.switch();
        }

        ret
    }

    pub fn diagonal_strip(&self, x : usize, y: usize, dir : &Dir) -> Vec<&T> {
        match dir {
            Up => self.unwrap_d_up(x, y),
            Down => self.unwrap_d_down(x, y)
        }
    }

    fn unwrap_d_up(&self, x: usize, y: usize) -> Vec<&T> {
        let mut offset = 0;
        let mut ret = Vec::new();
        while let Some(a) = self.get_at(x as i32 - offset, y as i32 + offset) {
            offset += 1;
            ret.push(a);
        }
        ret
    }

    fn unwrap_d_down(&self, x: usize, y: usize) -> Vec<&T> {
        let mut offset = 0;
        let mut ret = Vec::new();
        while let Some(a) = self.get_at(x as i32 - offset, y as i32 + offset) {
            offset += 1;
            ret.push(a);
        }
        ret.iter().rev().cloned().collect()

    }


}

struct RevCounter {
    curr : usize,
    rev : usize,
    switch : bool,
}

impl RevCounter {
    pub fn new(rev : usize) -> RevCounter {
        RevCounter {
            curr : 0,
            rev,
            switch : false
        }
    }

    pub fn count(&mut self) -> Option<PStore<usize>> {
        if self.switch {
            self.curr -= 1;
            if self.curr == 0 {
                None
            }
            else {
                Some(PointStore(self.curr, self.rev - self.curr, self.rev - 1))
            }
        }
        else {
            self.curr += 1;
            if self.curr == self.rev {
                self.switch = true;
            }
            Some(PointStore(self.curr, 0, self.curr - 1))
        }
    }
}

pub fn from_diagonal_wrap(nums : &[i8], size : usize) -> SquareMatrix<f32> {
    let mut ret = SquareMatrix::new_with(size, 0.0);
    let mut counter = RevCounter::new(size);
    let mut curr = 0;
    let mut cont = true;
    let mut dir = Down;
    while let Some(PointStore(length, x, y)) = counter.count() {
        if !cont { break }

        let sl  = if curr + length > nums.len() {
            cont = false;
            &nums[curr..]
        }
        else {
            &nums[curr..(curr + length)]
        };

        copy_diagonal(&mut ret, PointStore(&sl, x, y), &dir);


        dir.switch();
        curr += length;
    }
    ret
}

fn copy_diagonal(matrix : &mut SquareMatrix<f32>, points : PStore<&[i8]>, dir : &Dir){
    let PointStore(p, x, y) = points;

    let mut offset = 0;
    match dir {
        Up => {
            for i in p.iter() {
                matrix.set(x + offset, y - offset, *i as f32);
                offset += 1;
            }

        }
        Down => {
            for i in p.iter().rev() {
                matrix.set(x + offset, y - offset, *i as f32);
                offset += 1;
            }
        }
    };
}


impl<T> SquareMatrix<T> {

    pub fn from(from_vec : Vec<T>, size : usize) -> SquareMatrix<T> {
        SquareMatrix {
            matrix : from_vec,
            size
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get(&self, x : usize, y : usize) -> Option<&T> {
        self.get_at(x as i32, y as i32)

    }

    pub fn get_mut(&mut self, x : usize, y : usize) -> Option<&mut T> {
        if self.check(x as i32, y as i32) == false {
            None
        }
        else {
            let i = self.index(x, y);
            Some(&mut self.matrix[i])
        }
    }

    pub fn set(&mut self, x : usize, y : usize, new : T) -> bool{
        let v = self.get_mut(x, y);
        if let Some(c) = v {
            *c = new;
            true
        }
        else {
            false
        }
        /*
        let len = self.matrix.len();
        if x >= len || y >= len {
            return;
        }
        self.matrix[x][y] = new;
        */
    }

    fn get_at(&self, x : i32, y : i32) -> Option<&T> {
        if self.check(x, y) == false {
            None
        }
        else {
            Some(&self.matrix[self.index(x as usize, y as usize)])
        }

    }

    fn index(&self, x : usize, y : usize) -> usize {
        (y * self.size) + x
    }

    fn check(&self, x : i32, y : i32) -> bool {
        let len = self.size;
        if x >= len as i32 || y >= len as i32 || x < 0 || y < 0 {
            false
        }
        else {
            true
        }
    }
}

pub enum Dir {
    Up,
    Down
}

impl Dir {
    pub fn switch(&mut self) {
        match self {
            Up => *self = Down,
            Down => *self = Up,

        }
    }
}

