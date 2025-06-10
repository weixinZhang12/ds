// use Vector::Vector;

use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::ds::ds::{InsertPosition, TreeNode};

mod ds {
    pub mod ds;
}
fn bool_to_position(b: bool) -> InsertPosition {
    match b {
        true => InsertPosition::Right,
        false => InsertPosition::Left,
    }
}
fn main() {
    let treenode = TreeNode::new(1);
    let mut rng = rand::rng();
    let b: bool = rng.random();
    let value = RefCell::new(treenode);
    let value = Rc::new(value);
    let mut tr = Rc::clone(&value);

    for i in 0..3 {
        match TreeNode::insert_and_get_ref(&Some(tr), rng.random(), bool_to_position(b)) {
            Some(v) => tr = v,
            None => todo!(),
        };
    }
    println!("{:#?}", value)
}
