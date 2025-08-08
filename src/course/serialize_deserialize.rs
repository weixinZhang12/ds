use rand::{Rng, rngs::ThreadRng};
use std::{cell::RefCell, rc::Rc};

type NodeRef = Option<Rc<RefCell<TreeNode>>>;
type NodeRc = Rc<RefCell<TreeNode>>;
#[derive(Debug)]
pub struct TreeNode {
    value: i32,
    left: NodeRef,
    right: NodeRef,
}

impl TreeNode {
    pub fn insert(node: &NodeRef, value: i32, rng: &mut ThreadRng) -> NodeRef {
        let new_node = Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }));
        let direction = rng.random_bool(0.5);
        //    True left false right
        match node {
            Some(node) => {
                if direction {
                    let mut node_mut = node.borrow_mut();
                    node_mut.left = TreeNode::insert(&node_mut.left, value, rng);
                    let node = node.clone();
                    Some(node)
                } else {
                    let mut node_mut = node.borrow_mut();
                    node_mut.right = TreeNode::insert(&node_mut.right, value, rng);
                    let node = node.clone();
                    Some(node)
                }
            }
            None => Some(new_node),
        }
    }
}

pub fn serialize(node: &NodeRef, s: &mut String) -> String {

    if let Some(node) = node {
        let node_ref = node.borrow();
        s.push_str(&format!("{}", node_ref.value));
        s.push('(');
        serialize(&node_ref.left, s);
        s.push(',');
        serialize(&node_ref.right, s);
        s.push(')');
    }

    s.to_string()
}
#[test]
pub fn serialize_deserialize_test() {
    let mut rng = rand::rng();
    let mut node = TreeNode::insert(&None, 0, &mut rng);
    for i in 1..6 {
        node = TreeNode::insert(&node, i, &mut rng);
    }
    println!("{:#?}",node);
    let mut s="".to_string();
    let s=serialize(&node, &mut s);
    println!("{}",s);
}
