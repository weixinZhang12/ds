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
    let mut array: HashMap<i32, i32> = HashMap::new();
    let mut head = head;
    let mut _l = &mut head as *mut Option<Box<ListNode>>;
    let mut r = &mut head as *mut Option<Box<ListNode>>;
    while let Some(right_value) = unsafe { &mut *r } {
        _l = r;
        r = &mut right_value.next as *mut Option<Box<ListNode>>;
        // 如果哈希表中没有，插入该元素，如果存在，删除链表中的该元素
        if let std::collections::hash_map::Entry::Vacant(e) = array.entry(right_value.val) {
            e.insert(right_value.val);
        } else {
            let l = unsafe { &mut *_l };
            let r = unsafe { &mut *r };
            l.take().unwrap().next = r.take().unwrap().next;
        }
    }
    head
}
