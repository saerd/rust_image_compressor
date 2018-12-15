use self::Dir::*;

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

pub struct SquareMatrix {
    matrix : Vec<u32>,
    size : usize,
}

impl SquareMatrix {
    pub fn new(size : usize) -> SquareMatrix {
        SquareMatrix {
            matrix : vec![0; size * size],
            size
        }
    }

    pub fn get(&self, x : usize, y : usize) -> Option<&u32> {
        self.get_at(x as i32, y as i32)

    }

    pub fn get_mut(&mut self, x : usize, y : usize) -> Option<&mut u32> {
        if self.check(x as i32, y as i32) == false {
            None
        }
        else {
            let i = self.index(x, y);
            Some(&mut self.matrix[i])
        }
    }

    pub fn set(&mut self, x : usize, y : usize, new : u32) {
        let v = self.get_mut(x, y);
        if let Some(c) = v {
            *c = new;
        }
        /*
        let len = self.matrix.len();
        if x >= len || y >= len {
            return;
        }
        self.matrix[x][y] = new;
        */
    }

    fn get_at(&self, x : i32, y : i32) -> Option<&u32> {
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

    pub fn diagonal_unwrap(&self) -> Vec<&u32> {
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

    pub fn diagonal_strip(&self, x : usize, y: usize, dir : &Dir) -> Vec<&u32> {
        match dir {
            Up => self.unwrap_d_up(x, y),
            Down => self.unwrap_d_down(x, y)
        }
    }

    fn unwrap_d_up(&self, x: usize, y: usize) -> Vec<&u32> {
        let mut offset = 0;
        let mut ret = Vec::new();
        while let Some(a) = self.get_at(x as i32 - offset, y as i32 + offset) {
            offset += 1;
            ret.push(a);
        }
        ret

    }

    fn unwrap_d_down(&self, x: usize, y: usize) -> Vec<&u32> {
        let mut offset = 0;
        let mut ret = Vec::new();
        while let Some(a) = self.get_at(x as i32 - offset, y as i32 + offset) {
            offset += 1;
            ret.insert(0, a);
        }
        ret

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
