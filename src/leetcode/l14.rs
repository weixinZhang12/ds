pub fn longest_comon_prefix(strs: Vec<String>) -> String {
    let mut string = String::new();
    let _length=strs.len();
    let first_word=&strs[0];
    // index为当前正在匹配的索引
    for current_index in 0..first_word.len() {
        for word_index in 1..strs.len() {
           if current_index>=strs[word_index].len()||first_word[0..=current_index]!=strs[word_index][0..=current_index] {
               return string;
           }
        }
        // 通过添加到string中
           string=first_word[0..=current_index].to_string();
    }
    string
}
