impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false;
        }
        let mut freqs:HashMap<char, i32>=HashMap::new();
        let mut s_chars=s.chars();
        let mut t_chars=t.chars();
        for c in s_chars{
            *freqs.entry(c).or_insert(0)+=1;
        }
        for c in t_chars{
            if let Some(&freq)=freqs.get(&c){
                if freq==0 {
                    return false;
                }
                freqs.insert(c, freq-1);
            }else{
                return false;
            }
        }
        return true;
    }
}
