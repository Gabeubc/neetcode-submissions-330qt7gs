impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut freqs:HashMap::<Vec<i32>, Vec<String>>=HashMap::new();
        for s in strs{
            freqs.entry(Self::compute_k(&s)).or_insert(Vec::new()).push(s);
        }
        return freqs.into_values().collect();
    }
    pub fn compute_k(s:&str)->Vec<i32>{
        let mut res=vec![0;26];
        let s_bytes=s.as_bytes();
        for &pos in s_bytes{
            res[(pos%26) as usize]+=1;
        }
        return res;
    }

}
