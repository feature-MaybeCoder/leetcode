pub fn find_relative_ranks(mut score: Vec<i32>) -> Vec<String> {
    let len = score.len();
    let mut res = vec![String::new(); len];
    let mut score: Vec<(usize,i32)> = score.into_iter().enumerate().collect();
    score.sort_by_key(|item|-item.1);
    let custom: [&str; 3] = ["Gold Medal", "Silver Medal","Bronze Medal"];
    for (idx, (or_idx, _)) in score.into_iter().enumerate(){
        if idx < 3{
            res[or_idx] = custom[idx].to_string();
            continue;
        }
        res[or_idx] = (idx+1).to_string();
    }
    
    return res
}
#[cfg(test)]
mod test {
    use super::find_relative_ranks;

    #[test]
    fn base_case() {
        assert_eq!(find_relative_ranks(vec![10,3,8,9,4]),["Gold Medal","5","Bronze Medal","Silver Medal","4"]);
    }
}
