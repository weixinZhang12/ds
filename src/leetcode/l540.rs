use std::collections::HashMap;

    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut map=HashMap::new();
        for i in nums.iter() {
            // 
           match map.get(&i) {
               Some(v)=>{
                map.insert(i, v+1);
               }
                None=>{
                map.insert(i, 1);

                }
               }
           } 
        for i in nums.iter() {
            if map.get(&i).unwrap()==&1 {
                return *i;
            }
        }
        0
        }

    

