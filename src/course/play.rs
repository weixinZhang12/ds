use std::{
    alloc::{Layout, alloc, dealloc, realloc},
    ptr::NonNull,
};
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
    v.print();
    println!("{:?}", v);
}
