impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut max=0;
        let mut res:i32=0;
        let mut l:usize=0;
        let mut s_vec:Vec::<char>=s.chars().collect();
        let mut freqs:HashMap::<char, i32>=HashMap::new();
        let len=s.len();
        for r in 0..len{
            *freqs.entry(s_vec[r]).or_insert(0)+=1;
            if let Some(&freq)=freqs.get(&s_vec[r]){
                max=max.max(freq);
                while ((r-l+1) as i32)-max>k{
                    *freqs.entry(s_vec[l]).or_insert(0)-=1;
                    l+=1;
                }
                res=(r-l+1) as i32;
            }
        }
        return res;
    }
}
