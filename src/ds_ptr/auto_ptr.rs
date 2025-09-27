use std::{
    alloc::{Layout, alloc, dealloc},
    ops::{Deref, DerefMut},
};

///我自己的指针，自带布局的不安全指针
pub struct Ptr<T> {
    layout: Layout,
    ptr: *mut T,
}

impl<T> Ptr<T> {
    pub fn new(val: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc(layout) };
        let ptr = ptr as *mut T;
        unsafe {
            ptr.write(val);
        }
        Self { layout, ptr }
    }
    pub fn set(&mut self, val: T) {
        unsafe {
            self.ptr.write(val);
        }
    }
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
impl<T> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
impl<T> Drop for Ptr<T> {
    fn drop(&mut self) {
        unsafe {
            self.ptr.drop_in_place();
            dealloc(self.ptr as *mut u8, self.layout);
        }
    }
}

#[test]

fn test() {
    let mut ptr: Ptr<[i32; 2]> = Ptr::<[i32; 2]>::new([0, 0]);
    let ptr = ptr.as_mut();
    ptr[0] = 0;
    ptr[1] = 1;
    dbg!(ptr.as_ref());
}
