use self::Dir::*;

pub struct SquareMatrix {
    matrix : Vec<Vec<u32>>
}

impl SquareMatrix {
    pub fn new(size : usize) -> SquareMatrix {
        SquareMatrix {
            matrix : vec![vec![0; size]; size]
        }
    }

    pub fn get(&self, x : i32, y : i32) -> Option<u32> {
        let len = self.matrix.len();
        if x >= len as i32 || y >= len as i32 || x < 0 || y < 0 {
            None
        }
        else {
            Some(self.matrix[x as usize][y as usize])
        }
    }

    pub fn set(&mut self, x : usize, y : usize, new : u32) {
        let len = self.matrix.len();
        if x >= len || y >= len {
            return;
        }
        self.matrix[x][y] = new;
    }

    pub fn diagonal_unwrap(&self) -> Vec<u32> {
        let len = self.matrix.len();
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

    pub fn diagonal_strip(&self, x : usize, y: usize, dir : &Dir) -> Vec<u32> {
        match dir {
            Up => self.unwrap_d_up(x, y),
            Down => self.unwrap_d_down(x, y)
        }
    }

    fn unwrap_d_up(&self, x: usize, y: usize) -> Vec<u32> {
        let mut offset = 0;
        let mut ret = Vec::new();
        while let Some(a) = self.get(x as i32 - offset, y as i32 + offset) {
            offset += 1;
            ret.push(a);
        }
        ret

    }

    fn unwrap_d_down(&self, x: usize, y: usize) -> Vec<u32> {
        let mut offset = 0;
        let mut ret = Vec::new();
        while let Some(a) = self.get(x as i32 - offset, y as i32 + offset) {
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
