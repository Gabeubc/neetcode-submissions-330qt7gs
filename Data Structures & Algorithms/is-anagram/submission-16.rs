impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false;
        }
        let mut freqs:HashMap<char, i32>=HashMap::new();
        for c in s.chars(){
            *freqs.entry(c).or_insert(0)+=1;
        }
        for c in t.chars(){
            if let Some(freq)=freqs.get(&c){
                freqs.insert(c, freq-1);
            }else{
                return false;
            }
        }
        for (k, v) in freqs{
            if v!=0{
                return false;
            }
        }
        return true;
    }
}
