impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut buckets:HashMap<Vec<i32>, Vec<String>>=HashMap::new();
        for s in strs{
            buckets.entry(Self::compute_key(s.as_str())).or_insert(Vec::new()).push(s);
        }
        return buckets.into_values().collect();
    }
    fn compute_key(s: &str) -> Vec<i32>{
        let mut bucket:Vec<i32>=vec![0;26];
        let s_chars=s.chars();
        for c in s_chars{
            let pos=(c as usize)%26;
            bucket[pos]+=1;
        }
        return bucket;
    }
}
