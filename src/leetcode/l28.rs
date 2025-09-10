pub fn str_str(haystack: String, needle: String) -> i32 {
    let str_length = haystack.len();
    let pattern_length = needle.len();
    if str_length == 0 || pattern_length > str_length {
        return -1;
    }

    let str_array: Vec<char> = haystack.chars().collect();
    let needle_array: Vec<char> = needle.chars().collect();
    for start in 0..str_length - pattern_length + 1 {
        let mut is_found = true;
        for j in 0..pattern_length {
            if str_array[start+j] != needle_array[j] {
                is_found = false;
                break;
            }
        }
        if is_found {
            return start as i32;
        }
    }
    return -1 as i32;
}
