use std::rc::Rc;
use self::HuffLink::*;
use std::cell::RefCell;
use std::collections::HashMap;

use std::cmp::Ordering::{Less, Greater};

type NodeMut<'a> = std::cell::RefMut<'a, HuffNode>;
type NodeRef<'a> = std::cell::Ref<'a, HuffNode>;

#[derive(Debug)]
pub struct HuffmanEncoder {

    huff_struct : Vec<Vec<Rc<RefCell<HuffNode>>>>,
    symbols : Vec<(String, f32)>
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

}

impl HuffLink {
    fn get_encoding(&self) -> String {
        let mut ret = String::new();
        match self {
            Solid(num, node) => {
                ret.push_str(&num.to_string());
                ret.push_str(&HuffmanEncoder::get(node).link.get_encoding());

            }
            Dotted(node) => {
                ret.push_str(&HuffmanEncoder::get(node).link.get_encoding());
            }
            _ => ()
        }
        ret

    }
}

enum Proc {
    Cont,
    Stop
}

impl HuffmanEncoder {
    pub fn new(symbols : &[(String, f32)]) -> HuffmanEncoder {
        HuffmanEncoder {
            huff_struct : vec![symbols.iter().map(|a| Self::new_node(a.1)).collect()],
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
           if Self::get(a).value < Self::get(b).value {
               Less
           }
           else {
               Greater
           }
        });

    }

    fn last_step(&self) -> HashMap<String, String> {
        let mut ret = HashMap::new();
        for (node, (symbol, _)) in self.huff_struct[0].iter().zip(self.symbols.iter()){
            let n = Self::get(node);
            ret.insert(symbol.clone(), n.link.get_encoding());

        }
        ret
    }

    pub fn encode(&mut self) -> HashMap<String, String>{

        self.start();
        while let Proc::Cont = self.single_step() {
            let last = self.huff_struct.last_mut().unwrap();
            last.sort_by(|a, b| {
               if Self::get(a).value < Self::get(b).value {
                   Less
               }
               else {
                   Greater
               }
            });
        }
        self.last_step()

    }

    fn single_step(&mut self) -> Proc {

    //    self.huff_struct[0][0] = Rc::new(HuffNode{value : 0.1, link : Solid(Rc::new(HuffNode::new(0.2)))};

        let mut new_vec = Vec::new();
        let index = self.huff_struct.len() - 1;

        {
            let mut nodes = self.huff_struct[index].iter_mut();
            let mut a = Self::get_mut(nodes.next().unwrap());
            
            if let Some(check) = nodes.next() {
                let mut b = Self::get_mut(check);

                let new_node = Self::new_node(a.value + b.value);
                new_vec.push(Rc::clone(&new_node));
                a.link = Solid(0, Rc::clone(&new_node));
                b.link = Solid(1, new_node);
            }
            else {
                return Proc::Stop;
            }


            for node in nodes {
                let mut n = Self::get_mut(node);
                let new_node = Self::new_node(n.value);
                new_vec.push(Rc::clone(&new_node));
                n.link = Dotted(new_node);
            }
        }

        self.huff_struct.push(new_vec);

        Proc::Cont
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

    pub fn show(&self) {
        println!("{:#?}", self.huff_struct);
        println!("{:#?}", self.symbols);


    }
}
