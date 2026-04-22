impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars=s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
        return chars.clone().eq(chars.rev());
    }
}
