impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s:String=s.to_lowercase();
        let mut l=0;
        let s_chars:Vec<char>=s.chars().collect();
        let mut r=s_chars.len()-1;
        while l<r {
            while l<r && !s_chars[l].is_alphanumeric(){
                l+=1;
            }
            while l<r && !s_chars[r].is_alphanumeric(){
                r-=1;
            }
            if l<r{
                if s_chars[l]!=s_chars[r]{
                    return false;
                }
            }else{
                break;
            }
            l+=1;
            r-=1;
        }
        return true;
    }
}
