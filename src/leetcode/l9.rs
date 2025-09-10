pub fn is_palindrome(x: i32) -> bool {
    // num = 22;
    // x<0或者最后一位为0 一定不是，最后一位为0
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut reserved_half = 0;
    let mut temp = x;
    if x < 0 {
        return false;
    }
    // 值查询一半
    while temp > reserved_half {
        reserved_half = reserved_half * 10 + temp % 10;
        temp /= 10;
    }
    println!("{}", reserved_half);
    // 对比另一半
    reserved_half == temp || reserved_half / 10 == temp
}
