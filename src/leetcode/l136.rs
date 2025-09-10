use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for &i in nums.iter() {
        let result = map.get(&i);
        match result {
            Some(v) => {
                map.insert(i, v + 1);
            }
            None => {
                map.insert(i, 1);
            }
        }
    }
    let num: Vec<i32> = map
        .iter()
        .filter(|&(_, &v)| v == 1)
        .map(|(&k, _)| k)
        .collect();
    num[0]
}
