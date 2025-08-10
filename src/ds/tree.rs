use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
    // 返回插入节点的引用，不使用克隆
    pub fn auto_insert(node: &NodeRef, value: i32, rng: &mut ThreadRng) -> NodeRef {
        match node {
            None => Some(Rc::new(RefCell::new(TreeNode::new(value)))),
            Some(v) => {
                // 创建新的节点
                // let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                match TreeNode::bool_to_position(rng.random_bool(0.5)) {
                    // 插入到左侧
                    InsertPosition::Left => {
                        let mut temp = v.borrow_mut();
                        temp.left = Self::auto_insert(&temp.left, value, rng);
                        Some(Rc::clone(v))
                    }
                    // 插入到右侧
                    InsertPosition::Right => {
                        let mut temp = v.borrow_mut();

                        temp.right = Self::auto_insert(&temp.right, value, rng);
                        Some(Rc::clone(v))
                    }
                }
                // 返回节点
            }
        }
    }
    ///手动插入数据
    pub fn insert(node: &NodeRef, value: i32, pos: InsertPosition) -> NodeRef {
        match node {
            None => Some(Rc::new(RefCell::new(TreeNode::new(value)))),
            Some(v) => {
                // 创建新的节点
                // let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                // 插入到左侧
                let mut temp = v.borrow_mut();
                temp.left = Self::insert(&temp.left, value, pos);
                Some(Rc::clone(v))
                // 返回节点
            }
        }
    }

    // 广度优先遍历
    pub fn bfs(node: &NodeRef, deque: &mut VecDeque<Rc<RefCell<TreeNode>>>) {
        if let Some(v) = node {
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
    }
    ///深度优先搜索
    pub fn dfs(node: &NodeRef, arr: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(v) = node {
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
    }
    /// 广度优先遍历打印
    pub fn print_by_bfs(node: &Rc<RefCell<TreeNode>>) {
        let mut deque: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let node = Rc::clone(node);
        Self::bfs(&Some(node), &mut deque);
    }
    /// 深度优先遍历打印
    pub fn print_by_dfs(node: &Rc<RefCell<TreeNode>>) {
        let mut deque: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let node = Rc::clone(node);
        Self::dfs(&Some(node), &mut deque);
    }
    ///前序遍历
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
    ///中序遍历
    pub fn in_order(node: &NodeRef) {
        if let Some(value) = node {
            let node = value.borrow();
            TreeNode::in_order(&node.left);
            print!("{} ", node.value);
            TreeNode::in_order(&node.right);
        }
    }
    ///后序遍历
    pub fn post_order(node: &NodeRef) {
        if let Some(value) = node {
            let node = value.borrow();
            TreeNode::post_order(&node.left);
            TreeNode::post_order(&node.right);
            print!("{} ", node.value);
        }
    }
    ///布尔转换为插入位置
    fn bool_to_position(b: bool) -> InsertPosition {
        match b {
            true => InsertPosition::Right,
            false => InsertPosition::Left,
        }
    }

    /// 树的序列化
    pub fn serialize(node: &NodeRef, s: &mut String) -> String {
        if let Some(node) = node {
            let node_ref = node.borrow();
            s.push_str(&format!("{}", node_ref.value));
            // 如果不存在左右子树，那么直接跳过输出括号
            if let None = node_ref.left
                && let None = node_ref.right
            {
                return "".to_string();
            }
            s.push('(');
            TreeNode::serialize(&node_ref.left, s);
            // 如果不存在右子树，跳过输出逗号
            if node_ref.right.is_some() {
                s.push(',');
            }
            TreeNode::serialize(&node_ref.right, s);
            s.push(')');
        }
        s.to_string()
    }
    ///树的反序列化
    pub fn deselialize(s: String) -> NodeRef {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        // 迭代器
        let mut iter = s.chars().peekable();
        let mut temp = "".to_string();
        // 用于标注插入的位置
        let mut _pos = InsertPosition::Left;
        // 用于存储最后一个元素
        let mut last_node = None;

        while let Some(c) = iter.next() {
            match c {
                // 如果是数字
                '0'..='9' => {
                    // 将该字符添加进去
                    temp.push(c);
                }
                '(' => {
                    let node_value = temp.parse::<i32>().expect("输入的不是数字");
                    temp.clear();
                    let treenode: TreeNode = TreeNode::new(node_value);
                    let treenode = Rc::new(RefCell::new(treenode));
                    match stack.last() {
                        Some(node) => {
                            let mut node_mut = node.borrow_mut();
                            match _pos {
                                InsertPosition::Left => {
                                    node_mut.left = Some(treenode.clone());
                                }
                                InsertPosition::Right => {
                                    node_mut.right = Some(treenode.clone());
                                }
                            }
                        }
                        None => {}
                    }
                    stack.push(treenode.clone());
                    _pos = InsertPosition::Left;
                }
                ',' => {
                    if temp.is_empty() {
                        _pos = InsertPosition::Right;
                        continue;
                    }
                    let node_value = temp.parse::<i32>().expect("输入的不是数字");
                    temp.clear();
                    let treenode: TreeNode = TreeNode::new(node_value);
                    let treenode = Rc::new(RefCell::new(treenode));
                    match stack.last() {
                        Some(node) => {
                            let mut node_ref = node.borrow_mut();
                            match _pos {
                                InsertPosition::Left => {
                                    node_ref.left = Some(treenode.clone());
                                }
                                InsertPosition::Right => {
                                    node_ref.right = Some(treenode.clone());
                                }
                            }
                        }
                        None => {}
                    }
                    // 查看传入的节点
                    _pos = InsertPosition::Right;
                }
                ')' => {
                    if temp.is_empty() {
                        last_node = stack.pop();
                        continue;
                    }
                    let node_value = temp.parse::<i32>().expect("输入的不是数字");
                    temp.clear();
                    let treenode: TreeNode = TreeNode::new(node_value);
                    let treenode = Rc::new(RefCell::new(treenode));
                    // 查看传入的节点
                    match stack.last() {
                        Some(node) => {
                            let mut node_ref = node.borrow_mut();
                            match _pos {
                                InsertPosition::Left => {
                                    node_ref.left = Some(treenode.clone());
                                }
                                InsertPosition::Right => {
                                    node_ref.right = Some(treenode.clone());
                                }
                            }
                        }
                        None => {}
                    }
                    last_node = stack.pop();
                }
                _ => {
                    println!("{}", c)
                }
            }
        }

        last_node
    }
}

#[test]
// 测试树的插入是否正常
pub fn tree_insert() {
    let treenode = TreeNode::new(1);
    let mut rng = rand::rng();
    let value = RefCell::new(treenode);
    let value = Rc::new(value);
    let mut tr = Rc::clone(&value);
    // 向树中插入元素
    for _ in 0..5 {
        match TreeNode::auto_insert(&Some(tr), rng.random_range(1..100), &mut rng) {
            Some(v) => tr = v,
            None => return,
        };
    }
    TreeNode::print_by_bfs(&value);
    TreeNode::print_by_dfs(&value);
}
#[test]
pub fn tree_test() {
    let treenode = TreeNode::new(1);
    let mut rng = rand::rng();
    let value = RefCell::new(treenode);
    let value = Rc::new(value);
    let mut tr = Rc::clone(&value);
    // 向树中插入元素
    for _ in 0..5 {
        match TreeNode::auto_insert(&Some(tr), rng.random_range(1..100), &mut rng) {
            Some(v) => tr = v,
            None => return,
        };
    }
    TreeNode::pre_order(&Some(value.clone()));
    println!();
    TreeNode::in_order(&Some(value.clone()));
    println!();
    TreeNode::post_order(&Some(value.clone()));
}
#[test]
pub fn serialize_deserialize_test() {
    let mut rng = rand::rng();
    let mut node = TreeNode::auto_insert(&None, 0, &mut rng);
    for i in 1..5 {
        node = TreeNode::auto_insert(&node, i, &mut rng);
    }
    let mut s = "".to_string();
    let s1 = TreeNode::serialize(&node, &mut s);
    println!("{}", s1);
    let tree = TreeNode::deselialize(s1.clone());
    let mut s = "".to_string();
    let s2 = TreeNode::serialize(&tree, &mut s);
    println!("{}", s2);
    assert_eq!(s1, s2)
}
#[test]
pub fn _test() {
    for _ in 0..50000 {
        let mut rng = rand::rng();
        let mut node = TreeNode::auto_insert(&None, 0, &mut rng);
        for i in 1..20 {
            node = TreeNode::auto_insert(&node, i, &mut rng);
        }
        let mut s = "".to_string();
        let s1 = TreeNode::serialize(&node, &mut s);
        println!("{}", s1);
        let tree = TreeNode::deselialize(s1.clone());
        let mut s = "".to_string();
        let s2 = TreeNode::serialize(&tree, &mut s);
        println!("{}", s2);
        assert_eq!(s1, s2)
    }
}
