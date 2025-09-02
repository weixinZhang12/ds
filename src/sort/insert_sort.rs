pub fn insert_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mut left = 0;
    // 当left小于最后一个元素索引时就一直排序
    while left < len - 1 {
        // 如果前面的元素大于后面的元素，就交换他们的位置
        if arr[left] > arr[left + 1] {
            arr.swap(left, left + 1);
            let mut j=left;
            // 如果检查已排好序的区域是否需要重新调整
            while j>0&&arr[j-1]>arr[j] {
                if arr[j] < arr[j-1] {
                    arr.swap(j, j -1);
                }
                j-=1;
            }
        }
        left += 1;
    }
}
#[test]
fn test() {
     for _ in 0..1000 {
        let mut arr = [0; 10];
        rand::fill(&mut arr);
        let mut target = arr;
        target.sort();
        insert_sort(&mut arr);
        assert_eq!(arr, target);
        println!("{:?}", arr)
    }
}
