mod square_matrix;
mod bits;

use square_matrix::SquareMatrix;
use bits::{BitVec, BitString};

fn main() {
    println!("Hello, world!");

    let len = 4;
    let mut m = SquareMatrix::new(len);

    let mut k = 1;

    for i in 0..len {
        for j in 0..len {
            m.set(i, j, k);
            k += 1;
        }
    }

    let r = m.diagonal_unwrap();
    println!("{:?}", r);

    let mut b = BitVec::new();
    let s = BitString::new("0010000000000101");
    println!("{:?}", s);
    b.push_bits(s);
    let s = BitString::new("0010000000000101");
    println!("{:?}", s);
    b.push_bits(s);
    println!("{:?}", b);

}
