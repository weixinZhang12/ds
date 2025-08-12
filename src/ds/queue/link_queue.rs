use std::{cell::RefCell, rc::Rc};

type NodeRef = Option<Rc<RefCell<QueueNode>>>;
#[derive(Debug)]
pub struct QueueNode {
    val: i32,
    next: NodeRef,
}
impl QueueNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}
#[derive(Debug)]

pub struct LinkQueue {
    front: NodeRef,
    rear: NodeRef,
    len: usize,
}

impl LinkQueue {
    pub fn new() -> Self {
        Self {
            front: None,
            rear: None,
            len: 0,
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(QueueNode::new(val)));
        let mut current_node = self.rear.clone();
        // 当没有任何节点的时候
        if let Some(node) = self.rear.clone() {
            node.borrow_mut().next = Some(new_node.clone());
            self.rear = Some(new_node);
            self.len+=1;
        } else {
            self.front = Some(new_node.clone());
            self.rear = Some(new_node);
            self.len+=1;
        }
    }
}
#[test]
fn _link_queue() {
    let mut queue = LinkQueue::new();
    for i in 0..5 {
        queue.push(i);
    }
    println!("{queue:?}")
}
