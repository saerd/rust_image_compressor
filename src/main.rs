mod square_matrix;
mod bits;
mod huffman;

use square_matrix::SquareMatrix;
use bits::{BitVec, BitString};
use huffman::HuffmanEncoder;

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

    println!("");

    let mut h = HuffmanEncoder::new(
        &[(String::from("a"), 0.2),
          (String::from("b"), 0.1),
          (String::from("c"), 0.25),
          (String::from("d"), 0.45),
         ]
        );
//    h.single_step(0.7);
//    h.single_step(0.8);

    let s = h.encode();
    println!("");

    h.show();
    println!("{:#?}", s);


}
