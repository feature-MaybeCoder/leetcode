use std::cmp;

fn dfs(
    text1: &[u8],
    text2: &[u8],
    mut idx1: usize,
    mut idx2: usize,
    cache: &mut Vec<Vec<i32>>,
) -> i32 {
    if idx1 >= text1.len() || idx2 >= text2.len() {
        return 0;
    }
    if cache[idx1][idx2] != -1 {
        return cache[idx1][idx2];
    }
    if text1[idx1] == text2[idx2] {
        let res = dfs(text1, text2, idx1 + 1, idx2 + 1, cache) + 1;
        cache[idx1][idx2] = res;
        return res;
    }
    let max = cmp::max(
        dfs(text1, text2, idx1 + 1, idx2, cache),
        dfs(text1, text2, idx1, idx2 + 1, cache),
    );
    cache[idx1][idx2] = max;
    return max;
}
pub fn longest_common_subsequence_dfs(text1: String, text2: String) -> i32 {
    let text1_b = text1.as_bytes();
    let text2_b = text2.as_bytes();
    let mut cache = vec![vec![-1; text2.len()]; text1.len()];
    let res = dfs(text1_b, text2_b, 0, 0, &mut cache);
    return res;
}
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let text1_b = text1.as_bytes();
    let text2_b = text2.as_bytes();
    let height = text1.len();
    let width = text2.len();
    let mut cache = vec![vec![0; width+1]; height+1];
    for x in (0..height).rev() {
        for y in (0..width).rev() {
            if text1_b[x] == text2_b[y] {
                cache[x][y] = cache[x + 1][y + 1] +1;
                continue;
            }
            cache[x][y] = cmp::max(cache[x + 1][y], cache[x][y + 1]);
        }
    }
    return cache[0][0];
}
#[cfg(test)]
mod test {
    use super::longest_common_subsequence;

    #[test]
    fn base_case() {
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }
    #[test]
    fn one_match_case() {
        assert_eq!(
            longest_common_subsequence("bsbininm".to_string(), "jmjkbkjkv".to_string()),
            1
        );
    }
    #[test]
    fn match_case() {
        assert_eq!(
            longest_common_subsequence("abcba".to_string(), "abcbcba".to_string()),
            5
        );
    }
}
