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
#[derive(Debug)]
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
    pub fn take_child_str(&self,index:usize,len:usize)->Option<&[char]>{
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
            let sc=s.take_child_str(0, slen)?;
            // 从主串提取子串长度
            let self_c=self.take_child_str(i, slen)?;
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
}
#[test]
fn sq_string() {
    let s1 = SString::new();
    let s2 = SString::from("1234567").unwrap();
    let s3 = SString::from("67").unwrap();
    assert_eq!(s2.len, 7);
    println!("{s1:?}");
    println!("{s2:?}");
    let x = s1.compare(&s2);
    assert_eq!(s2.take_child_str(0, 3),Some(&['1','2','3'][..]));
    assert_eq!(x, CompareResult::UnEqual(0));
    let index=s2.index(s3);
    assert_eq!(index,Some(5));
}
