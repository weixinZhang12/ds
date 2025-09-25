use std::{
    alloc::{Layout, alloc, dealloc, realloc},
    ptr::NonNull,
};

use thiserror::Error;
#[derive(Error, Debug)]
pub enum DataStructErr {
    #[error("无效的索引")]
    InvalidIndex,
}
#[derive(Debug)]
pub struct Vector {
    len: usize,
    data: NonNull<i32>,
    cap: usize,
}
impl Vector {
    pub fn new() -> Self {
        let layout = Layout::array::<i32>(5).expect("Invalid memory layout");
        let ptr = unsafe { alloc(layout) as *mut i32 };
        let non_ptr = NonNull::new(ptr).expect("falid");
        Self {
            len: 0,
            data: non_ptr,
            cap: 5,
        }
    }
    // 在末尾添加值
    pub fn push(&mut self, val: i32) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            let res = self.data.as_ptr().add(self.len);
            res.write(val);
        };
        self.len += 1;
    }
    ///获取索引处的值
    pub fn get(&self, index: usize) -> Option<&i32> {
        if index >= self.len {
            return None;
        }
        let prt = self.data.as_ptr();
        unsafe {
            let end = prt.add(index);
            end.as_ref()
        }
    }
    ///设置索引处的值，如果传入无效索引
    pub fn set(&mut self, index: usize, val: i32) -> Result<(), DataStructErr> {
        if index >= self.len {
            return Err(DataStructErr::InvalidIndex);
        }
        unsafe {
            let end = self.data.as_ptr().add(index);
            end.write(val);
        }
        Ok(())
    }
    ///交换索引处的值，当索引不正确的时候直接忽略该该次操作
    pub fn swap(&mut self, i1: usize, i2: usize) {
        if i1 >= self.len || i2 >= self.len {
            return;
        }
        let _temp = *self.get(i1).unwrap();
        let i2_val = self.get(i2).unwrap();
        self.set(i1, *i2_val).unwrap();
        self.set(i2, _temp).unwrap();
    }
    pub fn delete(&mut self, index: usize) -> Option<i32> {
        //获取当前表的长度，索引是否小于表长度
        if index >= self.len {
            return None;
        }
        // 当只有一个元素的时候
        let temp = unsafe {
            let end = self.data.add(index);
            let temp=end.read();
            end.write(0);
            dbg!(temp);
            temp

        };
        // 当只有一个元素的时候
        if self.len == 1 {
            self.len = 0;
            return Some(temp);
        }
        // 长度-1
        self.len-=1;
        for i in index..self.len - 1 {
            self.swap(i, i + 1);
        }
        Some(temp)
    }
    pub fn grow(&mut self) {
        let new_cap = self.cap * 2;
        let new_layout = Layout::array::<i32>(new_cap).expect("fail");
        let old_layout = Layout::array::<i32>(self.cap).unwrap();
        unsafe {
            let old_ptr = self.data.as_ptr() as *mut u8;
            let new_ptr = realloc(old_ptr, old_layout, new_layout.size());
            self.data = NonNull::new(new_ptr as *mut i32).unwrap();
            self.cap = new_cap;
        }
    }

    pub fn print(&self) {
        let ptr = self.data.as_ptr();
        for i in 0..self.len {
            unsafe {
                let data = ptr.add(i).as_ref().unwrap();
                println!("{}", data);
            }
        }
    }
}
impl Drop for Vector {
    fn drop(&mut self) {
        let layout = Layout::array::<i32>(self.cap).unwrap();
        unsafe {
            dealloc(self.data.as_ptr() as *mut u8, layout);
        }
    }
}
#[test]
fn test() {
    let mut v = Vector::new();
    for i in 0..100 {
        v.push(i);
    }
    // v.swap(0, 1);
    v.print();
    let vals=v.delete(2);
    v.print();
}
