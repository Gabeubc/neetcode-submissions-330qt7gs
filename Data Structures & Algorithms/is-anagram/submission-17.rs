impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false;
        }
        let mut freqs=vec![0;26];
        for ascii in s.as_bytes(){
            freqs[(ascii%26) as usize]+=1;
        }
        for ascii in t.as_bytes(){
            if freqs[(ascii%26) as usize]==0{
                return false;
            }
            freqs[(ascii%26) as usize]-=1;
        }
        for ascii in s.as_bytes(){
            if freqs[(ascii%26) as usize]!=0{
                return false;
            }
        }
        return true;
    }
}
