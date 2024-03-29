type Cache = Vec<Option<Vec<Vec<i32>>>>;
fn dfs(index: usize, nums: &Vec<i32>, cache: &mut Cache, res: &mut Vec<Vec<i32>>) {
    if cache[index].is_some() {
        return;
    }
    let mut level_res: Vec<Vec<i32>> = Vec::with_capacity(2_usize.pow((nums.len()-1-index)as u32));
    level_res.push(vec![nums[index]]);
    for next_idx in index + 1..nums.len() {
        if cache[next_idx].is_some() {
            let mut arr = cache[next_idx].clone().unwrap();
            for sub_arr in &mut arr {
                sub_arr.push(nums[index]);
            }
            level_res.append(&mut arr);
        } else {
            dfs(next_idx, nums, cache, res);
            let mut arr = cache[next_idx].clone().unwrap();
            for sub_arr in &mut arr {
                sub_arr.push(nums[index]);
            }
            level_res.append(&mut arr);
        }
        
    }
    cache[index] = Some(level_res.clone());
    res.append(&mut level_res);
}
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let power:usize = 2;
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(power.pow(nums.len() as u32));
    let mut cache: Cache = vec![None; nums.len()];
    res.push(vec![]);
    dfs(0, &nums, &mut cache, &mut res);
    return res;
}

#[cfg(test)]
mod test {
    use super::subsets;

    #[test]
    fn base_case() {
        let assert_val: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(subsets(vec![1, 2, 3]), assert_val);
    }
}
