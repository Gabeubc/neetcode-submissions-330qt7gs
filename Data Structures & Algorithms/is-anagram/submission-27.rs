impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false;
        }
        let mut bucket:Vec<i32>=vec![0;26];
        let s_chars=s.chars();
        let t_chars=t.chars();
        for c in s_chars{
            let pos=(c as usize)%26;
            bucket[pos]+=1;
        }
        for c in t_chars{
            let pos=(c as usize)%26;
            if bucket[pos]==0{
                return false;
            }
            bucket[pos]-=1;
        }
        return true;
    }
}
