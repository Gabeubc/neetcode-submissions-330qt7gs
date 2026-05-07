impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s_chars:Vec<char>=s.chars().collect();
        let len=s_chars.len();
        let mut set:HashSet<char>=HashSet::new();
        let mut l:usize=0;
        let mut max:i32=0;
        for r in 0..len{
            while l<r && set.contains(&s_chars[r]){
                set.remove(&s_chars[l]);
                l+=1;
            }
            max=max.max((r-l+1) as i32);
            set.insert(s_chars[r]);
        }
        return max;
    }
}
