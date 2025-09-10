use std::collections::HashMap;

pub fn roman_to_int(s: String)->i32 {
    let mut map = HashMap::new();
    let mut sum = 0;
    map.insert("I", 1);
    map.insert("V", 5);
    map.insert("X", 10);
    map.insert("L", 50);
    map.insert("C", 100);
    map.insert("D", 500);
    map.insert("M", 1000);
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();
    for index in 0..length-1 {
        let current_value = map.get(&chars[index].to_string()[..]).unwrap();
        let next_value=map.get(&chars[index+1].to_string()[..]).unwrap();
        if current_value<next_value {
            sum-=current_value;
        }
        else {
            sum+=current_value;
        }

        
    }
    sum+=map.get(&chars[length-1].to_string()[..]).unwrap();
    return sum;
}
