use std::{cell::RefCell, rc::Rc};
type NodeRef = Option<Rc<RefCell<TreeNode>>>;
pub enum InsertPosition {
    Left,
    Right,
}
#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: NodeRef,
    pub right: NodeRef,
}

impl TreeNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
    // 返回插入节点的引用，不使用克隆
    pub fn insert_and_get_ref(node: &NodeRef, value: i32, position: InsertPosition) -> NodeRef {
        match node {
            None => Some(Rc::new(RefCell::new(TreeNode::new(value)))),
            Some(v) => {
                // 创建新的节点
                let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                match position {
                    // 插入到左侧
                    InsertPosition::Left => {
                        v.borrow_mut().left = Some(Rc::clone(&new_node));
                    }
                    // 插入到右侧
                    InsertPosition::Right => {
                        v.borrow_mut().right = Some(Rc::clone(&new_node));
                    }
                }
                // 返回节点
                Some(new_node)
            }
        }
    }
   
}

