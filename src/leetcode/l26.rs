pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut unique_array = Vec::new();
    for  item in nums.iter() {
        if !unique_array.contains(item) {
            unique_array.push(*item);
        }
    }
    println!("{:?}",unique_array);
    nums.clear();
    nums.extend(&unique_array);
    unique_array.len() as i32
}
