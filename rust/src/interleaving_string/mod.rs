use std::{collections::HashMap, hash::Hash};

fn dfs(
    s1: &str,
    s2: &str,
    s3: &str,
    bytes_s1: &[u8],
    bytes_s2: &[u8],
    bytes_s3: &[u8],
    cache: &mut HashMap<(usize, usize, usize), bool>,
) -> bool {
    let s3_index = s3.len();
    let s1_index = s1.len();
    let s2_index = s2.len();
    if s3_index == 0 {
        return s1_index == 0 && s2_index == 0;
    }
    let cache_key = (s1_index, s2_index, s3_index);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    let mut res = false;
    if s1_index != 0 {
        if bytes_s1[s1_index - 1] == bytes_s3[s3_index - 1] {
            res = dfs(
                &s1[..s1_index - 1],
                s2,
                &s3[..s3_index - 1],
                bytes_s1,
                bytes_s2,
                bytes_s3,
                cache,
            );
        }
    }
    if res {
        cache.insert(cache_key, true);
        return res;
    }
    if s2_index != 0 {
        if bytes_s2[s2_index - 1] == bytes_s3[s3_index - 1] {
            res = dfs(
                s1,
                &s2[..s2_index - 1],
                &s3[..s3_index - 1],
                bytes_s1,
                bytes_s2,
                bytes_s3,
                cache,
            );
        }
    }
    cache.insert(cache_key, res);

    return res;
}
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    return dfs(
        &s1[..],
        &s2[..],
        &s3[..],
        s1.as_bytes(),
        s2.as_bytes(),
        s3.as_bytes(),
        &mut HashMap::new(),
    );
}
pub fn is_interleave_dp(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    let s1_len = s1.len();
    let s2_len = s2.len();
    let mut dp = vec![vec![false; s2_len + 1]; s1_len + 1];
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let s3_bytes = s3.as_bytes();
    dp[s1_len][s2_len] = true;
    for x in (0..s1_len + 1).rev() {
        for y in (0..s2_len + 1).rev() {
            if x < s1_len && s1_bytes[x] == s3_bytes[x+y]&& dp[x + 1][y] {
                dp[x][y] = true
            }
            if y < s2_len && s2_bytes[y] == s3_bytes[x+y]&& dp[x][y+1] {
                dp[x][y] = true
            }
            
        }
    }
    
    return dp[0][0];
}
#[cfg(test)]
mod tests {

    use crate::interleaving_string::{is_interleave, is_interleave_dp};

    #[test]
    fn basic_str_truth_test() {
        assert_eq!(
            is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbccbca".to_string()
            ),
            true
        );
        assert_eq!(
            is_interleave_dp(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbccbca".to_string()
            ),
            true
        )
    }
    #[test]
    fn basic_str_falsy_test() {
        assert_eq!(
            is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
        assert_eq!(
            is_interleave_dp(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        )
    }
    #[test]
    fn empty_string() {
        assert_eq!(
            is_interleave("".to_string(), "".to_string(), "".to_string()),
            true
        );
        assert_eq!(
            is_interleave_dp("".to_string(), "".to_string(), "".to_string()),
            true
        )
    }
    #[test]
    fn one_symbol_test_case() {
        assert_eq!(
            is_interleave("a".to_string(), "b".to_string(), "a".to_string()),
            false
        );
        assert_eq!(
            is_interleave_dp("a".to_string(), "b".to_string(), "a".to_string()),
            false
        )
    }
    #[test]
    fn runt_time_test() {
        assert_eq!(
            is_interleave(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
            ),
            false
        );
        assert_eq!(
            is_interleave_dp(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
            ),
            false
        )
    }
}
