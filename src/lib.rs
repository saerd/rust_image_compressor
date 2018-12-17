#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]

mod square_matrix;
mod bits;
mod huffman;
mod compressor;
mod trie;
mod decoder;
mod point_store;

use square_matrix::{SquareMatrix, SubSquare::SSquare};
use point_store::PStore::PointStore;
use bits::{BitVec, BitString};
use huffman::HuffmanEncoder;
use compressor::Compressor;
use trie::Trie;
use decoder::Decoder;

use std::thread;
use std::sync::mpsc;

use std::f32::consts::PI;

fn jpg_compress(pixels : &SquareMatrix<u8>) -> Vec<i8> {
    let v : Vec<_> = pixels.iter().map(|x| *x as f32 - 128.0).collect();
    let fpixels = SquareMatrix::from(v, pixels.len());

    let mut res = SquareMatrix::new_with(8, 0.0);

    for i in 0..8 {
        for j in 0..8 {
            let e = res.get_mut(j, i).unwrap();
            *e = dct(j, i, &fpixels);
        }
    }

    let mut q = [
        16.0, 11.0, 10.0, 16.0, 24.0, 40.0, 51.0, 61.0,
        12.0, 12.0, 14.0, 19.0, 26.0, 58.0, 60.0, 55.0,
        14.0, 13.0, 16.0, 24.0, 40.0, 57.0, 69.0, 56.0,
        14.0, 17.0, 22.0, 29.0, 51.0, 87.0, 80.0, 62.0,
        18.0, 22.0, 37.0, 56.0, 68.0, 109.0, 103.0, 77.0,
        24.0, 35.0, 55.0, 64.0, 81.0, 104.0, 113.0, 92.0,
        49.0, 64.0, 78.0, 87.0, 103.0, 121.0, 120.0, 101.0,
        72.0, 92.0, 95.0, 98.0, 112.0, 100.0, 103.0, 99.0
    ].iter();

    for i in res.iter_mut() {
        *i /= q.next().unwrap();
    }

    res.diagonal_unwrap().iter().map(|x| x.round() as i8).collect()
}

fn dct(u : usize, v : usize, pixels : &SquareMatrix<f32>) -> f32 {
    0.125 * pixels.iter_enum().fold(0.0, 
                 |acc, PointStore(e, x, y) | {
                        acc + (e * dct_cos(x, u) * dct_cos(y, v))
                 })
}

fn dct_cos(x : usize, u : usize) -> f32{
    ((((2 * x + 1) * u) as f32) * (PI / 16.0)).cos()
}

pub fn run() {
    let v = vec![
    
        52, 55, 61, 66, 70, 61, 64, 73,
        63, 59, 55, 90, 109, 85, 69, 72,
        62, 59, 68, 113, 144, 104, 66, 73,
        63, 58, 71, 122, 154, 106, 70, 69,
        67, 61, 68, 104, 126, 88, 68, 70,
        79, 65, 60, 70, 77, 68, 58, 75,
        85, 71, 64, 59, 55, 61, 65, 83,
        87, 79, 69, 68, 65, 76, 78, 94
    ];
//    let v = vec![0; 64];


    let r = jpg_compress(&SquareMatrix::from(v, 8));
    println!("{:?}", r);


}
