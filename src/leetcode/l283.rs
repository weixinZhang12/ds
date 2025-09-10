use core::num;

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut left = 0;
    let mut temp = 0;
    for right in 0..nums.len() {
        if nums[right] != 0 {
            temp = nums[left];
            nums[left] = nums[right];
            nums[right] = temp;
            left += 1;
        }
    }
}
