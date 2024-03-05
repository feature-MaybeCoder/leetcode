pub fn character_replacement(s: String, k: i32) -> i32 {
    let k = k as usize;
    let mut max_len = 1;
    let mut max_cap = 0;
    let mut char_to_cap: [usize; 26] = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let s = s.into_bytes();
    while right < s.len() {
        let right_char = s[right] as usize - 65;
        let left_char = s[left] as usize - 65;

        let num = char_to_cap[right_char] + 1;
        char_to_cap[right_char] = num;
        max_cap = max_cap.max(num);
        let mut cap = (right - left)+1;
        let mut cur_replacement_needed = cap - max_cap;

        while cur_replacement_needed > k {
            char_to_cap[left_char] -= 1;
            left += 1;
            cap = (right - left)+1;
            cur_replacement_needed = cap - max_cap;
        }
        right+=1;
        max_len = max_len.max(cap);
    }

    max_len as i32
}
#[cfg(test)]
mod test {
    use super::character_replacement;

    #[test]
    fn base_case() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
    }
}
