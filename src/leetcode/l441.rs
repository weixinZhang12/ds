    pub fn arrange_coins(n: i32) -> i32 {
        let mut n=n;
        for i in 1..=i32::MAX {
            n -= i;
            if n<0 {
                return i-1;
            }
        }

        12
    }
