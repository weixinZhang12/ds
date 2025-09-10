pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // Step 1: 保证 nums1 的前 m 个元素是有效的，删除其余部分
    nums1.truncate(m as usize);
    
    // Step 2: 合并 nums2 到 nums1
    nums1.extend(nums2.iter().take(n as usize));

    // Step 3: 排序 nums1
    nums1.sort();

    // Step 4: 移除所有的零
    // nums1.retain(|&x| x != 0);
}

