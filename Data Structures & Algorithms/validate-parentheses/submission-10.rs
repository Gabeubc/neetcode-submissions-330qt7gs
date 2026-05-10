impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack:Vec<char>=Vec::new();
        let mut s_chars:Vec<char>=s.chars().collect();
        for c in s_chars{
            match c {
                ')' => if stack.pop()!=Some('(') { return false },
                '}' => if stack.pop()!=Some('{') { return false },
                ']' => if stack.pop()!=Some('[') { return false },
                _ => stack.push(c)
            }
        }
        return stack.len()==0;
    }
}
