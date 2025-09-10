use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in nums {
        let result = map.get(&i);
        match result {
            Some(v) => return true,
            None => {
                map.insert(i, i);
            }
        }
    }
    false
}
