use std::collections::HashSet;

pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut hs:HashSet<i32  >=HashSet::new();
    nums.retain(|&x|hs.insert(x));
    nums.sort();
    println!("{:?}", nums);
    // 如果长度小于三，直接返回最后一个元素即可
    if nums.len()<3 {
        return nums.pop().unwrap();
    } 
    nums[nums.len()-3]  
    
}
