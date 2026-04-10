impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false;
        }
        let mut map_s=HashMap::new();
        let mut map_t=HashMap::new();
        for c in s.chars(){
            let count = map_s.get(&c).unwrap_or(&0);
            map_s.insert(c, count + 1); 
        }
        for c in t.chars(){
            let count = map_t.get(&c).unwrap_or(&0);
            map_t.insert(c, count + 1); 
        }
        return map_s==map_t;
    }
}
