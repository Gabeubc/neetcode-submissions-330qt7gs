impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set=HashSet::new();
        for &num in &nums{
            set.insert(num);
        }
        return set.len()!=nums.len();
    }
}
