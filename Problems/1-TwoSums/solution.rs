impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = std::collections::HashMap::<i32, i32>::new();

        for (index, number) in nums.iter().enumerate() {
            let difference = target - number;

            if hashmap.contains_key(&difference) {
                return vec![*(hashmap.get(&difference).unwrap()), index as i32];
            }

            hashmap.insert(*number, index as i32);
        }

        vec![-1, -1]
    }
}
