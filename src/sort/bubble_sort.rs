pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    for i in (1..len).rev() {
        for i in 0..i {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    } 
}
#[test]
fn test() {
    for _ in 0..1000 {
        let mut arr = [0; 5];
        rand::fill(&mut arr);
        let mut target = arr;
        target.sort();
        bubble_sort(&mut arr);
        assert_eq!(arr, target);
        println!("{:?}", arr)
    }
}
