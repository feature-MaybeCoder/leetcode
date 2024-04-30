use std::collections::HashMap;
type Cache = HashMap<(usize, i32), i32>;
fn dfs(idx: usize, nums: &Vec<i32>, cur: i32, target: i32, bound: usize, cache: &mut Cache) -> i32 {
    let key = (idx, cur);
    if cache.contains_key(&key) {
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
    cache.insert(key, v1 + v2);
    return v1 + v2;
}
pub fn find_target_sum_ways_dfs(nums: Vec<i32>, target: i32) -> i32 {
    let mut cache: Cache = HashMap::with_capacity(nums.len().pow(2));
    return dfs(0, &nums, 0, target, nums.len() - 1, &mut cache);
}
pub fn find_target_sum_ways_map(nums: Vec<i32>, target: i32) -> i32 {
    // use hashmap for iteration compression
    let mut items: HashMap<i32, i32> = HashMap::with_capacity(nums.len().pow(2));
    // insert base case
    items.insert(0, 1);
    for num in nums {
        let mut cur = HashMap::new();
        for (key, amount) in items.into_iter() {
            // icrement two possible values, use prev amount because on prev itter step we could had more then 1 of this value
            *cur.entry(key + num).or_insert(0) += amount;
            *cur.entry(key - num).or_insert(0) += amount;
        }

        items = cur;
    }
    return *items.get(&target).unwrap_or(&0i32);
}
pub fn find_target_sum_ways_dp(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    // keep track of two state, current and prev
    // use 2001 cause of constrains of the problem (values in nums are in rage from -1000 to 1000 + 0 value )
    let mut state = vec![vec![0; 2001]; 2];
    let mut idx = 0;
    // set 0 as base_case in the middle
    state[idx][1000] = 1;
    for num in nums {
        let opposite = 1 - idx;
        for state_idx in 0..2000 {
            // iterate over all values from prev iteration step
            let val = state[idx][state_idx];
            // reset prev iter step value, so on next 2nth we will get empty state
            state[idx][state_idx] = 0;
            if val == 0 {
                continue;
            }
            // increment current state
            state[opposite][state_idx + num as usize] += val;
            state[opposite][state_idx - num as usize] += val;
        }
        // oposite state as current state
        idx = opposite;
    }
    // target in range -1000 to 1000, so target is -1000 we whant to get first item from vec
    return state[idx][target + 1000];
}
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    
    return 0
}
#[cfg(test)]
mod test {
    use super::find_target_sum_ways;

    #[test]
    fn base_case() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
}
