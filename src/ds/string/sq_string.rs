
use thiserror::Error;

const MAXINDEX: usize = 10;
#[derive(Debug, Error)]
pub enum SStringError {
    #[error("输入的字符长度太长了")]
    TooLang,
}
#[derive(Debug,PartialEq, Eq)]
pub enum CompareResult {
    Equal,
    ///内部为不相等的字符索引
    UnEqual(usize),
}
#[derive(Debug,Clone, Copy)]
pub struct SString {
    data: [char; MAXINDEX],
    len: usize,
}

impl SString {
    pub fn new() -> Self {
        Self {
            data: ['\0'; MAXINDEX],
            len: 0,
        }
    }

    pub fn from<T: AsRef<str>>(s: T) -> Result<Self, SStringError> {
        let s = s.as_ref();
        let slen = s.len();
        if slen > MAXINDEX {
            return Err(SStringError::TooLang);
        }
        let mut temp = ['\0'; 10];
        for (i, c) in s.chars().enumerate() {
            temp[i] = c;
        }
        Ok(Self {
            data: temp,
            len: slen,
        })
    }
    pub fn get_child_str(&self,index:usize,len:usize)->Option<&[char]>{
        if index+len>MAXINDEX{
            None
        }
        else {
            Some(&self.data[index..index+len])
        }
    }
    pub fn index(&self,s:SString)->Option<usize>{
        let slen=s.len;
        let self_len=self.len;
        if slen>self_len{
            return None;
        }
        for i in 0..self_len{
            // 获取子串所有有效部分
            let sc=s.get_child_str(0, slen)?;
            // 从主串提取子串长度
            let self_c=self.get_child_str(i, slen)?;
            if sc==self_c{
                return Some(i);
            }
        }
        None
    }
    ///如果相等返回相等 `Euqal` ，不相等返回差异字符的索引 `UnEqual(index)`
    pub fn compare(&self, s: &SString) -> CompareResult {
        for i in 0..usize::max(self.len, s.len) {
            if self.data[i] != s.data[i] {
                return CompareResult::UnEqual(i);
            }
        }
        CompareResult::Equal
    }
    pub fn clear(&mut self){
        self.data=['\0';MAXINDEX];
        self.len=0;
    }
    pub fn push(&mut self,s:SString)->Result<(),SStringError>{
        if self.len+s.len>MAXINDEX{
            return  Err(SStringError::TooLang)
        }
        // 将s的字符复制到后面
        for i in 0.. s.len{
            self.data[self.len]=s.data[i];
            self.len+=1;
        }
        Ok(())
    }

}
#[test]
fn sq_string() {
    let s1 = SString::new();
    let mut s2 = SString::from("012").unwrap();
    let s3 = SString::from("34").unwrap();
    assert_eq!(s2.len, 3);
    println!("{s1:?}");
    println!("{s2:?}");
    let x = s1.compare(&s2);
    assert_eq!(s2.get_child_str(0, 3),Some(&['0','1','2'][..]));
    assert_eq!(x, CompareResult::UnEqual(0));
    let index=s2.index(s3);
    assert_eq!(index,None);
    s2.push(s3).unwrap();
    assert_eq!(s2.get_child_str(0, 5),Some(&['0','1','2','3','4'][..]));
    assert_eq!(s2.get_child_str(0, 11),None);
    assert_eq!(s2.get_child_str(0, 0),Some(&[][..]));
}
