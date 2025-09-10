pub fn length_of_last_word(s: String) -> i32 {
    let s_array: Vec<char> = s.chars().collect();
    let length = s_array.len();
    let mut index = length-1;
    let mut sum = 0;
    while s_array[index]==' '  {
        index-=1;
    }
    while s_array[index] != ' '{
        sum += 1;
        if index==0 {
            return sum;
        }
        index -= 1;
    }
    sum
}
