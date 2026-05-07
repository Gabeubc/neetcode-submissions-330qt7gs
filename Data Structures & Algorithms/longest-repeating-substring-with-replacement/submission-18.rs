impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s_chars:Vec<char>=s.chars().collect();
        let len=s_chars.len();
        let mut freqs:HashMap<char,usize>=HashMap::new();
        let mut l=0;
        let mut max:usize=0;
        let mut res:i32=0;
        for r in 0..len{
            *freqs.entry(s_chars[r]).or_insert(0)+=1;
            let freq=*freqs.get(&s_chars[r]).unwrap();
            max=max.max(freq);
            while l<r && (r-l+1)-max>(k as usize) {
                *freqs.entry(s_chars[l]).or_insert(0)-=1;
                l+=1;
            }
            res=(r-l+1) as i32;
        }
        return res;
    }
}
