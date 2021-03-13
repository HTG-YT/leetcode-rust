impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let mut actual = Vec::new();

        actual.append(&mut nums1);
        actual.append(&mut nums2);
        actual.sort_unstable();

        let middle = actual.len() / 2;

        if actual.len() % 2 == 0 {
            (actual[middle - 1] + actual[middle]) as f64 / 2.0
        } else {
            actual[middle] as f64
        }
    }
}
