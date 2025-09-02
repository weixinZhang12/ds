pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len == 0 || len == 1 {
        return;
    }
    let mut left = 0;
    let mut right = len - 1;
    // 取第一关元素作为privot
    let privot = arr[0];
    while left < right {
        // 当右指针元素大与privot的时候直接向左移动，即right右边的元素一定大于privot
        while arr[right] >= privot && left < right {
            right -= 1;
        }
        arr.swap(left, right);
        while arr[left] <= privot && left < right {
            left += 1;
        }
        arr.swap(left, right);
    }
    quick_sort(&mut arr[0..left]);
    quick_sort(&mut arr[left + 1..len]);
}

#[test]
fn test() {
    for _ in 0..1000 {
        let mut arr = [0; 10];
        rand::fill(&mut arr);
        let mut target = arr;
        target.sort();
        quick_sort(&mut arr);
        assert_eq!(arr, target);
        println!("{:?}", arr)
    }
}
