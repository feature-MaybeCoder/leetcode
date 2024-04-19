pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let coins: Vec<usize> = coins.into_iter().filter(|item| *item <= amount).map(|item|item as usize).collect();
    let mut cache = [0usize; 5001];
    cache[0] = 1;
    for coin in coins {
        for i in 1..amount as usize + 1 {
            let diff = i.checked_sub(coin);
            if diff.is_some(){
                cache[i] = cache[i] + cache[diff.unwrap()];
            }
        }
    }
    return cache[amount as usize] as i32;
}
#[cfg(test)]
mod test {
    use super::change;

    #[test]
    fn base_case() {
        assert_eq!(change(5, vec![1, 2, 5]), 4);
    }
    #[test]
    fn same_path_case() {
        assert_eq!(change(6, vec![1, 2, 3]), 7);
    }
    #[test]
    fn large_case() {
        assert_eq!(change(10, vec![1, 2, 5]), 10);
    }
}
