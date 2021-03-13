impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars = Vec::new();
        let mut longest = 0;

        s.chars().for_each(|character| {
            if chars.contains(&character) {
                longest = std::cmp::max(longest, chars.len());
                chars = chars[(chars.iter().position(|c| *c == character).unwrap() + 1)..].to_vec();
            }

            chars.push(character);
        });

        std::cmp::max(longest as i32, chars.len() as i32)
    }
}
