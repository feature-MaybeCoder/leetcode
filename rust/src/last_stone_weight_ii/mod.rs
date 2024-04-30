pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let mut dp = [0i32; 101*30];
    let sum:i32 = stones.iter().sum();
    let bag_size = (sum / 2) as usize;
    for stone in stones{
        for i in (stone as usize..=bag_size as usize).rev() {
            dp[i] = dp[i].max(dp[i - stone as usize] + stone);
        }
    }
    return sum-dp[bag_size]*2
}
#[cfg(test)]
mod test {
    use super::last_stone_weight_ii;

    #[test]
    fn base_case() {
        assert_eq!(last_stone_weight_ii(vec![1,2,4,8,16,32,64,12,25,51]), 1);
    }
    #[test]
    fn large_case() {
        assert_eq!(last_stone_weight_ii(vec![1,1,2,3,5,8,13,21,34,55,89,14,23,37,61,98]), 1);
    }
}
