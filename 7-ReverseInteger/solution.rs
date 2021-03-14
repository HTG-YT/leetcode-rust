impl Solution {
    pub fn reverse(x: i32) -> i64 {
        match String::from_utf8({ let string = x.to_string();let mut bytes = string.as_bytes().to_vec();bytes.reverse();if x.is_negative() { let negative_sign = bytes.pop().unwrap();bytes.insert(0, negative_sign); }bytes }).unwrap().parse() { Ok(int) => int, Err(_) => 0 }
    }
}
