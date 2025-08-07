use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::Rc,
};

use rand::{Rng, rngs::ThreadRng};
type NodeRef = Option<Rc<RefCell<TreeNode>>>;
type NodeRc = Rc<RefCell<TreeNode>>;
pub enum InsertPosition {
    Left,
    Right,
}
#[derive(Debug, Clone)]
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
    #[allow(dead_code)]
    // 返回插入节点的引用，不使用克隆
    pub fn insert_and_get_ref(node: &NodeRef, value: i32, rng: &mut ThreadRng) -> NodeRef {
        match node {
            None => Some(Rc::new(RefCell::new(TreeNode::new(value)))),
            Some(v) => {
                // 创建新的节点
                // let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                match bool_to_position(rng.random_bool(0.5)) {
                    // 插入到左侧
                    InsertPosition::Left => {
                        let mut temp = v.borrow_mut();
                        temp.left = Self::insert_and_get_ref(&temp.left, value, rng);
                        Some(Rc::clone(v))
                    }
                    // 插入到右侧
                    InsertPosition::Right => {
                        let mut temp = v.borrow_mut();

                        temp.right = Self::insert_and_get_ref(&temp.right, value, rng);
                        Some(Rc::clone(v))
                    }
                }
                // 返回节点
            }
        }
    }
    // 仅仅打印值，不消耗Node
    #[allow(dead_code)]
    pub fn bfs(node: &NodeRef, deque: &mut VecDeque<Rc<RefCell<TreeNode>>>) {
        match node {
            Some(v) => {
                // 如果节点有值，压入队列
                let node = Rc::clone(v);
                deque.push_back(node);
                // 如果没有元素，那么遍历完毕
                while let Some(v) = &deque.pop_front() {
                    let temp = v.borrow();
                    let left = &temp.left;
                    let right = &temp.right;
                    if left.is_none() && right.is_none() {
                        println!("{}->None", temp.value);
                    }
                    // 有左节点就压入
                    if let Some(v) = left {
                        let value = v.borrow();
                        println!("{} -> {} (left)", temp.value, value.value);
                        deque.push_back(Rc::clone(&v));
                    }
                    // 有右节点就压入
                    if let Some(v) = right {
                        let value = v.borrow();
                        println!("{} -> {}(right)", temp.value, value.value);
                        deque.push_back(Rc::clone(&v));
                    }
                }
            }
            None => {}
        }
    }
    pub fn dfs(node: &NodeRef, arr: &mut Vec<Rc<RefCell<TreeNode>>>) {
        match node {
            Some(v) => {
                let value = v.borrow();
                if let Some(v) = &value.left {
                    let v = Rc::clone(v);
                    Self::dfs(&Some(v), arr);
                }
                if let Some(v) = &value.right {
                    let v = Rc::clone(v);
                    Self::dfs(&Some(v), arr);
                }
                println!("{}", value.value)
            }
            None => return,
        }
    }
    pub fn print_by_bfs(node: &Rc<RefCell<TreeNode>>) {
        let mut deque: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let node = Rc::clone(node);
        Self::bfs(&Some(node), &mut deque);
    }
    pub fn print_by_dfs(node: &Rc<RefCell<TreeNode>>) {
        let mut deque: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let node = Rc::clone(node);
        Self::dfs(&Some(node), &mut deque);
    }

    pub fn pre_order(node: &NodeRef) {
        match node {
            Some(value) => {
                let node = value.borrow();
                print!("{} ", node.value);
                TreeNode::pre_order(&node.left);
                TreeNode::pre_order(&node.right);
            }
            None => {
                return;
            }
        }
    }
    pub fn in_order(node: &NodeRef) {
        match node {
            Some(value) => {
                let node = value.borrow();
                TreeNode::in_order(&node.left);
                print!("{} ", node.value);
                TreeNode::in_order(&node.right);
            }
            None => {
                return;
            }
        }
    }
    pub fn post_order(node: &NodeRef) {
        match node {
            Some(value) => {
                let node = value.borrow();
                TreeNode::post_order(&node.left);
                TreeNode::post_order(&node.right);
                print!("{} ", node.value);
            }
            None => {
                return;
            }
        }
    }
}

fn bool_to_position(b: bool) -> InsertPosition {
    match b {
        true => InsertPosition::Right,
        false => InsertPosition::Left,
    }
}
#[test]
pub fn tree_insert() {
    let treenode = TreeNode::new(1);
    let mut rng = rand::rng();
    let value = RefCell::new(treenode);
    let value = Rc::new(value);
    let mut tr = Rc::clone(&value);
    // 向树中插入元素
    for _ in 0..5 {
        match TreeNode::insert_and_get_ref(&Some(tr), rng.random_range(1..100), &mut rng) {
            Some(v) => tr = v,
            None => return,
        };
    }
    // println!("{:#?}",value)
    TreeNode::print_by_bfs(&value);
    TreeNode::print_by_dfs(&value);
}
#[test]
pub fn tree_test(){
     let treenode = TreeNode::new(1);
    let mut rng = rand::rng();
    let value = RefCell::new(treenode);
    let value = Rc::new(value);
    let mut tr = Rc::clone(&value);
    // 向树中插入元素
    for _ in 0..5 {
        match TreeNode::insert_and_get_ref(&Some(tr), rng.random_range(1..100), &mut rng) {
            Some(v) => tr = v,
            None => return,
        };
    }
    TreeNode::pre_order(&Some(Rc::clone(&value)));
    println!();
    TreeNode::in_order(&Some(Rc::clone(&value)));
    println!();
    TreeNode::post_order(&Some(Rc::clone(&value)));
}
