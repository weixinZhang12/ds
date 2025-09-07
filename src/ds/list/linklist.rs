use std::{cell::RefCell, rc::Rc};

type NodeRef = Option<Rc<RefCell<LinkNode>>>;
// 普通单向链表实现
#[derive(Debug)]
pub struct LinkNode {
    value: i32,
    next: Option<Rc<RefCell<LinkNode>>>,
}

impl LinkNode {
    ///新建一个linknode
    pub fn new(value: i32) -> Self {
        Self { value, next: None }
    }
    pub fn insert(node: &mut NodeRef, value: i32) {
        // 如果当前节点有值，那么就进入下一个节点
        match node {
            Some(v) => {
                LinkNode::insert(&mut v.borrow_mut().next, value);
            }
            // 如果运行到这里节点已经没有值了
            None => {
                 let new_node = LinkNode::new(value);
                *node = Some(Rc::new(RefCell::new(new_node)))
            }
        }
    }
    ///在头部插入
    pub fn insert_to_front(node: &mut NodeRef, value: i32) -> Rc<RefCell<LinkNode>> {
        match node {
            Some(v) => {
                let new_node = LinkNode::new(value);
                let new_node = Rc::new(RefCell::new(new_node));
                // 让new_node指向传入的node节点
                new_node.borrow_mut().next = Some(Rc::clone(v));
                new_node
            }
            None => {
                let new_node = LinkNode::new(value);
                Rc::new(RefCell::new(new_node))
            }
        }
    }
}
#[test]
pub fn linklist_test() {
    let node = LinkNode::new(0);
    let mut node = Rc::new(RefCell::new(node));
    for i in 1..5 {
        node = LinkNode::insert_to_front(&mut Some(Rc::clone(&node)), i);
    }
    println!("{node:#?}")
}
