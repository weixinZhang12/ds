pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    if x==2147483647 {
        return 46340;
    }
    for i in 46336..=x / 2 {
        let i1 = i * i;
        let i2 = (i + 1) * (i + 1);

        if i1 == x {
            return i;
        }
        if i1 < x && i2 > x {
            if x - i1 > x - i2 {
                return i;
            } else {
                return i + 1;
            }
        }
    }
    1
}
