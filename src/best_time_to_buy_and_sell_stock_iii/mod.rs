pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let target_len = len-1;
    let mut prefix_cache: Vec<i32> = vec![0; len];
    let mut postfix_cache: Vec<i32> = vec![0; len];
    let mut cur_min_peek_prefix = prices[0];
    let mut cur_max_peek_postfix = 0;
    for ((prefix_i, &prefix_price), (postfix_i, &postfix_price)) in prices
        .iter()
        .enumerate()
        .zip(prices.iter().enumerate().rev())
    {
        if prefix_i != 0 && postfix_i != target_len {
            prefix_cache[prefix_i] = prefix_cache[prefix_i-1].max(prefix_price-cur_min_peek_prefix); 
            postfix_cache[postfix_i] = postfix_cache[postfix_i+1].max(cur_max_peek_postfix-postfix_price); 
        }
        cur_min_peek_prefix = cur_min_peek_prefix.min(prefix_price);
        cur_max_peek_postfix = cur_max_peek_postfix.max(postfix_price);
    }
    let mut max = 0;
    for i in 0..len {
        let prefix = prefix_cache[i];
        let postfix = postfix_cache[i];
        let sum = prefix + postfix;
        if sum > max {
            max = sum;
        }
    }
    return max;
}
#[cfg(test)]
mod test {
    use super::max_profit;

    #[test]
    fn base_case() {
        assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6)
    }
}
