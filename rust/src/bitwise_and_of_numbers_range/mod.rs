pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
    let mut cnt: i32 = 0;
    
    while left != right {
        left >>= 1;
        right >>= 1;
        cnt += 1;
    }
    
    left<<cnt
}
pub fn range_bitwise_and_bit_by_bit(mut left: i32, right: i32) -> i32 {
    let mut res = 0;
    let mut prev_cur_change_interval = 0;
    let mut cur_change_interval = 2;
    let diff = (right - left) + 1;

    for index in 0..32 {
        let mut cur_diff = diff;
        if diff > prev_cur_change_interval {
            cur_diff += prev_cur_change_interval
        }
        if cur_diff < cur_change_interval {
            let bit = (left >> index & right >> index) & 1 << index;
            res |= bit;
        }
        prev_cur_change_interval = cur_change_interval;
        cur_change_interval = cur_change_interval.checked_mul(2).unwrap_or(i32::MAX);
    }
    res
}
#[cfg(test)]
mod test {
    use crate::bitwise_and_of_numbers_range::range_bitwise_and;

    #[test]
    fn base_case() {
        assert_eq!(range_bitwise_and(5, 7), 4);
    }
    #[test]
    fn interval() {
        assert_eq!(range_bitwise_and(3, 6), 0);
    }
    #[test]
    fn interval_ii() {
        assert_eq!(range_bitwise_and(6, 12), 0);
    }
}
