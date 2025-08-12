use std::{cell::RefCell, rc::{Rc, Weak}};

type NodeRef = Option<Rc<RefCell<QueueNode>>>;
type NodeWeakRef = Option<Weak<RefCell<QueueNode>>>;
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
        // 当没有任何节点的时候
        if let Some(node) = self.rear.clone() {
            node.borrow_mut().next = Some(new_node.clone());
            self.rear = Some(new_node);
            self.len += 1;
        } else {
            self.front = Some(new_node.clone());
            self.rear = Some(new_node);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        // 将当前头节点取出来
        if let Some(node) = self.front.take() {
             let node_ref = node.borrow();
            let val = node_ref.val;
            // 将头节点指向下一个节点

            self.front = node_ref.next.clone();
            self.len -= 1;
            if let Some(node_rear)=&self.rear
                && Rc::ptr_eq(node_rear, &node){
                    self.rear=self.front.clone();
                }
            return Some(val);
        }
        None
    }
}
#[test]
fn _link_queue() {
    let mut queue = LinkQueue::new();
    for i in 0..5 {
        queue.push(i);
    }
    for i in 0..5 {
        let val=queue.pop();
        assert_eq!(val,Some(i));
    }
    println!("{queue:?}");
    // println!("")
}
