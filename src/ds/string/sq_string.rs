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

    pub fn from(s: &str) -> Result<Self, SStringError> {
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
    ///如果相等返回相等，不相等返回差异字符的索引
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
    assert_eq!(s2.len, 7);
    println!("{s1:?}");
    println!("{s2:?}");
    let x = s1.compare(&s2);
    assert_eq!(x, CompareResult::UnEqual(0));
}
