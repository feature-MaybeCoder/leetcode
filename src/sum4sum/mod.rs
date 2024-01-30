fn move_out_duplicates(mut index: usize, arr: &Vec<i32>, is_incriment: bool) -> usize {
    let mut prev = index;
    if is_incriment {
        index += 1;
    } else {
        index -= 1;
    }
    if is_incriment {
        while index < arr.len() && arr[prev] == arr[index] {
            prev = index;
            index += 1;
        }
    } else {
        while index > 0 && arr[prev] == arr[index] {
            prev = index;
            index -= 1;
        }
    }
    return index;
}
fn find_two_sum(
    start_index: usize,
    nums: &Vec<i32>,
    target: i32,
    sum: i32,
    cur_res: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    let mut left = start_index;
    let mut right = nums.len() - 1;
    let local_target = target.checked_sub(sum);
    if local_target.is_none(){
        return
    }
    let local_target = local_target.unwrap();
    while left < right {
        let left_val = nums[left];
        let right_val = nums[right];
        let local_sum = left_val + right_val;
        if local_sum == local_target {
            cur_res.push(left_val);
            cur_res.push(right_val);
            res.push(cur_res.clone());
            cur_res.pop();
            cur_res.pop();
            right = move_out_duplicates(right, nums, false);
            left = move_out_duplicates(left, nums, true);
            continue;
        }
        if local_sum > local_target {
            right = move_out_duplicates(right, nums, false);
            continue;
        }
        left = move_out_duplicates(left, nums, true);
    }
}
fn dfs(
    mut index: usize,
    nums: &Vec<i32>,
    target: i32,
    sum: i32,
    cur_res: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    let len_right = nums.len() as i32 - (index as i32);
    let needed_len = 4 - cur_res.len() as i32;

    if needed_len > len_right || i32::MAX < sum {
        return;
    }

    if cur_res.len() == 2 {
        find_two_sum(index, nums, target, sum, cur_res, res);
        return;
    }
    while index < nums.len() {
        cur_res.push(nums[index]);
        let next_sum = sum.checked_add(nums[index]);
        if next_sum.is_some() {
            dfs(index + 1, nums, target, next_sum.unwrap(), cur_res, res);
        }

        cur_res.pop();
        index = move_out_duplicates(index, nums, true);
    }
}
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort();
    let mut res: Vec<Vec<i32>> = Vec::new();

    dfs(0, &nums, target, 0, &mut Vec::<i32>::new(), &mut res);
    return res;
}
#[cfg(test)]
mod test {
    use crate::sum4sum::four_sum;

    #[test]
    fn basic_case() {
        let sample_data = [1, 0, -1, 0, -2, 2];
        assert_eq!(
            four_sum(sample_data.iter().copied().collect(), 0),
            [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
    }
    #[test]
    fn duplicates_edge_case() {
        let sample_data = [2, 2, 2, 2, 2, 2, 2, 2];
        assert_eq!(
            four_sum(sample_data.iter().copied().collect(), 8),
            [[2, 2, 2, 2]]
        );
    }
    #[test]
    fn edge_case() {
        let sample_data = [-3, -2, -1, 0, 0, 1, 2, 3];
        assert_eq!(
            four_sum(sample_data.iter().copied().collect(), 0),
            [
                [-3, -2, 2, 3],
                [-3, -1, 1, 3],
                [-3, 0, 0, 3],
                [-3, 0, 1, 2],
                [-2, -1, 0, 3],
                [-2, -1, 1, 2],
                [-2, 0, 0, 2],
                [-1, 0, 0, 1]
            ]
        );
    }
    #[test]
    fn overflow_test_case() {
        let sample_data = [1000000000, 1000000000, 1000000000, 1000000000];
        let return_assert: Vec<Vec<i32>> = vec![];
        assert_eq!(
            four_sum(sample_data.iter().copied().collect(), -294967296),
            return_assert
        );
    }
}
