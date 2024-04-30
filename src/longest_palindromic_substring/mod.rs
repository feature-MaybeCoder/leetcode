fn is_palindromic(s: &[char]) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    return true;
}
pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut right = s.len() - 1;
    while right > 0 {
        for idx in 0..(s.len() - right) {
            let split = &chars[0 + idx..=right + idx];
            if is_palindromic(split) {
                return split.iter().collect();
            }
        }
        right -= 1;
    }
    return String::new();
}
#[cfg(test)]
mod test {
    use super::longest_palindrome;

    #[test]
    fn base_case() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab");
    }
}
