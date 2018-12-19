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

use square_matrix::{SquareMatrix, SubSquare::SSquare, from_diagonal_wrap};
use point_store::PStore::PointStore;
use bits::{BitVec, BitString};
use huffman::HuffmanEncoder;
use compressor::Compressor;
use trie::Trie;
use decoder::Decoder;

use std::thread;
use std::sync::mpsc;

use std::f32::consts::PI;

use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom, Read};



fn jpg_compress(pixels : SquareMatrix<u8>) {

    let mut x = 0;
    let mut y = 0;

    let mut v = Vec::new();

    let (tx, rx) = mpsc::channel();

    while y < pixels.len() {
        while let Some(SSquare(m, _, _)) = pixels.sub(x, y, 8) {

            let ctx = mpsc::Sender::clone(&tx);

            thread::spawn(move || {
                ctx.send(PointStore(compress_matrix(&m), x, y))
            });


            x += 8;
        }
        x = 0;
        y += 8;
    }

    drop(tx);

    for ps in rx {
        v.push(ps);
    }

    v.sort();

    let mut jpg = SquareMatrix::new_with(pixels.len(), 0);

    for PointStore(d, x, y) in v.iter() {
//        println!("{:?}", d);
//        let s = from_diagonal_wrap(&d, 8);
        let r =decompress_into_matrix(&d);

        jpg.copy_sub(&SSquare(r, *x, *y));
    }

//    println!("{}", pixels);

//    println!("{}", jpg);

//    println!("{}", jpg.matrix.len());
    image::save_buffer("test.bmp", &pixels.matrix, jpg.len() as u32, jpg.len() as u32, image::Gray(8)).unwrap();

//    v

}

fn alpha(x : usize) -> f32 {
    if x == 0 {
        0.5
    }
    else {
        1.0
    }
}

static Q : [f32 ; 64] = [
    16.0, 11.0, 10.0, 16.0, 24.0, 40.0, 51.0, 61.0,
    12.0, 12.0, 14.0, 19.0, 26.0, 58.0, 60.0, 55.0,
    14.0, 13.0, 16.0, 24.0, 40.0, 57.0, 69.0, 56.0,
    14.0, 17.0, 22.0, 29.0, 51.0, 87.0, 80.0, 62.0,
    18.0, 22.0, 37.0, 56.0, 68.0, 109.0, 103.0, 77.0,
    24.0, 35.0, 55.0, 64.0, 81.0, 104.0, 113.0, 92.0,
    49.0, 64.0, 78.0, 87.0, 103.0, 121.0, 120.0, 101.0,
    72.0, 92.0, 95.0, 98.0, 112.0, 100.0, 103.0, 99.0
];

fn compress_matrix(pixels : &SquareMatrix<u8>) -> Vec<i8> {
    let v : Vec<_> = pixels.iter().map(|x| *x as f32 - 128.0).collect();
    let fpixels = SquareMatrix::from(v, pixels.len());

    let mut res = SquareMatrix::new_with(8, 0.0);

    for i in 0..8 {
        for j in 0..8 {
            let e = res.get_mut(j, i).unwrap();
            *e = dct(j, i, &fpixels);
        }
    }

    let mut q = Q.iter();

    for i in res.iter_mut() {
        *i /= q.next().unwrap();
    }

    res.diagonal_unwrap().iter().map(|x| x.round() as i8).collect()
}

fn decompress_into_matrix(bytes : &[i8]) -> SquareMatrix<u8> {

    let mut dctq = from_diagonal_wrap(bytes, 8);
    let mut q = Q.iter();

    for i in dctq.iter_mut() {
        *i *= q.next().unwrap();
    }

    let mut res = SquareMatrix::new_with(8, 0);

    for i in 0..8 {
        for j in 0..8 {
            let e = res.get_mut(j, i).unwrap();
            *e = (idct(j, i, &dctq).round() + 128.0) as u8;
        }
    }

    res
}

fn dct(u : usize, v : usize, pixels : &SquareMatrix<f32>) -> f32 {
    0.125 * pixels.iter_enum().fold(0.0, 
                 |acc, PointStore(e, x, y) | {
                        acc + (e * dct_cos(x, u) * dct_cos(y, v))
                 })
}

fn idct(u : usize, v : usize, pixels : &SquareMatrix<f32>) -> f32 {
    0.5 * pixels.iter_enum().fold(0.0, 
                 |acc, PointStore(e, x, y) | {
                        acc + (alpha(x) * alpha(y) * e * dct_cos(u, x) * dct_cos(v, y))
                 })
}

fn dct_cos(x : usize, u : usize) -> f32{
    ((((2 * x + 1) * u) as f32) * (PI / 16.0)).cos()
}

pub fn run(image : String) -> Result<(), std::io::Error> {

    let file = File::open(&image)?;

    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::Start(10))?;

    let mut buffer = [0; 4];
    reader.read(&mut buffer)?;

    let offset = unsafe {
        std::mem::transmute::<[u8; 4], u32>(buffer)
    };

    reader.seek(SeekFrom::Start(18))?;

    let mut buffer = [0; 8];
    reader.read(&mut buffer)?;

    let (height, width) = unsafe {
        std::mem::transmute::<[u8; 8], (i32, i32)>(buffer)
    };

    println!("{}, {}, {}", offset, height.abs(), width.abs());

    let mut bytes = Vec::with_capacity((height.abs() * width.abs()) as usize);

    reader.seek(SeekFrom::Start(offset as u64))?;

    println!("{}", bytes.len());

    reader.read_to_end(&mut bytes)?;

    println!("{}", bytes.len());

    jpg_compress(SquareMatrix::from(bytes, height as usize));

//    image::save_buffer("test.bmp", &bytes, height as u32, width as u32, image::Gray(8)).unwrap();

    /*

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


    let r = jpg_compress(SquareMatrix::from(v, 8));
    println!("{:?}", r);

    */

    Ok(())


}
