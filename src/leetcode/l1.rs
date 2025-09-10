use std::{collections::HashMap, vec};

pub fn two_nums(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 创建一个哈希表
    let mut map=HashMap::new();
    for (i,num) in nums.iter().enumerate()  {
       let find_value=target- num;
       if let Some(index)=map.get(&find_value) {
           return vec![ *index as i32,i as i32];
       }
    //    如果没有找到该值，插入到哈希表
       map.insert(num, i);
    }
    vec![]

}
