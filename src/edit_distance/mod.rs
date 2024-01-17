use std::collections::HashMap;

pub fn min_distance_dfs(word1: String, word2: String) -> i32 {
    fn dfs(slice1: &[u8], slice2: &[u8], cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        let mut last1 = slice1.len();
        let mut last2 = slice2.len();
        if last2 == 0 && last1 == 0 {
            return 0;
        }
        if last1 == 0 || last2 == 0 {
            return last1.max(last2) as i32;
        }
        last1 -= 1;
        last2 -= 1;
        let cache_key = (last1, last2);
        if cache.contains_key(&cache_key) {
            return *cache.get(&cache_key).unwrap();
        }
        if slice1[last1] == slice2[last2] {
            return dfs(&slice1[0..last1], &slice2[0..last2], cache);
        }
        let mut min = std::cmp::min(
            dfs(&slice1[0..last1], slice2, cache),
            dfs(slice1, &slice2[0..last2], cache),
        );
        min = min.min(dfs(&slice1[0..last1], &slice2[0..last2], cache));
        cache.insert(cache_key, min + 1);
        return min + 1;
    }

    return dfs(word1.as_bytes(), word2.as_bytes(), &mut HashMap::new());
}
pub fn min_distance(word1: String, word2: String) -> i32 {
    let first_len = word1.len();
    let second_len = word2.len();

    if first_len == 0 && second_len == 0 {
        return 0;
    }
    if first_len == 0 || second_len == 0 {
        return first_len.max(second_len) as i32;
    }

    let mut dp = vec![vec![0; second_len + 1]; first_len + 1];

    for x in 0..first_len {
        dp[x][second_len] = first_len as i32 - x as i32;
    }
    for y in 0..second_len {
        dp[first_len][y] = second_len as i32 - y as i32;
    }
    dp[first_len][second_len] = 0;

    let word1_b = word1.as_bytes();
    let word2_b = word2.as_bytes();
    for x in (0..first_len).rev() {
        for y in (0..second_len).rev() {
            let first_char = word1_b[x];
            let second_char = word2_b[y];

            if first_char == second_char {
                dp[x][y] = dp[x + 1][y + 1];
                continue;
            }   
            dp[x][y] = std::cmp::min(dp[x + 1][y], std::cmp::min(dp[x][y + 1], dp[x + 1][y + 1])) + 1;
        }
    }
    return dp[0][0];
}

#[cfg(test)]
mod test {
    use crate::edit_distance::{min_distance, min_distance_dfs};
    #[test]
    fn basic_strings() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(min_distance_dfs("horse".to_string(), "ros".to_string()), 3);
    }
    #[test]
    fn removal_string() {
        assert_eq!(min_distance("hors".to_string(), "hos".to_string()), 1);
        assert_eq!(min_distance_dfs("hors".to_string(), "hos".to_string()), 1);
    }
    #[test]
    fn example_2_string() {
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );
        assert_eq!(
            min_distance_dfs("intention".to_string(), "execution".to_string()),
            5
        );
    }
    #[test]
    fn empty_string() {
        assert_eq!(min_distance("".to_string(), "".to_string()), 0);
        assert_eq!(min_distance_dfs("".to_string(), "".to_string()), 0);
    }
    #[test]
    fn empty_and_string() {
        assert_eq!(min_distance("some".to_string(), "".to_string()), 4);
        assert_eq!(min_distance_dfs("some".to_string(), "".to_string()), 4);
    }
    #[test]
    fn edge_case() {
        assert_eq!(min_distance("sea".to_string(), "eat".to_string()), 2);
        assert_eq!(min_distance_dfs("sea".to_string(), "eat".to_string()), 2);
    }
    #[test]
    fn edge_case_2() {
        assert_eq!(
            min_distance(
                "zoologicoarchaeologist".to_string(),
                "zoogeologist".to_string()
            ),
            10
        );
        assert_eq!(
            min_distance_dfs(
                "zoologicoarchaeologist".to_string(),
                "zoogeologist".to_string()
            ),
            10
        );
    }
}
