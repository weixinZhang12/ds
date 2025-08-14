use std::{cell::RefCell, rc::Rc};

use thiserror::Error;

use crate::ds::string::sq_string::CompareResult;

type NodeRef = Option<Rc<RefCell<LStringNode>>>;
type LS = dyn AsRef<str>;
#[derive(Debug, Error)]

pub enum LStringError {
    #[error("输入的字符长度太长了")]
    TooLang,
}
#[derive(Debug)]

pub struct LStringNode {
    data: [char; 4], //16字节
    next: NodeRef,   //8字节
    len: usize,      //8字节
}

impl LStringNode {
    pub fn new() -> Self {
        Self {
            data: ['\0'; 4],
            next: None,
            len: 0,
        }
    }
    // 从rust 类型中转换
    pub fn from<T: AsRef<str>>(c: T) -> Result<Self, LStringError> {
        let c = c.as_ref();
        let clen = c.len();
        if clen > 4 {
            return Err(LStringError::TooLang);
        }
        let mut ca = ['\0'; 4];
        for (i, c) in c.chars().enumerate() {
            ca[i] = c;
        }
        Ok(Self {
            data: ca,
            next: None,
            len: clen,
        })
    }
    // 从[char;4]类型转换
    pub fn from_char4(ca: [char; 4]) -> Self {
        let mut index = 0;
        for c in ca {
            if c != '\0' {
                index += 1;
            }
        }
        Self {
            data: ca,
            next: None,
            len: index,
        }
    }
    pub fn get_len(&self) -> usize {
        let s = self.data;
        let mut index = 0;
        for c in s {
            if c != '\0' {
                index += 1;
            }
        }
        index
    }
}
#[derive(Debug)]

pub struct LString {
    next: NodeRef, //8字节
    rear: NodeRef, //8字节
    len: usize,    //8字节
}

impl LString {
    pub fn new() -> Self {
        Self {
            next: None,
            rear: None,
            len: 0,
        }
    }

    // pub fn from<T:AsRef<str>>(s:T){
    //     let s=s.as_ref();
    //     let slen=s.len();
    //     let mut index=0;
    //     let mut lsting=LString::new();

    //    while  index<slen{
    //        if slen-index<4{
    //         let temp=&s[index..];
    //         let new_node=LStringNode::from(temp);
    //         // lsting.next
    //        }
    //    }
    // }
    pub fn get_four_lang_string<T: AsRef<str>>(s: T, index: usize) -> Option<[char; 4]> {
        let s = s.as_ref();
        // 传入字符长度
        let slen = s.len();
        // 长度为0直接返回空，因为无法获取子串
        if slen == 0 {
            return None;
        }
        let mut ca = ['\0'; 4];
        let mut _slice = "";
        if index + 4 < slen {
            _slice = &s[index..index + 4];
        } else {
            _slice = &s[index..]
        }
        for (i, c) in _slice.chars().enumerate() {
            ca[i] = c
        }
        Some(ca)
    }
    pub fn compare(&self,s:&LString)->CompareResult{
        let self_len=self.len;
        let slen=s.len;
        // 当两个字符串不相等的时候
        if slen!=self_len{
            return CompareResult::UnEqual( usize::min(self_len, slen)-1);
        }
        // let max=usize::max(slen, self_len);
        
        CompareResult::Equal
    }
    ///
    pub fn push<T: AsRef<str>>(&mut self, s: T) {
        let s = s.as_ref();
        let mut index = 0;
        let slen = s.len();
        loop {
            // 当
            if index < slen {
                let ss = Self::get_four_lang_string(s, index);
                // 以下部分和链表操作差不多了
                // =========================链表部分==============================
                if let Some(ss) = ss {
                    let node = Rc::new(RefCell::new(LStringNode::from_char4(ss)));

                    if self.next.is_none() {
                        self.len += node.borrow().len;
                        self.next = Some(node.clone());
                        self.rear = Some(node)
                    } else if let Some(last_node) = self.rear.clone() {
                        let mut node_mut = last_node.borrow_mut();
                        self.len += node.borrow().len;
                        node_mut.next = Some(node.clone());
                        self.rear = Some(node)
                    }
                    // =====================链表部分==========================

                    index += 4;
                }
            } else {
                break;
            }
        }
    }
}
#[test]
fn _lstring() {
    let mut ls = LString::new();
    assert_eq!(ls.len, 0);
    ls.push("0123456789");
    println!("{ls:#?}");
    assert_eq!(ls.len, 10);
}
