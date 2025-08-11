use std::{cell::RefCell, rc::Rc};
pub type StackNodeRef = Option<Rc<RefCell<LinkStackNode>>>;
#[derive(Debug)]
pub struct LinkStackNode {
    val: i32,
    next: StackNodeRef,
}
#[derive(Debug)]

pub struct LinkStack {
    node: StackNodeRef,
    len: usize,
}
impl LinkStackNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

impl LinkStack {
    pub fn new() -> Self {
        Self { node: None, len: 0 }
    }
    pub fn pop(&mut self) -> Option<i32> {
        // 如果栈内没有节点
        if self.node.is_none() {
            return None;
        }
        // 如果栈内只有一个节点

        if let Some(node) = self.node.clone() {
            if node.borrow().next.is_none() {
                self.node.take();
                self.len -= 1;
                return Some(node.borrow().val);
            }
        }
        // 以下部分代表栈内至少有一个节点
        // 当前节点设置为头节点
        let mut current_node = None;

        current_node = self.node.as_ref().and_then(|n| Some(n.clone()));

        // 如果当前节点还有节点那么继续
        while let Some(current) = current_node {
            let mut current_mut = current.borrow_mut();
            if let Some(current_next_node) = current_mut.next.clone() {
                let cur_next_ref = current_next_node.borrow();
                if cur_next_ref.next.is_none() {
                    let val = cur_next_ref.val;
                    self.len -= 1;
                    let next_node = current_mut.next.take();
                    return Some(val);
                }
            }
            current_node = current_mut.next.clone();
        }
        None
    }
    pub fn first(&self) -> Option<Rc<RefCell<LinkStackNode>>> {
        if let Some(v) = &self.node {
            return Some(v.clone());
        }
        None
    }
    pub fn push(&mut self, val: i32) {
        let new_node: Rc<RefCell<LinkStackNode>> = Rc::new(RefCell::new(LinkStackNode::new(val)));
        let mut current_node = None;
        // 头节点没有,直接添加节点到头节点
        if let Some(v) = &self.node {
            current_node = Some(v.clone())
        } else {
            self.node = Some(new_node);
            self.len += 1;
            return;
        }
        // 如果有节点,判断当前节点的下一个节点是否为空,为空就把新节点加到当前节点
        while let Some(node) = &current_node {
            let node = node.clone();
            let mut node_borrow = node.borrow_mut();
            if node_borrow.next.is_none() {
                node_borrow.next = Some(new_node.clone());
                break;
            }
            current_node = node_borrow.next.as_ref().and_then(|n| Some(n.clone()))
        }
        self.len += 1;
    }
    pub fn get_len(&self) -> usize {
        return self.len;
    }
    pub fn is_empty(&self) -> bool {
        if self.len == 0 {
            return true;
        }
        false
    }
}
#[test]
pub fn _test() {
    let mut stack = LinkStack::new();
    const LEN: i32 = 2;
    // 将栈添加满
    for i in 0..LEN {
        stack.push(i)
    }
    assert_eq!(stack.len, LEN as usize);
    println!("{:?}", stack);
    assert_eq!(stack.is_empty(), false);
    let len = stack.get_len();
    for i in 0..LEN as i32 {
        let data = stack.pop();
        // 检查弹出元素是否正确
        assert_eq!(data, Some(len as i32 - 1 - i))
    }
    println!("{:?}", stack);
    // // 查看栈是否为空
    assert_eq!(stack.get_len(), 0);
    assert_eq!(stack.is_empty(), true);
}
