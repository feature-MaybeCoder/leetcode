use std::collections::HashMap;
type Cache = HashMap<(usize,i32), i32>;
fn dfs(idx: usize, nums: &Vec<i32>, cur: i32, target: i32, bound: usize, cache: &mut Cache) -> i32 {
    let key = (idx, cur);
    if cache.contains_key(&key){
        return *cache.get(&key).unwrap();
    }
    let v1 = cur + nums[idx];
    let v2 = cur - nums[idx];
    if idx == bound {
        let mut ways = 0;
        if v1 == target {
            ways += 1;
        }
        if v2 == target {
            ways += 1;
        }
        cache.insert(key, ways);
        return ways;
    }
    let v1 = dfs(idx + 1, nums, v1, target, bound, cache);
    let v2 = dfs(idx + 1, nums, v2, target, bound, cache);
    cache.insert(key, v1+v2);
    return v1 + v2;
}
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut cache: Cache = HashMap::with_capacity(nums.len().pow(2));
    return dfs(0, &nums, 0, target, nums.len() - 1, &mut cache);
}
#[cfg(test)]
mod test {
    use super::find_target_sum_ways;

    #[test]
    fn base_case() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
}
