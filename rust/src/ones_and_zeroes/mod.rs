use std::collections::HashMap;

type Cache = HashMap<(i32, i32, usize), i32>;
fn count_ones_zeros(str: &str) -> (i32, i32) {
    let mut ones = 0;
    let mut zeros = 0;
    for char in str.chars() {
        if char == '1' {
            ones += 1;
            continue;
        }
        zeros += 1;
    }
    return (ones, zeros);
}
fn dfs(
    idx: usize,
    zeros: i32,
    ones: i32,
    cur_zeros: i32,
    cur_ones: i32,
    cache: &mut Cache,
    strs: &Vec<String>,
) -> i32 {
    if idx == strs.len() || cur_zeros > zeros || cur_ones > ones {
        return 0;
    }
    let key = (cur_ones, cur_zeros, idx);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let (str_ones, str_zeros) = count_ones_zeros(&strs[idx]);
    let new_zeros = str_zeros + cur_zeros;
    let new_ones = str_ones + cur_ones;

    let mut max = 0;
    if new_zeros <= zeros && new_ones <= ones {
        max = max.max(dfs(idx + 1, zeros, ones, new_zeros, new_ones, cache, strs) + 1);
    }

    max = max.max(dfs(idx + 1, zeros, ones, cur_zeros, cur_ones, cache, strs));
    cache.insert(key, max);
    return max;
}
pub fn find_max_form_dfs(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let len = strs.len();
    let mut dfs_cache: Cache = HashMap::with_capacity(len);
    return dfs(0, m, n, 0, 0, &mut dfs_cache, &strs);
}
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let m_u = m as usize;
    let n_u = n as usize;
    let len = strs.len();
    let mut dp = vec![vec![vec![0; n_u as usize + 1]; m_u as usize + 1]; len + 1];
    for str_dx in 1..=len {
        let (ones, zeros) = count_ones_zeros(&strs[str_dx - 1]);
        for m_idx in 0..=m_u {
            for n_idx in 0..=n_u {
                let m_diff = m_idx as i32 - zeros;
                let n_diff = n_idx as i32 - ones;
                let mut max = dp[str_dx - 1][m_idx][n_idx];
                if m_diff >= 0 && n_diff >= 0 {
                    max = max.max(dp[str_dx - 1][m_diff as usize][n_diff as usize] + 1);
                }
                dp[str_dx][m_idx][n_idx] = max;
            }
        }
    }
    return dp[len][m_u][n_u];
}
#[cfg(test)]
mod test {
    use super::find_max_form;

    #[test]
    fn base_case() {
        let sample = ["10", "0001", "111001", "1", "0"];

        assert_eq!(
            find_max_form(sample.iter().map(|str| str.to_string()).collect(), 5, 3),
            4
        );
    }
    #[test]
    fn jump_case() {
        let sample = ["10", "0001", "111001", "1", "0"];

        assert_eq!(
            find_max_form(sample.iter().map(|str| str.to_string()).collect(), 4, 3),
            3
        );
    }
    #[test]
    fn min_case() {
        let sample = ["10", "0", "1"];

        assert_eq!(
            find_max_form(sample.iter().map(|str| str.to_string()).collect(), 1, 1),
            2
        );
    }
}
