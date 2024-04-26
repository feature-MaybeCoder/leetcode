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
        let mut res = dfs(idx_1 + 1, idx_2 + 1, str1, str2, cache);
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
pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let str1: Vec<_> = str1.chars().collect();
    let str2: Vec<_> = str2.chars().collect();
    let mut cache: Cache = vec![vec![None; str2.len()]; str1.len()];
    let res = dfs(0, 0, &str1, &str2, &mut cache);
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
}
