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
    let mut arr = [2, 3, 9, 1, 3, 2, 5, 7, 65,45,34,3,34,3,43,4,4,2,6,5,7,7,8,7,57,4,43,5,7,6,64,54,3,34,54,56,7,5,56,46,4,4,5,54,6,565,67,657,56,54,6,545,4,54,5,4,54,54,5,4,5434347,6,86,4,65,6,54,56,5456];
    quick_sort(&mut arr);
    println!("{:?}", arr)
}
