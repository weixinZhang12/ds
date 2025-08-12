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
    data: [i32; MAXINDEX],
    front: usize,
    rear: usize,
    len: usize,
}

impl SqQueue {
    pub fn new() -> Self {
        Self {
            data: [0; MAXINDEX],
            front: 0,
            rear: 0,
            len: 0,
        }
    }
    ///向栈内添加元素
    pub fn push(&mut self, val: i32) -> Result<(), SqQueueError> {
        // 如果当前队尾指针为none，证明队列还没有满
        if self.len < MAXINDEX {
            self.data[self.rear] = val;
            self.rear = (self.rear + 1) % MAXINDEX;
            self.len += 1;
            Ok(())
        } else {
            Err(SqQueueError::Full)
        }
    }
    ///弹出第一个元素
    pub fn pop(&mut self) -> Option<i32> {
        // 队头为空直接返回
        if self.len == 0 {
            return None;
        }
        let res = self.data[self.front];
        self.data[self.front] = 0;
        self.front = (self.front + 1) % MAXINDEX;
        self.len -= 1;
        Some(res)
    }
    ///查看栈是否已经满了
    pub fn is_full(&self) -> bool {
        if self.len == MAXINDEX {
            return true;
        }
        false
    }
    ///查看栈是否为空
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
#[test]
fn _sq_queue() {
    let mut queue = SqQueue::new();
    for i in 0..MAXINDEX as i32 {
        if let Err(e) = queue.push(i) {
            println!("{e}")
        }
    }
    println!("full: {queue:?}");
    assert!(queue.is_full());
    assert!(!queue.is_empty());

    for i in 0..MAXINDEX as i32 {
        let val = queue.pop();
        assert_eq!(val, Some(i));
    }
    println!("empty:{queue:?}");
    assert!(queue.is_empty());
    if let Err(e) = queue.push(1) {
        println!("{e}")
    }
    assert_eq!(queue.data[0],1);
    if let Err(e) = queue.push(2) {
        println!("{e}")
    }
    assert_eq!(queue.data[1],2);

    println!("can_cycle:{queue:?}");

}
