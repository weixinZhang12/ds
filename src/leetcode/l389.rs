    pub fn find_the_difference(s: String, t: String) -> char {
        // let s=s+&t;
        let mut xor=0;
        for c in s.chars(){
            xor ^= (c as u8)
        }
         for c in t.chars(){
            xor ^= (c as u8)
        }
        xor as char
    }