pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut v = nums;
    v.sort();
    for (index, item) in v.iter().enumerate() {
        if index != *item as usize {
            return index as i32;
        }
    }
    v.len() as i32
}
