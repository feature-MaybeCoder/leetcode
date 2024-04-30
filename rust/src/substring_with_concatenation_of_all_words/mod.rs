use std::collections::{HashMap, HashSet};
fn change_amount<'a>(slice: &'a str, map: &mut HashMap<&'a str, i32>, is_increment: bool) -> i32 {
    if is_increment {
        if map.contains_key(slice) {
            let new_val = map[slice] + 1;
            map.insert(slice, new_val);
            return new_val;
        }
        map.insert(slice, 1);
        return 1;
    }
    if map[slice] == 1 {
        map.remove(slice);
        return 0;
    }

    let new_val = map[slice] - 1;
    map.insert(slice, new_val);
    return new_val;
}
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(s.len() / words[0].len());
    let mut words_map: HashMap<&str, i32> = HashMap::with_capacity(words.len());
    let word_len = words[0].len();
    let p_len = words.len() * word_len;
    for index in 0..words.len() {
        if words_map.contains_key(&words[index][..]) {
            words_map.insert(&words[index][..], words_map[&words[index][..]] + 1);
            continue;
        }
        words_map.insert(&words[index][..], 1);
    }
    let mut added: HashSet<usize> = HashSet::with_capacity(s.len()/word_len);
    for i in 0..word_len {
        let mut start: usize = i;
        let mut end: usize = i;
        let mut words_map_cur: HashMap<&str, i32> = words_map.clone();

        while end <= s.len() {
            
            if end - start >= p_len && words_map_cur.len() == 0 {
                if !added.contains(&start){

                    res.push(start as i32);
                    added.insert(start);
                }
                change_amount(&s[start..start + word_len], &mut words_map_cur, true);
                start += word_len;
                continue;
            }
            let new_index = end + word_len;
            if new_index > s.len() {
                break;
            }

            let slice = &s[end..new_index];
            
            if !words_map.contains_key(slice) {
                while end - start >= word_len {
                    change_amount(&s[start..start + word_len], &mut words_map_cur, true);
                    start += word_len
                }
                end += word_len;
                start = end;
                continue;
            }
            if !words_map_cur.contains_key(slice) {
                change_amount(&s[start..start + word_len], &mut words_map_cur, true);
                start += word_len;
                continue;
            }
            change_amount(slice, &mut words_map_cur, false);
            end += word_len;
        }
    }

    return res;
}
#[cfg(test)]
mod test {
    use crate::substring_with_concatenation_of_all_words::find_substring;

    #[test]
    fn shuld_correctly_determine_empty_substr() {
        let words = ["word", "good", "best", "word"];
        assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            []
        );
    }
    #[test]
    fn base_case() {
        let words = ["bar", "foo", "the"];
        assert_eq!(
            find_substring(
                "barfoofoobarthefoobarman".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            [6, 9, 12]
        );
    }
    #[test]
    fn should_correctly_get_multi_intervals() {
        let words = ["bar", "foo"];
        assert_eq!(
            find_substring(
                "barfoobarfoo".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            [0, 3, 6]
        );
    }
    #[test]
    fn should_extract_odd() {
        let words = ["fooo", "barr", "wing", "ding", "wing"];
        assert_eq!(
            find_substring(
                "lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            [13]
        );
    }
    #[test]
    fn out_of_range_case() {
        let words = ["a"];
        assert_eq!(
            find_substring(
                "a".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            [0]
        );
    }
    #[test]
    fn should_correctly_get_all_permutations_on_diff_indeses() {
        let words = ["aa","aa"];
        assert_eq!(
            find_substring(
                "aaaaaaaaaaaaaa".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            [0,2,4,6,8,10,1,3,5,7,9]
        );
    }
    #[test]
    fn should_correctly_get_all_ermutations() {
        let words = ["si","is"];
        assert_eq!(
            find_substring(
                "mississippi".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            [4,1]
        );
    }
    #[test]
    fn should_correctly_track_cur_state_of_window() {
        let words = ["cb","bc"];
        assert_eq!(
            find_substring(
                "cbaccbcbbc".to_string(),
                words.iter().map(|array| array.to_string()).collect()
            ),
            [6]

        );
    }
}
