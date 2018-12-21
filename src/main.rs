#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]

extern crate jpg;

use jpg::run;

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

use std::env;

fn main() {

    let image = match parse_args(env::args()) {
        Ok(image) => image,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    match run(image) {
        Ok(()) => (),
        Err(err) => eprintln!("{:?}", err),
    }
}

fn parse_args(mut args : env::Args) -> Result<String, &'static str> {

    args.next();

    match args.next() {
        Some(arg) => Ok(arg),
        None => Err("No image provided")
    }
}


/*
fn main() {
    println!("Hello, world!");

    let len = 4;
    let mut m : SquareMatrix<u8> = SquareMatrix::new_with(len, 0);

    let mut k = 1;

    for i in 0..len {
        for j in 0..len {
            m.set(j, i, k);
            k += 1;
        }
    }

    /*
    for i in 0..len {
        for j in 0..len {
            println!("{:?}", m.get(i, j));
        }
    }
    */

    let r = m;
    //let r = m.diagonal_unwrap();
    //let r : Vec<_> = r.iter().map(|b| *b as f32).collect();
    //let r = SquareMatrix::from(r, 4);
    println!("{:?}", r);
    let sr1 = r.sub(0, 0, 2).unwrap();
    let sr2 = r.sub(2, 0, 2).unwrap();
    let sr3 = r.sub(0, 2, 2).unwrap();
    let sr4 = r.sub(2, 2, 2).unwrap();

    let (tx, rx) = mpsc::channel();

    let mut v = Vec::new();
    let mut m = vec![sr1, sr2, sr3, sr4];

    while let Some(sr) = m.pop() {
        let ctx = mpsc::Sender::clone(&tx);

        let SSquare(m, x, y) = sr;
        v.push(thread::spawn(move || {
            ctx.send(PointStore(m, x, y))
        }));

    }

    drop(tx);

    let mut r = Vec::new();
    for rm in rx {
        r.push(rm);
    }
    r.sort();

    println!("{:?}", r);


    //th.join();

/*
    let mut b = BitVec::new();
    let s = BitString::new("0010000000000101");
    println!("{:?}", s);
    b.push_bits(&s);
    let s = BitString::new("0010000000000101");
    println!("{:?}", s);
    b.push_bits(&s);
    println!("{:?}", b);

    println!("");

    let mut h = HuffmanEncoder::new(
        &[(String::from("a"), 0.2),
          (String::from("b"), 0.1),
          (String::from("c"), 0.25),
          (String::from("d"), 0.45),
         ]
        );

    let s = h.encode(2);
    println!("");

//    println!("{:#?}", h);
 //   println!("{:#?}", s);

    */
    let mut h = HuffmanEncoder::new(
        &[('a', 0.2),
          ('s', 0.1),
          ('e', 0.25),
          ('r', 0.45),
         ]
        );

    let c = h.encode(2);
    println!("");

    //println!("{:#?}", h);

    let sentence = "sssaersssersaersaerssssssssssssssr";
    let chars : Vec<_> = sentence.chars().collect();
    let v = c.compress(&chars);
    println!("{:?}", sentence);
    println!("{}", v);

    let iter = v.iter();
    for i in iter {
        print!("{}", i);
    }
    println!("\n");

    let mut t = Trie::new();
    t.insert(&[1, 1, 0], 's');
    t.insert(&[1, 0, 0], 'a');
    t.insert(&[0], 'r');
    t.insert(&[0, 1], 'e');
    println!("{:#?}", t);

    test_trie(&t);


    let d = Decoder::new(&c);
    let r = d.decode(&v);
    println!("{:#?}", d);
    println!("{}", sentence);
    println!("{:?}", r);

//    test_file();

}

fn test_trie(t : &Trie<char>) {
    let mut s = t;
    for i in &[1, 1, 0, 1, 0] {
        match s.get_node(*i) {
            Some(n) => s = n,
            None => break,
        }
    }

    println!("{:#?}", s);
}

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn test_file() {

    let len = 56;
    let mut v : Vec<u8> = Vec::new();
    v.extend(vec![25 ; len * (len / 2)]);
    v.extend(vec![50 ; len * (len / 2)]);

    image::save_buffer("image.bmp", &v, len as u32, len as u32, image::Gray(8)).unwrap();

    let file = File::open("image.bmp").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1078];
    reader.read(&mut buffer);
    let mut buffer1 = [0 ; 56];

    loop {
        match reader.read(&mut buffer1) {
            Ok(n) if n == 0 => break,
            Ok(n) => println!("{:?}", &buffer1[0..n]),
            Err(_) => panic!("Something went wrong"),
        }

    }
    /*
    let img = image::open("/home/saer/windows/Downloads/dark_8.bmp").unwrap();
    for p in img.pixels() {
        println!("{:?}", p);

//        let s : () = p;
    }
    */



}
*/

/*
fn main() {
    run();

}
*/

