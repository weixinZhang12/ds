use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
#[allow(dead_code)]
type NodeRef = Option<Rc<RefCell<DoubleNode>>>;
type PeekRef = Option<Weak<RefCell<DoubleNode>>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DoubleNode {
    value: i32,
    next: NodeRef,
    peek: PeekRef,
}

impl DoubleNode {

#[allow(dead_code)]
    pub fn new(value: i32) -> Self {
        Self {
            value,
            next: None,
            peek: None,
        }
    }
    pub fn insert(node: &mut NodeRef, value: i32, peek: &PeekRef)->Rc<RefCell<DoubleNode>>{
        match node {
            // 如果当前节点有值，那么查找下一个节点
            Some(v) => {
                let mut  current=v.borrow_mut();
                let new=DoubleNode::insert(&mut current.next, value,&Some(Rc::downgrade(v)));
                return new;
            }
            None => {
                // 新建一个节点
                let mut new_node = DoubleNode::new(value);
                // 如果上一个节点有值，那么新节点指向上一个节点
                if let Some(v) = peek {
                    new_node.peek = Some(Weak::clone(v));
                }
                let temp=Rc::new(RefCell::new(new_node));
                *node=Some(Rc::clone(&temp));
                return temp;
            }
        }
    }
}
#[test]
pub fn insert() {
    let mut head: NodeRef = None;
    let mut last: PeekRef = None;
    for i in 0..5 {
        let new_node = DoubleNode::insert(&mut head, i, &last);
        last = Some(Rc::downgrade(&new_node));
    }
    println!("{:#?}", head);
}
