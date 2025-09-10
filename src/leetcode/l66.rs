pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let _length = digits.len();
    let mut add_num = 1;
    for i in (0..digits.len()).rev() {
        // 先加上add_num的数
        digits[i] += add_num;
        // 如果加上之后小于10就退出
        if digits[i] < 10 {
            return digits;
        }
        // 否则add_num
        if digits[i] == 10 {
            digits[i] = 0;
            add_num = 1;
        }
    }
    if add_num != 0 {
        digits.insert(0, 1);
    }

    digits
}
