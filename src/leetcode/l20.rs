pub fn is_vaild(s: String) -> bool {
    let mut array = Vec::new();
    for c in s.chars() {
        if c == ')' {
            // 弹出的不对就不是
            if array.pop() != Some('(') {
                return false;
            }
        } else if c == ']' {
            if array.pop()!=Some('[') {
                return false;
            }
        } else if c == '}' {
            if array.pop()!=Some('{') {
                return false;
            }
        } else {
            array.push(c);
        }
    }
    array.is_empty()
}
