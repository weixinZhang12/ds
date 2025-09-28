use std::collections::HashMap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut map = HashMap::new();
    // 如果当前节点有东西就继续，否则直接返回
    let current = Box::into_raw(head.clone()?);
    // 上一个节点
    let mut last_node = current;
    let mut current_node = unsafe { current.as_ref().unwrap() };
    map.insert(current_node.val, current_node.val);
    while let Some(next_node) = &current_node.next {
        current_node = next_node;
        let res = map.get(&current_node.val);
        // 查看是否重复
        match res {
            // 如果重复
            Some(v) => {
                last_node = Box::into_raw(next_node.clone());
                let next = unsafe { &current.as_mut().unwrap().next };
                unsafe { last_node.as_mut() }.unwrap().next = next.clone();
            }
            // 没有重复
            None => {
                map.insert(current_node.val, current_node.val);
            }
        }
    }

    // 查看当前节点是否有值
    head
}
