impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        x.to_string() == String::from_utf8({ let string = x.to_string(); let mut bytes = string.as_bytes().to_vec(); bytes.reverse(); bytes }).unwrap()
    }
}
