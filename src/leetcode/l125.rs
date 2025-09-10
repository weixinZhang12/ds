pub fn is_palindrome(s: String) -> bool {
    let mut s = s;
    s.retain(|c| c.is_ascii_alphabetic()||c.is_ascii_digit());
    s = s.to_lowercase();
    if s.is_empty(){return true;}
    // if s.len()==1{return false;}
    s == s.chars().rev().collect::<String>()
}
