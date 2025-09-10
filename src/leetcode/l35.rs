
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = nums.len();
    let mut mid: usize = 0;
    while left < right {
        mid = (left + right) / 2;
        // 目标值在右侧
        if nums[mid] < target {
            left = mid + 1;
        }
        // 目标值在左侧
        else  {
            right = mid;
        } 
    }
    left as i32
}
