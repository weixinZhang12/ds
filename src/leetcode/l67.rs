pub fn add_binary(a: String, b: String) -> String {
    let mut a: Vec<i32> = a
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as i32)
        .collect();
    let mut b: Vec<i32> = b
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as i32)
        .collect();
    let mut v: Vec<i32> = vec![];
    let mut sum = 0;

    // 反转数字便于计算
    a.reverse();
    b.reverse();
    // 对齐数组
    while a.len() > b.len() {
        b.push(0);
    }
    while b.len() > a.len() {
        a.push(0);
    }
    for i in 0..a.len() {
        if sum > 0 {
            a[i] += 1;
            sum -= 1;
        }
        if a[i] + b[i] >= 2 {
            sum = (a[i] + b[i]) / 2;
            v.push(a[i] + b[i] - 2);
        } else {
            v.push(a[i] + b[i]);
        }
    }
    if sum != 0 {
        v.push(sum);
    }
    v.reverse();
    let s = v.iter().map(|n| n.to_string()).collect();
    s
    // v.to_string()
}
