fn dfs(s_idx: usize, t_idx: usize, s: &[u8], t: &[u8], cache: &mut Vec<Vec<i32>>) -> i32 {
    if t_idx >= t.len() {
        return 1;
    }
    if s_idx >= s.len() {
        return 0;
    }
    if cache[s_idx][t_idx] != -1 {
        return cache[s_idx][t_idx];
    }
    let mut sum = dfs(s_idx + 1, t_idx, s, t, cache);
    if s[s_idx] == t[t_idx] {
        sum += dfs(s_idx + 1, t_idx + 1, s, t, cache);
    }
    cache[s_idx][t_idx] = sum;
    return sum;
}
pub fn num_distinct_dfs(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut dp = vec![vec![-1; t.len()]; s.len()];
    dfs(0, 0, s, t, &mut dp)
}
pub fn num_distinct(s: String, t: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let height = s.len();
    let width = t.len();
    let mut dp = vec![vec![0; width + 1]; height + 1];
    for x in 0..=height {
        dp[x][width] = 1;
    }
    for x in (0..height).rev() {
        for y in (0..width).rev() {
            if s[x] == t[y] {
                dp[x][y] = dp[x + 1][y] + dp[x + 1][y+1];
            }else{
                dp[x][y] = dp[x + 1][y];
            }
        }
    }
    return dp[0][0];
}
#[cfg(test)]
mod test {
    use super::num_distinct;

    #[test]
    fn base_case() {
        assert_eq!(num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    }

    #[test]
    fn repeat_case() {
        assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }

    #[test]
    fn double_match_case() {
        assert_eq!(num_distinct("bbaa".to_string(), "ba".to_string()), 4);
    }
}
