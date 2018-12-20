use std::rc::Rc;
use self::HuffLink::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

use compressor::Compressor;
use bits::BitString;

use std::cmp::Ordering::{Less, Greater};

use self::Proc::*;

type NodeMut<'a> = std::cell::RefMut<'a, HuffNode>;
type NodeRef<'a> = std::cell::Ref<'a, HuffNode>;

#[derive(Debug)]
pub struct HuffmanEncoder<T : Hash + Clone + Eq> {

    huff_struct : Vec<Vec<Rc<RefCell<HuffNode>>>>,
    symbols : Vec<(T, f32)>
}

#[derive(Debug)]
pub struct HuffNode {
    value : f32,
    link : HuffLink,
}

#[derive(Debug)]
pub enum HuffLink {
    Dotted(Rc<RefCell<HuffNode>>),
    Solid(u32, Rc<RefCell<HuffNode>>),
    Nil
}

impl HuffNode {
    pub fn new(prob : f32) -> HuffNode {
        HuffNode {
            value : prob,
            link : Nil
        }
    }

    fn new_node(prob : f32) -> Rc<RefCell<HuffNode>> {
        Rc::new(RefCell::new(HuffNode::new(prob)))
    }

    fn get(node : & Rc<RefCell<HuffNode>>) -> NodeRef {
        (**node).borrow()
    }

    fn get_mut(node : &mut Rc<RefCell<HuffNode>>) -> NodeMut {
        (**node).borrow_mut()

    }

}

impl HuffLink {
    fn get_encoding(&self) -> String {
        let mut ret = String::new();
        match self {
            Solid(num, node) => {
                ret.push_str(&num.to_string());
                ret.push_str(&HuffNode::get(node).link.get_encoding());

            }
            Dotted(node) => {
                ret.push_str(&HuffNode::get(node).link.get_encoding());
            }
            _ => (),
        }
        ret

    }
}

enum Proc {
    Cont,
    Stop
}

impl<T : Hash + Clone + Eq> HuffmanEncoder<T>{
    pub fn from(map : HashMap<T, f32>) -> HuffmanEncoder<T> {
        let mut v = Vec::new();

        for _ in map.iter().map(|(key, val)| v.push((key.clone(), *val))) {}

        HuffmanEncoder::new(&v)
    }

    pub fn new(symbols : &[(T, f32)]) -> HuffmanEncoder<T> {
        HuffmanEncoder {
            huff_struct : vec![symbols.iter().map(|a| HuffNode::new_node(a.1)).collect()],
            symbols : Vec::from(symbols)
        }
    }

    fn start(&mut self) {
        self.symbols.sort_by(|a, b| {
            if a.1 < b.1 {
                Less
            }
            else {
                Greater
            }
        });
        let last = self.huff_struct.last_mut().unwrap();
        last.sort_by(|a, b| {
           if HuffNode::get(a).value < HuffNode::get(b).value {
               Less
           }
           else {
               Greater
           }
        });

    }

    fn last_step(&self) -> HashMap<T, BitString> {
        let mut ret = HashMap::new();
        for (node, (symbol, _)) in self.huff_struct[0].iter().zip(self.symbols.iter()){
            let n = HuffNode::get(node);
            println!("{:?}", n);
            let mut s : String = n.link.get_encoding().chars().rev().collect();
            if s == "" {
                s = "0".to_string();
            }
            ret.insert(symbol.clone(), BitString::new(&s));

        }
        ret
    }

    pub fn encode(&mut self, radix : u32) -> Compressor<T>{

        self.start();
        while let Cont = self.single_step(radix) {
            let last = self.huff_struct.last_mut().unwrap();
            last.sort_by(|a, b| {
               if HuffNode::get(a).value < HuffNode::get(b).value {
                   Less
               }
               else {
                   Greater
               }
            });
        }
        Compressor::from(self.last_step())

    }

    fn single_step(&mut self, radix : u32) -> Proc {

        let mut new_vec = Vec::new();
        let index = self.huff_struct.len() - 1;

        {

            if let Stop = self.check(radix) {
                return Stop;
            }

            let mut new_node = HuffNode::new_node(0.0);
            let mut sum = 0.0;
            let mut nodes = self.huff_struct[index].iter_mut();
            
            for i in 0..radix {
                if let Some(node) = nodes.next() {
                    let mut n = HuffNode::get_mut(node);
                    n.link = Solid(i, Rc::clone(&new_node));
                    sum += n.value;


                }

            }
            {
                let mut n = HuffNode::get_mut(&mut new_node);
                n.value = sum;
            }

            new_vec.push(new_node);

            for node in nodes {
                let mut n = HuffNode::get_mut(node);
                let new_node = HuffNode::new_node(n.value);
                new_vec.push(Rc::clone(&new_node));
                n.link = Dotted(new_node);
            }
        }

        self.huff_struct.push(new_vec);

        Cont
    }

    fn check(&self, radix : u32) -> Proc {
        match self.huff_struct.last().unwrap()
            .iter().take(radix as usize).count() {

            x if x == radix as usize => Cont,
            _ => Stop

        }


    }

}
