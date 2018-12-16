use self::PStore::PointStore;

use std::cmp::Ordering;

#[derive(Debug)]
pub enum PStore<T> {
    PointStore(T, usize, usize)
}

impl<T> PartialOrd for PStore<T> {
    fn partial_cmp(&self, cmp : &PStore<T>) -> Option<Ordering> {
        let PointStore(_, x1, y1) = self;
        let PointStore(_, x2, y2) = cmp;

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

impl<T> Ord for PStore<T> {
    fn cmp(&self, cmp : &PStore<T>) -> Ordering {
        self.partial_cmp(cmp).unwrap()
    }
}

impl<T> Eq for PStore<T> {}

impl<T> PartialEq for PStore<T> {

    fn eq(&self, cmp : &PStore<T>) -> bool {
        let PointStore(_, x1, y1) = self;
        let PointStore(_, x2, y2) = cmp;
        x1 == x2 && y1 == y2
    }

}
