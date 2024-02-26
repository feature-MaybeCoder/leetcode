pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let k = k as usize * 2;
    let mut transactions = vec![0; k];
    for index in 0..k {
        if index % 2 == 0 {
            transactions[index] = i32::MIN;
        }
    }
    for price_index in 0..len {
        let price = prices[price_index];
        for day_index in 0..k {
            if day_index == 0 {
                transactions[day_index] = transactions[day_index].max(-price);
                continue;
            }
            let module = day_index % 2;
            if module == 0 {
                transactions[day_index] =
                    transactions[day_index].max(transactions[day_index - 1] - price);
            } else {
                transactions[day_index] =
                    transactions[day_index].max(transactions[day_index - 1] + price);
            }
        }
    }

    return transactions.pop().unwrap();
}
#[cfg(test)]
mod test {
    use super::max_profit;

    #[test]
    fn max_amount_of_transactions() {
        assert_eq!(max_profit(20, vec![0, 3, 1, 4, 0, 5]), 11);
    }
    #[test]
    fn shrink() {
        assert_eq!(max_profit(2, vec![0, 3, 1, 4, 0, 5]), 9);
    }
}
