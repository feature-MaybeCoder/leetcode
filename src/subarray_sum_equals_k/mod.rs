use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix_cace: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    prefix_cace.insert(0, 1);
    let mut cur_prefix = 0;
    let mut amount = 0;

    for num in nums {
        cur_prefix += num;
        let target = cur_prefix - k;
        if prefix_cace.contains_key(&target) {
            amount += prefix_cace[&target];
        }
        prefix_cace.insert(cur_prefix, *prefix_cace.get(&cur_prefix).unwrap_or(&0) + 1);
    }

    amount
}
#[cfg(test)]
mod test {
    use super::subarray_sum;

    #[test]
    fn base_case() {
        assert_eq!(subarray_sum(vec![1, 1, 1, -1, 1], 2), 4);
    }
    #[test]
    fn base_case_ii() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    }
    #[test]
    fn zeros_case() {
        assert_eq!(subarray_sum(vec![1, 1, 0, 0, 0, 1, 1, 0, 1, 1], 2), 16);
    }
}
