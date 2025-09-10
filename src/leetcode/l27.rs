pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut sum=0;
    nums.retain(|&x|{
        if x!=val {
            sum+=1;
            return true;
        }
        else {
            false
        }
    });
    sum
}
