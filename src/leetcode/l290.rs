use std::collections::HashMap;
// 1.将模式字符串和匹配字符串转换为数组
// 2.将模式字符串数组中的值与匹配字符串中的值作映射
// 3.有映射不一样的直接false
pub fn word_pattern(pattern: String, s: String) -> bool {
    let pattern_arr: Vec<char> = pattern.chars().collect();
    let s_arr: Vec<&str> = s.split(" ").collect();
    if pattern_arr.len()!=s_arr.len() {
        return false    ;
    }
    let mut map: HashMap<char, &str> = HashMap::new();
    let mut map2: HashMap<&str,char> = HashMap::new();
    
    // index为当前的字符的索引，item为
    for (index, &item) in pattern_arr.iter().enumerate() {
        let result = map.get(&item);
        match result {
            Some(&v) => {
                if v != s_arr[index] {
                    return false;
                }
            }
            None => {
                map.insert(item, s_arr[index]);
            }
        }
        let result = map2.get(s_arr[index]);
        match result {
            Some(&v) => {
                if v != item {
                    return false;
                }
            }
            None => {
                map2.insert(s_arr[index],item);
            }
        }
    }

   
    true
}
