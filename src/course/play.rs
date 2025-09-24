use std::{
    alloc::{Layout, alloc, dealloc, realloc},
    ptr::NonNull,
};

use rand::seq::index;
#[derive(Debug)]
pub struct Vector {
    len: usize,
    data: NonNull<i32>,
    cap: usize,
}
impl Vector {
    pub fn new() -> Self {
        let layout = Layout::array::<i32>(5).expect("Invalid memory");
        let ptr = unsafe { alloc(layout) as *mut i32 };
        let non_ptr = NonNull::new(ptr).expect("falid");
        Self {
            len: 0,
            data: non_ptr,
            cap: 5,
        }
    }
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
    pub fn set(&mut self, index: usize, val: i32) {
        if index >= self.len {
            return;
        }
        unsafe {
            let end = self.data.as_ptr().add(index);
            end.write(val);
        }
    }
    ///交换索引处的值，当索引不正确的时候直接忽略该该次操作
    pub fn swap(&mut self, i1: usize, i2: usize) {
        if i1 >= self.len || i2 >= self.len {
            return;
        }
        let mut _temp = 0;
        _temp = *self.get(i1).unwrap();
        let i2_val = self.get(i2).unwrap();
        self.set(i1, *i2_val);
        self.set(i2, _temp);
    }
    pub fn delete(&mut self, index: usize) -> Option<i32> {
        //获取当前表的长度，索引是否小于表长度
        if index >= self.len {
            return None;
        }

        todo!()
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
    for i in 0..15 {
        v.push(i);
    }
    v.swap(0, 1);
    v.print();
    let val = v.get(1);
    println!("{}", val.unwrap());
    println!("{:?}", v);
}
