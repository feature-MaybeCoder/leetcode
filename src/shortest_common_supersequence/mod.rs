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
    let mut cache: Vec<Vec<Option<String>>> = vec![vec![None; width + 1]; height + 1];
    for x in 0..height {
        cache[x][width] = Some(str1[x..].iter().collect());
    }
    for y in 0..width {
        cache[height][y] = Some(str2[y..].iter().collect());
    }
    cache[height][width] = Some(String::new());

    for x in (0..height).rev() {
        for y in (0..width).rev() {
            if str1[x] == str2[y] {
                let prev = cache[x+1][y+1].take().unwrap();
                let mut str = String::with_capacity(prev.len() + 1);
                str.push(str1[x]);
                str.push_str(&prev);
                cache[x][y] = Some(str);
                continue;
            }
            let prev1 = cache[x][y+1].as_ref().unwrap();
            let prev2 = cache[x+1][y].as_ref().unwrap();
            if prev1.len() > prev2.len(){
                let mut str = String::with_capacity(prev2.len()+1);
                str.push(str1[x]);
                str.push_str(prev2);
                cache[x][y] = Some(str);
                continue;
            }
            let mut str = String::with_capacity(prev1.len()+1);
            str.push(str2[y]);
            str.push_str(prev1);
            cache[x][y] = Some(str);
        }
    }
    return cache[0][0].take().unwrap();
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
