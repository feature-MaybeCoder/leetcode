type Cache = Vec<Vec<Option<String>>>;
fn dfs(
    idx_1: usize,
    idx_2: usize,
    str1: &Vec<char>,
    str2: &Vec<char>,
    cache: &mut Cache,
) -> String {
    if idx_1 >= str1.len() {
        return str2[idx_2..str2.len()].iter().collect();
    }
    if idx_2 >= str2.len() {
        return str1[idx_1..str1.len()].iter().collect();
    }
    if cache[idx_1][idx_2].is_some() {
        return cache[idx_1][idx_2].take().unwrap();
    }
    if str1[idx_1] == str2[idx_2] {
        let res = dfs(idx_1 + 1, idx_2 + 1, str1, str2, cache);
        let mut str = String::with_capacity(res.len() + 1);
        str.push(str1[idx_1]);
        str.push_str(res.as_str());
        cache[idx_1][idx_2] = Some(str.clone());
        return str;
    }
    let res_1 = dfs(idx_1, idx_2 + 1, str1, str2, cache);
    let res_2 = dfs(idx_1 + 1, idx_2, str1, str2, cache);
    if res_1.len() > res_2.len() {
        let mut str = String::with_capacity(res_2.len() + 1);
        str.push(str1[idx_1]);
        str.push_str(&res_2);
        cache[idx_1][idx_2] = Some(str.clone());
        return str;
    }
    let mut str = String::with_capacity(res_1.len() + 1);
    str.push(str2[idx_2]);
    str.push_str(&res_1);
    cache[idx_1][idx_2] = Some(str.clone());
    return str;
}
pub fn shortest_common_supersequence_dfs(str1: String, str2: String) -> String {
    let str1: Vec<_> = str1.chars().collect();
    let str2: Vec<_> = str2.chars().collect();
    let mut cache: Cache = vec![vec![None; str2.len()]; str1.len()];
    let res = dfs(0, 0, &str1, &str2, &mut cache);
    return res;
}
pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let str1: Vec<_> = str1.chars().collect();
    let str2: Vec<_> = str2.chars().collect();
    let height = str1.len();
    let width = str2.len();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; width + 1]; height + 1];
    for y in 0..width {
        dp[height][y] = width - y;
    }
    for x in 0..height {
        dp[x][width] = height - x;
    }
    for x in (0..height).rev() {
        for y in (0..width).rev() {
            if str1[x] == str2[y] {
                dp[x][y] = dp[x + 1][y + 1] + 1;
                continue;
            }
            dp[x][y] = dp[x][y + 1].min(dp[x + 1][y]) + 1;
        }
    }
    let mut res = String::new();
    let mut x = 0;
    let mut y = 0;
    while x < height && y < width {
        if str1[x] == str2[y] {
            res.push(str1[x]);
            x += 1;
            y += 1;
            continue;
        }
        if dp[x][y + 1] > dp[x + 1][y] {
            res.push(str1[x]);
            x += 1;
            continue;
        }
        res.push(str2[y]);
        y += 1;
    }
    while x < height {
        res.push(str1[x]);
        x += 1;
    }
    while y < width {
        res.push(str2[y]);
        y += 1;
    }

    return res;
}
#[cfg(test)]
mod test {
    use super::shortest_common_supersequence;

    #[test]
    fn base_case() {
        assert_eq!(
            shortest_common_supersequence("abac".to_string(), "cab".to_string()),
            "cabac"
        );
    }
    #[test]
    fn common_start_diff_end() {
        assert_eq!(
            shortest_common_supersequence("trpabacrand".to_string(), "trpcabtrp".to_string()),
            "trpcabtacrapnd"
        );
    }
    #[test]
    fn revert_case() {
        assert_eq!(
            shortest_common_supersequence("baaacbcbc".to_string(), "bacbcaca".to_string()),
            "baaacbcbaca"
        );
    }
}
