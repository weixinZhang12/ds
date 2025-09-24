#[test]
fn test() {
    use std::alloc::{Layout, alloc, dealloc};

    let layouy = Layout::array::<i32>(8).expect("alloc failed");
    unsafe {
        let ptr = alloc(layouy) as * mut [i32;8];
        if ptr.is_null() {
            return;
        }
        println!("{:?}",layouy)
        ;
        assert!(!ptr.is_null());
        ptr.write([1;8]);
        println!("normal ptr: {:?}",ptr.read());
        dealloc(ptr as *mut u8, layouy);
        // This position ptr has become a wild pointer.
        println!("wild pointer: {:?}",*ptr);    
    }
}
