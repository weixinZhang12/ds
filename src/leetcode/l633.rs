pub fn judge_square_sum(c: i32) -> bool {
    for a in 0..=(c as f64).sqrt() as i32 {
        let b = ((c - (a * a)) as f64).sqrt();
        // 不是整数就换下一个
        if b.fract() != 0.0 {
            continue;
        }
        if b < a as f64 {
            return false;
        }
        if (a * a) as f64 + b * b == c as f64 {
            return true;
        }
    }
    false
}
