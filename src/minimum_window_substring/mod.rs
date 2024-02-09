use std::{
    collections::{HashMap, HashSet},
    net,
};
fn change_amount(slice: u8, map: &mut HashMap<u8, i32>, is_increment: bool) -> i32 {
    if is_increment {
        if map.contains_key(&slice) {
            let new_val = map[&slice] + 1;
            map.insert(slice, new_val);
            return new_val;
        }
        map.insert(slice, 1);
        return 1;
    }
    if !map.contains_key(&slice) {
        return 0;
    }
    if map[&slice] == 0 {
        return 0;
    }

    let new_val = map[&slice] - 1;
    map.insert(slice, new_val);
    return new_val;
}
pub fn min_window(s: String, t: String) -> String {
    let s_b = s.as_bytes();
    let t_b = t.as_bytes();
    let mut keyes: HashMap<u8, i32> = HashMap::with_capacity(t.len());
    for index in 0..t_b.len() {
        change_amount(t_b[index], &mut keyes, true);
    }
    let mut counter: HashMap<u8, i32> = HashMap::with_capacity(t.len());
    let mut left = 0;

    let mut min_left = 0;
    let mut min_right = s_b.len();
    let mut res_len = usize::MAX;
    let mut matched = 0;
    let mut neeed = keyes.len();
    for right in 0..s.len() {
        let b = s_b[right];
        if !keyes.contains_key(&b) {
            continue;
        }
        change_amount(b, &mut counter, true);

        if counter[&b] == keyes[&b] {
            matched += 1;
        }
        while matched == neeed {
            if (right - left )+ 1 < res_len {
                min_left = left;
                min_right = right;
                res_len = (right-left)+1
            }
            if !keyes.contains_key(&s_b[left]) {
                left+=1;
                continue;
            }
            change_amount(s_b[left], &mut counter, false);
            if counter[&s_b[left]] < keyes[&s_b[left]] {
                matched -= 1;
            }
            left+=1;
        }
    }

    if res_len == usize::MAX {
        return String::new();
    }
    return (&s[min_left..min_left+res_len]).to_string();
}
#[cfg(test)]
mod test {
    use crate::minimum_window_substring::min_window;

    #[test]
    fn base_case() {
        assert_eq!(
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
    }
    #[test]
    fn one_char_case() {
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
    }
    #[test]
    fn empty_case() {
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
    }
    #[test]
    fn empty_case_2() {
        assert_eq!(min_window("a".to_string(), "b".to_string()), "");
    }
    #[test]
    fn substring_inside_substring_case() {
        assert_eq!(
            min_window(
                "ASOWKFEPFOKWEOABCPFKEPOWKFWEPOBC".to_string(),
                "ABC".to_string()
            ),
            "ABC"
        );
    }
    #[test]
    fn left_window_case() {
        assert_eq!(min_window("ab".to_string(), "a".to_string()), "a");
    }
    #[test]
    fn substring_starts_in_the_end_of_another_substr() {
        assert_eq!(min_window("bdab".to_string(), "ab".to_string()), "ab");
    }
    #[test]
    fn substr_at_the_end() {
        assert_eq!(
            min_window("cabefgecdaecf".to_string(), "cae".to_string()),
            "aec"
        );
    }
    #[test]
    fn two_substr_at_the_end() {
        assert_eq!(
            min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string()),
            "cwae"
        );
    }
}
