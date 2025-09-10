pub fn title_to_number(column_title: String) -> i32 {
    let mut res=0;
    let mut len = column_title.len().saturating_sub(1);
    for c in column_title.chars() {
        let index = c as u8 - 64;
        res+=index as u32*26_u32.pow(len as u32);
        len=len.saturating_sub(1);
    }
    res as i32
}
