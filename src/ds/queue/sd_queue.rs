use thiserror::Error;

const MAXINDEX: usize = 10;
#[derive(Debug, Error)]
pub enum SqQueueError {
    #[error("栈已经满了")]
    Full,
}
#[derive(Debug)]
// 并非最优实现，使用Option占用内存太大，推荐使用定义一个字段size
pub struct SqQueue {
    data: [Option<i32>; MAXINDEX],
    front: usize,
    rear: usize,
}

impl SqQueue {
    pub fn new() -> Self {
        Self {
            data: [None; MAXINDEX],
            front: 0,
            rear: 0,
        }
    }
    pub fn push(&mut self, val: i32) -> Result<(), SqQueueError> {
        // 如果当前队尾指针为none，证明队列还没有满
        if self.data[self.rear].is_none() {
            self.data[self.rear] = Some(val);
            self.rear = (self.rear + 1) % MAXINDEX;
            Ok(())
        } else {
            Err(SqQueueError::Full)
        }
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.data[self.front]?;
        let res = self.data[self.front].take();
        self.front = (self.front + 1) % MAXINDEX;
        res
    }
    pub fn is_full(&self) -> bool {
        if self.rear == self.front {
            return self.data[self.rear].is_some();
        }
        false
    }
    pub fn is_empty(&self) -> bool {
        if self.rear == self.front {
            self.data[self.front].is_none()
        }
        else {
            false
        }
    }
}
#[test]
fn _test() {
    let mut queue = SqQueue::new();
    for i in 0..MAXINDEX as i32 {
        if let Err(e) = queue.push(i) {
            println!("{e}")
        }
    }
    println!("{queue:?}");
    assert!(queue.is_full());    
    assert!(!queue.is_empty());

    for i in 0..MAXINDEX as i32 {
        let val = queue.pop();
        assert_eq!(val, Some(i));
    }
    println!("{queue:?}");
    assert!(queue.is_empty());
    println!("{}",size_of::<Option<i32>>());   
     println!("{}",size_of::<i32>());


}
