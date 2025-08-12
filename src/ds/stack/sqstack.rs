use thiserror::Error;

const MAXINDEX: usize = 100;
#[derive(Debug, Error)]
pub enum StackError {
    #[error("栈已经满了")]
    Full,
}
#[derive(Debug)]
pub struct SqStack {
    data: [i32; MAXINDEX],
    top: usize,
}

impl SqStack {
    pub fn new() -> Self {
        Self {
            data: [0; MAXINDEX],
            top: 0,
            // len:0
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.data[self.top])
    }
    pub fn first(&self) -> Option<&i32> {
        if self.top>0 {
            return Some(&self.data[0]);
        }
        None
    }
    pub fn push(&mut self, val: i32) -> Result<(), StackError> {
        if self.top == MAXINDEX {
            return Err(StackError::Full);
        }
        self.data[self.top] = val;
        self.top += 1;
        Ok(())
    }
    pub fn get_len(&self) -> usize {
        self.top
    }
    pub fn is_empty(&self) -> bool {
        if self.top == 0 {
            return true;
        }
        false
    }
    pub fn is_full(&self) -> bool {
        if self.top == MAXINDEX {
            return true;
        }
        false
    }
}
#[test]
pub fn _test() {
    let mut stack = SqStack::new();
    // 将栈添加满
    for i in 0..MAXINDEX as i32 {
        if let Err(e) = stack.push(i) {
            println!("{}", e);
        }
    }
    assert_eq!(stack.get_len(), MAXINDEX);
    assert!(stack.is_full());
    assert!(!stack.is_empty());
    for i in 0..MAXINDEX as i32 {
        let data = stack.pop();
        // 检查弹出元素是否正确
        assert_eq!(data, Some(MAXINDEX as i32 - 1 - i))
    }
    // 查看栈是否为空
    assert_eq!(stack.get_len(), 0);
    assert!(stack.is_empty());
    assert!(!stack.is_full());

}
